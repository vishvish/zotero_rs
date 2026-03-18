//! Sync conflict retry helpers for 412 responses.

use std::future::Future;
use std::time::Duration;

use crate::client::config::RetryPolicy;
use crate::client::retry_policy::compute_exponential_delay;
use crate::client::{ZoteroClient, ZoteroClientError};

pub(crate) fn compute_sync_retry_delay(attempt_index: u32, policy: RetryPolicy) -> Duration {
    compute_exponential_delay(attempt_index, policy)
}

impl ZoteroClient {
    /// Runs an operation and retries when Zotero returns `412 Precondition Failed`.
    pub async fn run_with_sync_conflict_retry<T, F, Fut>(
        &self,
        mut operation: F,
    ) -> Result<T, ZoteroClientError>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, ZoteroClientError>>,
    {
        let attempts = self.options.retry_policy.max_attempts.max(1);

        for attempt in 0..attempts {
            match operation().await {
                Ok(value) => return Ok(value),
                Err(ZoteroClientError::PreconditionFailed { .. }) if attempt + 1 < attempts => {
                    let delay = compute_sync_retry_delay(attempt, self.options.retry_policy);
                    std::thread::sleep(delay);
                    continue;
                }
                Err(error) => return Err(error),
            }
        }

        Err(ZoteroClientError::HttpStatus {
            status: reqwest::StatusCode::PRECONDITION_FAILED,
            body: "sync conflict retry attempts exhausted"
                .to_owned()
                .into_boxed_str(),
            metadata: Box::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    use crate::client::config::RetryPolicy;
    use crate::client::{ClientOptions, ZoteroClient, ZoteroClientError};
    use crate::responses::response_metadata::ResponseMetadata;

    #[tokio::test]
    async fn retries_precondition_failed_until_success() {
        let client = ZoteroClient::new(ClientOptions {
            retry_policy: RetryPolicy {
                max_attempts: 3,
                base_delay: Duration::from_millis(1),
                max_delay: Duration::from_millis(2),
            },
            ..ClientOptions::default()
        })
        .expect("client");

        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_ref = Arc::clone(&attempts);

        let result = client
            .run_with_sync_conflict_retry(|| {
                let attempts_inner = Arc::clone(&attempts_ref);
                async move {
                    let current = attempts_inner.fetch_add(1, Ordering::SeqCst) + 1;
                    if current < 3 {
                        Err(ZoteroClientError::PreconditionFailed {
                            metadata: Box::new(ResponseMetadata::default()),
                        })
                    } else {
                        Ok(42)
                    }
                }
            })
            .await
            .expect("eventually succeeds");

        assert_eq!(result, 42);
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn computes_increasing_sync_delays() {
        let policy = RetryPolicy {
            max_attempts: 3,
            base_delay: Duration::from_millis(50),
            max_delay: Duration::from_millis(300),
        };

        let d0 = crate::client::sync_conflict_retry::compute_sync_retry_delay(0, policy);
        let d1 = crate::client::sync_conflict_retry::compute_sync_retry_delay(1, policy);
        assert!(d1 > d0);
    }
}

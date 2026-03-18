//! Retry policy helpers for transient errors.

use std::time::Duration;

use reqwest::StatusCode;

use crate::client::config::RetryPolicy;
use crate::responses::response_metadata::ResponseMetadata;

pub(crate) fn should_retry_response(is_safe_retry: bool, status: StatusCode) -> bool {
    is_safe_retry
        && (status == StatusCode::TOO_MANY_REQUESTS || status == StatusCode::SERVICE_UNAVAILABLE)
}

pub(crate) fn should_retry_transport_error(is_safe_retry: bool) -> bool {
    is_safe_retry
}

pub(crate) fn compute_retry_delay(
    attempt_index: u32,
    metadata: &ResponseMetadata,
    policy: RetryPolicy,
) -> Duration {
    let header_delay_seconds = metadata
        .retry_after_seconds
        .or(metadata.backoff_seconds)
        .unwrap_or(0);

    if header_delay_seconds > 0 {
        return Duration::from_secs(header_delay_seconds);
    }

    compute_exponential_delay(attempt_index, policy)
}

pub(crate) fn compute_exponential_delay(attempt_index: u32, policy: RetryPolicy) -> Duration {
    let shift = attempt_index.min(20);
    let factor = 1u32 << shift;
    let base = policy.base_delay.saturating_mul(factor);
    std::cmp::min(base, policy.max_delay)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use reqwest::StatusCode;

    use crate::client::config::RetryPolicy;
    use crate::client::retry_policy::{compute_retry_delay, should_retry_response};
    use crate::responses::response_metadata::ResponseMetadata;

    #[test]
    fn uses_retry_after_or_backoff_before_exponential() {
        let metadata = ResponseMetadata {
            retry_after_seconds: Some(7),
            backoff_seconds: Some(2),
            ..ResponseMetadata::default()
        };

        let policy = RetryPolicy {
            max_attempts: 3,
            base_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(10),
        };

        let delay = compute_retry_delay(0, &metadata, policy);
        assert_eq!(delay, Duration::from_secs(7));
    }

    #[test]
    fn retries_get_for_429_and_503() {
        assert!(should_retry_response(true, StatusCode::TOO_MANY_REQUESTS));
        assert!(should_retry_response(true, StatusCode::SERVICE_UNAVAILABLE));
        assert!(!should_retry_response(false, StatusCode::TOO_MANY_REQUESTS));
    }
}

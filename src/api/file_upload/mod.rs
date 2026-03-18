//! File upload endpoint operations.

mod authorize_file_upload;
mod compose_upload_body;
mod determine_upload_action;
mod register_file_upload;
mod upload_binary;
mod upload_item_file;

#[cfg(test)]
mod upload_binary_tests;

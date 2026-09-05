//! Successful structured-result size enforcement.

use std::num::NonZeroUsize;

use rmcp::model::{CallToolResponse, CallToolResult, ContentBlock};
use serde::Serialize;

/// Applies the configured byte cap to one complete successful result.
pub(super) fn apply(response: CallToolResponse, limit: NonZeroUsize) -> CallToolResponse {
    let CallToolResponse::Complete(result) = &response else {
        return response;
    };
    if result.is_error == Some(true) {
        return response;
    }
    let Some(structured) = result.structured_content.as_ref() else {
        return response;
    };
    let Ok(bytes) = serde_json::to_vec(structured) else {
        return response;
    };
    if bytes.len() <= limit.get() {
        return response;
    }

    CallToolResult::error(vec![ContentBlock::text(
        serde_json::to_string(&ResponseTooLarge {
            code: "response_too_large",
            message: "The structured JSON tool result exceeded the configured response limit.",
            bytes: bytes.len(),
            limit: limit.get(),
            hint: "Narrow filters or request a smaller page size.",
        })
        .expect("serializing a fixed response-limit error cannot fail"),
    )])
    .into()
}

#[derive(Serialize)]
struct ResponseTooLarge {
    code: &'static str,
    message: &'static str,
    bytes: usize,
    limit: usize,
    hint: &'static str,
}

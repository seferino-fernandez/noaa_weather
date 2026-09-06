//! Successful structured and binary result size enforcement.

use std::num::NonZeroUsize;

use base64::Engine as _;
use base64::prelude::BASE64_STANDARD;
use noaa_weather_client::BinaryPayload;
use rmcp::model::{CallToolResponse, CallToolResult, ContentBlock, ResourceContents};
use serde::Serialize;

/// MCP content representation for a binary NOAA response.
pub(super) enum BinaryContent {
    /// A directly renderable image block.
    Image,
    /// An embedded binary resource retaining its final response URI.
    Resource,
}

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

    too_large(
        bytes.len(),
        limit,
        "The structured JSON tool result exceeded the configured response limit.",
        "Narrow filters or request a smaller page size.",
    )
    .into()
}

/// Converts one bounded binary NOAA response into its native MCP content.
pub(super) fn binary(
    payload: BinaryPayload,
    content: BinaryContent,
    limit: NonZeroUsize,
) -> CallToolResult {
    if payload.len() > limit.get() {
        return too_large(
            payload.len(),
            limit,
            "The binary tool result exceeded the configured response limit.",
            "Increase --max-response-bytes if the caller can accept a larger binary payload.",
        );
    }

    let encoded = BASE64_STANDARD.encode(payload.as_bytes());
    let mime_type = payload.content_type().to_string();
    let block = match content {
        BinaryContent::Image => ContentBlock::image(encoded, mime_type),
        BinaryContent::Resource => ContentBlock::resource(
            ResourceContents::blob(encoded, payload.final_url().as_str()).with_mime_type(mime_type),
        ),
    };
    CallToolResult::success(vec![block])
}

fn too_large(
    bytes: usize,
    limit: NonZeroUsize,
    message: &'static str,
    hint: &'static str,
) -> CallToolResult {
    CallToolResult::error(vec![ContentBlock::text(
        serde_json::to_string(&ResponseTooLarge {
            code: "response_too_large",
            message,
            bytes,
            limit: limit.get(),
            hint,
        })
        .expect("serializing a fixed response-limit error cannot fail"),
    )])
}

#[derive(Serialize)]
struct ResponseTooLarge {
    code: &'static str,
    message: &'static str,
    bytes: usize,
    limit: usize,
    hint: &'static str,
}

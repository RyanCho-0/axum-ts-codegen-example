//! REQ/RES DTOs. Each type derives BOTH:
//! - `utoipa::ToSchema` → OpenAPI spec → openapi-typescript output
//! - `ts_rs::TS`        → direct .ts file per type (exported by `cargo test`)
//!
//! So you can compare the two pipelines on identical inputs.

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, TS)]
#[ts(export)]
pub struct CreatePostRequest {
    pub title: String,
    pub body: String,
    /// Optional tags — shows Option<Vec<T>> handling in both outputs.
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, TS)]
#[ts(export)]
pub struct PostResponse {
    pub id: String,
    pub title: String,
    pub body: String,
    pub status: PostStatus,
    pub created_at: i64,
}

/// Tagged enum — the interesting case: compare how serde `tag`/`rename_all`
/// is rendered by ts-rs vs the OpenAPI route.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[ts(export)]
pub enum PostStatus {
    Draft,
    Published { published_at: i64 },
}

/// Generic list wrapper — ts-rs keeps the generic, OpenAPI monomorphizes it.
#[derive(Debug, Serialize, Deserialize, ToSchema, TS)]
#[ts(export)]
pub struct ListResponse<T: TS> {
    pub items: Vec<T>,
    pub bookmark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, TS)]
#[ts(export)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

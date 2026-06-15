//! REQ/RES DTOs. Each type derives `utoipa::ToSchema` (or `IntoParams` for
//! query strings) → OpenAPI spec → generated TypeScript.
//!
//! The set is intentionally varied so the generated TS exercises many shapes:
//! nested structs, arrays of structs, tagged enums, plain enums, maps,
//! optional fields (PATCH), query-param structs, and a generic list wrapper.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};

// ─────────────────────────── Posts ───────────────────────────

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePostRequest {
    pub title: String,
    pub body: String,
    /// Optional list → `string[] | undefined`.
    pub tags: Option<Vec<String>>,
    /// Arbitrary metadata → `Record<string, string>` / `{ [key: string]: string }`.
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// PATCH semantics: every field optional → partial update payload.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub body: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostResponse {
    pub id: String,
    pub title: String,
    pub body: String,
    pub status: PostStatus,
    /// Nested object.
    pub author: Author,
    pub tags: Vec<String>,
    /// Nested array of objects.
    pub comments: Vec<Comment>,
    pub created_at: i64,
}

/// Tagged enum → discriminated union. Three variants incl. data-carrying ones.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PostStatus {
    Draft,
    Published { published_at: i64 },
    Archived { reason: String },
}

/// Plain enum used both as a field and as a query parameter → string union.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PostSort {
    Newest,
    Oldest,
    Popular,
}

/// Query-string params (`?status=&sort=&limit=&cursor=`) → `IntoParams`.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListPostsQuery {
    /// Filter by status discriminant, e.g. `published`.
    pub status: Option<String>,
    pub sort: Option<PostSort>,
    pub limit: Option<u32>,
    pub cursor: Option<String>,
}

// ─────────────────────────── Comments ───────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Comment {
    pub id: String,
    pub author: Author,
    pub text: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateCommentRequest {
    pub text: String,
}

// ─────────────────────────── Users ───────────────────────────

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub email: String,
    pub display_name: String,
    pub role: Option<UserRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub role: UserRole,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Admin,
    Member,
    Guest,
}

/// Reused author summary embedded in posts and comments.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Author {
    pub id: String,
    pub display_name: String,
}

// ─────────────────────────── Shared ───────────────────────────

/// Generic paginated wrapper. utoipa monomorphizes per instantiation
/// (`ListResponse_PostResponse`, `ListResponse_UserResponse`, …).
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
    pub bookmark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

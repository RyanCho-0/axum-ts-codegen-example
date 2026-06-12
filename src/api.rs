//! Handlers registered through `utoipa_axum::OpenApiRouter`, so the OpenAPI
//! paths are collected from the SAME `routes!()` calls that build the axum
//! router — no separate `#[openapi(paths(...))]` list to drift.

use axum::{extract::Path, http::StatusCode, Json};
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::dto::*;

#[utoipa::path(
    post,
    path = "/posts",
    operation_id = "create_post",
    tag = "posts",
    request_body = CreatePostRequest,
    responses(
        (status = 200, body = PostResponse),
        (status = 400, body = ErrorResponse),
    )
)]
async fn create_post(Json(req): Json<CreatePostRequest>) -> Json<PostResponse> {
    Json(PostResponse {
        id: Uuid::new_v4().to_string(),
        title: req.title,
        body: req.body,
        status: PostStatus::Draft,
        created_at: 1_700_000_000_000,
    })
}

#[utoipa::path(
    get,
    path = "/posts",
    operation_id = "list_posts",
    tag = "posts",
    responses((status = 200, body = ListResponse<PostResponse>))
)]
async fn list_posts() -> Json<ListResponse<PostResponse>> {
    Json(ListResponse {
        items: vec![],
        bookmark: None,
    })
}

#[utoipa::path(
    get,
    path = "/posts/{id}",
    operation_id = "get_post",
    tag = "posts",
    params(("id" = String, Path, description = "Post id")),
    responses(
        (status = 200, body = PostResponse),
        (status = 404, body = ErrorResponse),
    )
)]
async fn get_post(
    Path(id): Path<String>,
) -> Result<Json<PostResponse>, (StatusCode, Json<ErrorResponse>)> {
    Ok(Json(PostResponse {
        id,
        title: "hello".into(),
        body: "world".into(),
        status: PostStatus::Published {
            published_at: 1_700_000_000_000,
        },
        created_at: 1_700_000_000_000,
    }))
}

/// Build the API router. `split_for_parts()` on the result yields both the
/// plain `axum::Router` and the collected `utoipa::openapi::OpenApi`.
pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(create_post, list_posts))
        .routes(routes!(get_post))
}

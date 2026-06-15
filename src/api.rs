//! Handlers registered through `utoipa_axum::OpenApiRouter`, so OpenAPI paths
//! are collected from the SAME `routes!()` calls that build the axum router.
//!
//! Endpoints span three tags (posts / comments / users) and exercise:
//! POST · GET · PATCH · DELETE, path params, query-param structs, 204
//! No Content, nested response objects, and a generic list wrapper.

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::dto::*;

/// Seeds the component registry with schemas that are only referenced from
/// query params (via `IntoParams`). utoipa-axum auto-collects schemas used in
/// request/response bodies, but a param-only enum like `PostSort` would emit a
/// dangling `$ref` otherwise — list it here so the `$ref` resolves.
#[derive(OpenApi)]
#[openapi(components(schemas(PostSort)))]
struct ApiDoc;

// ─────────────── sample data helpers (canned responses) ───────────────

fn sample_author() -> Author {
    Author {
        id: "user_1".into(),
        display_name: "Ryan".into(),
    }
}

fn sample_comment() -> Comment {
    Comment {
        id: "comment_1".into(),
        author: sample_author(),
        text: "nice post".into(),
        created_at: 1_700_000_000_000,
    }
}

fn sample_post(id: String) -> PostResponse {
    PostResponse {
        id,
        title: "hello".into(),
        body: "world".into(),
        status: PostStatus::Published {
            published_at: 1_700_000_000_000,
        },
        author: sample_author(),
        tags: vec!["demo".into()],
        comments: vec![sample_comment()],
        created_at: 1_700_000_000_000,
    }
}

fn sample_user(id: String) -> UserResponse {
    UserResponse {
        id,
        email: "ryan@biyard.co".into(),
        display_name: "Ryan".into(),
        role: UserRole::Admin,
    }
}

// ─────────────────────────── Posts ───────────────────────────

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
    let mut post = sample_post(Uuid::new_v4().to_string());
    post.title = req.title;
    post.body = req.body;
    if let Some(tags) = req.tags {
        post.tags = tags;
    }
    post.status = PostStatus::Draft;
    Json(post)
}

#[utoipa::path(
    get,
    path = "/posts",
    operation_id = "list_posts",
    tag = "posts",
    params(ListPostsQuery),
    responses((status = 200, body = ListResponse<PostResponse>))
)]
async fn list_posts(Query(_q): Query<ListPostsQuery>) -> Json<ListResponse<PostResponse>> {
    Json(ListResponse {
        items: vec![sample_post("post_1".into())],
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
    Ok(Json(sample_post(id)))
}

#[utoipa::path(
    patch,
    path = "/posts/{id}",
    operation_id = "update_post",
    tag = "posts",
    params(("id" = String, Path, description = "Post id")),
    request_body = UpdatePostRequest,
    responses(
        (status = 200, body = PostResponse),
        (status = 404, body = ErrorResponse),
    )
)]
async fn update_post(
    Path(id): Path<String>,
    Json(req): Json<UpdatePostRequest>,
) -> Json<PostResponse> {
    let mut post = sample_post(id);
    if let Some(title) = req.title {
        post.title = title;
    }
    if let Some(body) = req.body {
        post.body = body;
    }
    if let Some(tags) = req.tags {
        post.tags = tags;
    }
    Json(post)
}

#[utoipa::path(
    delete,
    path = "/posts/{id}",
    operation_id = "delete_post",
    tag = "posts",
    params(("id" = String, Path, description = "Post id")),
    responses(
        (status = 204, description = "Deleted"),
        (status = 404, body = ErrorResponse),
    )
)]
async fn delete_post(Path(_id): Path<String>) -> StatusCode {
    StatusCode::NO_CONTENT
}

// ─────────────────────────── Comments ───────────────────────────

#[utoipa::path(
    get,
    path = "/posts/{id}/comments",
    operation_id = "list_comments",
    tag = "comments",
    params(("id" = String, Path, description = "Post id")),
    responses((status = 200, body = ListResponse<Comment>))
)]
async fn list_comments(Path(_id): Path<String>) -> Json<ListResponse<Comment>> {
    Json(ListResponse {
        items: vec![sample_comment()],
        bookmark: None,
    })
}

#[utoipa::path(
    post,
    path = "/posts/{id}/comments",
    operation_id = "create_comment",
    tag = "comments",
    params(("id" = String, Path, description = "Post id")),
    request_body = CreateCommentRequest,
    responses((status = 200, body = Comment))
)]
async fn create_comment(
    Path(_id): Path<String>,
    Json(req): Json<CreateCommentRequest>,
) -> Json<Comment> {
    let mut comment = sample_comment();
    comment.id = Uuid::new_v4().to_string();
    comment.text = req.text;
    Json(comment)
}

// ─────────────────────────── Users ───────────────────────────

#[utoipa::path(
    post,
    path = "/users",
    operation_id = "create_user",
    tag = "users",
    request_body = CreateUserRequest,
    responses((status = 200, body = UserResponse))
)]
async fn create_user(Json(req): Json<CreateUserRequest>) -> Json<UserResponse> {
    let mut user = sample_user(Uuid::new_v4().to_string());
    user.email = req.email;
    user.display_name = req.display_name;
    if let Some(role) = req.role {
        user.role = role;
    }
    Json(user)
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    operation_id = "get_user",
    tag = "users",
    params(("id" = String, Path, description = "User id")),
    responses(
        (status = 200, body = UserResponse),
        (status = 404, body = ErrorResponse),
    )
)]
async fn get_user(Path(id): Path<String>) -> Json<UserResponse> {
    Json(sample_user(id))
}

#[utoipa::path(
    get,
    path = "/users",
    operation_id = "list_users",
    tag = "users",
    responses((status = 200, body = ListResponse<UserResponse>))
)]
async fn list_users() -> Json<ListResponse<UserResponse>> {
    Json(ListResponse {
        items: vec![sample_user("user_1".into())],
        bookmark: None,
    })
}

/// Build the API router. `split_for_parts()` yields both the plain
/// `axum::Router` and the collected `utoipa::openapi::OpenApi`.
pub fn router() -> OpenApiRouter {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(create_post, list_posts))
        .routes(routes!(get_post, update_post, delete_post))
        .routes(routes!(list_comments, create_comment))
        .routes(routes!(create_user, list_users))
        .routes(routes!(get_user))
}

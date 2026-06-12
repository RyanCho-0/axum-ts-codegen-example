use axum::{routing::get, Json};

#[tokio::main]
async fn main() {
    let (router, api) = axum_ts_example::api::router().split_for_parts();

    // Serve the live spec too, so you can diff file output vs running server.
    let app = router.route(
        "/openapi.json",
        get(move || async move { Json(api.clone()) }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://localhost:3000  (spec: /openapi.json)");
    axum::serve(listener, app).await.unwrap();
}

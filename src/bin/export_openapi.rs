//! Dumps the OpenAPI spec to ./openapi.json.
//! build.rs can't reference the crate's own types, so a tiny bin target is
//! the standard way to get "regenerate on every build/watch tick".

fn main() {
    let (_router, api) = axum_ts_example::api::router().split_for_parts();
    let json = api.to_pretty_json().expect("serialize openapi");
    std::fs::write("openapi.json", &json).expect("write openapi.json");
    println!("wrote openapi.json ({} bytes)", json.len());
}

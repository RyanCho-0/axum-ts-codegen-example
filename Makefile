# Two parallel TS-codegen pipelines from the same Rust DTOs:
#
#   A) ts-rs   : #[derive(TS)] → `cargo test export_bindings` → ts/ts-rs/*.ts
#   B) utoipa  : #[utoipa::path] → export_openapi bin → openapi.json
#                → openapi-typescript → ts/openapi/api-types.ts
#
# `make watch` re-runs BOTH on every src/ change.

.PHONY: gen gen-tsrs gen-openapi gen-heyapi spec watch serve clean

gen: gen-tsrs spec gen-openapi gen-heyapi

# Dump the OpenAPI spec once; the two OpenAPI-based generators reuse it.
spec:
	cargo run --quiet --bin export_openapi

gen-tsrs:
	TS_RS_EXPORT_DIR=$(CURDIR)/ts/ts-rs cargo test export_bindings --quiet

# A) types only, single file
gen-openapi: spec
	npx openapi-typescript openapi.json -o ts/openapi/api-types.ts

# B) split files + SDK functions + TanStack Query options
gen-heyapi: spec
	npx @hey-api/openapi-ts

watch:
	cargo watch -w src -s 'make gen'

serve:
	cargo run --bin axum-ts-example

clean:
	rm -rf ts/ts-rs ts/openapi openapi.json target

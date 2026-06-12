# Axum (utoipa) → TypeScript codegen, with a one-command dev loop.
#
# Chosen stack (React + React Query + RN):
#   Rust DTO + #[utoipa::path]  →  openapi.json  →  @hey-api/openapi-ts
#                                                    (types + SDK + TanStack Query)
#
# `make dev`  → single watcher: regenerate TS types AND (re)run the API server
#               on every src/ change. ONE process, ONE command.
#
# Comparison generators (ts-rs, openapi-typescript) are kept under `make gen`
# for the internal discussion; they are NOT part of the dev loop.

BIN := axum-ts-example

.PHONY: dev gen gen-tsrs gen-openapi gen-heyapi spec watch serve clean help

# ── Primary entry point ──────────────────────────────────────────────
# One watcher does both, in order, on every src/ save. Both steps live in a
# SINGLE -s command so the order is guaranteed (codegen THEN server) — cargo
# -watch does not preserve interleaved -s/-x ordering, and the server blocks,
# so codegen must come first in one chained shell command.
# cargo-watch SIGTERMs the running server before each cycle, so no port clash.
dev:
	cargo watch -w src -s 'make spec gen-heyapi && cargo run --bin $(BIN)'

# ── One-shot codegen ─────────────────────────────────────────────────
# Real stack only (what the apps consume):
gen: spec gen-heyapi

# Everything incl. comparison generators (for the discussion/demo):
gen-all: gen-tsrs spec gen-openapi gen-heyapi

# Dump the OpenAPI spec once; OpenAPI-based generators reuse it.
spec:
	cargo run --quiet --bin export_openapi

# Chosen: split files + SDK functions + TanStack Query options.
gen-heyapi: spec
	npx @hey-api/openapi-ts

# Comparison A: types only, single file.
gen-openapi: spec
	npx openapi-typescript openapi.json -o ts/openapi/api-types.ts

# Comparison B: ts-rs (#[derive(TS)] → cargo test exports per-type .ts).
gen-tsrs:
	TS_RS_EXPORT_DIR=$(CURDIR)/ts/ts-rs cargo test export_bindings --quiet

# ── Variants ─────────────────────────────────────────────────────────
# Codegen-only watch (no server) — for type work without hitting the API.
watch:
	cargo watch -w src -s 'make gen'

# Server only, no watch.
serve:
	cargo run --bin $(BIN)

clean:
	rm -rf ts/ts-rs ts/openapi ts/hey-api openapi.json target

help:
	@echo "make dev       서버 + 타입 재생성, 한 워처/한 명령 (개발용)"
	@echo "make gen       선택 스택 타입 1회 생성 (hey-api)"
	@echo "make gen-all   비교용 포함 전체 생성 (ts-rs / openapi-typescript / hey-api)"
	@echo "make watch     타입 재생성만 watch (서버 없이)"
	@echo "make serve     Axum 서버만 실행"
	@echo "make clean     생성물 + target 제거"

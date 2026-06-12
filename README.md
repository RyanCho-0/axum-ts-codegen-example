# axum-ts-codegen-example

Rust(axum) DTO → TypeScript 타입 자동 생성을 **두 파이프라인으로 동시에** 돌려서 비교하는 예제.
같은 DTO(`src/dto.rs`)에 `ToSchema`(utoipa)와 `TS`(ts-rs)를 모두 derive해 두었다.

```
                       ┌─ A) ts-rs ──── cargo test export_bindings ──→ ts/ts-rs/*.ts
src/dto.rs + src/api.rs┤
                       └─ B) utoipa ─── export_openapi bin ──→ openapi.json
                                        └─ npx openapi-typescript ──→ ts/openapi/api-types.ts
```

## Setup

```bash
npm install               # openapi-typescript + @hey-api/openapi-ts
cargo install cargo-watch # 없으면
```

## Usage

```bash
make dev      # ★ 한 명령: 서버 + 타입 재생성을 한 워처에서 (개발용)
make gen      # 선택 스택(hey-api) 타입 1회 생성
make gen-all  # 비교용 포함 전체 (ts-rs / openapi-typescript / hey-api)
make watch    # 타입 재생성만 watch (서버 없이)
make serve    # Axum 서버만
make help     # 명령 요약
```

### `make dev` 하나로 끝

`make dev`는 **터미널 하나, cargo-watch 하나**로 다음을 `src/` 저장마다 순서대로 실행한다:

1. `openapi.json` 재생성 → `ts/hey-api/*` (types + SDK + TanStack Query) 갱신
2. Axum API 서버 (재)실행 (http://localhost:3000)

> "서버 두 대"가 아니다. 서버는 Axum 하나뿐이고, 타입 재생성은 서버가 아니라
> 같은 워처에 묶인 빌드 스텝이다. (codegen과 server를 한 `-s` 명령으로 체이닝해
> 순서를 보장 — cargo-watch는 `-s`/`-x` 혼합 순서를 보장하지 않기 때문.)

`make dev` 띄워놓고 `src/dto.rs`에 필드를 추가/변경하면 `ts/hey-api/`가 즉시 갱신되고
서버도 새 코드로 재시작된다. (`make gen-all`을 쓰면 비교용 `ts/ts-rs/`,
`ts/openapi/api-types.ts`도 함께 생성된다.)

## React Native / TanStack Query에서 쓰기

hey-api 출력(`ts/hey-api/`)은 `fetch` 기반이라 RN에서 네이티브 의존성 없이 동작한다.
`@tanstack/react-query` 플러그인이 만든 `getPostOptions()` / `createPostMutation()` /
`listPostsQueryKey()`를 `useQuery`/`useMutation`에 그대로 넣으면 된다.
복붙용 레퍼런스: [`examples/rn/`](examples/rn/README.md) (Provider 설정 · baseUrl ·
NetInfo/AppState 연동 · 화면 사용 예).

## 비교 포인트 (생성물에서 직접 확인)

| 관찰 대상 | ts-rs (`ts/ts-rs/`) | utoipa (`ts/openapi/api-types.ts`) |
|---|---|---|
| 라우트 정보 | 없음 — 타입 파일만 | `paths["/posts"]["post"]` 에 메서드·경로·status까지 |
| 제네릭 `ListResponse<T>` | `ListResponse<T>` 그대로 보존 | `ListResponse_PostResponse` 로 모노모픽화 |
| tagged enum `PostStatus` | discriminated union | oneOf 스키마 → union |
| 파일 구조 | 타입당 1파일 (barrel 직접 관리) | 단일 파일 |

## 왜 build.rs가 아니라 bin + watch인가

build.rs는 자기 크레이트의 타입을 import할 수 없어서 `ApiDoc::openapi()`를 부를 수 없다.
그래서 export 전용 bin(`src/bin/export_openapi.rs`)을 두고 cargo-watch로 돌리는 게 표준 패턴.
ts-rs는 derive가 `export_bindings_*` 테스트를 생성하므로 `cargo test export_bindings`가 export 트리거다.

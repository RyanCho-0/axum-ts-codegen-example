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
npm install               # openapi-typescript
cargo install cargo-watch # 없으면
```

## Usage

```bash
make gen      # 두 파이프라인 1회 실행
make watch    # src/ 변경 시마다 둘 다 재생성 (실시간 반영)
make serve    # axum 서버 (http://localhost:3000/openapi.json 로 라이브 spec)
```

`make watch` 띄워놓고 `src/dto.rs`에 필드 하나 추가해 보면
`ts/ts-rs/`와 `ts/openapi/api-types.ts`가 둘 다 즉시 갱신된다.

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

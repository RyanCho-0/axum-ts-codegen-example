# axum-ts-codegen-example

Axum(utoipa) 서버의 Rust DTO에서 **TypeScript를 자동 생성**해 React / React Native가
타입 안전하게 API를 호출하는 베이스 예제.

```
src/dto.rs + src/api.rs (Rust DTO + #[utoipa::path])
        │  export_openapi bin
        ▼
   openapi.json   ← git에 커밋됨 (스펙 자체를 바로 확인/외부 제공 가능)
        │
        ├─ @hey-api/openapi-ts ──→ ts/hey-api/   (★ 선택 스택: types + SDK + TanStack Query)
        └─ openapi-typescript  ──→ ts/openapi/api-types.ts  (비교용: 단일 파일, 타입만)
```

> 생성된 TS와 `openapi.json`을 **레포에 커밋**해 두어, 클론하지 않고 GitHub에서 바로 비교할 수 있다.

## Setup

```bash
npm install               # @hey-api/openapi-ts + openapi-typescript
cargo install cargo-watch # 없으면
```

## Usage

```bash
make dev      # ★ 한 명령: 서버 + 타입 재생성을 한 워처에서 (개발용)
make gen      # 선택 스택(hey-api) 타입 1회 생성
make gen-all  # 비교용 포함 (hey-api + openapi-typescript)
make watch    # 타입 재생성만 watch (서버 없이)
make serve    # Axum 서버만
make help     # 명령 요약
```

### `make dev` 하나로 끝

`make dev`는 **터미널 하나, cargo-watch 하나**로 `src/` 저장마다 순서대로 실행한다:

1. `openapi.json` 재생성 → `ts/hey-api/*` (types + SDK + TanStack Query) 갱신
2. Axum API 서버 (재)실행 (http://localhost:3000)

> "서버 두 대"가 아니다. 서버는 Axum 하나뿐이고, 타입 재생성은 서버가 아니라
> 같은 워처에 묶인 빌드 스텝이다. (codegen과 server를 한 `-s` 명령으로 체이닝해
> 순서를 보장 — cargo-watch는 `-s`/`-x` 혼합 순서를 보장하지 않기 때문.)

`make dev` 띄워놓고 `src/dto.rs`에 필드를 추가/변경하면 `ts/hey-api/`가 즉시 갱신되고
서버도 새 코드로 재시작된다.

## API 표면 (생성물이 다양해지도록 의도적으로 구성)

| 메서드 · 경로 | tag | 보여주는 추출 패턴 |
|---|---|---|
| `POST /posts` | posts | request/response body, `HashMap` → `Record<string,string>` |
| `GET /posts` | posts | **쿼리 파라미터 구조체**(`IntoParams`), enum 쿼리(`PostSort`) |
| `GET /posts/{id}` | posts | path param, 404 에러 응답 타입 |
| `PATCH /posts/{id}` | posts | 전부 optional인 **부분 수정** 페이로드 |
| `DELETE /posts/{id}` | posts | **204 No Content** (body 없음) |
| `GET /posts/{id}/comments` | comments | 중첩 객체 배열, 제네릭 `ListResponse<Comment>` |
| `POST /posts/{id}/comments` | comments | path + body 조합 |
| `POST /users` · `GET /users/{id}` · `GET /users` | users | 별도 도메인(tag), 평범한 enum(`UserRole`) |

추출되는 타입 모양: 중첩 구조체(`Author`), 중첩 배열(`Vec<Comment>`),
tagged enum(`PostStatus` → discriminated union), 평범한 enum(string union),
map, optional 필드, 제네릭 리스트(`ListResponse_PostResponse` 식 모노모픽화).

## React Native / TanStack Query에서 쓰기

hey-api 출력(`ts/hey-api/`)은 `fetch` 기반이라 RN에서 네이티브 의존성 없이 동작한다.
`@tanstack/react-query` 플러그인이 만든 `getPostOptions()` / `createPostMutation()` /
`listPostsQueryKey()`를 `useQuery`/`useMutation`에 그대로 넣으면 된다.
복붙용 레퍼런스: [`examples/rn/`](examples/rn/README.md) (Provider 설정 · baseUrl ·
NetInfo/AppState 연동 · 화면 사용 예).

> 참고: hey-api 플러그인은 `useGetPost()` 같은 훅을 직접 만들지 않고, `useQuery`에
> 넣을 **options/key/mutation 팩토리**를 생성한다(더 유연). 완성된 훅을 원하면 orval로
> 바꾸면 된다.

## 두 OpenAPI 제너레이터 비교 (`ts/hey-api/` vs `ts/openapi/`)

| 관찰 대상 | openapi-typescript (`ts/openapi/api-types.ts`) | @hey-api (`ts/hey-api/`) |
|---|---|---|
| 출력 구조 | 단일 파일 | 관심사별 분할 (types / sdk / client / tanstack) |
| 호출 방식 | `api.POST('/posts', …)` 경로 문자열 | `createPost({ body })` 함수 |
| TanStack Query | 직접 작성 | `getPostOptions()` 등 자동 생성 |
| 라우트↔타입 연결 | 있음 (둘 다 OpenAPI 기반) | 있음 |

둘 다 같은 `openapi.json`에서 파생된다 — 백엔드(utoipa)는 그대로 두고 프론트 제너레이터만 고르면 된다.

## 왜 build.rs가 아니라 bin + watch인가

build.rs는 자기 크레이트의 타입을 import할 수 없어서 `ApiDoc::openapi()`를 부를 수 없다.
그래서 export 전용 bin(`src/bin/export_openapi.rs`)을 두고 cargo-watch로 돌리는 게 표준 패턴.
빌드/CI 시점 보장은 이 bin을 빌드 단계에 끼우고, 재생성 결과와 커밋된 산출물의
`git diff`가 비면 통과시키는 식으로 drift를 막는다.

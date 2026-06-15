# React + RN 공유 예제

생성된 API 클라이언트와 React Query 레이어를 **web(React)과 mobile(RN)이 어떻게 공유**하는지
보여주는 레퍼런스. 핵심 원칙은 **"플랫폼 무관한 건 한 군데, 플랫폼 차이만 각자"**.

```
examples/
├── shared/          ← 플랫폼 무관. web·RN이 똑같이 import
│   ├── api.ts         생성된 클라이언트 re-export + configureApiClient
│   ├── query-client.ts createQueryClient() (공통 기본값)
│   ├── posts.ts        usePosts / usePost / useCreatePost / useUpdatePost / useDeletePost …
│   ├── users.ts        useUsers / useUser / useCreateUser
│   └── index.ts        barrel
├── web/             ← React 전용 부트스트랩
│   ├── main.tsx        baseUrl 설정 + Provider (focus/online은 브라우저 자동)
│   └── PostsPage.tsx   div/button + 공유 훅
└── rn/              ← React Native 전용 부트스트랩
    ├── App.tsx         baseUrl 설정 + Provider + NetInfo/AppState 매니저
    └── PostScreen.tsx  View/Text/Button + 공유 훅
```

## 공유 전략 (무엇을 어디에)

| 레이어 | 위치 | 공유? | 이유 |
|---|---|---|---|
| 생성된 타입·SDK·TanStack 팩토리 (`ts/hey-api`) | 루트 | ✅ 100% | `fetch` 기반, 플랫폼 무관 |
| 커스텀 훅 (`usePosts`, `useCreatePost`…) | `shared/` | ✅ 100% | `@tanstack/react-query`만 의존 |
| QueryClient 기본값 | `shared/query-client.ts` | ✅ 100% | 동일 정책 |
| baseUrl / 토큰 소스 | `web/main.tsx`, `rn/App.tsx` | ❌ 플랫폼별 | 환경마다 다름 |
| Provider 마운트 | 각 진입점 | ❌ 플랫폼별 | `createRoot` vs RN 루트 |
| online/focus 매니저 | `rn/App.tsx`만 | ❌ RN만 | web은 `window`로 자동, RN은 NetInfo/AppState 수동 |

**결과**: `usePosts({ sort: 'newest' })` 같은 데이터 호출 코드는 web과 RN이 **글자 그대로 동일**하고,
화면 요소(`div` vs `View`)와 부트스트랩만 다릅니다.

## 실제 프로젝트에서의 배치

이 예제는 단일 레포라 `shared/`가 루트의 `ts/hey-api`를 상대경로로 import한다.
실무 모노레포에서는:

```
packages/
  api/            ← `make gen` 산출물(ts/hey-api) + shared 훅을 패키지로
apps/
  web/            ← packages/api 의존
  mobile/ (RN)    ← packages/api 의존
```

`shared/api.ts`의 re-export를 `export * from '@your-org/api'` 한 줄로 바꾸면
나머지(훅·화면)는 그대로 동작한다.

## 주의

- 이 `.tsx`/`.ts` 파일들은 **레퍼런스**다. 이 레포엔 React/RN 의존성이 설치돼 있지 않아
  타입 체크되지 않는다. 실제 앱에 복사하고 import 경로만 맞추면 그대로 컴파일된다.
- RN 추가 의존성: `@tanstack/react-query`, `@hey-api/client-fetch`,
  `@react-native-community/netinfo`(online 브릿지용, 선택).
- hey-api 플러그인은 `useGetPost()` 같은 완성 훅이 아니라 **options/key/mutation 팩토리**를
  생성한다. `shared/posts.ts`의 얇은 래퍼가 그 위에 우리 훅을 만든다(원하면 orval로 바꿔 완성 훅 생성 가능).

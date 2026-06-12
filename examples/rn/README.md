# React Native usage reference

Copy-paste reference for consuming the generated **hey-api + TanStack Query**
output from a React Native app. The code in `../ts/hey-api/` is `fetch`-based,
so it runs in RN with **no native deps and no polyfills**.

## Install (in your RN project)

```bash
npm install @tanstack/react-query @hey-api/client-fetch @react-native-community/netinfo
```

`@react-native-community/netinfo` is only needed for the offline/online bridge
in `App.tsx` — drop it if you don't want connectivity-aware retries.

## Files

| File | Role |
|------|------|
| `api-client.ts` | One-time `client.setConfig({ baseUrl })` + auth header interceptor |
| `App.tsx` | `QueryClientProvider` + RN `onlineManager`/`focusManager` bridges |
| `PostScreen.tsx` | `useQuery` / `useMutation` with generated option & key factories |

## How the pieces connect

```
Rust DTO + #[utoipa::path]  →  openapi.json  →  @hey-api/openapi-ts
                                                      │
                              ts/hey-api/@tanstack/react-query.gen.ts
                                                      │
                  getPostOptions() / createPostMutation() / listPostsQueryKey()
                                                      │
                         useQuery(...) / useMutation(...)  ← App.tsx, PostScreen.tsx
```

Same generated files are imported by both web and RN — only `api-client.ts`
(baseUrl) and the `App.tsx` managers differ per platform.

## Notes

- **Import paths** here point at `../../ts/hey-api/...` for this example repo.
  In a real RN app, copy that output into the app (or publish it as a shared
  workspace package) and fix the import paths accordingly.
- These `.tsx` files are a **reference**, not part of the Rust build, and are
  not type-checked by this repo (RN deps aren't installed here).
- The `status.type === 'published'` narrowing in `PostScreen.tsx` shows the
  Rust tagged enum (`PostStatus`) flowing through as a TypeScript
  discriminated union.

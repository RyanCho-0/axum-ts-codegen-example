/**
 * Single import surface for the generated API client — imported by BOTH web
 * and RN. Nothing here is platform-specific.
 *
 * In THIS example the generated code lives at repo-root `ts/hey-api`. In a real
 * monorepo you'd publish that as a workspace package and replace these
 * re-exports with `export * from '@your-org/api'` — and nothing downstream
 * (the hooks, the screens) changes.
 */
export * from '../../ts/hey-api/types.gen';
export * from '../../ts/hey-api/sdk.gen';
export * from '../../ts/hey-api/@tanstack/react-query.gen';

import { client } from '../../ts/hey-api/client.gen';
export { client };

/**
 * Call ONCE at app start. Each platform supplies its own baseUrl and token
 * source — that's the only API-client difference between web and RN.
 */
export function configureApiClient(opts: {
  baseUrl: string;
  getToken?: () => string | null;
}) {
  client.setConfig({ baseUrl: opts.baseUrl });

  client.interceptors.request.use((request) => {
    const token = opts.getToken?.();
    if (token) request.headers.set('Authorization', `Bearer ${token}`);
    return request;
  });
}

/**
 * One-time client setup. Import this ONCE at app entry (above the component
 * tree) so every generated SDK call / TanStack option uses the same baseUrl
 * and auth headers.
 *
 * The generated client (`client.gen.ts`) wraps `fetch`, which RN provides
 * natively — no axios adapter, no polyfill.
 *
 * NOTE: adjust the import path to wherever you copy the generated `ts/hey-api`
 * output inside your RN project (or to your shared workspace package).
 */
import { client } from '../../ts/hey-api/client.gen';

export function configureApiClient(getToken?: () => string | null) {
  client.setConfig({
    baseUrl: 'https://api.your-server.com',
  });

  // Attach an auth header on every request, if you have a token source.
  client.interceptors.request.use((request) => {
    const token = getToken?.();
    if (token) request.headers.set('Authorization', `Bearer ${token}`);
    return request;
  });
}

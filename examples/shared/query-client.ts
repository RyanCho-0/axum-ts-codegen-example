/**
 * Shared QueryClient factory. Same defaults on web and RN — each platform just
 * mounts the returned client in its own <QueryClientProvider>.
 */
import { QueryClient } from '@tanstack/react-query';

export function createQueryClient() {
  return new QueryClient({
    defaultOptions: {
      queries: {
        retry: 2,
        staleTime: 30_000,
        refetchOnWindowFocus: true, // web: automatic · RN: driven by focusManager (see rn/App.tsx)
      },
    },
  });
}

/**
 * React (web) entry point.
 *
 * Platform-specific bits ONLY:
 *  - baseUrl for the API client
 *  - mounting the QueryClientProvider
 * On web, window focus + online/offline are detected automatically by
 * TanStack Query, so there are no manager bridges (unlike RN/App.tsx).
 */
import React from 'react';
import { createRoot } from 'react-dom/client';
import { QueryClientProvider } from '@tanstack/react-query';

import { configureApiClient, createQueryClient } from '../shared';
import { PostsPage } from './PostsPage';

configureApiClient({
  baseUrl: import.meta.env.VITE_API_URL ?? 'http://localhost:3000',
  // getToken: () => localStorage.getItem('token'),
});

const queryClient = createQueryClient();

createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <PostsPage />
    </QueryClientProvider>
  </React.StrictMode>,
);

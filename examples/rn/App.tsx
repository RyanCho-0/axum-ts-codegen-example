/**
 * App root: wires QueryClientProvider + RN-specific online/focus managers.
 *
 * Why the managers: on web, TanStack Query auto-detects window focus and
 * online/offline. RN has no `window`, so you bridge them manually with
 * NetInfo (connectivity) and AppState (foreground/background). This is the
 * official RN pattern — queries still work without it, but you lose
 * "refetch on app resume" and "pause retries when offline".
 */
import React, { useEffect } from 'react';
import { AppState, type AppStateStatus, Platform } from 'react-native';
import NetInfo from '@react-native-community/netinfo';
import {
  QueryClient,
  QueryClientProvider,
  focusManager,
  onlineManager,
} from '@tanstack/react-query';

import { configureApiClient } from './api-client';
import { RootNavigator } from './RootNavigator'; // your app's navigator

configureApiClient(/* () => store.getState().authToken */);

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 2,
      staleTime: 30_000,
    },
  },
});

// Connectivity → TanStack online state.
onlineManager.setEventListener((setOnline) =>
  NetInfo.addEventListener((state) => setOnline(!!state.isConnected)),
);

export default function App() {
  // Foreground/background → TanStack focus state (drives refetchOnWindowFocus).
  useEffect(() => {
    const sub = AppState.addEventListener('change', (status: AppStateStatus) => {
      if (Platform.OS !== 'web') {
        focusManager.setFocused(status === 'active');
      }
    });
    return () => sub.remove();
  }, []);

  return (
    <QueryClientProvider client={queryClient}>
      <RootNavigator />
    </QueryClientProvider>
  );
}

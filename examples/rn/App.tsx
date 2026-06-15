/**
 * React Native entry point.
 *
 * Platform-specific bits ONLY (everything else comes from `../shared`):
 *  - baseUrl for the API client
 *  - mounting the QueryClientProvider
 *  - bridging RN connectivity/foreground into TanStack's online/focus managers
 *    (web gets these from `window` for free; RN has no window, so we wire
 *    NetInfo + AppState manually — the official RN pattern).
 */
import React, { useEffect } from 'react';
import { AppState, type AppStateStatus, Platform } from 'react-native';
import NetInfo from '@react-native-community/netinfo';
import {
  QueryClientProvider,
  focusManager,
  onlineManager,
} from '@tanstack/react-query';

import { configureApiClient, createQueryClient } from '../shared';
import { RootNavigator } from './RootNavigator'; // your app's navigator

configureApiClient({
  baseUrl: 'https://api.your-server.com',
  // getToken: () => store.getState().authToken,
});

const queryClient = createQueryClient();

// Connectivity → TanStack online state (pause retries when offline).
onlineManager.setEventListener((setOnline) =>
  NetInfo.addEventListener((state) => setOnline(!!state.isConnected)),
);

export default function App() {
  // Foreground/background → TanStack focus state (drives refetchOnWindowFocus).
  useEffect(() => {
    const sub = AppState.addEventListener('change', (status: AppStateStatus) => {
      if (Platform.OS !== 'web') focusManager.setFocused(status === 'active');
    });
    return () => sub.remove();
  }, []);

  return (
    <QueryClientProvider client={queryClient}>
      <RootNavigator />
    </QueryClientProvider>
  );
}

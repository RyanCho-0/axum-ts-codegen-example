/**
 * Shared Users query/mutation hooks — used identically by web and RN.
 */
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';

import {
  listUsersOptions,
  listUsersQueryKey,
  getUserOptions,
  createUserMutation,
} from './api';

export function useUsers() {
  return useQuery(listUsersOptions());
}

export function useUser(id: string) {
  return useQuery(getUserOptions({ path: { id } }));
}

export function useCreateUser() {
  const qc = useQueryClient();
  return useMutation({
    ...createUserMutation(),
    onSuccess: () => qc.invalidateQueries({ queryKey: listUsersQueryKey() }),
  });
}

/**
 * Shared Posts query/mutation hooks — used identically by web and RN.
 *
 * Built ON TOP of the hey-api generated TanStack factories (`*Options`,
 * `*QueryKey`, `*Mutation`). This is where you'd normally write hooks by hand;
 * here the factories + types are generated, so these wrappers stay tiny and
 * fully type-safe (params, body, response all inferred from the Rust DTOs).
 *
 * Mutations read their `path`/`body` from the mutate() variables, so the hooks
 * take no arguments and invalidation keys are derived from those variables.
 */
import {
  useInfiniteQuery,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query';

import {
  listPostsOptions,
  listPostsQueryKey,
  listPostsInfiniteOptions,
  getPostOptions,
  getPostQueryKey,
  createPostMutation,
  updatePostMutation,
  deletePostMutation,
  listCommentsOptions,
  listCommentsQueryKey,
  createCommentMutation,
  type ListPostsData,
} from './api';

/** Query filter object: `{ status?, sort?, limit?, cursor? }` (typed from the DTO). */
export type PostFilter = ListPostsData['query'];

// ── Queries ──────────────────────────────────────────────
export function usePosts(query?: PostFilter) {
  return useQuery(listPostsOptions({ query }));
}

/** Cursor-paginated variant — getNextPageParam is wired by the generator. */
export function useInfinitePosts(query?: PostFilter) {
  return useInfiniteQuery(listPostsInfiniteOptions({ query }));
}

export function usePost(id: string) {
  return useQuery(getPostOptions({ path: { id } }));
}

export function usePostComments(postId: string) {
  return useQuery(listCommentsOptions({ path: { id: postId } }));
}

// ── Mutations (with cache invalidation) ──────────────────
export function useCreatePost() {
  const qc = useQueryClient();
  return useMutation({
    ...createPostMutation(),
    onSuccess: () => qc.invalidateQueries({ queryKey: listPostsQueryKey() }),
  });
}

export function useUpdatePost() {
  const qc = useQueryClient();
  return useMutation({
    ...updatePostMutation(),
    onSuccess: (_data, variables) => {
      qc.invalidateQueries({ queryKey: getPostQueryKey({ path: variables.path }) });
      qc.invalidateQueries({ queryKey: listPostsQueryKey() });
    },
  });
}

export function useDeletePost() {
  const qc = useQueryClient();
  return useMutation({
    ...deletePostMutation(),
    onSuccess: () => qc.invalidateQueries({ queryKey: listPostsQueryKey() }),
  });
}

export function useCreateComment() {
  const qc = useQueryClient();
  return useMutation({
    ...createCommentMutation(),
    onSuccess: (_data, variables) =>
      qc.invalidateQueries({ queryKey: listCommentsQueryKey({ path: variables.path }) }),
  });
}

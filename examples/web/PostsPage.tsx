/**
 * React (web) page. Note: imports the SAME shared hooks as the RN screen
 * (`../shared`). The only difference between this and PostScreen.tsx is the
 * host elements (div/button vs View/Button) — the data layer is identical.
 */
import React from 'react';

import { usePosts, useCreatePost, useDeletePost } from '../shared';

export function PostsPage() {
  // query params are type-checked: sort is "newest" | "oldest" | "popular"
  const { data, isLoading, error } = usePosts({ sort: 'newest', limit: 20 });
  const createPost = useCreatePost();
  const deletePost = useDeletePost();

  if (isLoading) return <p>Loading…</p>;
  if (error) return <p>Failed to load</p>;

  return (
    <div>
      <button
        disabled={createPost.isPending}
        onClick={() =>
          // body is type-checked against CreatePostRequest
          createPost.mutate({ body: { title: 'Hello', body: 'from web', metadata: {} } })
        }
      >
        {createPost.isPending ? 'Creating…' : 'New post'}
      </button>

      <ul>
        {data?.items.map((post) => (
          <li key={post.id}>
            <strong>{post.title}</strong> — by {post.author.display_name}
            {/* Rust tagged enum → discriminated union, narrowed here */}
            {post.status.type === 'published' && <em> (published)</em>}
            <button onClick={() => deletePost.mutate({ path: { id: post.id } })}>
              delete
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
}

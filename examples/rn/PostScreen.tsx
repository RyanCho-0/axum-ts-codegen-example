/**
 * React Native screens. Imports the SAME shared hooks as the web page
 * (`../shared`) — only the host elements differ (View/Text/Button/FlatList
 * vs div/ul/button). The data layer is 100% shared.
 */
import React from 'react';
import { ActivityIndicator, Button, FlatList, Text, View } from 'react-native';

import { usePost, usePosts, useCreatePost, useDeletePost } from '../shared';

export function PostDetailScreen({ id }: { id: string }) {
  const { data, isLoading, error } = usePost(id);
  //      ^ data: PostResponse | undefined

  if (isLoading) return <ActivityIndicator />;
  if (error) return <Text>Failed to load</Text>;

  return (
    <View>
      <Text>{data?.title}</Text>
      <Text>by {data?.author.display_name}</Text>
      {/* Rust tagged enum (PostStatus) → discriminated union, narrowed here */}
      {data?.status.type === 'published' && (
        <Text>published_at: {String(data.status.published_at)}</Text>
      )}
    </View>
  );
}

export function PostListScreen() {
  const { data } = usePosts({ sort: 'newest', limit: 20 });
  const createPost = useCreatePost();
  const deletePost = useDeletePost();

  return (
    <View>
      <FlatList
        data={data?.items ?? []}
        keyExtractor={(p) => p.id}
        renderItem={({ item }) => (
          <View>
            <Text>{item.title}</Text>
            <Button title="delete" onPress={() => deletePost.mutate({ path: { id: item.id } })} />
          </View>
        )}
      />
      <Button
        title={createPost.isPending ? 'Creating…' : 'New post'}
        disabled={createPost.isPending}
        onPress={() =>
          createPost.mutate({ body: { title: 'Hello', body: 'from RN', metadata: {} } })
        }
      />
    </View>
  );
}

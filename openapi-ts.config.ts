import { defineConfig } from '@hey-api/openapi-ts';

// hey-api config — controls output path, file naming, and which plugins
// generate code. The TanStack Query plugin reads the same spec and emits
// query/mutation option factories.
export default defineConfig({
  input: 'openapi.json',
  output: {
    path: 'ts/hey-api',
    postProcess: ['prettier'],
  },
  plugins: [
    '@hey-api/client-fetch',   // client.gen.ts
    '@hey-api/typescript',     // types.gen.ts
    '@hey-api/sdk',            // sdk.gen.ts (createPost, getPost, ...)
    '@tanstack/react-query',   // @tanstack/react-query.gen.ts (options + keys)
  ],
});

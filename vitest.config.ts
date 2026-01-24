import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    globals: true,        // 'describe', 'it', 'expect' without imports
    environment: 'node',  // provides Node 'fetch'
    include: ['tests/**/*.test.ts'],
    watch: false,
  },
});

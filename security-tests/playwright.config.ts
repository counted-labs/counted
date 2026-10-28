import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './tests',
  workers: 1,
  retries: 0,
  timeout: 60_000,
  reporter: [['list'], ['json', { outputFile: 'results/security-tests.json' }]],
});

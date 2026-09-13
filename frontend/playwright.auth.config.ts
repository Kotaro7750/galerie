import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests',
  testMatch: 'auth.spec.ts',
  use: { baseURL: 'http://127.0.0.1:4174', trace: 'retain-on-failure' },
  projects: [{ name: 'auth', use: { ...devices['Desktop Chrome'] } }],
  webServer: {
    command: 'VITE_AUTH_ENABLED=true VITE_AUTH_AUTHORITY=https://cognito-idp.us-east-1.amazonaws.com/us-east-1_test VITE_AUTH_CLIENT_ID=galerie npm run dev -- --host 127.0.0.1 --port 4174 --strictPort',
    url: 'http://127.0.0.1:4174',
    reuseExistingServer: false,
  },
});

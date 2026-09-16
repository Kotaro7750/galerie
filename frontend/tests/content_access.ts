import type { Page } from '@playwright/test';

export async function mockContentAccess(page: Page) {
  await page.route('**/api/v0/content-access', (route) => route.fulfill(
    route.request().method() === 'POST' ? { json: {} } : { status: 204 },
  ));
}

import type { Page } from '@playwright/test';

export const defaultTagSchema = { version: '0', allowAdditionalTags: true, required: [], optional: [] };

export async function mockTagSchema(page: Page, schema: object = defaultTagSchema) {
  await page.route('**/api/v0/tag-schema', (route) => route.fulfill({ json: schema }));
}

export async function enterAdditionalTagKey(page: Page, key: string) {
  await page.getByLabel('タグ名', { exact: true }).fill(key);
}

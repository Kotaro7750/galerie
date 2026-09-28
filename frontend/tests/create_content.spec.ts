import { expect, test } from '@playwright/test';
import { mockContentAccess } from './content_access';

const sample = '../sample/00065786-f916-4e2c-85bc-3db5e4c0cb71.avif';
const id = '10000000-0000-4000-8000-000000000001';

test.beforeEach(async ({ page }) => { await mockContentAccess(page); });

test('selects and previews a local AVIF, then sends typed tags only on registration', async ({ page }) => {
  const posts: { contentType: string; body: string }[] = [];
  await page.route('**/api/v0/contents', async (route) => {
    const request = route.request();
    posts.push({ contentType: request.headers()['content-type'], body: request.postData() || '' });
    await route.fulfill({ status: 201, json: { id, mediaType: 'image/avif', contentUrl: '/created.avif', thumbnailUrl: '/created.avif', tags: [], diagnostics: [] } });
  });
  await page.route(`**/api/v0/contents/${id}`, (route) => route.fulfill({ json: { id, mediaType: 'image/avif', contentUrl: '/created.avif', thumbnailUrl: '/created.avif', tags: [], diagnostics: [] } }));
  await page.route('**/created.avif', (route) => route.fulfill({ path: sample, contentType: 'image/avif' }));
  await page.goto('/');
  await page.getByRole('link', { name: 'コンテンツを登録' }).click();
  await expect(page).toHaveURL(/#\/contents\/new$/);
  await expect(page.getByRole('button', { name: '登録する' })).toBeDisabled();
  const mediaType = page.getByRole('radio', { name: 'image/avif' });
  await expect(mediaType).toBeChecked();
  await expect(mediaType.locator('..').locator('svg')).toHaveClass(/lucide-image/);
  await expect(mediaType.locator('..')).toHaveClass(/btn-soft/);
  await page.getByLabel('アップロードするファイル').setInputFiles(sample);
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toBeVisible();
  await page.getByLabel('タグ名').fill('category');
  await page.getByLabel('タグの型').selectOption('text');
  await page.getByLabel('値 1', { exact: true }).fill('landscape');
  await page.getByRole('button', { name: 'タグを追加' }).click();
  await page.getByLabel('タグ名').fill('rating');
  await page.getByLabel('タグの型').selectOption('integer');
  await page.getByLabel('値 1', { exact: true }).fill('5');
  await page.getByRole('button', { name: 'タグを追加' }).click();
  await page.getByLabel('タグ名').fill('authors');
  await page.getByLabel('タグの型').selectOption('textSet');
  await page.getByLabel('値 1', { exact: true }).fill('Alice');
  await page.getByRole('button', { name: '値を追加' }).click();
  await page.getByLabel('値 2', { exact: true }).fill('Bob');
  await page.getByRole('button', { name: 'タグを追加' }).click();
  expect(posts).toHaveLength(0);
  await page.getByRole('button', { name: '登録する' }).click();
  await expect(page).toHaveURL(new RegExp(`#\\/contents\\/${id}$`));
  expect(posts).toHaveLength(1);
  expect(posts[0].contentType).toContain('multipart/form-data; boundary=');
  expect(posts[0].body).toContain('Content-Type: image/avif');
  expect(posts[0].body).toContain('Content-Type: application/json');
  expect(posts[0].body).toContain('"tags":[{"key":"category","type":"text","value":"landscape"},{"key":"rating","type":"integer","value":5},{"key":"authors","type":"textSet","values":["Alice","Bob"]}]');
});

test('fetches a URL into the local preview and keeps API requests pending', async ({ page }) => {
  let posts = 0;
  await page.route('**/api/v0/contents', (route) => { posts++; return route.abort(); });
  await page.route('**/source.avif', (route) => route.fulfill({ path: sample, contentType: 'image/avif' }));
  await page.goto('/#/contents/new');
  await expect(page.getByLabel('コンテンツ URL')).toHaveCount(0);
  await page.getByRole('button', { name: 'URL から追加' }).click();
  await expect(page.getByLabel('アップロードするファイル')).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'URL から追加' }).locator('svg')).toHaveClass(/lucide-link(?!-2)/);
  await expect(page.getByLabel('コンテンツ URL').locator('..').locator('svg')).toHaveClass(/lucide-link(?!-2)/);
  await expect(page.getByRole('button', { name: 'URL をプレビュー' }).locator('svg')).toHaveClass(/lucide-eye/);
  await page.getByLabel('コンテンツ URL').fill(new URL('/source.avif', page.url()).href);
  await page.getByRole('button', { name: 'URL をプレビュー' }).click();
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toBeVisible();
  expect(posts).toBe(0);
  await expect(page.getByRole('button', { name: '登録する' })).toBeEnabled();
  await page.getByRole('button', { name: 'ファイルから追加' }).click();
  await expect(page.getByLabel('コンテンツ URL')).toHaveCount(0);
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: '登録する' })).toBeDisabled();
});

test('clears the URL and its prepared preview', async ({ page }) => {
  await page.route('**/source.avif', (route) => route.fulfill({ path: sample, contentType: 'image/avif' }));
  await page.goto('/#/contents/new');
  await page.getByRole('button', { name: 'URL から追加' }).click();
  await page.getByLabel('コンテンツ URL').fill(new URL('/source.avif', page.url()).href);
  await page.getByRole('button', { name: 'URL をプレビュー' }).click();
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toBeVisible();
  await page.getByRole('button', { name: 'URL をクリア' }).click();
  await expect(page.getByLabel('コンテンツ URL')).toHaveValue('');
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: '登録する' })).toBeDisabled();
});

test('pastes an image into the same local preparation flow', async ({ page }) => {
  await page.goto('/#/contents/new');
  await page.getByRole('button', { name: 'URL から追加' }).click();
  await page.getByLabel('コンテンツ URL').fill('https://example.com/image.png');
  await page.evaluate(async () => {
    const canvas = document.createElement('canvas');
    canvas.width = 12; canvas.height = 12;
    canvas.getContext('2d')!.fillRect(0, 0, 12, 12);
    const blob = await new Promise<Blob>((resolve) => canvas.toBlob((value) => resolve(value!), 'image/png'));
    const data = new DataTransfer();
    data.items.add(new File([blob], 'clipboard.png', { type: 'image/png' }));
    document.dispatchEvent(new ClipboardEvent('paste', { bubbles: true, cancelable: true, clipboardData: data }));
  });
  await expect(page.getByRole('button', { name: 'ファイルから追加' })).toHaveAttribute('aria-pressed', 'true');
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toHaveJSProperty('naturalWidth', 12);
  await expect(page.getByText('clipboard.avif', { exact: false })).toBeVisible();
  await expect(page.getByRole('button', { name: '登録する' })).toBeEnabled();
  await page.getByRole('button', { name: 'URL から追加' }).click();
  await expect(page.getByLabel('コンテンツ URL')).toHaveValue('');
});

test('listing link opens registration and a rejected request keeps the local draft', async ({ page }) => {
  let posts = 0;
  await page.route('**/api/v0/contents?*', (route) => route.fulfill({ json: { items: [] } }));
  await page.route('**/api/v0/contents', (route) => {
    posts++;
    return route.fulfill({ status: 400, contentType: 'application/problem+json', body: JSON.stringify({ title: 'Invalid content registration', status: 400, detail: 'The tag key is not allowed.' }) });
  });
  await page.goto('/#/contents');
  await page.getByRole('link', { name: 'コンテンツを登録' }).click();
  await page.getByLabel('アップロードするファイル').setInputFiles(sample);
  await page.getByLabel('タグ名').fill('category');
  await page.getByRole('button', { name: 'タグを追加' }).click();
  await page.getByRole('button', { name: '登録する' }).click();
  await expect(page.getByRole('alert')).toHaveText('The tag key is not allowed.');
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toBeVisible();
  await expect(page.getByLabel('category', { exact: true })).toBeVisible();
  expect(posts).toBe(1);
});


test('converts a PNG locally before preview and sends AVIF only on registration', async ({ page }) => {
  const posts: { contentType: string; body: string }[] = [];
  await page.route('**/api/v0/contents', (route) => {
    const request = route.request();
    posts.push({ contentType: request.headers()['content-type'], body: request.postData() || '' });
    return route.fulfill({ status: 400, contentType: 'application/problem+json', body: JSON.stringify({ status: 400, title: 'Invalid request' }) });
  });
  await page.goto('/#/contents/new');
  const png = await page.evaluate(() => {
    const canvas = document.createElement('canvas');
    canvas.width = 16; canvas.height = 16;
    const context = canvas.getContext('2d')!;
    context.fillStyle = '#ed641a'; context.fillRect(0, 0, 16, 16);
    return canvas.toDataURL('image/png').split(',')[1];
  });
  await page.getByLabel('アップロードするファイル').setInputFiles({ name: 'sample.png', mimeType: 'image/png', buffer: Buffer.from(png, 'base64') });
  const preview = page.getByRole('img', { name: '登録するコンテンツのプレビュー' });
  await expect(preview).toBeVisible();
  await expect(preview).toHaveJSProperty('naturalWidth', 16);
  await expect(page.getByText('sample.avif', { exact: false })).toBeVisible();
  expect(posts).toHaveLength(0);
  await page.getByRole('button', { name: '登録する' }).click();
  await expect(page.getByRole('alert')).toBeVisible();
  expect(posts).toHaveLength(1);
  expect(posts[0].body).toContain('filename="sample.avif"');
  expect(posts[0].body).toContain('Content-Type: image/avif');
});

test('converts an image from a URL in the browser without contacting the registration API', async ({ page }) => {
  let posts = 0;
  await page.route('**/api/v0/contents', (route) => { posts++; return route.abort(); });
  await page.goto('/#/contents/new');
  const png = await page.evaluate(() => {
    const canvas = document.createElement('canvas');
    canvas.width = 12; canvas.height = 12;
    const context = canvas.getContext('2d')!;
    context.fillStyle = '#147a90'; context.fillRect(0, 0, 12, 12);
    return canvas.toDataURL('image/png').split(',')[1];
  });
  await page.route('**/source.png', (route) => route.fulfill({ body: Buffer.from(png, 'base64'), contentType: 'image/png' }));
  await page.getByRole('button', { name: 'URL から追加' }).click();
  await page.getByLabel('コンテンツ URL').fill(new URL('/source.png', page.url()).href);
  await page.getByRole('button', { name: 'URL をプレビュー' }).click();
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toHaveJSProperty('naturalWidth', 12);
  await expect(page.getByText('source.avif', { exact: false })).toBeVisible();
  expect(posts).toBe(0);
});

import { expect, test } from '@playwright/test';
import type { Page } from '@playwright/test';

const id = '00065786-f916-4e2c-85bc-3db5e4c0cb71';
const content = (index: number) => ({
  id: index === 0 ? id : `10000000-0000-4000-8000-${String(index).padStart(12, '0')}`,
  mediaType: 'image/avif',
  contentUrl: 'http://media.test/original.avif',
  thumbnailUrl: 'http://media.test/thumbnail.avif',
  tags: [],
  invalidTags: [],
});

async function mockImages(page: Page) {
  await page.route('http://media.test/**', (route) => route.fulfill({
    path: '../sample/00065786-f916-4e2c-85bc-3db5e4c0cb71.avif', contentType: 'image/avif',
  }));
}

test('home introduces the gallery without fetching content', async ({ page }) => {
  let requested = false;
  page.on('request', (request) => { if (new URL(request.url()).pathname.startsWith('/api/')) requested = true; });
  await page.goto('/');
  await expect(page.getByRole('heading', { level: 1 })).toContainText('好きなものを');
  await expect(page.getByRole('link', { name: 'ギャラリーを開く' })).toHaveAttribute('href', '#/contents');
  expect(requested).toBe(false);
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
  await page.screenshot({ path: `test-results/home-${test.info().project.name}.png`, fullPage: true });
});

test('scroll forwards the opaque cursor, opens original content and supports direct reload', async ({ page }) => {
  const cursor = 'opaque+/=日本語';
  const requests: (string | null)[] = [];
  await mockImages(page);
  await page.route('**/api/v0/contents?*', (route) => {
    const url = new URL(route.request().url());
    requests.push(url.searchParams.get('cursor'));
    expect(url.searchParams.get('limit')).toBe('30');
    return route.fulfill({ json: url.searchParams.has('cursor')
      ? { items: [content(30)] }
      : { items: Array.from({ length: 30 }, (_, index) => content(index)), nextCursor: cursor } });
  });
  await page.route(`**/api/v0/contents/${id}`, (route) => route.fulfill({ json: content(0) }));
  await page.goto('/#/contents');
  await expect(page.getByRole('link', { name: /画像 \d+ を開く/ })).toHaveCount(30);
  await page.getByRole('link', { name: '画像 30 を開く', exact: true }).scrollIntoViewIfNeeded();
  await expect(page.getByRole('link', { name: /画像 \d+ を開く/ })).toHaveCount(31);
  await expect(page.getByText('すべての画像を表示しました')).toBeVisible();
  expect(requests).toEqual([null, cursor]);
  await page.getByRole('button', { name: '小さく', exact: true }).click();
  await expect(page.getByRole('button', { name: '小さく', exact: true })).toHaveAttribute('aria-pressed', 'true');
  await page.getByRole('link', { name: '画像 1 を開く', exact: true }).click();
  await expect(page).toHaveURL(new RegExp(`/contents/${id}$`));
  const original = page.getByRole('img', { name: `コンテンツ ${id}` });
  await expect(original).toHaveAttribute('src', 'http://media.test/original.avif');
  await expect.poll(() => original.evaluate((img: HTMLImageElement) => img.naturalWidth)).toBeGreaterThan(0);
  await page.reload();
  await expect(original).toBeVisible();
  await page.getByRole('link', { name: 'ギャラリーに戻る' }).click();
  await expect(page.getByRole('button', { name: '小さく', exact: true })).toHaveAttribute('aria-pressed', 'false');
  await page.getByRole('button', { name: '一覧を更新' }).click();
  await expect.poll(() => requests.at(-1)).toBe(null);
  await page.screenshot({ path: `test-results/gallery-${test.info().project.name}.png`, fullPage: true });
});

test('empty, loading and request failure states offer recovery', async ({ page }) => {
  let failing = true;
  await page.route('**/api/v0/contents?*', async (route) => {
    await new Promise((resolve) => setTimeout(resolve, 200));
    await route.fulfill(failing
      ? { status: 400, contentType: 'application/problem+json', json: { type: 'about:blank', title: 'Bad request', status: 400, detail: 'カーソルが無効です。' } }
      : { json: { items: [] } });
  });
  await page.goto('/#/contents');
  await expect(page.getByRole('status')).toContainText('読み込み中');
  await expect(page.getByRole('alert')).toContainText('カーソルが無効です。');
  failing = false;
  await page.getByRole('button', { name: '再試行', exact: true }).click();
  await expect(page.getByText('まだコンテンツがありません')).toBeVisible();
});

test('a failed next page retains images and retries the same cursor', async ({ page }) => {
  await mockImages(page);
  let nextAttempts = 0;
  await page.route('**/api/v0/contents?*', (route) => {
    const cursor = new URL(route.request().url()).searchParams.get('cursor');
    if (!cursor) return route.fulfill({ json: { items: [content(0)], nextCursor: 'next-page' } });
    expect(cursor).toBe('next-page');
    nextAttempts++;
    return route.fulfill(nextAttempts === 1
      ? { status: 400, json: { type: 'about:blank', title: 'Bad request', status: 400 } }
      : { json: { items: [content(1)] } });
  });
  await page.goto('/#/contents');
  await expect(page.getByRole('alert')).toBeVisible();
  await expect(page.getByRole('link', { name: /画像 \d+ を開く/ })).toHaveCount(1);
  await page.getByRole('button', { name: '再試行', exact: true }).click();
  await expect(page.getByRole('link', { name: /画像 \d+ を開く/ })).toHaveCount(2);
  expect(nextAttempts).toBe(2);
});

test('handles missing content and broken image delivery', async ({ page }) => {
  await page.route(`**/api/v0/contents/${id}`, (route) => route.fulfill({ status: 404, json: { type: 'about:blank', title: 'Not found', status: 404 } }));
  await page.goto(`/#/contents/${id}`);
  await expect(page.getByRole('alert')).toContainText('コンテンツが見つかりません');
  await page.unroute(`**/api/v0/contents/${id}`);
  await page.route(`**/api/v0/contents/${id}`, (route) => route.fulfill({ json: content(0) }));
  await page.route('http://media.test/**', (route) => route.abort());
  await page.getByRole('button', { name: '再試行', exact: true }).click();
  await expect(page.getByText('画像を読み込めませんでした')).toBeVisible();
  await page.unroute('http://media.test/**');
  await mockImages(page);
  await page.getByRole('button', { name: '画像を再読み込み' }).click();
  await expect.poll(() => page.getByRole('img').evaluate((img: HTMLImageElement) => img.naturalWidth)).toBeGreaterThan(0);
  await page.goto('/#/missing');
  await expect(page.getByRole('heading', { name: 'ページが見つかりません' })).toBeVisible();
});

test('shows API tags on hover or focus and on the content page', async ({ page, isMobile }) => {
  const taggedContent = {
    ...content(0),
    tags: [
      { key: 'animation', type: 'keyOnly' },
      { key: 'category', type: 'text', value: 'landscape' },
      { key: 'rating', type: 'integer', value: 0 },
      { key: 'score', type: 'real', value: -3.14 },
      { key: 'authors', type: 'textSet', values: ['Alice', 'Bob'] },
      { key: 'pages', type: 'integerSet', values: [1, 3] },
      { key: 'weights', type: 'realSet', values: [0.5, 1.5] },
      { key: 'empty', type: 'text', value: '' },
      { key: 'emptySet', type: 'textSet', values: [] },
      { key: 'long', type: 'text', value: '長いタグ'.repeat(100) },
    ],
    invalidTags: [
      { key: 'Invalid-Key', reason: 'INVALID_KEY' },
      { key: 'OrderedArray', reason: 'UNSUPPORTED_VALUE_TYPE' },
      { key: 'BooleanFalse', reason: 'INVALID_VALUE' },
    ],
  };
  await mockImages(page);
  await page.route('**/api/v0/contents?*', (route) => route.fulfill({ json: { items: [taggedContent, content(1)] } }));
  await page.route(`**/api/v0/contents/${id}`, (route) => route.fulfill({ json: taggedContent }));
  await page.goto('/#/contents');
  const card = page.getByRole('link', { name: '画像 1 を開く', exact: true });
  const panel = page.getByRole('region', { name: '画像 1 のタグ', exact: true, includeHidden: true });
  if (!isMobile) {
    await expect(panel).toBeHidden();
    await card.hover();
    await expect(panel).toBeVisible();
    await panel.hover();
    await expect(panel).toBeVisible();
    await page.getByRole('heading', { name: 'ギャラリー', exact: true }).hover();
    await expect(panel).toBeHidden();
    await card.focus();
    await expect(panel).toBeVisible();
    await page.keyboard.press('Tab');
    await expect(panel).toBeFocused();
  }
  await expect(panel).toBeVisible();
  await expect(panel.getByRole('list', { name: 'タグ', exact: true }).getByRole('listitem')).toHaveText([
    'animation', 'category: landscape', 'rating: 0', 'score: -3.14',
    'authors: Alice, Bob', 'pages: 1, 3', 'weights: 0.5, 1.5',
    'empty: （空文字）', 'emptySet: （空集合）', `long: ${'長いタグ'.repeat(100)}`,
  ]);
  expect(await panel.evaluate((element) => element.scrollHeight > element.clientHeight)).toBe(true);
  await panel.getByText('BooleanFalse: 値が無効です', { exact: true }).scrollIntoViewIfNeeded();
  await expect(panel.getByText('BooleanFalse: 値が無効です', { exact: true })).toBeInViewport();
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
  await page.screenshot({ path: `test-results/tags-gallery-${test.info().project.name}.png`, fullPage: true });
  await card.click();
  const detail = page.getByRole('region', { name: 'タグ情報' });
  await expect(detail.getByRole('list', { name: 'タグ', exact: true }).getByRole('listitem')).toHaveCount(10);
  await expect(detail.getByRole('list', { name: '無効なタグ', exact: true }).getByRole('listitem')).toHaveText([
    'Invalid-Key: タグ名が無効です', 'OrderedArray: 未対応の型です', 'BooleanFalse: 値が無効です',
  ]);
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
  await page.screenshot({ path: `test-results/tags-detail-${test.info().project.name}.png`, fullPage: true });
});

test('shows an empty tag state in both views', async ({ page }) => {
  await mockImages(page);
  await page.route('**/api/v0/contents?*', (route) => route.fulfill({ json: { items: [content(0)] } }));
  await page.route(`**/api/v0/contents/${id}`, (route) => route.fulfill({ json: content(0) }));
  await page.goto('/#/contents');
  const card = page.getByRole('link', { name: '画像 1 を開く', exact: true });
  await card.focus();
  await expect(page.getByText('タグはありません', { exact: true })).toBeVisible();
  await card.click();
  await expect(page.getByText('タグはありません', { exact: true })).toBeVisible();
  await expect(page.getByText('無効なタグ', { exact: true })).toHaveCount(0);
});

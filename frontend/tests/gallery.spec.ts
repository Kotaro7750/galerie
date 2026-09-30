import { expect, test } from '@playwright/test';
import type { Page } from '@playwright/test';
import { mockContentAccess } from './content_access';
import { mockTagSchema } from './tag_schema';

test.beforeEach(async ({ page }) => { await mockContentAccess(page); });

const id = '00065786-f916-4e2c-85bc-3db5e4c0cb71';
const content = (index: number) => ({
  id: index === 0 ? id : `10000000-0000-4000-8000-${String(index).padStart(12, '0')}`,
  mediaType: 'image/avif',
  contentUrl: 'https://media.test/original.avif',
  thumbnailUrl: 'https://media.test/thumbnail.avif',
  tags: [],
  diagnostics: [],
});

async function mockImages(page: Page) {
  await page.route('https://media.test/**', (route) => route.fulfill({
    path: '../sample/00065786-f916-4e2c-85bc-3db5e4c0cb71.avif', contentType: 'image/avif',
  }));
}

test('colors tag badges by schema category in the gallery, search, and detail', async ({ page, isMobile }) => {
  await mockTagSchema(page, { version: '0', allowAdditionalTags: true, required: [{ key: 'subjects', type: 'textSet' }], optional: [{ key: 'category', type: 'text' }] });
  const taggedContent = { ...content(0), tags: [
    { key: 'subjects', type: 'textSet', values: ['sky'] },
    { key: 'category', type: 'text', value: 'landscape' },
    { key: 'extra', type: 'text', value: 'other' },
  ] };
  await mockImages(page);
  await page.route('**/api/v0/contents?*', (route) => route.fulfill({ json: { items: [taggedContent] } }));
  await page.route(`**/api/v0/contents/${id}`, (route) => route.fulfill({ json: taggedContent }));
  await page.goto('/#/contents');
  const card = page.getByRole('link', { name: '画像 1 を開く' });
  if (!isMobile) await card.locator('../..').hover({ position: { x: 4, y: 4 } });
  const panel = page.getByRole('region', { name: '画像 1 のタグ' });
  await expect(panel.getByLabel('subjects: sky').locator('.badge')).toHaveClass(/badge-primary/);
  await expect(panel.getByLabel('category: landscape').locator('.badge')).toHaveClass(/badge-secondary/);
  await expect(panel.getByLabel('extra: other').locator('.badge')).toHaveClass(/badge-accent/);
  await panel.getByLabel('category: landscape').click();
  await expect(page.getByRole('list', { name: '検索条件（すべてに一致）' }).getByLabel('category: landscape').locator('.badge')).toHaveClass(/badge-secondary/);
  await card.click({ position: { x: 4, y: 4 } });
  const detail = page.getByRole('region', { name: 'タグ情報' });
  await expect(detail.getByLabel('subjects: sky').locator('.badge')).toHaveClass(/badge-primary/);
  await expect(detail.getByLabel('category: landscape').locator('.badge')).toHaveClass(/badge-secondary/);
  await expect(detail.getByLabel('extra: other').locator('.badge')).toHaveClass(/badge-accent/);
});

test('home introduces the gallery without fetching content', async ({ page }) => {
  const requestedPaths: string[] = [];
  page.on('request', (request) => {
    const path = new URL(request.url()).pathname;
    if (path.startsWith('/api/')) requestedPaths.push(path);
  });
  await page.goto('/');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Galerie');
  const galleryLink = page.getByRole('link', { name: 'ギャラリーを開く' });
  await expect(galleryLink).toHaveAttribute('href', '#/contents');
  await expect(galleryLink).toHaveClass(/btn-circle.*btn-primary/);
  await expect(galleryLink.locator('svg')).toHaveClass(/lucide-images/);
  const createLink = page.getByRole('link', { name: 'コンテンツを登録' });
  const [galleryBounds, createBounds] = await Promise.all([galleryLink.boundingBox(), createLink.boundingBox()]);
  expect(galleryBounds).not.toBeNull();
  expect(createBounds).not.toBeNull();
  expect(createBounds!.x).toBeGreaterThan(galleryBounds!.x + galleryBounds!.width);
  expect(createBounds!.y).toBeCloseTo(galleryBounds!.y, 0);
  await expect(page.locator('.divider')).toHaveText('OR');
  expect(requestedPaths).toEqual(['/api/v0/content-access', '/api/v0/tag-schema']);
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
  await expect(original).toHaveAttribute('src', 'https://media.test/original.avif');
  await expect.poll(() => original.evaluate((img: HTMLImageElement) => img.naturalWidth)).toBeGreaterThan(0);
  const resolution = await original.evaluate((img: HTMLImageElement) => `${img.naturalWidth} × ${img.naturalHeight}`);
  await expect(page.getByRole('img', { name: '解像度（ピクセル）' })).toBeVisible();
  await expect(page.getByRole('img', { name: '解像度（ピクセル）' })).toHaveClass(/lucide-ruler/);
  await expect(page.getByText(resolution, { exact: true })).toBeVisible();
  await page.reload();
  await expect(original).toBeVisible();
  await expect(page.getByText(resolution, { exact: true })).toBeVisible();
  await page.getByRole('link', { name: 'ギャラリーに戻る' }).click();
  await expect(page.getByRole('button', { name: '小さく', exact: true })).toHaveAttribute('aria-pressed', 'false');
  await page.getByRole('button', { name: '一覧を更新' }).click();
  await expect.poll(() => requests.at(-1)).toBe(null);
  await page.screenshot({ path: `test-results/gallery-${test.info().project.name}.png`, fullPage: true });
});

test('moves within the opened search results without fetching more pages and stays fullscreen', async ({ page }) => {
  const entries = [content(0), content(1), content(2)].map((item) => ({
    ...item, contentUrl: `https://media.test/${item.id}.avif`, thumbnailUrl: `https://media.test/${item.id}.avif`,
  }));
  let listRequests = 0;
  await page.route('**/api/v0/contents?*', (route) => {
    listRequests++;
    return route.fulfill({ json: { items: entries } });
  });
  await page.route('**/api/v0/contents/*', (route) => {
    const item = entries.find(({ id: itemId }) => route.request().url().endsWith(`/${itemId}`));
    return route.fulfill(item ? { json: item.id === entries[1].id ? { ...item, contentUrl: 'https://media.test/updated.avif' } : item } : { status: 404 });
  });
  await page.route('https://media.test/**', (route) => route.fulfill({
    path: '../sample/00065786-f916-4e2c-85bc-3db5e4c0cb71.avif', contentType: 'image/avif',
  }));
  await page.goto('/#/contents');
  await page.getByRole('link', { name: '画像 1 を開く', exact: true }).click();
  await page.getByRole('button', { name: '前のコンテンツ' }).click();
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[2].id}$`));
  await page.getByRole('button', { name: '次のコンテンツ' }).click();
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[0].id}$`));
  await page.getByRole('button', { name: '次のコンテンツ' }).click();
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[1].id}$`));
  await expect(page.getByRole('img', { name: `コンテンツ ${entries[1].id}` })).toHaveAttribute('src', 'https://media.test/updated.avif');
  await page.getByRole('button', { name: '全画面表示', exact: true }).click();
  const fullscreen = page.locator(':fullscreen');
  await expect(fullscreen).toHaveCount(1);
  await fullscreen.getByRole('button', { name: '次のコンテンツ' }).click();
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[2].id}$`));
  await expect(fullscreen).toHaveCount(1);
  await expect(fullscreen.getByRole('img', { name: `コンテンツ ${entries[2].id}` })).toHaveAttribute('src', entries[2].contentUrl);
  await fullscreen.getByRole('button', { name: '次のコンテンツ' }).click();
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[0].id}$`));
  await expect(fullscreen).toHaveCount(1);
  await fullscreen.getByRole('button', { name: '前のコンテンツ' }).click();
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[2].id}$`));
  await expect(fullscreen).toHaveCount(1);
  await fullscreen.getByRole('button', { name: '全画面表示を解除' }).click();
  await page.goBack();
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[0].id}$`));
  await expect(page.locator('.swiper-slide-active').getByRole('img', { name: `コンテンツ ${entries[0].id}` })).toBeVisible();
  expect(listRequests).toBe(1);
});

test('slideshow shares playback controls with fullscreen and restores excluded search results', async ({ page }) => {
  const entries = [content(0), content(1), content(2)].map((item) => ({ ...item, contentUrl: `https://media.test/${item.id}.avif` }));
  await page.route('**/api/v0/contents?*', (route) => route.fulfill({ json: { items: entries } }));
  await page.route('**/api/v0/contents/*', (route) => {
    const item = entries.find(({ id: itemId }) => route.request().url().endsWith(`/${itemId}`));
    return route.fulfill(item ? { json: item } : { status: 404 });
  });
  await mockImages(page);
  await page.goto('/#/contents');
  await page.getByRole('link', { name: '画像 1 を開く', exact: true }).click();
  const controls = page.getByRole('group', { name: 'スライドショー' });
  const [controlsBounds, backBounds] = await Promise.all([
    controls.boundingBox(), page.getByRole('link', { name: 'ギャラリーに戻る' }).boundingBox(),
  ]);
  expect(controlsBounds).not.toBeNull();
  expect(backBounds).not.toBeNull();
  expect(controlsBounds!.y + controlsBounds!.height).toBeLessThanOrEqual(backBounds!.y);
  await expect(controls).toContainText('3 / 3');
  const eye = controls.getByRole('button', { name: '表示中の画像をスライドショー対象から除外' });
  await expect(eye.locator('svg')).toHaveClass(/lucide-eye-off/);
  await expect(eye).toHaveAttribute('aria-pressed', 'false');
  await expect(eye).toHaveClass(/btn-ghost/);
  await expect(eye.locator('..')).toHaveAttribute('data-tip', '再生対象（クリックで除外）');
  await expect(controls.getByRole('button', { name: 'スライドショー対象をリセット' }).locator('..')).toHaveAttribute('data-tip', '除外した画像をすべて戻す');
  await expect(controls.getByRole('button', { name: 'スライドショー対象をリセット' }).locator('svg')).toHaveClass(/lucide-funnel-x/);
  await expect(controls.locator('svg.lucide-timer')).toBeVisible();
  const intervalSlider = controls.getByRole('slider', { name: 'スライドショーの表示間隔' });
  await expect(intervalSlider).toHaveAttribute('max', '15');
  await intervalSlider.fill('1');
  await eye.click();
  await expect(controls).toContainText('2 / 3');
  await expect(eye).toHaveAttribute('aria-pressed', 'true');
  await expect(eye).toHaveClass(/btn-primary/);
  await controls.getByRole('button', { name: 'スライドショーを再生' }).click();
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[1].id}$`));
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[2].id}$`));
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[1].id}$`));
  await controls.getByRole('button', { name: 'スライドショーを一時停止' }).click();
  await page.getByRole('button', { name: '全画面表示', exact: true }).click();
  const fullscreenControls = page.locator(':fullscreen').getByRole('group', { name: 'スライドショー' });
  await expect(fullscreenControls).toContainText('2 / 3');
  await expect(fullscreenControls).toContainText('1s');
  const fullscreenEye = fullscreenControls.getByRole('button', { name: '表示中の画像をスライドショー対象から除外' });
  await fullscreenEye.click();
  await expect(fullscreenControls).toContainText('1 / 3');
  await expect(fullscreenControls.getByRole('button', { name: 'スライドショーを再生' })).toBeDisabled();
  await expect(fullscreenEye).toHaveAttribute('aria-pressed', 'true');
  await expect(fullscreenEye).toHaveClass(/btn-primary/);
  await expect(fullscreenEye.locator('..')).toHaveAttribute('data-tip', '再生対象から除外中（クリックで戻す）');
  await fullscreenEye.click();
  await expect(fullscreenControls).toContainText('2 / 3');
  await expect(fullscreenEye).toHaveAttribute('aria-pressed', 'false');
  await expect(fullscreenEye).toHaveClass(/btn-ghost/);
  await fullscreenEye.click();
  await expect(fullscreenControls).toContainText('1 / 3');
  await fullscreenControls.getByRole('button', { name: 'スライドショー対象をリセット' }).click();
  await expect(fullscreenControls).toContainText('3 / 3');
  await fullscreenControls.getByRole('button', { name: 'スライドショーを再生' }).click();
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[2].id}$`));
  await fullscreenControls.getByRole('button', { name: 'スライドショーを一時停止' }).click();
  await expect(page.locator(':fullscreen')).toHaveCount(1);
});

test('fullscreen slideshow controls hide when idle and return on input', async ({ page, isMobile }) => {
  const entries = [content(0), content(1)];
  await page.route('**/api/v0/contents?*', (route) => route.fulfill({ json: { items: entries } }));
  await page.route('**/api/v0/contents/*', (route) => {
    const item = entries.find(({ id: itemId }) => route.request().url().endsWith(`/${itemId}`));
    return route.fulfill(item ? { json: item } : { status: 404 });
  });
  await mockImages(page);
  await page.goto('/#/contents');
  await page.getByRole('link', { name: '画像 1 を開く', exact: true }).click();
  await page.getByRole('button', { name: '全画面表示', exact: true }).click();
  const fullscreen = page.locator(':fullscreen');
  const controls = fullscreen.getByRole('group', { name: 'スライドショー' });
  const opacity = () => controls.evaluate((element) => getComputedStyle(element).opacity);
  await expect.poll(opacity).toBe('1');
  await expect.poll(opacity, { timeout: 5000 }).toBe('0');
  if (isMobile) await page.touchscreen.tap(30, 80);
  else await page.mouse.move(30, 80);
  await expect.poll(opacity).toBe('1');
  if (isMobile) return;
  await expect.poll(opacity, { timeout: 5000 }).toBe('0');
  await page.keyboard.press('ArrowRight');
  await expect.poll(opacity).toBe('1');
  await controls.getByRole('button', { name: 'シャッフル再生' }).focus();
  await expect(controls.locator(':focus-visible')).toHaveCount(1);
  await page.waitForTimeout(3300);
  await expect.poll(opacity).toBe('1');
});

test('shuffle visits each target once before repeating and keeps its setting in fullscreen', async ({ page }) => {
  const entries = [content(0), content(1), content(2), content(3)];
  await page.addInitScript(() => { Math.random = () => 0; });
  await page.route('**/api/v0/contents?*', (route) => route.fulfill({ json: { items: entries } }));
  await page.route('**/api/v0/contents/*', (route) => {
    const item = entries.find(({ id: itemId }) => route.request().url().endsWith(`/${itemId}`));
    return route.fulfill(item ? { json: item } : { status: 404 });
  });
  await mockImages(page);
  await page.goto('/#/contents');
  await page.getByRole('link', { name: '画像 1 を開く', exact: true }).click();
  const controls = page.getByRole('group', { name: 'スライドショー' });
  await controls.getByRole('slider', { name: 'スライドショーの表示間隔' }).fill('1');
  const shuffle = controls.getByRole('button', { name: 'シャッフル再生' });
  await shuffle.click();
  await expect(shuffle).toHaveAttribute('aria-pressed', 'true');
  await controls.getByRole('button', { name: 'スライドショーを再生' }).click();
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[2].id}$`));
  await page.getByRole('button', { name: '全画面表示', exact: true }).click();
  const fullscreenControls = page.locator(':fullscreen').getByRole('group', { name: 'スライドショー' });
  await expect(fullscreenControls.getByRole('button', { name: 'シャッフル再生' })).toHaveAttribute('aria-pressed', 'true');
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[3].id}$`));
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[1].id}$`));
  await fullscreenControls.getByRole('button', { name: 'スライドショーを一時停止' }).click();
  await fullscreenControls.getByRole('button', { name: 'シャッフル再生' }).click();
  await expect(fullscreenControls.getByRole('button', { name: 'シャッフル再生' })).toHaveAttribute('aria-pressed', 'false');
  await fullscreenControls.getByRole('button', { name: 'スライドショーを再生' }).click();
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[2].id}$`));
});

test('mobile flick changes the selected content in normal and fullscreen views', async ({ page, isMobile }) => {
  test.skip(!isMobile, 'touch gesture');
  const entries = [content(0), content(1), content(2)].map((item) => ({
    ...item, contentUrl: `https://media.test/${item.id}.avif`, thumbnailUrl: `https://media.test/${item.id}.avif`,
  }));
  await page.route('**/api/v0/contents?*', (route) => route.fulfill({ json: { items: entries } }));
  await page.route('**/api/v0/contents/*', (route) => {
    const item = entries.find(({ id: itemId }) => route.request().url().endsWith(`/${itemId}`));
    return route.fulfill(item ? { json: item } : { status: 404 });
  });
  await page.route('https://media.test/**', (route) => route.fulfill({
    path: '../sample/00065786-f916-4e2c-85bc-3db5e4c0cb71.avif', contentType: 'image/avif',
  }));
  const session = await page.context().newCDPSession(page);
  async function flick(target: ReturnType<typeof page.locator>) {
    const box = await target.boundingBox();
    expect(box).not.toBeNull();
    const y = box!.y + box!.height / 2;
    const startX = box!.x + box!.width * 0.8;
    const endX = box!.x + box!.width * 0.2;
    await session.send('Input.dispatchTouchEvent', { type: 'touchStart', touchPoints: [{ x: startX, y }] });
    for (let step = 1; step <= 5; step++) {
      await session.send('Input.dispatchTouchEvent', { type: 'touchMove', touchPoints: [{ x: startX + (endX - startX) * step / 5, y }] });
    }
    await session.send('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
  }
  await page.goto('/#/contents');
  await page.getByRole('link', { name: '画像 2 を開く', exact: true }).click();
  await expect(page.locator('.swiper-slide-active').getByRole('img', { name: `コンテンツ ${entries[1].id}` })).toBeVisible();
  await flick(page.locator('.swiper'));
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[2].id}$`));
  await page.getByRole('button', { name: '全画面表示', exact: true }).click();
  const fullscreen = page.locator(':fullscreen');
  await flick(fullscreen.locator('.swiper'));
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[0].id}$`));
  await expect(fullscreen).toHaveCount(1);
});

test('does not load the next search page while moving between contents', async ({ page }) => {
  const entries = Array.from({ length: 30 }, (_, index) => content(index));
  const cursors: (string | null)[] = [];
  await page.route('**/api/v0/contents?*', (route) => {
    const cursor = new URL(route.request().url()).searchParams.get('cursor');
    cursors.push(cursor);
    return route.fulfill({ json: cursor ? { items: [content(30)] } : { items: entries, nextCursor: 'later' } });
  });
  await page.route('**/api/v0/contents/*', (route) => {
    const item = entries.find(({ id: itemId }) => route.request().url().endsWith(`/${itemId}`));
    return route.fulfill(item ? { json: item } : { status: 404 });
  });
  await mockImages(page);
  await page.goto('/#/contents');
  await page.getByRole('link', { name: '画像 1 を開く', exact: true }).click();
  await page.getByRole('button', { name: '次のコンテンツ' }).click();
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[1].id}$`));
  expect(cursors).toEqual([null]);
});

test('two loaded contents also wrap in both directions', async ({ page }) => {
  const entries = [content(0), content(1)];
  await page.route('**/api/v0/contents?*', (route) => route.fulfill({ json: { items: entries } }));
  await page.route('**/api/v0/contents/*', (route) => {
    const item = entries.find(({ id: itemId }) => route.request().url().endsWith(`/${itemId}`));
    return route.fulfill(item ? { json: item } : { status: 404 });
  });
  await mockImages(page);
  await page.goto('/#/contents');
  await page.getByRole('link', { name: '画像 1 を開く', exact: true }).click();
  await page.getByRole('button', { name: '前のコンテンツ' }).click();
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[1].id}$`));
  await page.getByRole('button', { name: '次のコンテンツ' }).click();
  await expect(page).toHaveURL(new RegExp(`/contents/${entries[0].id}$`));
});

test('empty, loading and request failure states offer recovery', async ({ page, isMobile }) => {
  if (isMobile) await page.setViewportSize({ width: 375, height: 667 });
  let failing = true;
  await page.route('**/api/v0/contents?*', async (route) => {
    await new Promise((resolve) => setTimeout(resolve, 200));
    await route.fulfill(failing
      ? { status: 400, contentType: 'application/problem+json', json: { type: 'about:blank', title: 'Bad request', status: 400, detail: 'カーソルが無効です。' } }
      : { json: { items: [] } });
  });
  await page.goto('/#/contents');
  await expect(page.getByRole('status').getByText('読み込み中…', { exact: true })).toBeVisible();
  await expect(page.getByRole('alert')).toContainText('カーソルが無効です。');
  const retry = page.getByRole('button', { name: '再試行', exact: true });
  await expect(retry.locator('svg')).toHaveClass(/lucide-refresh-cw/);
  await expect(retry).toHaveText('');
  const retryBounds = await retry.boundingBox();
  expect(retryBounds!.x + retryBounds!.width).toBeLessThanOrEqual(page.viewportSize()!.width);
  failing = false;
  await retry.click();
  await expect(page.getByText('コンテンツが見つかりません')).toBeVisible();
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
  await page.getByRole('link', { name: '画像 1 を開く', exact: true }).scrollIntoViewIfNeeded();
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
  await page.route('https://media.test/**', (route) => route.abort());
  await page.getByRole('button', { name: '再試行', exact: true }).click();
  await expect(page.getByText('画像を読み込めませんでした')).toBeVisible();
  await expect(page.getByRole('img', { name: '解像度（ピクセル）' })).toHaveCount(0);
  await page.unroute('https://media.test/**');
  await mockImages(page);
  await page.getByRole('button', { name: '画像を再読み込み' }).click();
  await expect.poll(() => page.getByRole('img', { name: `コンテンツ ${id}`, exact: true }).evaluate((img: HTMLImageElement) => img.naturalWidth)).toBeGreaterThan(0);
  await expect(page.getByRole('button', { name: '前のコンテンツ' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: '次のコンテンツ' })).toHaveCount(0);
  await page.goto('/#/missing');
  await expect(page.getByRole('heading', { name: 'ページが見つかりません' })).toBeVisible();
});

test('shows API tags on hover or focus and on the content page', async ({ page, isMobile }) => {
  const requestedConditions: unknown[] = [];
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
      { key: 'emptySet', type: 'textSet', values: [] },
      { key: 'long', type: 'text', value: '長いタグ'.repeat(100) },
    ],
    diagnostics: [
      { key: 'Invalid-Key', kind: 'invalidKey' },
      { key: 'OrderedArray', kind: 'unsupportedXmpValueType' },
      { key: 'EmptyStringSet', kind: 'unparseableTagValue' },
    ],
  };
  await mockImages(page);
  await page.route('**/api/v0/contents?*', (route) => {
    const condition = new URL(route.request().url()).searchParams.get('condition');
    requestedConditions.push(condition ? JSON.parse(condition) : undefined);
    return route.fulfill({ json: { items: [taggedContent, content(1)] } });
  });
  await page.route(`**/api/v0/contents/${id}`, (route) => route.fulfill({ json: taggedContent }));
  await page.goto('/#/contents');
  const card = page.getByRole('link', { name: '画像 1 を開く', exact: true });
  const cardContainer = card.locator('../..');
  const panel = page.getByRole('region', { name: '画像 1 のタグ', exact: true, includeHidden: true });
  if (!isMobile) {
    await expect(panel).toBeHidden();
    await cardContainer.hover({ position: { x: 4, y: 4 } });
    await expect(panel).toBeVisible();
    await page.getByRole('link', { name: 'Galerie トップページ', exact: true }).hover();
    await expect(panel).toBeHidden();
    await card.focus();
    await expect(panel).toBeVisible();
  }
  await expect(panel).toBeVisible();
  const badges = panel.getByRole('list', { name: 'タグ', exact: true }).getByRole('listitem');
  await expect(badges).toHaveCount(10);
  expect(await badges.locator(':scope > [aria-label]').evaluateAll((items) => items.map((item) => item.getAttribute('aria-label')))).toEqual([
    '診断情報 3 件: Invalid-Key: タグ名が無効です、OrderedArray: 未対応のXMP値形式です、EmptyStringSet: タグの値を解釈できません',
    'animation', 'category: landscape', 'rating: 0', 'score: -3.14',
    'authors: Alice, Bob', 'pages: 1, 3', 'weights: 0.5, 1.5',
    'emptySet: ', `long: ${'長いタグ'.repeat(100)}`,
  ]);
  const diagnosticSummary = panel.getByLabel(/診断情報 3 件:/);
  await expect(diagnosticSummary).toHaveText('');
  await expect(diagnosticSummary.locator('.lucide-triangle-alert')).toBeVisible();
  await expect(diagnosticSummary).toHaveClass(/badge-error/);
  await expect(diagnosticSummary).not.toHaveClass(/badge-dash/);
  const tooltip = panel.getByRole('tooltip', { includeHidden: true });
  await expect(tooltip).toBeHidden();
  if (isMobile) await diagnosticSummary.focus();
  else await diagnosticSummary.hover();
  await expect(tooltip).toBeVisible();
  await expect(tooltip.getByRole('list', { name: '診断の一覧' }).getByRole('listitem')).toHaveText([
    'Invalid-Key: タグ名が無効です', 'OrderedArray: 未対応のXMP値形式です', 'EmptyStringSet: タグの値を解釈できません',
  ]);
  const tooltipBounds = await tooltip.boundingBox();
  const cardBounds = await cardContainer.boundingBox();
  expect(tooltipBounds!.y).toBeGreaterThanOrEqual(cardBounds!.y);
  expect(tooltipBounds!.y + tooltipBounds!.height).toBeLessThanOrEqual(cardBounds!.y + cardBounds!.height + 1);
  const scrollArea = panel.getByLabel('タグをスクロール');
  expect(await scrollArea.evaluate((element) => element.scrollHeight > element.clientHeight)).toBe(true);
  await scrollArea.evaluate((element) => { element.scrollTop = element.scrollHeight; });
  expect(await scrollArea.evaluate((element) => element.scrollTop)).toBeGreaterThan(0);
  const category = panel.getByLabel('category: landscape', { exact: true });
  await expect(category.locator('.lucide-tag')).toBeVisible();
  await expect(category.locator('.lucide-hash')).toBeHidden();
  for (const label of ['authors: Alice, Bob', 'pages: 1, 3', 'weights: 0.5, 1.5', 'emptySet:']) {
    await expect(panel.getByLabel(label, { exact: true }).locator('.lucide-tags')).toBeVisible();
  }
  await expect(category.getByText('landscape', { exact: true })).toBeVisible();
  await category.click();
  await expect(page).toHaveURL(/#\/contents$/);
  await expect(page.getByRole('list', { name: '検索条件（すべてに一致）' }).getByLabel('category: landscape', { exact: true })).toBeVisible();
  await expect.poll(() => requestedConditions.at(-1)).toEqual([{ kind: 'tagMatch', key: 'category', values: ['landscape'] }]);
  await page.getByRole('button', { name: '条件をクリア', exact: true }).click();
  if (!isMobile) {
    await cardContainer.hover({ position: { x: 4, y: 4 } });
    await category.hover();
    await expect(category.locator('.lucide-tag')).toBeHidden();
    await expect(category.locator('.lucide-hash')).toBeVisible();
    await expect(category.getByText('landscape', { exact: true })).toBeHidden();
    await expect(category.getByText('category', { exact: true })).toBeVisible();
    await page.getByRole('link', { name: 'Galerie トップページ' }).hover();
  }
  const { imageBounds, overlayBounds } = await panel.evaluate((element) => ({
    imageBounds: element.closest('.card')!.getBoundingClientRect().toJSON(),
    overlayBounds: element.getBoundingClientRect().toJSON(),
  }));
  expect(imageBounds).not.toBeNull();
  expect(overlayBounds).not.toBeNull();
  expect(overlayBounds!.y).toBeGreaterThanOrEqual(imageBounds!.y - 1);
  expect(overlayBounds!.height).toBeLessThanOrEqual(imageBounds!.height * 2 / 3 + 1);
  expect(overlayBounds!.y + overlayBounds!.height).toBeLessThan(imageBounds!.y + imageBounds!.height - 1);
  await expect(panel.getByText('タグ', { exact: true })).toHaveCount(0);
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
  await page.screenshot({ path: `test-results/tags-gallery-${test.info().project.name}.png`, fullPage: true });
  await card.click({ position: { x: 4, y: 4 } });
  const detail = page.getByRole('region', { name: 'タグ情報' });
  await expect(detail.getByRole('list', { name: 'タグ', exact: true }).getByRole('listitem')).toHaveCount(12);
  const detailAnimation = detail.getByLabel('animation', { exact: true });
  await expect(detailAnimation).toHaveClass(/cursor-pointer/);
  const detailCategory = detail.getByLabel('category: landscape', { exact: true });
  if (!isMobile) {
    const background = await detailAnimation.locator('.badge').evaluate((element) => getComputedStyle(element).backgroundColor);
    await detailAnimation.hover();
    await expect.poll(() => detailAnimation.locator('.badge').evaluate((element) => getComputedStyle(element).backgroundColor)).not.toBe(background);
  }
  await detailCategory.focus();
  await expect(detailCategory.locator('.lucide-tag')).toBeHidden();
  await expect(detailCategory.locator('.lucide-hash')).toBeVisible();
  await expect(detailCategory.getByText('landscape', { exact: true })).toBeHidden();
  await expect(detailCategory.getByText('category', { exact: true })).toBeVisible();
  const detailAuthors = detail.getByLabel('authors: Alice, Bob', { exact: true });
  await detailAuthors.focus();
  await expect(detailAuthors.locator('.lucide-tags')).toBeHidden();
  await expect(detailAuthors.locator('.lucide-hash')).toBeVisible();
  await expect(detailCategory.locator('.lucide-tag')).toBeVisible();
  await expect(detail.getByRole('list', { name: 'タグ', exact: true }).locator('.badge-error')).toHaveText([
    'Invalid-Key', 'OrderedArray', 'EmptyStringSet',
  ]);
  const diagnosticBadge = detail.getByLabel('Invalid-Key: タグ名が無効です', { exact: true });
  await expect(diagnosticBadge.locator('.badge-error')).toBeVisible();
  await expect(diagnosticBadge.locator('.badge')).not.toHaveClass(/badge-dash/);
  await expect(diagnosticBadge.locator('.lucide-tag')).toBeVisible();
  await expect(diagnosticBadge).toHaveAttribute('data-tip', 'タグ名が無効です');
  if (!isMobile) {
    await diagnosticBadge.hover();
    await expect.poll(() => diagnosticBadge.evaluate((element) => getComputedStyle(element, '::before').opacity)).toBe('1');
  }
  await diagnosticBadge.focus();
  await expect.poll(() => diagnosticBadge.evaluate((element) => getComputedStyle(element, '::before').opacity)).toBe('1');
  await detailAuthors.click();
  await expect(page).toHaveURL(/#\/contents$/);
  await expect.poll(() => requestedConditions.at(-1)).toEqual([{ kind: 'tagMatch', key: 'authors', values: ['Alice', 'Bob'] }]);
  await page.getByRole('button', { name: '条件をクリア', exact: true }).click();
  await card.focus();
  await panel.getByLabel('rating: 0', { exact: true }).click();
  await expect.poll(() => requestedConditions.at(-1)).toEqual([{ kind: 'tagMatch', key: 'rating', values: ['0'] }]);
  await page.getByRole('button', { name: '条件をクリア', exact: true }).click();
  await card.focus();
  await panel.getByLabel('emptySet: ', { exact: true }).click();
  await expect.poll(() => requestedConditions.at(-1)).toEqual([{ kind: 'tagExists', key: 'emptySet' }]);
  await page.getByRole('button', { name: '条件をクリア', exact: true }).click();
  await card.focus();
  await panel.getByLabel('animation', { exact: true }).click();
  await expect.poll(() => requestedConditions.at(-1)).toEqual([{ kind: 'tagExists', key: 'animation' }]);
  await card.click({ position: { x: 4, y: 4 } });
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
  await page.screenshot({ path: `test-results/tags-detail-${test.info().project.name}.png`, fullPage: true });
});

test('omits empty tag overlays and shows the empty state on the content page', async ({ page }) => {
  await mockImages(page);
  await page.route('**/api/v0/contents?*', (route) => route.fulfill({ json: { items: [content(0)] } }));
  await page.route(`**/api/v0/contents/${id}`, (route) => route.fulfill({ json: content(0) }));
  await page.goto('/#/contents');
  const card = page.getByRole('link', { name: '画像 1 を開く', exact: true });
  await card.focus();
  await expect(page.getByRole('region', { name: '画像 1 のタグ', exact: true })).toHaveCount(0);
  await card.click();
  await expect(page.getByText('タグはありません', { exact: true })).toBeVisible();
  await expect(page.getByRole('button', { name: '前のコンテンツ' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: '次のコンテンツ' })).toHaveCount(0);
});

test('starts a short tag list at the top of the thumbnail', async ({ page, isMobile }) => {
  await mockImages(page);
  const item = { ...content(0), tags: [
    { key: 'animation', type: 'keyOnly' },
    { key: 'category', type: 'text', value: 'abstract' },
  ], diagnostics: [{ key: 'Invalid-Key', kind: 'invalidKey' }] };
  await page.route('**/api/v0/contents?*', (route) => route.fulfill({ json: { items: [item] } }));
  await page.goto('/#/contents');
  const card = page.getByRole('link', { name: '画像 1 を開く', exact: true });
  if (!isMobile) await card.focus();
  const panel = page.getByRole('region', { name: '画像 1 のタグ' });
  const firstTag = panel.getByLabel('animation', { exact: true });
  const positions = await panel.evaluate((element) => ({
    panelTop: element.getBoundingClientRect().top,
    diagnosticTop: element.querySelector('.badge-error')!.getBoundingClientRect().top,
    tagTop: element.querySelector('[aria-label="animation"]')!.getBoundingClientRect().top,
    diagnosticRight: element.querySelector('.badge-error')!.getBoundingClientRect().right,
    tagLeft: element.querySelector('[aria-label="animation"]')!.getBoundingClientRect().left,
  }));
  expect(positions.diagnosticTop - positions.panelTop).toBeLessThanOrEqual(16);
  expect(Math.abs(positions.tagTop - positions.diagnosticTop)).toBeLessThanOrEqual(2);
  expect(positions.tagLeft).toBeGreaterThan(positions.diagnosticRight);
  await expect(firstTag).toBeVisible();
  expect(await panel.getByLabel('タグをスクロール').evaluate((element) => element.scrollHeight <= element.clientHeight)).toBe(true);
});

test('shows an overlay for content with only diagnostics', async ({ page }) => {
  await mockImages(page);
  const item = { ...content(0), diagnostics: [
    { key: 'Invalid-Key', kind: 'invalidKey' },
    { key: 'category', kind: 'notAllowedTagValue', definition: { key: 'category', type: 'text', allowedValues: ['landscape'] } },
    { key: 'author', kind: 'missingRequiredTag', definition: { key: 'author', type: 'text' } },
  ] };
  await page.route('**/api/v0/contents?*', (route) => route.fulfill({ json: { items: [item] } }));
  await page.goto('/#/contents');
  await page.getByRole('link', { name: '画像 1 を開く', exact: true }).focus();
  const panel = page.getByRole('region', { name: '画像 1 のタグ', exact: true });
  const badge = panel.getByLabel(/診断情報 3 件:/);
  await expect(badge).toHaveText('');
  await expect(badge.locator('.lucide-triangle-alert')).toBeVisible();
  await expect(badge).not.toHaveClass(/badge-dash/);
  await badge.focus();
  await expect(panel).toBeVisible();
  await expect(badge).toBeVisible();
  await expect(panel.getByRole('tooltip').getByRole('listitem')).toHaveText([
    'Invalid-Key: タグ名が無効です', 'category: タグの値がスキーマの制約を満たしていません', 'author: 必須タグがありません',
  ]);
  await expect(panel.getByRole('button')).toHaveCount(0);
  await expect(page.getByText('タグはありません', { exact: true })).toHaveCount(0);
});

test('outlines only diagnostics for tags absent from XMP on the content page', async ({ page }) => {
  const item = { ...content(0), diagnostics: [
    { key: 'Count', kind: 'unparseableTagValue' },
    { key: 'Count', kind: 'missingRequiredTag', definition: { key: 'Count', type: 'integer' } },
    { key: 'author', kind: 'missingRequiredTag', definition: { key: 'author', type: 'text' } },
  ] };
  await page.route(`**/api/v0/contents/${id}`, (route) => route.fulfill({ json: item }));
  await page.goto(`/#/contents/${id}`);
  const detail = page.getByRole('region', { name: 'タグ情報' });
  for (const name of ['Count: タグの値を解釈できません', 'Count: 必須タグがありません']) {
    await expect(detail.getByLabel(name, { exact: true }).locator('.badge')).not.toHaveClass(/badge-dash/);
  }
  await expect(detail.getByLabel('author: 必須タグがありません', { exact: true }).locator('.badge')).toHaveClass(/badge-dash/);
});

test('theme toggle persists across reloads and overrides later system changes', async ({ page }) => {
  await page.route('**/api/v0/contents?*', (route) => route.fulfill({ json: { items: [] } }));
  for (const colorScheme of ['light', 'dark'] as const) {
    await page.emulateMedia({ colorScheme });
    await page.goto('/');
    await page.evaluate(() => localStorage.removeItem('galerie-theme'));
    await page.reload();
    const toggle = page.getByRole('checkbox', { name: 'ダークモード' });
    await expect(toggle).toBeChecked({ checked: colorScheme === 'dark' });
    const scheme = () => page.evaluate(() => getComputedStyle(document.documentElement).colorScheme);
    await expect.poll(scheme).toBe(colorScheme);
    const chosenTheme = colorScheme === 'dark' ? 'light' : 'dark';
    await toggle.setChecked(chosenTheme === 'dark');
    await expect.poll(scheme).toBe(chosenTheme);
    await expect.poll(() => page.evaluate(() => localStorage.getItem('galerie-theme'))).toBe(chosenTheme);
    await page.reload();
    await expect(toggle).toBeChecked({ checked: chosenTheme === 'dark' });
    await expect.poll(scheme).toBe(chosenTheme);
    await page.emulateMedia({ colorScheme: chosenTheme === 'dark' ? 'light' : 'dark' });
    await page.reload();
    await expect(toggle).toBeChecked({ checked: chosenTheme === 'dark' });
    await expect.poll(scheme).toBe(chosenTheme);
    await page.getByRole('link', { name: 'ギャラリーを開く' }).click();
    await expect(toggle).toBeChecked({ checked: chosenTheme === 'dark' });
    await expect.poll(scheme).toBe(chosenTheme);
  }
});

test('fullscreen shows the image and restores the detail view on exit', async ({ page }) => {
  await mockImages(page);
  await page.route(`**/api/v0/contents/${id}`, (route) => route.fulfill({ json: content(0) }));
  await page.goto(`/#/contents/${id}`);
  await expect(page.getByRole('button', { name: '全画面表示', exact: true }).locator('svg')).toHaveClass(/lucide-fullscreen/);
  await page.getByRole('button', { name: '全画面表示', exact: true }).click();
  const fullscreen = page.locator(':fullscreen');
  await expect(fullscreen).toHaveCount(1);
  await expect(fullscreen.getByRole('img', { name: `コンテンツ ${id}`, exact: true })).toBeVisible();
  await expect(fullscreen.getByRole('button', { name: '全画面表示を解除', exact: true }).locator('svg')).toHaveClass(/lucide-minimize/);
  await fullscreen.getByRole('button', { name: '全画面表示を解除', exact: true }).click();
  await expect(fullscreen).toHaveCount(0);
  await expect(page.getByRole('button', { name: '全画面表示', exact: true })).toBeVisible();
  await page.getByRole('button', { name: '全画面表示', exact: true }).click();
  await expect(fullscreen).toHaveCount(1);
  await page.evaluate(() => document.exitFullscreen());
  await expect(fullscreen).toHaveCount(0);
  await expect(page.getByRole('button', { name: '全画面表示', exact: true })).toBeVisible();
});

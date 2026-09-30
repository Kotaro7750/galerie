import { expect, test } from '@playwright/test';
import { mockContentAccess } from './content_access';
import { enterAdditionalTagKey, mockTagSchema } from './tag_schema';

const sample = '../sample/00065786-f916-4e2c-85bc-3db5e4c0cb71.avif';
const id = '10000000-0000-4000-8000-000000000001';

test.beforeEach(async ({ page }) => { await mockContentAccess(page); await mockTagSchema(page); });

test('selects and previews a local AVIF, then sends typed tags only on registration', async ({ page }) => {
  await mockTagSchema(page, { version: '0', allowAdditionalTags: true, required: [], optional: [{ key: 'rating', type: 'integer' }] });
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
  const caption = page.locator('figcaption');
  await expect(caption.getByLabel('表示中の画像')).toHaveText('1 / 1');
  const [positionBounds, sizeBounds, resolutionBounds] = await Promise.all([
    caption.getByLabel('表示中の画像').boundingBox(),
    caption.getByRole('img', { name: 'ファイルサイズ' }).boundingBox(),
    caption.getByRole('img', { name: '解像度（ピクセル）' }).boundingBox(),
  ]);
  expect(positionBounds).not.toBeNull();
  expect(sizeBounds).not.toBeNull();
  expect(resolutionBounds).not.toBeNull();
  expect(positionBounds!.x).toBeLessThan(sizeBounds!.x);
  expect(sizeBounds!.x).toBeLessThan(resolutionBounds!.x);
  expect(sizeBounds!.y).toBeCloseTo(resolutionBounds!.y, 0);
  const [captionBounds, previewBounds] = await Promise.all([caption.boundingBox(), page.getByRole('img', { name: '登録するコンテンツのプレビュー' }).boundingBox()]);
  expect(captionBounds).not.toBeNull();
  expect(previewBounds).not.toBeNull();
  expect(captionBounds!.y + captionBounds!.height).toBeLessThanOrEqual(previewBounds!.y);
  await expect(caption).not.toContainText('00065786-f916-4e2c-85bc-3db5e4c0cb71.avif');
  await expect(caption.getByRole('img', { name: 'ファイルサイズ' })).toBeVisible();
  await expect(caption.getByText(/MiB/)).toBeVisible();
  await expect(caption.getByRole('img', { name: '解像度（ピクセル）' })).toBeVisible();
  await expect(page.getByRole('button', { name: '登録する' }).locator('svg')).toHaveClass(/lucide-save/);
  await enterAdditionalTagKey(page, 'category');
  await page.getByLabel('タグの型').selectOption('text');
  await page.getByLabel('値 1', { exact: true }).fill('landscape');
  await page.getByRole('button', { name: 'タグを追加' }).click();
  await page.getByLabel('タグ名', { exact: true }).fill('rating');
  await expect(page.getByLabel('タグの型')).toHaveCount(0);
  await page.getByLabel('値 1', { exact: true }).fill('5');
  await page.getByRole('button', { name: 'タグを追加' }).click();
  await enterAdditionalTagKey(page, 'authors');
  await page.getByLabel('タグの型').selectOption('textSet');
  await page.getByLabel('値 1', { exact: true }).fill('Alice');
  await page.getByRole('button', { name: '値を追加' }).click();
  await page.getByLabel('値 2', { exact: true }).fill('Alice');
  await expect(page.getByText('集合に同じ値は追加できません。').first()).toBeVisible();
  await page.getByLabel('値 2', { exact: true }).fill('Bob');
  await expect(page.getByText('集合に同じ値は追加できません。')).toHaveCount(0);
  await page.getByRole('button', { name: 'タグを追加' }).click();
  await expect(page.getByLabel('category: landscape').locator('.badge')).toHaveClass(/badge-accent/);
  await expect(page.getByLabel('rating: 5').locator('.badge')).toHaveClass(/badge-secondary/);
  await expect(page.getByLabel('authors: Alice, Bob').locator('.badge')).toHaveClass(/badge-accent/);
  expect(posts).toHaveLength(0);
  await page.getByRole('button', { name: '登録する' }).click();
  await expect(page).toHaveURL(new RegExp(`#\\/contents\\/${id}$`));
  expect(posts).toHaveLength(1);
  expect(posts[0].contentType).toContain('multipart/form-data; boundary=');
  expect(posts[0].body).toContain('Content-Type: image/avif');
  expect(posts[0].body).toContain('Content-Type: application/json');
  expect(posts[0].body).toContain('"tags":[{"key":"category","type":"text","value":"landscape"},{"key":"rating","type":"integer","value":5},{"key":"authors","type":"textSet","values":["Alice","Bob"]}]');
});

test('blocks registration until required and defined tags satisfy the schema', async ({ page }) => {
  await mockTagSchema(page, {
    version: '0', allowAdditionalTags: false,
    required: [{ key: 'rating', type: 'integer', min: 1, max: 5 }],
    optional: [{ key: 'category', type: 'text', allowedValues: ['landscape', 'abstract'] }, { key: 'title', type: 'text', minLength: 2, maxLength: 5 }],
  });
  let posts = 0;
  await page.route('**/api/v0/contents', (route) => { posts++; return route.fulfill({ status: 201, json: { id, mediaType: 'image/avif', contentUrl: '/created.avif', thumbnailUrl: '/created.avif', tags: [], diagnostics: [] } }); });
  await page.goto('/#/contents/new');
  await page.getByLabel('アップロードするファイル').setInputFiles(sample);
  await expect(page.getByRole('button', { name: '登録する' })).toBeDisabled();
  await expect(page.getByLabel('タグスキーマの確認結果')).toContainText('必須タグ rating を追加してください。');
  const key = page.getByLabel('タグ名', { exact: true });
  await key.focus();
  await expect(page.getByRole('group', { name: 'Required' }).getByRole('option', { name: 'rating' })).toBeVisible();
  await expect(page.getByRole('group', { name: 'Optional' }).getByRole('option', { name: 'category' })).toBeVisible();
  await expect(page.getByRole('group', { name: 'Required' }).getByText('Required')).toHaveClass(/bg-base-200/);
  await expect(page.getByRole('group', { name: 'Optional' }).getByText('Optional')).toHaveClass(/bg-base-200/);
  await key.fill('rating');
  await expect(key.locator('..').locator('svg')).toHaveClass(/text-primary/);
  await expect(page.getByLabel('タグの型')).toHaveCount(0);
  await expect(page.getByRole('img', { name: '必須タグ', exact: true })).toHaveCount(0);
  await expect(page.getByRole('img', { name: '必須の整数タグ' })).toHaveCount(0);
  await expect(page.getByText('必須 · 整数')).toHaveCount(0);
  await expect(page.getByLabel('値 1', { exact: true })).toHaveAttribute('type', 'number');
  await expect(page.getByLabel('値 1', { exact: true })).toHaveAttribute('min', '1');
  await expect(page.getByLabel('値 1', { exact: true })).toHaveAttribute('max', '5');
  await page.getByLabel('値 1', { exact: true }).fill('0');
  await expect(page.getByLabel('値 1', { exact: true })).toHaveAttribute('aria-invalid', 'true');
  await expect(page.getByText('1以上で入力してください。')).toBeVisible();
  await page.getByRole('button', { name: 'タグを追加' }).click();
  await expect(page.getByRole('alert')).toContainText('1以上で入力してください。');
  await page.getByLabel('値 1', { exact: true }).fill('6');
  await page.getByRole('button', { name: 'タグを追加' }).click();
  await expect(page.getByRole('alert')).toContainText('5以下で入力してください。');
  await page.getByLabel('値 1', { exact: true }).fill('3');
  await expect(page.getByLabel('値 1', { exact: true })).toHaveAttribute('aria-invalid', 'false');
  await expect(page.locator('.validator-hint')).toHaveCount(0);
  await page.getByRole('button', { name: 'タグを追加' }).click();
  await expect(page.getByLabel('rating: 3').locator('.badge')).toHaveClass(/badge-primary/);
  await key.fill('rating');
  expect(await key.locator('..').locator('svg').evaluate((icon) => getComputedStyle(icon).color)).toBe(
    await page.getByLabel('rating: 3').locator('.badge').evaluate((badge) => getComputedStyle(badge).color));
  await expect(page.getByRole('button', { name: '登録する' })).toBeEnabled();
  await key.fill('category');
  await expect(key.locator('..').locator('svg')).toHaveClass(/text-secondary/);
  await expect(page.getByRole('img', { name: '必須タグ', exact: true })).toHaveCount(0);
  await expect(page.getByRole('img', { name: '任意のテキストタグ' })).toHaveCount(0);
  await expect(page.getByLabel('値 1', { exact: true })).toHaveJSProperty('tagName', 'SELECT');
  await expect(page.getByLabel('値 1', { exact: true }).locator('option')).toContainText(['値を選択', 'landscape', 'abstract']);
  expect(posts).toBe(0);
  await page.getByLabel('値 1', { exact: true }).selectOption('landscape');
  await page.getByRole('button', { name: 'タグを追加' }).click();
  await key.fill('title');
  await page.getByLabel('値 1', { exact: true }).fill('x');
  await expect(page.getByLabel('値 1', { exact: true })).toHaveAttribute('aria-invalid', 'true');
  await expect(page.getByText('2文字以上で入力してください。')).toBeVisible();
  await page.getByRole('button', { name: 'タグを追加' }).click();
  await expect(page.getByRole('alert')).toContainText('2文字以上で入力してください。');
  await page.getByLabel('値 1', { exact: true }).fill('too long');
  await page.getByRole('button', { name: 'タグを追加' }).click();
  await expect(page.getByRole('alert')).toContainText('5文字以下で入力してください。');
  await page.getByLabel('値 1', { exact: true }).fill('valid');
  await page.getByRole('button', { name: 'タグを追加' }).click();
  await page.getByRole('button', { name: '登録する' }).click();
  await expect.poll(() => posts).toBe(1);
});

test('filters grouped key suggestions and accepts a new key directly', async ({ page, isMobile }) => {
  if (isMobile) await page.setViewportSize({ width: 375, height: 667 });
  await mockTagSchema(page, {
    version: '0', allowAdditionalTags: true,
    required: [{ key: 'subjects', type: 'textSet' }],
    optional: [{ key: 'category', type: 'text' }],
  });
  await page.goto('/#/contents/new');
  const key = page.getByLabel('タグ名', { exact: true });
  await expect(key).toHaveJSProperty('tagName', 'INPUT');
  const typeBounds = await page.getByLabel('タグの型').boundingBox();
  const keyBounds = await key.boundingBox();
  expect(typeBounds).not.toBeNull();
  expect(keyBounds).not.toBeNull();
  if (isMobile) expect(keyBounds!.y + keyBounds!.height).toBeLessThan(typeBounds!.y);
  else expect(keyBounds!.x + keyBounds!.width).toBeLessThan(typeBounds!.x);
  await expect(page.getByLabel('タグの型').locator('..').locator('svg')).toHaveClass(/lucide-list-filter/);
  await expect(page.getByText('タグの型', { exact: true })).toHaveCount(0);
  await key.fill('subjects');
  await expect(key.locator('..').locator('svg')).toHaveClass(/text-primary/);
  await expect(page.getByRole('img', { name: '必須タグ', exact: true })).toHaveCount(0);
  await expect(page.getByRole('img', { name: '必須のテキスト集合タグ' })).toHaveCount(0);
  await key.fill('category');
  await expect(key.locator('..').locator('svg')).toHaveClass(/text-secondary/);
  await expect(page.getByLabel('タグの型')).toHaveCount(0);
  expect((await key.boundingBox())!.x).toBe(keyBounds!.x);
  await key.fill('');
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
  await key.focus();
  if (isMobile) {
    const suggestions = await page.getByRole('listbox', { name: 'タグ名の候補' }).boundingBox();
    expect(suggestions!.width).toBeGreaterThan(200);
    expect(suggestions!.x + suggestions!.width).toBeLessThanOrEqual(page.viewportSize()!.width);
  }
  await expect(page.getByRole('group', { name: 'Required' }).getByRole('option', { name: 'subjects' })).toBeVisible();
  await expect(page.getByRole('group', { name: 'Optional' }).getByRole('option', { name: 'category' })).toBeVisible();
  await enterAdditionalTagKey(page, 'other');
  await expect(key.locator('..').locator('svg')).toHaveClass(/text-accent/);
  await expect(page.getByRole('listbox', { name: 'タグ名の候補' })).toContainText('候補にないタグ名も入力できます');
  await expect(page.getByLabel('タグの型').locator('option')).toContainText(['キーのみ', 'テキスト', 'テキスト集合']);
  await expect(page.getByLabel('タグの型').locator('option')).toHaveCount(3);
  await page.getByLabel('タグの型').selectOption('textSet');
  expect(await page.getByLabel('タグの型').evaluate((select: HTMLSelectElement) => {
    const style = getComputedStyle(select);
    const canvas = document.createElement('canvas');
    const context = canvas.getContext('2d')!;
    context.font = style.font;
    return select.clientWidth - parseFloat(style.paddingLeft) - parseFloat(style.paddingRight) >= context.measureText(select.selectedOptions[0].text).width;
  })).toBe(true);
  await page.getByLabel('タグの型').selectOption('text');
  await page.getByLabel('値 1', { exact: true }).fill('free');
  await page.getByRole('button', { name: 'タグを追加' }).click();
  await expect(page.getByLabel('other: free')).toBeVisible();
  await expect(page.getByLabel('other: free').locator('.badge')).toHaveClass(/badge-accent/);
  const actions = page.getByRole('button', { name: 'other を編集' }).locator('..');
  if (isMobile) {
    await expect(actions).toHaveCSS('opacity', '0');
    await page.getByLabel('other: free').tap();
  } else {
    await page.getByRole('heading', { name: 'タグ', exact: true }).hover();
    await expect(actions).toHaveCSS('opacity', '0');
    await page.getByLabel('other: free').focus();
  }
  await expect(actions).toHaveCSS('opacity', '1');
  await expect(page.getByRole('button', { name: 'other を削除' })).toBeVisible();
  await page.getByRole('button', { name: 'other を編集' }).click();
  await expect(key).toHaveValue('other');
  await expect(page.getByLabel('値 1', { exact: true })).toHaveValue('free');
  await page.getByRole('button', { name: 'タグを追加' }).click();
  if (isMobile) await page.getByLabel('other: free').tap();
  else await page.getByLabel('other: free').hover();
  await page.getByRole('button', { name: 'other を削除' }).click();
  await expect(page.getByLabel('other: free')).toHaveCount(0);
});

test('tag actions do not leave gaps between badges until hover', async ({ page, isMobile }) => {
  test.skip(isMobile, 'touch controls remain visible');
  await page.goto('/#/contents/new');
  for (const key of ['firstTag', 'secondTag']) {
    await enterAdditionalTagKey(page, key);
    await page.getByRole('button', { name: 'タグを追加' }).click();
  }
  const tags = page.getByRole('list', { name: '登録するタグ' }).getByRole('listitem');
  const first = tags.nth(0);
  const second = tags.nth(1);
  const firstBounds = await first.boundingBox();
  const badgeBounds = await first.getByLabel('firstTag', { exact: true }).boundingBox();
  const secondBefore = await second.boundingBox();
  expect(firstBounds && badgeBounds && secondBefore).toBeTruthy();
  expect(firstBounds!.width).toBeCloseTo(badgeBounds!.width, 0);
  expect(secondBefore!.x - firstBounds!.x - firstBounds!.width).toBeCloseTo(12, 0);
  await first.getByLabel('firstTag', { exact: true }).hover();
  await expect(first.getByRole('button', { name: 'firstTag を編集' }).locator('..')).toHaveCSS('opacity', '1');
  expect((await second.boundingBox())!.x).toBe(secondBefore!.x);
  await first.getByRole('button', { name: 'firstTag を編集' }).click();
  await expect(page.getByLabel('タグ名', { exact: true })).toHaveValue('firstTag');
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
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toBeVisible();
  await expect(page.getByRole('button', { name: '登録する' })).toBeEnabled();
});

test('clears the URL while keeping the prepared image', async ({ page }) => {
  await page.route('**/source.avif', (route) => route.fulfill({ path: sample, contentType: 'image/avif' }));
  await page.goto('/#/contents/new');
  await page.getByRole('button', { name: 'URL から追加' }).click();
  await page.getByLabel('コンテンツ URL').fill(new URL('/source.avif', page.url()).href);
  await page.getByRole('button', { name: 'URL をプレビュー' }).click();
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toBeVisible();
  await page.getByRole('button', { name: 'URL をクリア' }).click();
  await expect(page.getByLabel('コンテンツ URL')).toHaveValue('');
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toBeVisible();
  await expect(page.getByRole('button', { name: '登録する' })).toBeEnabled();
});

test('shows the centered loading component while preparing a URL image', async ({ page }) => {
  let release!: () => void;
  const pending = new Promise<void>((resolve) => { release = resolve; });
  await page.route('**/slow.avif', async (route) => {
    await pending;
    await route.fulfill({ path: sample, contentType: 'image/avif' });
  });
  await page.goto('/#/contents/new');
  await page.getByRole('button', { name: 'URL から追加' }).click();
  await page.getByLabel('コンテンツ URL').fill(new URL('/slow.avif', page.url()).href);
  await page.getByRole('button', { name: 'URL をプレビュー' }).click();
  const loading = page.getByRole('status').filter({ hasText: 'コンテンツを準備中…' });
  await expect(loading.locator('.loading')).toBeVisible();
  await expect(loading).toHaveCSS('justify-content', 'center');
  release();
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toBeVisible();
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
  await expect(page.getByRole('img', { name: '解像度（ピクセル）' })).toBeVisible();
  await expect(page.getByRole('img', { name: '解像度（ピクセル）' })).toHaveClass(/lucide-ruler/);
  await expect(page.getByText('12 × 12', { exact: true })).toBeVisible();
  await expect(page.locator('figcaption')).not.toContainText('clipboard.avif');
  await expect(page.getByRole('button', { name: '登録する' })).toBeEnabled();
  await page.getByRole('button', { name: 'URL から追加' }).click();
  await expect(page.getByLabel('コンテンツ URL')).toHaveValue('');
  await expect(page.getByRole('img', { name: '解像度（ピクセル）' })).toBeVisible();
});

test('adds multiple images, shows progress, and retries only failures', async ({ page }) => {
  let posts = 0;
  let releaseSecond!: () => void;
  const secondPending = new Promise<void>((resolve) => { releaseSecond = resolve; });
  await page.route('**/api/v0/contents', async (route) => {
    posts++;
    if (posts === 1) {
      await route.fulfill({ status: 400, contentType: 'application/problem+json', body: JSON.stringify({ status: 400, title: 'Invalid', detail: 'First failed.' }) });
    } else {
      if (posts === 2) await secondPending;
      await route.fulfill({ status: 201, json: { id, mediaType: 'image/avif', contentUrl: '/created.avif', thumbnailUrl: '/created.avif', tags: [], diagnostics: [] } });
    }
  });
  await page.route(`**/api/v0/contents/${id}`, (route) => route.fulfill({ json: { id, mediaType: 'image/avif', contentUrl: '/created.avif', thumbnailUrl: '/created.avif', tags: [], diagnostics: [] } }));
  await page.route('**/created.avif', (route) => route.fulfill({ path: sample, contentType: 'image/avif' }));
  await page.goto('/#/contents/new');
  await page.getByLabel('アップロードするファイル').setInputFiles([sample, sample]);
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toHaveCount(2);
  await page.getByRole('button', { name: '次の画像' }).click();
  await page.getByRole('button', { name: '前の画像' }).click();
  await page.getByRole('button', { name: '登録する' }).click();
  await expect(page.getByRole('progressbar')).toHaveAttribute('value', '1');
  await expect(page.getByRole('status').filter({ has: page.getByRole('progressbar') })).toContainText('1 / 2');
  releaseSecond();
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toHaveCount(1);
  await expect(page.getByRole('alert')).toHaveText('First failed.');
  await expect(page.getByRole('button', { name: '登録する' })).toBeEnabled();
  expect(posts).toBe(2);
  await page.getByRole('button', { name: '登録する' }).click();
  await expect(page).toHaveURL(new RegExp(`#\\/contents\\/${id}$`));
  expect(posts).toBe(3);
});

test('removes one selected image from the carousel', async ({ page }) => {
  await page.goto('/#/contents/new');
  await page.getByLabel('アップロードするファイル').setInputFiles([sample, sample]);
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toHaveCount(2);
  const activeSlide = page.locator('.swiper-slide-active');
  const image = activeSlide.getByRole('img', { name: '登録するコンテンツのプレビュー' });
  const remove = activeSlide.getByRole('button', { name: '1 件目のコンテンツを削除' });
  const [imageBounds, buttonBounds] = await Promise.all([image.boundingBox(), remove.boundingBox()]);
  expect(imageBounds).not.toBeNull();
  expect(buttonBounds).not.toBeNull();
  expect(buttonBounds!.x).toBeGreaterThanOrEqual(imageBounds!.x);
  expect(buttonBounds!.x + buttonBounds!.width).toBeLessThanOrEqual(imageBounds!.x + imageBounds!.width);
  expect(buttonBounds!.y).toBeGreaterThanOrEqual(imageBounds!.y);
  expect(buttonBounds!.y + buttonBounds!.height).toBeLessThanOrEqual(imageBounds!.y + imageBounds!.height);
  await expect(remove).toHaveClass(/btn-circle/);
  await remove.click();
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toHaveCount(1);
  await page.getByRole('button', { name: '1 件目のコンテンツを削除' }).click();
  await expect(page.getByRole('button', { name: '登録する' })).toBeDisabled();
});

test('carousel buttons wrap at both ends and show the current position', async ({ page }) => {
  await page.goto('/#/contents/new');
  await page.getByLabel('アップロードするファイル').setInputFiles([sample, sample]);
  const position = page.getByLabel('表示中の画像');
  await expect(position).toHaveText('1 / 2');
  const imageBounds = await page.locator('.swiper-slide-active').getByRole('img', { name: '登録するコンテンツのプレビュー' }).boundingBox();
  const [previousBounds, nextBounds] = await Promise.all([
    page.getByRole('button', { name: '前の画像' }).boundingBox(),
    page.getByRole('button', { name: '次の画像' }).boundingBox(),
  ]);
  expect(imageBounds).not.toBeNull();
  expect(previousBounds).not.toBeNull();
  expect(nextBounds).not.toBeNull();
  expect(previousBounds!.x).toBeGreaterThanOrEqual(imageBounds!.x);
  expect(nextBounds!.x + nextBounds!.width).toBeLessThanOrEqual(imageBounds!.x + imageBounds!.width);
  expect(previousBounds!.y).toBeGreaterThan(imageBounds!.y);
  expect(nextBounds!.y + nextBounds!.height).toBeLessThan(imageBounds!.y + imageBounds!.height);
  await page.getByRole('button', { name: '前の画像' }).click();
  expect(parseFloat(await page.locator('.swiper-wrapper').evaluate((element) => element.style.transitionDuration))).toBeGreaterThan(0);
  await expect(page.locator('.swiper-slide-active').getByRole('button', { name: '2 件目のコンテンツを削除' })).toBeVisible();
  await expect(position).toHaveText('2 / 2');
  await page.getByRole('button', { name: '次の画像' }).click();
  await expect(page.locator('.swiper-slide-active').getByRole('button', { name: '1 件目のコンテンツを削除' })).toBeVisible();
  await expect(position).toHaveText('1 / 2');
  await page.getByRole('button', { name: '次の画像' }).click();
  await expect(position).toHaveText('2 / 2');
});

test('carousel height follows the displayed image', async ({ page }) => {
  await page.goto('/#/contents/new');
  const pngs = await page.evaluate(() => [[200, 100], [100, 250]].map(([width, height]) => {
    const canvas = document.createElement('canvas');
    canvas.width = width; canvas.height = height;
    canvas.getContext('2d')!.fillRect(0, 0, width, height);
    return canvas.toDataURL('image/png').split(',')[1];
  }));
  await page.getByLabel('アップロードするファイル').setInputFiles(pngs.map((png, index) => ({ name: `image-${index}.png`, mimeType: 'image/png', buffer: Buffer.from(png, 'base64') })));
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toHaveCount(2);
  const wrapper = page.locator('.swiper-wrapper');
  const active = page.locator('.swiper-slide-active');
  await expect(active.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toHaveJSProperty('naturalHeight', 100);
  await expect.poll(async () => Math.abs((await wrapper.boundingBox())!.height - (await active.boundingBox())!.height)).toBeLessThan(2);
  const firstHeight = (await wrapper.boundingBox())!.height;
  await page.getByRole('button', { name: '次の画像' }).click();
  await expect(active.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toHaveJSProperty('naturalHeight', 250);
  await expect.poll(async () => Math.abs((await wrapper.boundingBox())!.height - (await active.boundingBox())!.height)).toBeLessThan(2);
  expect((await wrapper.boundingBox())!.height).toBeGreaterThan(firstHeight + 50);
});

test('mobile touch swipe moves to the next draft image', async ({ page }) => {
  test.skip(test.info().project.name !== 'mobile');
  await page.goto('/#/contents/new');
  await page.getByLabel('アップロードするファイル').setInputFiles([sample, sample]);
  await expect(page.locator('.swiper-slide-active').getByRole('button', { name: '1 件目のコンテンツを削除' })).toBeVisible();
  const box = await page.locator('.swiper').boundingBox();
  expect(box).not.toBeNull();
  const session = await page.context().newCDPSession(page);
  const y = box!.y + box!.height / 2;
  async function flick(left: boolean) {
    const startX = box!.x + box!.width * (left ? 0.8 : 0.2);
    const endX = box!.x + box!.width * (left ? 0.2 : 0.8);
    await session.send('Input.dispatchTouchEvent', { type: 'touchStart', touchPoints: [{ x: startX, y }] });
    for (let step = 1; step <= 5; step++) {
      await session.send('Input.dispatchTouchEvent', { type: 'touchMove', touchPoints: [{ x: startX + (endX - startX) * step / 5, y }] });
    }
    await session.send('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
  }
  await flick(true);
  await expect(page.locator('.swiper-slide-active').getByRole('button', { name: '2 件目のコンテンツを削除' })).toBeVisible();
  await expect(page.getByLabel('表示中の画像')).toHaveText('2 / 2');
  await flick(true);
  await expect(page.locator('.swiper-slide-active').getByRole('button', { name: '1 件目のコンテンツを削除' })).toBeVisible();
  await expect(page.getByLabel('表示中の画像')).toHaveText('1 / 2');
  await flick(false);
  await expect(page.locator('.swiper-slide-active').getByRole('button', { name: '2 件目のコンテンツを削除' })).toBeVisible();
});

test('adds a URL image after a file and navigates after both registrations succeed', async ({ page }) => {
  const ids = [id, '10000000-0000-4000-8000-000000000002'];
  let posts = 0;
  await page.route('**/source.avif', (route) => route.fulfill({ path: sample, contentType: 'image/avif' }));
  await page.route('**/api/v0/contents', (route) => {
    const createdId = ids[posts++];
    return route.fulfill({ status: 201, json: { id: createdId, mediaType: 'image/avif', contentUrl: '/created.avif', thumbnailUrl: '/created.avif', tags: [], diagnostics: [] } });
  });
  await page.route(`**/api/v0/contents/${ids[1]}`, (route) => route.fulfill({ json: { id: ids[1], mediaType: 'image/avif', contentUrl: '/created.avif', thumbnailUrl: '/created.avif', tags: [], diagnostics: [] } }));
  await page.route('**/created.avif', (route) => route.fulfill({ path: sample, contentType: 'image/avif' }));
  await page.goto('/#/contents/new');
  await page.getByLabel('アップロードするファイル').setInputFiles(sample);
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toHaveCount(1);
  await page.getByRole('button', { name: 'URL から追加' }).click();
  await page.getByLabel('コンテンツ URL').fill(new URL('/source.avif', page.url()).href);
  await page.getByRole('button', { name: 'URL をプレビュー' }).click();
  await expect(page.getByRole('img', { name: '登録するコンテンツのプレビュー' })).toHaveCount(2);
  expect(posts).toBe(0);
  await page.getByRole('button', { name: '登録する' }).click();
  await expect(page).toHaveURL(new RegExp(`#\\/contents\\/${ids[1]}$`));
  expect(posts).toBe(2);
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
  await enterAdditionalTagKey(page, 'category');
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
  await expect(page.getByText('16 × 16', { exact: true })).toBeVisible();
  await expect(page.locator('figcaption')).not.toContainText('sample.avif');
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
  await expect(page.locator('figcaption')).not.toContainText('source.avif');
  expect(posts).toBe(0);
});

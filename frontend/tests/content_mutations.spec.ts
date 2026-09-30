import { expect, test } from '@playwright/test';
import { mockContentAccess } from './content_access';
import { mockTagSchema } from './tag_schema';

const firstId = '10000000-0000-4000-8000-000000000001';
const secondId = '10000000-0000-4000-8000-000000000002';
const content = (id: string) => ({ id, mediaType: 'image/avif', contentUrl: 'https://media.test/item.avif', thumbnailUrl: 'https://media.test/item.avif', tags: [{ key: 'category', type: 'text', value: 'old' }], diagnostics: [] });

test.beforeEach(async ({ page }) => {
  await mockContentAccess(page);
  await mockTagSchema(page, { version: '0', allowAdditionalTags: true, required: [], optional: [{ key: 'category', type: 'text' }] });
  await page.route('https://media.test/**', (route) => route.fulfill({ path: '../sample/00065786-f916-4e2c-85bc-3db5e4c0cb71.avif', contentType: 'image/avif' }));
  await page.route('**/api/v0/contents?*', (route) => route.fulfill({ json: { items: [content(firstId), content(secondId)] } }));
  await page.route(`**/api/v0/contents/${firstId}`, (route) => route.fulfill({ json: content(firstId) }));
});

test('requires confirmation before deleting a detail item', async ({ page }) => {
  const deleted: string[] = [];
  await page.route(`**/api/v0/contents/${firstId}`, (route) => {
    if (route.request().method() === 'DELETE') { deleted.push(firstId); return route.fulfill({ status: 204 }); }
    return route.fulfill({ json: content(firstId) });
  });
  await page.goto(`/#/contents/${firstId}`);
  const deleteButton = page.getByRole('button', { name: 'コンテンツを削除' });
  const originalLink = page.getByRole('link', { name: '元の画像を開く' });
  await expect(deleteButton).toHaveClass(/text-error/);
  expect((await deleteButton.boundingBox())!.x).toBeGreaterThan((await originalLink.boundingBox())!.x);
  const tag = await page.getByLabel('category: old').boundingBox();
  const edit = await page.getByRole('link', { name: 'タグを編集' }).boundingBox();
  expect(Math.abs(tag!.y + tag!.height / 2 - edit!.y - edit!.height / 2)).toBeLessThan(2);
  await deleteButton.click();
  const dialog = page.getByRole('dialog', { name: 'コンテンツを削除' });
  await expect(dialog).toHaveJSProperty('tagName', 'DIALOG');
  await expect(dialog).toHaveClass(/modal/);
  expect(await dialog.evaluate((element) => element.matches(':modal'))).toBe(true);
  await expect(dialog.getByRole('img', { name: '1 件目のサムネイル' })).toBeVisible();
  expect(deleted).toEqual([]);
  await page.keyboard.press('Escape');
  await expect(dialog).toHaveCount(0);
  await page.getByRole('button', { name: 'コンテンツを削除' }).click();
  await dialog.getByRole('button', { name: 'キャンセル' }).click();
  expect(deleted).toEqual([]);
  await page.getByRole('button', { name: 'コンテンツを削除' }).click();
  await dialog.getByRole('button', { name: '1 件を削除' }).click();
  await expect(page).toHaveURL(/#\/contents$/);
  expect(deleted).toEqual([firstId]);
});

test('replaces all tags on a single item', async ({ page, isMobile }) => {
  const patches: unknown[] = [];
  const withInvalidTag = { ...content(firstId), diagnostics: [{ key: 'Invalid-Key', kind: 'invalidKey' }] };
  await page.route(`**/api/v0/contents/${firstId}`, (route) => {
    if (route.request().method() === 'PATCH') { patches.push(route.request().postDataJSON()); return route.fulfill({ json: { ...withInvalidTag, tags: [{ key: 'category', type: 'text', value: 'new' }], diagnostics: [] } }); }
    return route.fulfill({ json: withInvalidTag });
  });
  await page.goto(`/#/contents/${firstId}`);
  await page.getByRole('link', { name: 'タグを編集' }).click();
  await expect(page.getByRole('img', { name: '1 件目のサムネイル' })).toBeVisible();
  await expect(page.getByRole('note')).toContainText('Invalid-Key: タグ名が無効です');
  if (isMobile) await page.getByLabel('category: old').tap();
  else await page.getByLabel('category: old').hover();
  await page.getByRole('button', { name: 'category を削除' }).click();
  await page.getByLabel('タグ名', { exact: true }).fill('category');
  await page.getByLabel('値 1', { exact: true }).fill('new');
  await page.getByRole('button', { name: 'タグを追加' }).click();
  await page.getByRole('button', { name: '1 件のタグを更新' }).click();
  await expect(page).toHaveURL(new RegExp(`#/contents/${firstId}$`));
  expect(patches).toEqual([{ tags: [{ key: 'category', type: 'text', value: 'new' }] }]);
});

test('bulk edit starts with identical tags and preserves each item’s other tags', async ({ page, isMobile }) => {
  const common = { key: 'category', type: 'text', value: 'shared' };
  const items = [
    { ...content(firstId), tags: [common, { key: 'subject', type: 'text', value: 'first' }, { key: 'firstOnly', type: 'text', value: 'one' }], diagnostics: [{ key: 'Invalid-Key', kind: 'invalidKey' }] },
    { ...content(secondId), tags: [common, { key: 'subject', type: 'text', value: 'second' }, { key: 'secondOnly', type: 'text', value: 'two' }], diagnostics: [{ key: 'broken', kind: 'notAllowedTagValue' }] },
  ];
  const patches: Record<string, unknown> = {};
  let gets = 0;
  await page.route('**/api/v0/contents?*', (route) => route.fulfill({ json: { items } }));
  await page.route('**/api/v0/contents/*', (route) => {
    const id = new URL(route.request().url()).pathname.split('/').pop()!;
    if (route.request().method() === 'PATCH') {
      const body = route.request().postDataJSON();
      patches[id] = body;
      return route.fulfill({ json: { ...items.find((item) => item.id === id), tags: body.tags, diagnostics: [] } });
    }
    gets++;
    return route.fulfill({ json: items.find((item) => item.id === id) });
  });
  await page.goto('/#/contents');
  await page.getByRole('button', { name: 'コンテンツを選択' }).click();
  await page.getByRole('button', { name: '画像 1 を選択' }).click();
  await page.getByRole('button', { name: '画像 2 を選択' }).click();
  await page.getByRole('button', { name: '選択したコンテンツを編集' }).click();
  await expect(page.getByRole('list', { name: '編集するタグ' }).getByRole('listitem')).toHaveCount(1);
  await expect(page.getByLabel('category: shared')).toBeVisible();
  await expect(page.getByRole('note')).toContainText('Invalid-Key: タグ名が無効です');
  await expect(page.getByRole('note')).toContainText('broken: タグの値がスキーマの制約を満たしていません');
  if (isMobile) await page.getByLabel('category: shared').tap();
  else await page.getByLabel('category: shared').hover();
  await page.getByRole('button', { name: 'category を削除' }).click();
  await page.getByLabel('タグ名', { exact: true }).fill('subject');
  await page.getByLabel('タグの型').selectOption('text');
  await page.getByLabel('値 1', { exact: true }).fill('uniform');
  await page.getByRole('button', { name: 'タグを追加' }).click();
  await page.getByRole('button', { name: '2 件のタグを更新' }).click();
  await expect(page).toHaveURL(/#\/contents$/);
  expect(gets).toBe(0);
  expect(patches).toEqual({
    [firstId]: { tags: [{ key: 'firstOnly', type: 'text', value: 'one' }, { key: 'subject', type: 'text', value: 'uniform' }] },
    [secondId]: { tags: [{ key: 'secondOnly', type: 'text', value: 'two' }, { key: 'subject', type: 'text', value: 'uniform' }] },
  });
});

test('bulk edit keeps required tags with different values and recognizes unordered sets as common', async ({ page }) => {
  await mockTagSchema(page, { version: '0', allowAdditionalTags: false, required: [{ key: 'category', type: 'text' }], optional: [{ key: 'authors', type: 'textSet' }] });
  const items = [
    { ...content(firstId), tags: [{ key: 'category', type: 'text', value: 'first' }, { key: 'authors', type: 'textSet', values: ['Alice', 'Bob'] }] },
    { ...content(secondId), tags: [{ key: 'category', type: 'text', value: 'second' }, { key: 'authors', type: 'textSet', values: ['Bob', 'Alice'] }] },
  ];
  const patches: Record<string, unknown> = {};
  await page.route('**/api/v0/contents?*', (route) => route.fulfill({ json: { items } }));
  await page.route('**/api/v0/contents/*', (route) => {
    const id = new URL(route.request().url()).pathname.split('/').pop()!;
    const body = route.request().postDataJSON();
    patches[id] = body;
    return route.fulfill({ json: { ...items.find((item) => item.id === id), tags: body.tags } });
  });
  await page.goto('/#/contents');
  await page.getByRole('button', { name: 'コンテンツを選択' }).click();
  await page.getByRole('button', { name: '画像 1 を選択' }).click();
  await page.getByRole('button', { name: '画像 2 を選択' }).click();
  await page.getByRole('button', { name: '選択したコンテンツを編集' }).click();
  await expect(page.getByRole('list', { name: '編集するタグ' }).getByRole('listitem')).toHaveCount(1);
  await expect(page.getByLabel('authors: Alice, Bob')).toBeVisible();
  await expect(page.getByRole('button', { name: '2 件のタグを更新' })).toBeEnabled();
  await page.getByRole('button', { name: '2 件のタグを更新' }).click();
  await expect(page).toHaveURL(/#\/contents$/);
  expect(patches).toEqual({
    [firstId]: { tags: [{ key: 'category', type: 'text', value: 'first' }, { key: 'authors', type: 'textSet', values: ['Alice', 'Bob'] }] },
    [secondId]: { tags: [{ key: 'category', type: 'text', value: 'second' }, { key: 'authors', type: 'textSet', values: ['Alice', 'Bob'] }] },
  });
});

test('removes an item in bulk confirmation and updates selected items', async ({ page }) => {
  const deleted: string[] = [];
  const patched: string[] = [];
  await page.route('**/api/v0/contents/*', (route) => {
    const id = new URL(route.request().url()).pathname.split('/').pop()!;
    if (route.request().method() === 'DELETE') { deleted.push(id); return route.fulfill({ status: 204 }); }
    if (route.request().method() === 'PATCH') { patched.push(id); return route.fulfill({ json: { ...content(id), tags: [] } }); }
    return route.fulfill({ json: content(id) });
  });
  await page.goto('/#/contents');
  await page.getByRole('button', { name: 'コンテンツを選択' }).click();
  await page.getByRole('button', { name: '画像 1 を選択' }).click();
  await page.getByRole('button', { name: '画像 2 を選択' }).click();
  await page.getByRole('button', { name: '選択したコンテンツを削除' }).click();
  const dialog = page.getByRole('dialog', { name: 'コンテンツを削除' });
  await expect(dialog.getByRole('img', { name: '2 件目のサムネイル' })).toHaveCount(1);
  const position = dialog.getByLabel('表示中の画像');
  const before = await position.boundingBox();
  const beforeOffset = await position.evaluate((element) => [(element as HTMLElement).offsetLeft, (element as HTMLElement).offsetTop]);
  const image = dialog.locator('.swiper-slide-active').getByRole('img', { name: '1 件目のサムネイル' });
  const [imageBounds, previousBounds, nextBounds] = await Promise.all([
    image.boundingBox(), dialog.getByRole('button', { name: '前の画像' }).boundingBox(), dialog.getByRole('button', { name: '次の画像' }).boundingBox(),
  ]);
  expect(before && imageBounds && previousBounds && nextBounds).toBeTruthy();
  expect(before!.x).toBeLessThan(imageBounds!.x + imageBounds!.width / 2);
  expect(previousBounds!.x).toBeGreaterThanOrEqual(imageBounds!.x);
  expect(nextBounds!.x + nextBounds!.width).toBeLessThanOrEqual(imageBounds!.x + imageBounds!.width);
  await dialog.getByRole('button', { name: '次の画像' }).click();
  await expect(position).toHaveText('2 / 2');
  expect(parseFloat(await dialog.locator('.swiper-wrapper').evaluate((element) => element.style.transitionDuration))).toBeGreaterThan(0);
  expect(await position.evaluate((element) => [(element as HTMLElement).offsetLeft, (element as HTMLElement).offsetTop])).toEqual(beforeOffset);
  await dialog.getByRole('button', { name: '2 件目を対象から取り除く' }).click();
  await dialog.getByRole('button', { name: '1 件を削除' }).click();
  await expect(dialog).toHaveCount(0);
  expect(deleted).toEqual([firstId]);
  await page.getByRole('button', { name: '画像 2 を選択解除' }).click();
  await page.getByRole('button', { name: '画像 1 を選択' }).click();
  await page.getByRole('button', { name: '画像 2 を選択' }).click();
  await page.getByRole('button', { name: '選択したコンテンツを編集' }).click();
  await expect(page.getByRole('img', { name: '1 件目のサムネイル' })).toBeVisible();
  await page.getByRole('button', { name: '2 件のタグを更新' }).click();
  await expect(page).toHaveURL(/#\/contents$/);
  expect(patched).toEqual([firstId, secondId]);
});

test('long press starts selection and a failed update retries only that item', async ({ page, isMobile }) => {
  test.skip(!isMobile, 'touch selection is checked in the mobile project');
  const patched: string[] = [];
  await page.route('**/api/v0/contents/*', (route) => {
    const id = new URL(route.request().url()).pathname.split('/').pop()!;
    if (route.request().method() === 'PATCH') {
      patched.push(id);
      if (id === secondId && patched.filter((item) => item === secondId).length === 1) return route.fulfill({ status: 500, json: { detail: 'temporary failure' } });
      return route.fulfill({ json: { ...content(id), tags: [] } });
    }
    return route.fulfill({ json: content(id) });
  });
  await page.goto('/#/contents');
  const firstCard = page.locator('.hover-3d').first();
  await firstCard.dispatchEvent('pointerdown', { pointerType: 'touch', clientX: 20, clientY: 20 });
  await expect(page.getByText('1 件選択中')).toBeVisible();
  await firstCard.dispatchEvent('pointerup', { pointerType: 'touch', clientX: 20, clientY: 20 });
  await page.getByRole('button', { name: '画像 2 を選択' }).click();
  await page.getByRole('button', { name: '選択したコンテンツを編集' }).click();
  await page.getByRole('button', { name: '2 件のタグを更新' }).click();
  await expect(page.getByRole('alert')).toContainText('temporary failure');
  await expect(page.getByRole('button', { name: '1 件のタグを更新' })).toBeVisible();
  await page.getByRole('button', { name: '1 件のタグを更新' }).click();
  await expect(page).toHaveURL(/#\/contents$/);
  expect(patched).toEqual([firstId, secondId, secondId]);
});

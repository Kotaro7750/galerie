import { expect, test } from '@playwright/test';

const authority = 'https://cognito-idp.us-east-1.amazonaws.com/us-east-1_test';
const storageKey = `oidc.user:${authority}:galerie`;

test('a protected route starts the OAuth authorization code flow', async ({ page }) => {
  await page.route(`${authority}/.well-known/openid-configuration`, (route) => route.fulfill({ json: {
    issuer: authority,
    authorization_endpoint: 'http://identity.test/authorize',
    token_endpoint: 'http://identity.test/token',
  } }));
  const authorizeRequest = page.waitForRequest('http://identity.test/authorize?*');
  await page.route('http://identity.test/authorize?*', (route) => route.abort());
  await page.goto('/#/contents');
  await expect(page.getByRole('heading', { name: 'ログインが必要です' })).toBeVisible();
  await page.getByRole('button', { name: 'ログイン', exact: true }).last().click();
  const url = new URL((await authorizeRequest).url());
  expect(url.searchParams.get('response_type')).toBe('code');
  expect(url.searchParams.get('client_id')).toBe('galerie');
  expect(url.searchParams.get('code_challenge_method')).toBe('S256');
});

test('an authenticated API request includes the bearer access token', async ({ page }) => {
  await page.addInitScript(({ key }) => {
    sessionStorage.setItem(key, JSON.stringify({
      access_token: 'test-access-token', token_type: 'Bearer',
      profile: { sub: 'test-user' }, expires_at: Math.floor(Date.now() / 1000) + 3600,
      scope: 'openid profile',
    }));
  }, { key: storageKey });
  let authorization: string | undefined;
  await page.route('**/api/v0/contents?*', (route) => {
    authorization = route.request().headers().authorization;
    return route.fulfill({ json: { items: [] } });
  });
  await page.goto('/#/contents');
  await expect(page.getByText('コンテンツが見つかりません')).toBeVisible();
  expect(authorization).toBe('Bearer test-access-token');
  await expect(page.getByRole('button', { name: 'ログアウト' })).toBeVisible();
});

test('Cognito logout includes its required client and logout URI', async ({ page }) => {
  await page.addInitScript(({ key }) => {
    sessionStorage.setItem(key, JSON.stringify({
      access_token: 'test-access-token', id_token: 'test-id-token', token_type: 'Bearer',
      profile: { sub: 'test-user' }, expires_at: Math.floor(Date.now() / 1000) + 3600,
      scope: 'openid profile',
    }));
  }, { key: storageKey });
  await page.route(`${authority}/.well-known/openid-configuration`, (route) => route.fulfill({ json: {
    issuer: authority,
    authorization_endpoint: 'http://identity.test/authorize',
    token_endpoint: 'http://identity.test/token',
    end_session_endpoint: 'http://identity.test/logout',
  } }));
  const logoutRequest = page.waitForRequest('http://identity.test/logout?*');
  await page.route('http://identity.test/logout?*', (route) => route.abort());
  await page.goto('/');
  await page.getByRole('button', { name: 'ログアウト' }).click();
  const url = new URL((await logoutRequest).url());
  expect(url.searchParams.get('client_id')).toBe('galerie');
  expect(url.searchParams.get('logout_uri')).toBe('http://127.0.0.1:4174/');
});

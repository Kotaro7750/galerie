# Galerie frontend
## Commands
### Start

```sh
cd frontend
mise install
mise run install
mise run dev
```

### Verify and Build

```sh
npm run build
npx playwright install chromium
# Verifies navigation, infinite scrolling, and error recovery with a mocked API.
npm test
# Verifies protected routes, PKCE, and Bearer tokens in an authenticated build.
npm run test:auth
npm run preview
```

### Regenerate the API Client

```sh
npm run api:generate
npm run build
```

## Environment Variables
### Common

| Variable | Required | Default | Description |
| --- | --- | --- | --- |
| `VITE_API_BASE_URL` | No | `/api/v0` | Base URL used by the API client. A trailing `/` is removed. |
| `VITE_AUTH_ENABLED` | No | `false` | Enables OAuth 2.0 / OpenID Connect authorization when set to `true`. |

To override values for local development, copy `.env.example` to `.env.local` and edit it.

### OAuth 2.0 Authorization

OAuth 2.0 authorization is disabled by default.
When enabled, the following environment variables configure the Authorization Code Flow with PKCE and OpenID Connect.

| Variable | Required | Default | Description |
| --- | --- | --- | --- |
| `VITE_AUTH_AUTHORITY` | Yes | - | OIDC issuer URL. |
| `VITE_AUTH_CLIENT_ID` | Yes | - | Public client ID. |
| `VITE_AUTH_RESOURCE` | No | SPA origin | OAuth 2.0 Resource Indicator (RFC 8707). |
| `VITE_AUTH_SCOPE` | No | `openid` | OAuth 2.0 scopes requested during authorization. |
| `VITE_AUTH_REDIRECT_URI` | No | URL serving the SPA | Redirect URL used after authorization completes. |
| `VITE_AUTH_POST_LOGOUT_REDIRECT_URI` | No | URL serving the SPA | Redirect URL used after logout completes. |

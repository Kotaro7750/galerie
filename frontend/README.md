# Galerie frontend

## 起動

```sh
cd frontend
mise install
mise run install
mise run dev
```

Node.js 24 と npm を直接使用する場合は `npm ci`、`npm run dev` でも起動できる。
開発サーバーの URL は起動時の出力を確認する。

バックエンドを `http://localhost:3000` で起動すると、開発サーバーが `/api` を転送する。
接続先を変更する場合は `.env.example` を `.env.local` にコピーして編集する。
画像は API が返す `thumbnailUrl` / `contentUrl` から直接取得するため、コンテンツ配信サーバーも別途起動する。

## 確認とビルド

```sh
npm run build
npx playwright install chromium
npm test
npm run test:auth
npm run preview
```

`npm run build` は型検査と本番ビルドを実施する。配信対象は `dist/`。
`npm test` は API をモックし、実際のサンプル AVIF を使ってデスクトップ・モバイルの画面遷移、無限スクロール、エラー回復を確認する。
`npm run test:auth`はテスト用の認証付きビルドを起動し、保護ルート、Authorization Code Flow with PKCE及びAPIのBearerトークンを確認する。

静的ホスティングでは `/#/`、`/#/contents`、`/#/contents/:contentId` を使うため、SPA 用のパス書き換えは不要。
本番環境では同一オリジンの `/api/v0` にバックエンドを配置するか、ビルド時に `VITE_API_BASE_URL` を指定する。
別オリジンの場合、API サーバー側でフロントエンドのオリジンを許可する CORS 設定が必要。
開発用プロキシは `dist/` や `npm run preview` には含まれない。

## API クライアントの再生成

リポジトリルートの mise で定義された Java と OpenAPI Generator CLI をインストールしたうえで実行する。

```sh
npm run api:generate
npm run build
```

契約は `../contracts/api/openapi.yaml`、出力先は `src/api/generated/`。
生成コードは手動編集せず、契約の変更後に再生成する。Generator は `openapitools.json` で 7.24.0 に固定している。

## 認証付きビルド

ローカル開発と通常のビルドは既定で認証なし。認証付きにする場合は`.env.example`を参照し、少なくとも次をビルド時に設定する。

```sh
VITE_AUTH_ENABLED=true
VITE_AUTH_AUTHORITY=https://id.example.com
VITE_AUTH_CLIENT_ID=galerie
npm run build
```

認証プロバイダーにはSPAのURLをリダイレクトURL及びログアウト後URLとして登録し、Authorization Code Flow、PKCE及び必要なscopeを有効にする。
認証付きビルドではコンテンツ一覧・コンテンツページが保護され、バックエンドAPIリクエストにアクセストークンをBearerトークンとして付与する。
`contentUrl`と`thumbnailUrl`への画像リクエストにはBearerトークンを付与しないため、必要な場合はコンテンツ配信側で期限付きURL等を使用する。

Amazon Cognitoでは`VITE_AUTH_POST_LOGOUT_REDIRECT_URI`の値を、アプリケーションクライアントの「許可されているサインアウトURL」に完全一致で登録する。
実装はCognitoのissuerを検出し、ログアウト時に同じ値を`logout_uri`として送信する。

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
npm run preview
```

`npm run build` は型検査と本番ビルドを実施する。配信対象は `dist/`。
`npm test` は API をモックし、実際のサンプル AVIF を使ってデスクトップ・モバイルの画面遷移、無限スクロール、エラー回復を確認する。

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

今回の実装は依頼時の指定により認証なし。既存の設計ドキュメントに記載された認証要件は変更していない。

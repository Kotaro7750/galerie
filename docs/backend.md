# バックエンド

バックエンドは Rust Axum サービスです。マウントされたメディアディレクトリをインデックスし、`/api/v1` エンドポイント、オリジナルファイルのストリーミング、サムネイル生成、OpenTelemetry 出力を担当します。

## 構成

```text
backend/src/
├── api/        HTTP handler と response/error envelope
├── cache/      JSON cache の読み書きと scan 補助
├── media/      media type、thumbnail、stream helper
├── o11y/       OpenTelemetry setup
├── routes/     router 構成、CORS、shared state、trace layer
├── services/   search などの application service
├── tags/       ファイル名 tag parser
├── config.rs   runtime configuration
├── lib.rs      test 向け library export
└── main.rs     binary entry point
```

## 実行時設定

主な入力:

- `GALARIE_MEDIA_ROOT`: インデックス・配信対象の read-only ディレクトリ。
- `GALARIE_CACHE_DIR`: 派生キャッシュを書き込むディレクトリ。
- `OTEL_EXPORTER_OTLP_ENDPOINT`: OTLP collector endpoint。
- `GALARIE_ENV`, `RUST_LOG`, `OTEL_SERVICE_NAME`: telemetry と logging の制御。

binary が対応している場合は、同等の CLI flag も利用できるようにします。

## インデックス規約

- ファイルシステムを唯一の正とする。
- 未対応 media type は unknown として検索対象に入れず、スキップしてログに残す。
- パスは `GALARIE_MEDIA_ROOT` からの相対パスとして正規化する。
- media root から外れるパスは絶対に配信しない。
- キャッシュには派生メタデータだけを保存する。

## API 規約

- 公開 endpoint は `/api/v1` 配下に置く。
- API failure は構造化 JSON error で返す。

```json
{ "error": { "code": "invalid_request", "message": "..." } }
```

- HTTP status code は一貫して使う。
  - `400`: validation error。
  - `404`: unknown media ID。
  - `500`: unexpected server failure。
- binary endpoint は可能な範囲で ETag などの cache validator を返す。

## ストリーミング規約

- `GET /api/v1/media/{id}/stream` はオリジナルファイルを配信する。
- `disposition=inline` をデフォルトにする。
- Range request は `206 Partial Content` と妥当な range header で返す。
- MIME type 判定は明示的かつ保守的に行う。
- 不正 ID、path traversal、存在しないファイルで host path を漏らさない。

## サムネイル規約

- サムネイルは on-demand で生成し、派生ファイルとしてキャッシュする。
- cache path は media ID と size から安定的に決める。
- 生成失敗時は予測可能な fallback または error を返す。
- 高コストなサムネイル処理で、無関係な検索リクエストをブロックしない。

## テスト

まず変更範囲に近いテストを実行します。

```bash
mise run backend:test
mise run backend:lint
mise run backend:fmt
```

優先して守る対象は、parser、cache rebuild、API error contract、Range streaming、path traversal 防止です。

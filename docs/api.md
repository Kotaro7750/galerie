# API

バックエンドは `/api/v1` 配下で REST API を公開します。破壊的変更は新しい version namespace で行います。

機械可読な API 定義は `docs/openapi.yaml` に置きます。Swagger UI は devcontainer の compose stack からこのファイルを読み込みます。

## 共通規約

- JSON endpoint は `application/json` を使う。
- binary endpoint は明示的な `Content-Type` を返す。
- validation error は `400`。
- 存在しない media は `404`。
- 想定外の failure は `500`。
- error response は次の envelope を使う。

```json
{ "error": { "code": "invalid_request", "message": "..." } }
```

## Endpoints

### `GET /api/v1/media`

catalog を検索または page 単位で取得します。

Query parameters:

- `tags`: comma-separated の tag/key 名。すべて存在する必要がある。
- `attributes[key]`: key に対して許容する値の comma-separated list。key 同士は AND、同一 key 内の値は OR。
- `page`: 1-based page number。
- `pageSize`: page size。上限は backend が制御する。

Response:

```json
{
  "items": [
    {
      "id": "string",
      "relativePath": "string",
      "mediaType": "image",
      "tags": [],
      "attributes": {},
      "filesize": 123,
      "thumbnailPath": "string",
      "indexedAt": "2026-01-01T00:00:00Z"
    }
  ],
  "total": 1,
  "page": 1,
  "pageSize": 60
}
```

### `GET /api/v1/media/{id}/thumbnail`

派生サムネイルを取得します。

Query parameters:

- `size`: 任意。`small`, `medium`, `large` などの thumbnail size。

期待する挙動:

- 利用可能な場合は `200` と image bytes。
- cache validator が一致する場合は `304`。
- media ID が存在しない場合は `404`。

### `GET /api/v1/media/{id}/stream`

オリジナルメディアファイルを stream 配信します。

Query parameters:

- `disposition`: `inline` または `attachment`。default は `inline`。

期待する挙動:

- full response は `200`。
- valid Range request は `206`。
- stream 可能な media では `Accept-Ranges: bytes` を返す。
- MIME type は file type に基づき保守的に判定する。

### `POST /api/v1/index/rebuild`

index rebuild を開始します。

Request body:

```json
{ "force": true }
```

Response:

```json
{ "status": "queued" }
```

`status` は実装経路に応じて `queued`, `running`, `complete`, `failed` を取り得ます。

## 互換性

- `/api/v1` では response field の追加を許容する。
- field 削除、filter semantics 変更、error envelope 変更は `/api/v2` を必要とする。
- frontend の request construction はこの文書と同期させる。

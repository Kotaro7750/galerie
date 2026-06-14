# 開発

Galarie の開発作業の入口です。build、test、runtime、contribution workflow が変わった場合はこの文書を更新します。

## 前提

- ローカル開発では、環境差分を減らすため Dev Container を基本の実行環境とする。
- Docker と Dev Container CLI、または VS Code Dev Containers。
- repository task runner として `mise`。devcontainer 内では初回作成時に `/workspace/.mise.toml` を trust する。
- backend 用の Rust stable。
- frontend package lock と互換性のある Node.js。
- media thumbnail workflow 用の `ffmpeg` と `gifsicle`。

## Devcontainer workflow

通常の開発サーバー起動、test、lint は devcontainer 内で実行します。

```bash
devcontainer up --workspace-folder .
devcontainer exec --workspace-folder . bash
```

container 内で runtime path を設定します。

```bash
export GALARIE_MEDIA_ROOT=/workspace/media
export GALARIE_CACHE_DIR=/workspace/.cache
mkdir -p "$GALARIE_CACHE_DIR"
```

必要に応じて sample data をコピーします。

```bash
cp sample-media/* media/
```

## Backend

devcontainer 内で実行します。

```bash
mise run backend:dev
mise run backend:stop-dev
mise run backend:test
mise run backend:lint
mise run backend:fmt
```

直接起動は、devcontainer を使えない場合の補助手順です。同等の Rust/Node 環境と `ffmpeg`、`gifsicle` を host 側で用意してください。

```bash
cd backend
cargo run -- \
  --media-root "$GALARIE_MEDIA_ROOT" \
  --cache-dir "$GALARIE_CACHE_DIR" \
  --listen 0.0.0.0:8080
```

## Frontend

devcontainer 内で実行します。

```bash
mise run frontend:install
mise run frontend:dev
mise run frontend:stop-dev
mise run frontend:test
mise run frontend:e2e
```

backend が default URL 以外の場合は `frontend/.env` を設定します。

```text
VITE_API_BASE=http://localhost:8080/api/v1
```

## API smoke check

```bash
curl -X POST http://localhost:8080/api/v1/index/rebuild \
  -H 'Content-Type: application/json' \
  -d '{"force":true}'

curl "http://localhost:8080/api/v1/media?page=1&pageSize=60"
```

## Contribution rules

- `docs/` を現在有効な設計・開発方針の source of truth とする。
- `specs/` や `.specify/` 配下に新しい仕様駆動開発 artifact を追加しない。
- 挙動、architecture、API semantics、workflow を変更する場合は、同じ変更で該当する `docs/*.md` を更新する。
- PR は小さく保ち、変更した挙動に対応する test を含める。
- handoff 前に backend/frontend の focused check を実行する。

## Troubleshooting

- tag や file が古く見える場合は `GALARIE_CACHE_DIR/index.json` を削除して rebuild する。
- `GALARIE_MEDIA_ROOT` が readable directory を指しているか確認する。
- container 実行では media mount が read-only になっているか確認する。
- Playwright browser は環境ごとに `mise run frontend:playwright-install` で一度 install する。

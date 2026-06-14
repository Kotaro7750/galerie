# Galarie

Galarie は、ローカルにマウントしたメディアディレクトリをそのまま source of truth として扱う DB-free media browser です。Rust/Axum backend がファイル名タグを解析して JSON cache を作成し、React/Vite frontend が `/api/v1` 経由で検索・サムネイル・ストリーミング UI を提供します。

## 主な機能

- ファイルシステムを直接走査し、外部 DB なしで catalog を構築。
- `sunset+location-okinawa_rating-5.png` のようなファイル名から tag と key/value attribute を抽出。
- tag/attribute の AND 検索、ページング、サムネイル表示。
- 検索結果から original media を即座に stream 再生。
- OpenTelemetry、Prometheus、Loki、Tempo、Grafana を含む local observability stack。
- Playwright による frontend E2E test。

## リポジトリ構成

```text
backend/        Rust/Axum backend
frontend/       React + Vite SPA
docs/           現在有効な設計・API・開発方針
sample-media/   テスト・デモ用 fixture
media/          ローカルメディア用 mount point。Git 管理外
.devcontainer/  devcontainer と observability stack
Dockerfile      production/devcontainer 用 multi-stage build
.mise.toml      backend/frontend 共通タスク
AGENTS.md       AI エージェント向け作業規約
```

## ドキュメント

- `docs/architecture.md` – システム境界、データフロー、タグ・キャッシュモデル。
- `docs/api.md` – `/api/v1` endpoint の挙動と互換性ルール。
- `docs/openapi.yaml` – Swagger UI でも利用する OpenAPI 定義。
- `docs/backend.md` – backend の構成と実装規約。
- `docs/frontend.md` – frontend の構成、状態責務、client 規約。
- `docs/observability.md` – telemetry signal と local observability stack。
- `docs/development.md` – 開発、テスト、contribution workflow。

AI エージェント向けの作業ルールは `AGENTS.md` に分離しています。README には利用者・開発者の入口として必要な情報だけを置きます。

## Quickstart

ローカル開発では、環境差分を減らすため Dev Container を基本の実行環境とします。Dev Container CLI または VS Code Dev Containers と Docker を用意します。共通タスクは `mise` で実行し、devcontainer 内では初回作成時に `/workspace/.mise.toml` を trust します。

```bash
git clone <repo> galarie
cd galarie
mkdir -p media
devcontainer up --workspace-folder .
devcontainer exec --workspace-folder . bash
```

container 内で runtime path を設定します。

```bash
export GALARIE_MEDIA_ROOT=/workspace/media
export GALARIE_CACHE_DIR=/workspace/.cache
mkdir -p "$GALARIE_CACHE_DIR"
```

必要に応じて sample media をコピーします。

```bash
cp sample-media/* media/
```

## 開発サーバー

backend:

```bash
mise run backend:dev
mise run backend:stop-dev
```

frontend:

```bash
mise run frontend:install
mise run frontend:dev
mise run frontend:stop-dev
```

frontend が backend と通信する API root は `frontend/.env` で指定します。

```text
VITE_API_BASE=http://localhost:8080/api/v1
```

## API と Swagger UI

主要 endpoint:

- `GET /api/v1/media`
- `GET /api/v1/media/{id}/thumbnail`
- `GET /api/v1/media/{id}/stream`
- `POST /api/v1/index/rebuild`

OpenAPI 定義は `docs/openapi.yaml` にあります。devcontainer の compose stack では Swagger UI を `http://localhost:8088` で起動します。

Smoke check:

```bash
curl -X POST http://localhost:8080/api/v1/index/rebuild \
  -H 'Content-Type: application/json' \
  -d '{"force":true}'

curl "http://localhost:8080/api/v1/media?page=1&pageSize=60"
```

## Observability

devcontainer 起動時に `.devcontainer/observability/` の compose stack も利用できます。

- OTel Collector: `localhost:4317`
- Prometheus: `http://localhost:9090`
- Loki: `http://localhost:3100`
- Tempo: `http://localhost:3200`
- Grafana: `http://localhost:3300`

詳細は `docs/observability.md` を参照してください。

## テスト

```bash
mise run backend:test
mise run backend:lint
mise run backend:fmt
mise run frontend:test
mise run frontend:lint
mise run frontend:build
mise run frontend:e2e
```

Playwright browser は環境ごとに一度 install します。

```bash
mise run frontend:playwright-install
```

## Docker build

```bash
docker build \
  --target prod-runtime \
  -t your-dockerhub-username/galarie:latest \
  .
```

```bash
docker run --rm \
  -p 8080:8080 \
  -v "$PWD/media":/data/media:ro \
  -v "$PWD/.cache":/data/cache \
  your-dockerhub-username/galarie:latest
```

GitHub Actions の Docker Hub publish workflow は `/.github/workflows/dockerhub-publish.yml` にあります。

## Contribution

1. 小さく焦点の合った branch を作る。
2. 挙動、API semantics、architecture、workflow を変える場合は関連する `docs/*.md` を同時に更新する。
3. 変更範囲に近い `mise` task を実行してから handoff する。

詳細な開発規約は `docs/development.md`、エージェント向け作業規約は `AGENTS.md` を参照してください。

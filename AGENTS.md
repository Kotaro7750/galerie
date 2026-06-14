# AGENTS.md

このファイルは、このリポジトリで作業する AI エージェント向けの入口です。設計方針や開発規約の本文は `docs/` 配下に置き、このファイルには参照順序と作業時の扱いだけを書きます。

## 適用範囲

- この `AGENTS.md` はリポジトリ全体に適用します。
- より深いディレクトリに別の `AGENTS.md` が追加された場合は、その配下ではより近いファイルの指示を優先します。

## README、docs、AGENTS.md の使い分け

- `README.md`: 初めて読む人向けの概要、起動方法、主要リンク、利用・開発の入口。
- `docs/`: 現在有効な設計・API・backend/frontend・observability・開発規約の本文。
- `AGENTS.md`: エージェントがどの `docs/` を参照すべきかを示す案内。

README に実装判断やエージェント固有の長い手順を増やさないでください。設計判断や規約の本文は `docs/` に置き、このファイルには重複記載しないでください。

## 参照先

- 全体設計: `docs/architecture.md`
- API 方針と endpoint: `docs/api.md`
- OpenAPI 定義: `docs/openapi.yaml`
- backend 規約: `docs/backend.md`
- frontend 規約: `docs/frontend.md`
- observability 方針: `docs/observability.md`
- 開発・検証・contribution workflow: `docs/development.md`

## 作業時の参照手順

- 作業前に、変更対象に対応する `docs/*.md` を確認してください。
- 設計、API、workflow の判断が必要な場合は、このファイルではなく該当する `docs/*.md` を根拠にしてください。
- `docs/` と実装が矛盾している場合は、作業内容に応じて実装または `docs/` を更新し、矛盾を残さないでください。
- README は利用者向け入口として必要な場合だけ更新してください。

## ドキュメント更新ルール

- 設計方針、API semantics、backend/frontend 規約、observability、開発手順は `docs/` に追記・修正してください。
- `AGENTS.md` に設計方針の本文を追加しないでください。
- 仕様駆動開発用の `specs/`、`.specify/`、`speckit.*` prompt は再追加しないでください。この方針の詳細は `docs/development.md` を参照してください。
- `docs/openapi.yaml` を移動する場合は、参照元も同時に更新してください。現在の参照関係は `docs/api.md` と `.devcontainer/docker-compose.yml` を確認してください。

## 検証

検証方針とコマンドは `docs/development.md` を参照してください。ドキュメントのみの変更ではテスト実行は不要ですが、リンク先、コマンド名、参照ファイルの整合性は確認してください。

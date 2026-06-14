# アーキテクチャ

Galarie は DB を持たないメディアブラウザです。マウントされたファイルシステムを唯一の正とし、バックエンドが JSON インデックスとサムネイルキャッシュを派生生成し、フロントエンドが検索・プレビュー・スライドショー・再生体験を提供します。

## 目的

- 外部 DB を導入せず、ローカルメディアを高速に閲覧できる Web UI を提供する。
- ファイル名に埋め込まれたタグを使い、単純タグと key/value 属性で検索できるようにする。
- 検索結果からサムネイルとストリーム API を通じて即座にメディアを開けるようにする。
- お気に入りやスライドショー順序など、ユーザーの閲覧状態はブラウザセッション内に閉じる。
- ローカル開発とコンテナ実行のどちらでも、本番相当の Observability を維持する。

## システム構成

```text
media/ または GALARIE_MEDIA_ROOT
  -> backend indexer
  -> GALARIE_CACHE_DIR 配下の JSON キャッシュ
  -> /api/v1 配下の Axum REST API
  -> React/Vite frontend
```

リポジトリ構成:

```text
backend/       Rust backend。API、index/cache/search/streaming を担当
frontend/      React + Vite SPA
sample-media/  テスト・デモ用のバージョン管理された fixture
media/         ユーザーのローカルメディア用マウントポイント。Git 管理外
.devcontainer/ 開発コンテナと Observability 用 compose stack
docs/          現在有効な設計・開発方針
```

## 基本データフロー

1. バックエンドが `GALARIE_MEDIA_ROOT` を走査する。
2. 対応するメディアファイルを image、GIF、video、audio、PDF に分類する。
3. ファイル名トークンを tag と attribute に正規化する。
4. メタデータを `GALARIE_CACHE_DIR/index.json` に保存する。
5. 検索リクエストはキャッシュ済みインデックスを参照する。
6. サムネイル・ストリームリクエストは media ID から、メディアルート配下の安全な相対パスへ解決する。

未対応または分類不能なファイルは検索対象に入れず、ログに記録してスキップします。元のメディアファイルは変更しません。

## タグモデル

ファイル名でタグを表現します。バックエンドはこれを次の形式に正規化します。

- 単純タグ: `okinawa` や `sunset` のようなトークン。
- key/value 属性: `rating-5` のようなトークン。key は `rating`、value は `5`。

検索は AND 条件です。

- `tags` は指定されたタグ名または key 名がすべて存在することを要求する。
- `attributes[key]` は指定 key の値が、指定値のいずれかに一致することを要求する。
- ページングは常に `page` と `pageSize` で明示する。

## キャッシュモデル

キャッシュは実装上の派生データであり、正ではありません。

- キャッシュディレクトリ: `GALARIE_CACHE_DIR`
- 主なスナップショット: `index.json`
- 再構築 API: `POST /api/v1/index/rebuild`
- 復旧方法: キャッシュを削除し、メディアルートから再構築する。

media ID は正規化された相対パスから生成します。そのため、ファイル名変更は ID 変更を伴うカタログ更新として扱います。

## 状態の境界

- バックエンド状態: 派生インデックス、サムネイルキャッシュ、実行時 telemetry。
- フロントエンド状態: フィルタ、お気に入り、スライドショーキュー、プレイヤー設定。
- ユーザー固有の永続状態は、明示的な機能追加がない限りサーバー側に保存しません。

## セキュリティ境界

- すべてのメディアアクセスは `GALARIE_MEDIA_ROOT` 配下に限定する。
- API handler は path traversal と不正な media ID を拒否する。
- コンテナでは media mount を read-only にする。
- 診断情報はログや stderr に出し、ユーザー所有のメディアファイルには書き込まない。

## 変更方針

この文書は現在有効なアーキテクチャだけを記述します。履歴用の decision record は作りません。変更理由が設計理解に必要な場合だけ、該当セクションに短く残します。

# フロントエンド

フロントエンドは React + Vite の SPA です。バックエンドの `/api/v1` API と通信し、検索結果、メディアプレビュー、スライドショー、再生 UI を提供します。

## 構成

```text
frontend/src/
├── components/  filter control、media card、result grid、preview overlay
├── config/      environment parsing
├── hooks/       filter と persistence hook
├── pages/       route-level screen
├── services/    API client
├── types/       frontend API/domain type
└── utils/       filter と media URL helper
```

## 実行時設定

- `VITE_API_BASE` は backend API root を指す。例: `http://localhost:8080/api/v1`
- test 以外では hostname や port を UI に hard-code しない。

## 状態の責務

フロントエンドが所有する状態:

- 現在の検索フィルタ。
- pagination と infinite scroll の進行状態。
- お気に入りとスライドショーキュー。
- preview/player UI state。

バックエンドが所有する状態:

- media index。
- thumbnail cache。
- stream access。

filter と favorites は、永続ユーザー保存を明示的に導入するまで browser session persistence で十分とします。

## 検索 UI 規約

- 検索は tag と attribute の AND filter として扱う。
- filter は browser session 中に保持する。
- result card は image、GIF、video、audio、PDF の media type を扱えるようにする。
- result を開くときは stream endpoint を使い、検索結果から即座にオリジナルメディアを表示できるようにする。

## API client 規約

- API URL 組み立ては `frontend/src/services` または URL helper に閉じる。
- backend error envelope はユーザーが理解できる message に変換する。
- request construction と form validation を超えて、backend filtering logic を UI に重複実装しない。

## テスト

hook と utility の単体テストを優先し、統合挙動は E2E で確認します。

```bash
mise run frontend:test
mise run frontend:e2e
```

優先して守る対象は、filter parsing、persisted filter、media URL construction、search-to-preview path です。

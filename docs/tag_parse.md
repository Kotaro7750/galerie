# ドキュメントのスコープ

本ドキュメントのスコープは以下とする

* コンテンツストレージからXMPサイドカーを取得してタグを抽出する方法の説明
* 抽出したタグによってメタデータ及び検索用インデックスを更新する方法の説明

正規コンテンツ及びXMPサイドカーへのアクセスには、[content_storage.md](./content_storage.md)で定義された境界を使用する。

# 処理フローの概要

```mermaid
flowchart TB
    start@{ shape: circle, label: "Start" }
    list_contents["コンテンツ識別子の一覧を取得"]
    content_loop_start[/"全てのコンテンツUUIDを処理するまで"\]
    is_xmp_exist@{ shape: diamond, label: "対応するXMPサイドカーが存在する" }
    parse_xmp["XMPサイドカーを取得して解析"]
    db_hit@{ shape: diamond, label: "DBにタグ情報のレコードが存在する" }
    is_current@{ shape: diamond, label: "DBとサイドカーファイルでバージョン情報が一致する" }
    create_xmp["空のXMPサイドカーを作成"]
    update_db["DBにタグ情報を登録"]
    content_loop_stop[\"ループ終了"/]
    stop@{ shape: dbl-circ, label: "Stop" }

    start --> list_contents
    list_contents --> content_loop_start
    content_loop_start --> is_xmp_exist
    is_xmp_exist -- Yes --> db_hit
    is_xmp_exist -- No --> create_xmp
    db_hit -- Yes --> is_current
    is_current -- Yes --> content_loop_stop
    is_current -- No --> parse_xmp
    db_hit -- No --> parse_xmp
    parse_xmp --> update_db
    create_xmp --> update_db
    update_db --> content_loop_stop
    content_loop_stop --> stop
```

# エラー処理
XMPサイドカーを取得できない場合、XMPとして解析できない場合又は新しいXMPサイドカーを作成できない場合は、当該コンテンツの更新処理を失敗とする。
この場合は既存のタグ情報を正常な最新情報として扱わず、当該コンテンツの更新失敗を記録して残りのコンテンツの処理を継続する。

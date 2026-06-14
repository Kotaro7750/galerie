# Observability

Galarie はローカル開発と本番相当コンテナの両方で Observability を利用できる状態を維持します。バックエンドは OpenTelemetry の trace、metric、log を出力し、devcontainer stack が collector と可視化基盤を提供します。

## ローカルスタック

`.devcontainer/observability/` 配下の compose stack は次を提供します。

- OTel Collector: `localhost:4317` と `localhost:4318`
- Prometheus: `http://localhost:9090`
- Loki: `http://localhost:3100`
- Tempo: `http://localhost:3200`
- Grafana: `http://localhost:3300`

## バックエンド signals

次の flow を計測します。

- index rebuild の開始、完了、失敗、skip file、duration。
- search request count、latency、tag count、result count、cache status。
- thumbnail generation count、latency、cache hit/miss、failure。
- streaming request count、bytes served、Range usage、failure class。

ユーザー向け error に host の absolute path を出してはいけません。debug に必要な場合は、sanitized relative path を log に含めます。

## 運用規約

- telemetry は local debugging で無効化できるようにする。exporter setup の失敗で media browsing を止めない。
- formatted log string より structured field を優先する。
- local と container run で service name と environment の規約を揃える。
- latency、cache correctness、media access に影響する挙動を追加・変更する場合は metric も追加する。

## 確認手順

1. devcontainer または observability compose stack を起動する。
2. `OTEL_EXPORTER_OTLP_ENDPOINT` を設定して backend を起動する。
3. index rebuild と search を実行する。
4. Grafana、Tempo、Loki で trace と log を確認する。

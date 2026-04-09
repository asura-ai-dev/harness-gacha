# MVP PRD

## Objective

Claude Code や Codex などのツール向けに、第三者クリエイターが作成した導入可能な harness pack を販売できるキュレーション型ストアを構築する。MVP では、ユーザーがリッチな TUI 上で pack を発見し、Web checkout で購入し、ローカルに導入できること、そして法務・運用リスクを抑えつつクリエイターへ売上分配できることを目標とする。

## Scope

- 招待制クリエイター onboarding
- ローンチ時はプラットフォームによる再販モデル
- 固定価格のデジタル商品販売
- TUI 上での無料カプセル風 discovery 体験
- Web 決済と購入後のアンロック導線
- pack の審査、メタデータ表示、安全性開示
- 月次のクリエイター売上分配

## Assumptions

- 最初のローンチ対象は日本市場とする
- 初期の販売主体はプラットフォーム運営者とする
- pack は標準化された manifest と導入手順を含む ZIP で配布する
- 公開前に法務レビューを行う
- 初期ユーザーは CLI / TUI に慣れた技術者層を想定する

## Problem Statement

`skills`、`agents`、`hooks`、template、workflow pack のような高品質な harness asset は、発見、比較、信頼判断、導入が難しい。現在は GitHub リポジトリ、チャット断片、アドホックな ZIP 配布に分散しており、クリエイターには明確な収益化手段がなく、購入者も購入前に互換性、安全性、保守品質を判断しにくい。

## Target Users

- Claude Code、Codex、または類似の coding-agent ツールを使う開発者
- 再利用可能な harness pack を導入したい技術チーム
- 繰り返し使える開発ワークフローを商品化したい上級クリエイター

## Key Capabilities

- TUI 上でキュレーションされた harness pack カタログを閲覧できる
- 1つずつおすすめ pack を提示する無料のランダム discovery を実行できる
- 購入前に pack の詳細を確認できる
- クリエイター情報、互換性、必要権限、同梱ファイル、更新履歴を閲覧できる
- Stripe Checkout または Payment Links を通じて pack を購入できる
- 決済完了後に download と install 手順がアンロックされる
- pack ごとの売上を追跡し、月次でクリエイターへ分配できる
- 掲載前に提出された pack をレビューできる

## Product Principles

- 新奇性より信頼性を優先する
- discovery は遊び心があってよいが、支払いは明快であるべき
- 安全性メタデータはバックオフィス情報ではなくプロダクト本体の一部とする
- 導入体験は手作業コピーより簡単であるべき
- MVP は marketplace の開放性よりも明確さを優先する

## Marketplace Model for MVP

初期の事業モデルはキュレーション型の再販とする。

- クリエイターは pack をプラットフォームに提出する
- プラットフォームは審査し、価格を設定し、掲載して販売する
- 購入者はプラットフォームから pack を購入する
- プラットフォームは純売上に応じてクリエイターへ合意済みの分配を行う

このモデルは、seller 表示、カスタマーサポート、返金、コンプライアンス、品質管理を単純化できるため、MVP ではオープン marketplace より優先する。

## Pack Requirements

各 pack は最低限以下を含むこと。

- `manifest.json`
- 明確な pack 名と version
- 対応ツール名と対応 version
- 同梱ファイル一覧
- 導入手順
- 必要権限
- 外部通信の有無と内容
- クリエイター表記
- ライセンス条件
- changelog または最終更新日

## Safety Review Requirements

各 pack は最低限以下を確認すること。

- 不審な shell command
- 隠れた network call
- credential exfiltration の危険
- 破壊的な git / filesystem 操作
- 第三者の知的財産またはライセンス違反
- 誤解を招く互換性表記

## User Journey

1. ユーザーが TUI ストアを開く
2. ユーザーが pack を閲覧するか、無料のカプセル風 discovery を実行する
3. ユーザーが pack 詳細画面を開き、metadata、files、trust signal を確認する
4. ユーザーが購入を選択する
5. ユーザーが Web checkout に遷移する
6. 決済が成功し、プラットフォームが entitlement を記録する
7. ユーザーが TUI に戻り、download と install 手順をアンロックする

## Functional Requirements

### Catalog and Discovery

- TUI は featured、recent、recommended な pack を一覧表示できること
- TUI は無料のランダム discovery を提供すること
- ランダム discovery の実行に課金を伴わないこと
- どの導線から入っても同じ商品情報に到達できること

### Product Detail

- 各 pack 詳細画面には creator、price、compatibility、included files、permissions、install steps、update date を表示すること
- 各 pack に短い preview または sample snippet を持たせること
- 危険度の高い permission は視認しやすくラベル表示すること

### Purchase and Entitlement

- 購入は TUI 内の決済入力ではなく、Web checkout で実行すること
- 決済成功後に downloadable artifact と install 手順をアンロックすること
- entitlement は user account 単位で保持すること

### Creator Operations

- MVP では招待された creator のみが pack を提出できること
- 各 submission は掲載前に手動レビューを通ること
- 運営者は pack を delist または suspend できること

### Revenue Share

- 各 pack に creator share の割合を設定できること
- gross sales、refunds、creator payout の計算基礎を記録できること
- 初期の精算サイクルは月次とすること

## Legal and Compliance Requirements

- 日本向けに必要な commerce disclosure を表示すること
- 表示内容には seller identity、price、payment timing、delivery timing、refund policy を含めること
- MVP では有料ランダム販売を避けること
- コレクション要素で法的リスクが上がる仕組みを避けること
- pack の内容とリスク関連 metadata を購入前に開示すること
- OpenAI や Anthropic との公式提携を誤認させる表現を避けること
- creator に対して、提出物に必要な権利を有していることを保証させること

## Non-Goals

- ローンチ時の open self-serve marketplace
- 有料ガチャ
- ローンチ時の creator 直接販売モデル
- ターミナル内でのネイティブなカード情報入力
- team workspace、org billing、private registry
- 海外 creator 向けの cross-border tax 対応
- モバイルアプリ client

## Acceptance Criteria

- ユーザーが課金なしで TUI 上から pack を発見できること
- ユーザーが購入前に pack の compatibility、files、risk metadata を確認できること
- ユーザーが Web checkout で支払いを完了できること
- 支払い成功後に pack の download と install 手順がアンロックされること
- 運営者が pack の review、approve、reject、delist、update を行えること
- 月次精算のための creator payout 金額を計算できること
- 対外的に「有料ガチャ」ではなく「キュレーション型ストア」として説明できること

## Success Metrics

- Catalog view から pack detail への遷移率
- Pack detail から checkout 開始への遷移率
- Checkout 完了率
- 購入から install 完了までの到達率
- Refund rate
- 月間 GMV
- 掲載 pack あたりの creator 収益
- Repeat purchase rate

## Operational Requirements

- Pack review checklist を維持すること
- Takedown と incident response の手順を整備すること
- Listing approval と delist の監査記録を残すこと
- Refund 処理と entitlement revocation に対応すること
- Pack と creator 単位で support request を追跡できること

## Open Questions

- 初期リリースでは匿名閲覧を許可するか、それとも checkout 前に sign-in を必須にするか
- Install は CLI command ベースにするか、手動 ZIP 展開のみも許可するか
- Creator revenue share は全体で固定にするか、pack ごとに変更可能にするか
- 配布物は ZIP のみをホストするか、git-based install source も扱うか

## Implementation Order

1. Pack manifest schema と review checklist を定義する
2. Catalog、detail、entitlement 画面を含む TUI 情報設計を作る
3. Product catalog と pack detail の体験を実装する
4. Web checkout と entitlement unlock flow を統合する
5. 運営者向け review tool と listing control を追加する
6. Creator accounting と月次 payout reporting を追加する
7. 公開前に legal copy、refund policy、seller disclosure の内容を確定する

## Validation Plan

- PRD を product、engineering、legal 関係者でレビューする
- 購入前に trust と compatibility の情報が十分伝わるかを検証する
- Stripe test mode で purchase と entitlement flow のドライランを行う
- Refund を模擬し、access revocation の挙動を確認する
- Sample pack を safety checklist でレビューし、manifest 要件を調整する
- 公開文言が公式ブランド誤認や有料ランダム販売を示さないことを確認する

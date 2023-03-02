# kirby-speedscore-recorders

Kirby-Speed/Score-Recorders(KSSRs)はカービィシリーズのゲームのタイム/スコアを集積するTA/RTAプラットフォームです。ボスやステージごとの記録集積に特化しています。
当サイト『Kirby-Speed/Score-Recorders』は、星のカービィシリーズ非公式の二次創作であり、HAL研究所・任天堂とは一切関係ありません。
不明点や疑問点等ありましたら、お手数ですがDiscordのサーバーやTwitterまでご連絡お願い致します。

Kirby-Speed/Score-Recorders(KSSRs) is the page that provides scoreboards about the games of the Kirby series, which specializes in recording time/score specific for each Boss and Stage.
This site is a fan-made website that has no relation with HAL Laboratory and Nintendo Co.
If you have any problems about using KSSRs, please contact us(me) on the Discord Server or Twitter.

# 新規プロジェクトの理由
このプロジェクトは、旧来KSSRs( https://github.com/Appbird/kss-recorders )の改良のために打ち立てられています。

## 目標とする改善点
- 旧来KSSRsにあるゲームモードによる分別
  - ゲームモードによるページの分割により、さまざまなゲームの記録を管理するKSSRsで、異なるゲームの記録が混雑しないことを狙った。
  - しかし、記録に辿り着くまでの手数が多く、複雑でない(手軽でない)。
  - 統一検索ボックスの導入により、記録に直ぐにたどり着けるようにユーザー体験を改良する。
- 旧来KSSRsの記録申請の煩雑さ
  - 記録申請をするために、申請者が記録, タイム, レギュレーション種別などを全て入力する必要がある。
  - しかし、一般には二重確認の必要性が薄い。(記録のメディアを見れば明らかなため)
  - このように煩雑であるため、一度に大量の記録を投稿することができない。
  - よって、記録, タイム, レギュレーション種別の入力を管理者側で入力する仕様に変更する。
- APIが複雑
  - 内部IDとアイテムの対応を理解していないとAPIを動かすことはできない、
  - この点を、内部IDを理解せずとも、英語の略称を使えるように改善する。
  - また、HTTPリクエストメソッドの基準に沿った、純粋なREST APIとなることを目指す。
- Reactの導入によりメンテナンス/拡張をより容易にすることを図る。

# 現在の開発方針
- アジャイル手法によって実装を進める。
  - フィードバックは実物がない限り得られないと考えたため。
  - 現状開発者本人が狙った仕様が本当にタイムアタックの環境において適切かどうか不明。



# リンク集
- KSSRsの専用Discordサーバー  [https://discord.gg/S7u9Cc5vnR]
- プロジェクトに関する開発/計画日誌 https://scrapbox.io/kssrecorders/Kirby-Speed%2FScore-Recorders


Encode and decode using Base2136 representation
===

ものの冗談でBase64っぽいことを常用漢字2,136文字でやってみた。効率はさておきbase58よりも日本人だったら漢字の方が覚えやすいんでは？というノリでつくってはみたものの、実際にあれこれエンコードしてみると英数字7文字が漢字4文字になるのが分かりやすいかどうかは微妙かな。

実際につくってみると常用漢字表の収集に始まって、固定長のバイト長をどう設定するかとか、それなりに考えるべきことがありますね。あと自分に縁のある番号を変換して「死」とか含まれていると気分を害してしまうし、常用漢字の中にもそこそこ難しい字、紛らわしい字が含まれているので、小学校で教わる漢字から縁起が悪いのを除くとか、漢字の範囲も見直す必要がありそう。

ブラウザ上での動作デモ (WASM)
---

[BASE2136デモ画面]https://masanork.github.io/bs2136-wasm/

使い方
---

標準入力から読み込んで標準出力にエンコード

-i: 整数モード、以前のint2b2136相当

-d: デコード

-h: ヘルプ

謝辞
---

提案してくださった @visvirial さんに感謝します。
[常用漢字2,136種類を4文字で20,816,369,750,016通り作れるので、日本人ならこれが一番簡単な気がします。](https://twitter.com/visvirial/status/1692827765874348308)

本プログラム等の作成にはChatGPT Code InterpreterとGitHub Copilotを利用しています。

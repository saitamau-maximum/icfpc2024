# Sor4chi's Notes

## 6/28

### [23:38] Tokenizerの実装完了

Rustで簡易的なTokenizerを実装、それに伴いstringify binを作成。

## 6/29

### [1:54] 暗号文の完全パースと実行に成功

暗号文をパースし、実行することに成功。
今のところバグなし。

### [2:17] できたのでとりあえずSubmitしてみる

language_test出力の`unary # is nota correct`を送信。

```txt
Unknown instruction: \"unary # is not...ect\"
```

language_test出力の`unary # is nota correct`に`echo `をpreprendして送信。

```txt
unary # is not correct

ou scored some points for using the echo service!
```

### [2:36] リーダーボード眺めてて`get lambdaman`できることに気づく

`get lambdaman`を送信。

[これが帰ってきた](./lambdaman/intro.md)

### [2:42] `get speaceship`する

`get spaceship`を送信。

[これが帰ってきた](./spaceship/intro.md)

### [2:49] `get 3d`する

まだできないよって言われた。

### [2:54] スペースシップ問題の仕様を読む

```text
1 2 3
4 5 6
7 8 9
```

っていうキーボードがあって、5を中心として右に下に正の、左に上に負の方向に速度を変えながら移動するゲーム。

### [4:06] ダウンローダー作った

問題が例えばspaceshipだったら`get spaceship1` ~ `get spaceship25`まで全部ダウンロードするやつ。

21以降の問題からダウンロードしてきた入力がまさかのプログラム実行しないと取得できない形式になってることに気づいた。作ってよかったEvaluator。

### [4:28] spaceship1を解く

巡回場所が少ない場合は初期解のビムサである程度とけた。
1,3を提出してかなりいい点が出てるはず。

でもそこまで考慮しなくても単純なTSPをして、経路決めうちで焼きなましとかできそう。

### [5:16] やっぱりおかしいのでhelloを見直す

language_testの出力が単純にエラーを教えてくれていることにやっと気がついた。
修正したらフラグが出てきたのでサブミットして終わり、ごめんなさい。

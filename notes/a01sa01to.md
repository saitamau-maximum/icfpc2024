# a01sa01to's memo

## day 1

21:00 読む (ほかの作業しながら)

なんだこの言語
<https://icfpcontest2024.github.io/icfp.html>

どうやら 94 文字使って書かれるらしい。

[詳しくはここに書く](./ICFP-language.md)

`https://boundvariable.space/communicate` に `S'%4}).$%8` を送って POST したらなんか返ってきた。

```text
SB%,,/}!.$}7%,#/-%}4/}4(%}M#(//,}/&}4(%}</5.$}P!2)!",%_~~<%&/2%}4!+).'}!}#/523%j}7%}35''%34}4(!4}9/5}(!6%}!},//+}!2/5.$l}S/5e2%}./7},//+).'}!4}4(%}u).$%8wl}N/}02!#4)#%}9/52}#/--5.)#!4)/.}3+),,3j}9/5}#!.}53%}/52}u%#(/w}3%26)#%l}@524(%2-/2%j}4/}+./7}(/7}9/5}!.$}/4(%2}345$%.43}!2%}$/).'j}9/5}#!.},//+}!4}4(%}u3#/2%"/!2$wl~~;&4%2},//+).'}!2/5.$j}9/5}-!9}"%}!$-)44%$}4/}9/52}&)234}#/523%3j}3/}-!+%}352%}4/}#(%#+}4()3}0!'%}&2/-}4)-%}4/}4)-%l}C.}4(%}-%!.4)-%j})&}9/5}7!.4}4/}02!#4)#%}-/2%}!$6!.#%$}#/--5.)#!4)/.}3+),,3j}9/5}-!9}!,3/}4!+%}/52}u,!.'5!'%y4%34wl~
```

まずは ICFP 言語を読む。

Integer 解析をいったん C++ で書いた。

```cpp
#include <bits/stdc++.h>
using namespace std;
#ifdef LOCAL
  #include "settings/debug.cpp"
  #define _GLIBCXX_DEBUG
#else
  #define Debug(...) void(0)
#endif
#define rep(i, n) for (int i = 0; i < (n); ++i)
using ll = long long;
using ull = unsigned long long;

int main() {
  string t;
  cin >> t;
  string s = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
  int ans = 0;
  for (char c : t) {
    int idx = s.find(c);
    ans = ans * 94 + idx;
  }
  cout << ans << endl;
  return 0;
}
```

22:00 いったん読み終わった。 Lambda と Limits 以外。
いったん文字列の対応表を作ってみる。
と思ったが <https://github.com/saitamau-maximum/icfpc2024/pull/2/commits/237e2bf936cd6cd2b88b7cbe226afc8150398264> のテストにすでに書いてあった、これほんとか？と試してみると、ちゃんと動いた。
あ、ちゃんと strings の部分に書いてあるやん。誤読してた。
最初のメッセージは

```text
Hello and welcome to the School of the Bound Variable!

Before taking a course, we suggest that you have a look around. You're now looking at the [index]. To practice your communication skills, you can use our [echo] service. Furthermore, to know how you and other students are doing, you can look at the [scoreboard].

After looking around, you may be admitted to your first courses, so make sure to check this page from time to time. In the meantime, if you want to practice more advanced communication skills, you may also take our [language_test].
```

となるらしい。
いったん「get echo」を送ってみる。

```md
The School of the Bound Variable provides a special echo service for you. If you send an ICFP expression evaluating to:

\`\`\`
echo <some text>
\`\`\`

it will respond with `<some text>`.

Hint: you can use this to validate the Macroware Insight evaluator has the expected behavior. Of course the usual limitations still apply.
```

echo の後に文字列を送るとそれに対応するやつを実際に作ってくれるっぽい？

試しに `hoge` を暗号化して `(/'%` なので `echo S(/'%` を暗号化して送ってみる。

```text
S(/&#39;%

You scored some points for using the echo service!
```

ん、そのまま返ってきた？ なんか `'` が `&#39;` になってるが、まあいっか。どうせ HTML エスケープしてるだけだし。
よくわからんながらもなんか得点ゲットした。

「get scoreboard」も送ってみる。順位表のテキストデータが表示された。

```text
You scored some points for looking at the scoreboard! You can also do `get scoreboard <coursename>` to see the scoreboard for a specific course.
```

そしたら language_test やってみる。

```text
? B= B$ B$ B$ B$ L$ L$ L$ L# v$ I" I# I$ I% I$ ? B= B$ L$ v$ I+ I+ ? B= BD I$ S4%34 S4 ? B= BT I$ S4%34 S4%3 ? B= B. S4% S34 S4%34 ? U! B& T F ? B& T T ? U! B| F F ? B| F T ? B< U- I$ U- I# ? B> I$ I# ? B= U- I" B% U- I$ I# ? B= I" B% I( I$ ? B= U- I" B/ U- I$ I# ? B= I# B/ I( I$ ? B= I' B* I# I$ ? B= I$ B+ I" I# ? B= U$ I4%34 S4%34 ? B= U# S4%34 I4%34 ? U! F ? B= U- I$ B- I# I& ? B= I$ B- I& I# ? B= S4%34 S4%34 ? B= F F ? B= I$ I$ ? T B. B. SM%,&k#(%#+}IEj}3%.$}z3/,6%},!.'5!'%y4%34} U$ B+ I# B* I$> I1~s:U@ Sz}4/}#,!)-}0/).43}&/2})4 S)&})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}q})3}./4}#/22%#4 S").!29}k})3}./4}#/22%#4 S5.!29}k})3}./4}#/22%#4 S5.!29}_})3}./4}#/22%#4 S5.!29}a})3}./4}#/22%#4 S5.!29}b})3}./4}#/22%#4 S").!29}i})3}./4}#/22%#4 S").!29}h})3}./4}#/22%#4 S").!29}m})3}./4}#/22%#4 S").!29}m})3}./4}#/22%#4 S").!29}c})3}./4}#/22%#4 S").!29}c})3}./4}#/22%#4 S").!29}r})3}./4}#/22%#4 S").!29}p})3}./4}#/22%#4 S").!29}{})3}./4}#/22%#4 S").!29}{})3}./4}#/22%#4 S").!29}d})3}./4}#/22%#4 S").!29}d})3}./4}#/22%#4 S").!29}l})3}./4}#/22%#4 S").!29}N})3}./4}#/22%#4 S").!29}>})3}./4}#/22%#4 S!00,)#!4)/.})3}./4}#/22%#4 S!00,)#!4)/.})3}./4}#/22%#4
```

？？？
ところどころにちりばめられた文字列を解析してみると、出来たら flag 的なものが出力されてこれを送ったらポイントもらえるよってことっぽい。
まじで CTF っぽいな

23:20 ラムダ式に戻るか。
なんもわかってない

24:00 ようやく理解した

24:28 lambda 実装
をしようとしたがコードリーディングで終わり

眠いので寝る

## day 2

09:50 起きた + ご飯食べたので再開

実装がすでに終わっているようなので進捗を見る。
spaceship の intro を読む。
どうやら 10 キーの数字の位置が移動を表しているらしい。

```text
7 8 9
4 5 6
1 2 3
```

上から 1 行目: vy +1
上から 2 行目: vy ±0
上から 3 行目: vy -1
左から 1 列目: vx -1
左から 2 列目: vx ±0
左から 3 列目: vx +1

めんどくさーい
でもこどげに同じような問題があった気がする、火星のやつ: <https://www.codingame.com/training/medium/mars-lander-episode-2>
なにか使えるかも？

lambdaman も読む。
あー経路探索ねぇ　これは解けそうな気がする
いったんデータ全ダウンロードする。
level 6 の DL で stackoverflow してる、なぜ？

```text
B. SF B$ B$ L" B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L$ L# ? B= v# I" v" B. v" B$ v$ B- v# I" Sl I#,
```

あー parse 部分で default で node そのまま返してるせい？
いったん飛ばして lambdaman を手作業で解いてみる
(根を lambdaman の初期位置として決めて ~~木 dp とかすればいいのかも、なんか前に AtCoder で見たような: <https://atcoder.jp/contests/arc179/tasks/arc179_d> (似てるのはこれだが、今回は瞬間移動できない)~~ 木じゃないからダメ)

とりあえず lambdaman 1, 2 を解いた。
3 以降はプログラム的に最適解を求めたほうがよさそう。 (実装思いついてないので後でやる)

ほかの問題もダウンロードしちゃう。
3d, efficiency と思ったがまだできないらしい。

spaceship の 2 が最適じゃなかったっぽいので適当に手作業で直した。

ごはん + JAG 模擬国内予選ぞいぞい

## day 3

いろいろあって day 2 はあのまま終了。

14:00 やるか
3d, efficiency が開放されたので見てみる。

3d 読んだ。めっちゃおもしろそう。
3d1 を試しにやってみる。

15:32 解けた。 17433 点。
ベストが 4800 なのでもっと減らせるらしい。まじ？

```text
. . . . . 1 . . .
A > . > . = . . .
v 1 . . . . > . .
. - . . A > . * S
. . . . v . . . .
. v . . . . . . .
. . > . * . . . .
. v . . . . . . .
. . . 0 @ 5 . . .
. v . . 5 . . . .
. . . . . . . . .
1 @ 10 . . . . . .
. 5 . . . . . . .
```

カウントアップしていったほうが短くなりそう？
書いた ↓
score = 6300 えー

```text
. > . > . . .
^ 1 . 1 * S .
A # . * . . .
. . v . > . .
. . . . 2 @ 3
. 1 + . . 3 .
. . . . . . .
. 1 @ 6 . . .
. . 3 . . . .
```

いったん 3d2 に移動。
2550 、これで 16 位なの？えー

```text
. 1 . . .
A + . . .
. . . . .
A % . . A
. . > . *
-1 # S . S
. . . . .
```

3d3: 3738

```text
. . . . 0 .
. A > . = S
1 + . . . .
. . . . . .
A % . . A .
. . > . / .
-1 = S . S .
```

18:30 ようやく 3d4 が解けた: 157920 点。やばすぎる。
ちょっと直して 150640 点。たすけて～

```text
. B . 1 .
A * . + .
. . v . .
. . . % .
. . . . .
. . -1 = .
. . < . .
. v A - .
. . . . .
. v A % .
. . . . .
. v -1 = .
. . . . .
1 + 1 + .
. . . . .
A + B + S
. S . . .
A > . > . > . . B > . > . > .
. . . . . . v . . . . . . . v
. . . < . < . . . . . < . < . .
. . v 1 . 0 . B . . v 1 . 0 . A
. . . - . = . + . . . - . = . +
. . . . v . . S . . . . v . . S
. . . < . . . . . . . < . . . .
. 0 @ 3 . . . . . 0 @ 3 . . . .
. . 3 . . . . . . . 3 . . . . .
A > . > . > . . B > . > . > .
. . . . . . v . . . . . . . v
. . . < . < . . . . . < . < . .
. . v 1 . 0 . A . . v 1 . 0 . B
. . . + . = . + . . . + . = . +
. . . . v . . S . . . . v . . S
. . . < . . . . . . . < . . . .
. 0 @ 3 . . . . . 0 @ 3 . . . .
. . 3 . . . . . . . 3 . . . . .
```

19:05 3d5 解けたと思ったが tick limit に引っかかってるらしい。
visualizer ないとさすがにきつすぎるので作る。

20:55 ごはんたべた。 visualizer も (たぶん) できた。これでデバッグがしやすくなるぞい
3d2 みたかんじ、なんかコーナーケースが -1, -2 で発生してるのに気づいたのはここだけの話。

21:33 3d5 解けた～ 21168

```text
. . . < . < B > . > . > . .
. 0 = . . A > . % . . . v .
. . . . . v . . . v . . . .
. . v . < . . . < . . . v .
. . . + . . . v . . . . . .
. B . . . . . . > . . . v .
A * . / S . . . 3 @ 6 . . .
. . . . . . . . . 6 . 7 @ 6
. . . . . . . . . . . . 6 .
```

22:04 3d6 も解けた、コーナーケース (2) に殺された。。。

```text
. . . . 1 . . . . 2 .
. 2 > . + . > . A = .
A % . . . . . v . . /
. . . A = . . . . . S
. v . . . . 6 @ 3 . .
. . . A / S . 4 . . .
0 = . . . . . . . . .
. S . . . . . . . . .
```

22:44 3d7 49896

```text
. . . . . .  . 10  .   .  .  .
. > . > . >  .  *  .   >  .  .
^ . < A . .  .  .  .   .  v  .
. v . . . .  .  . 10   .  .  .
^ . > . > .  >  .  %   .  +  .
. . 0 = . v 10  .  .   .  .  .
^ . . . . .  /  .  .  10  @ -1
0 > . + . .  .  >  .   . 10  .
. . < . . .  .  .  v   .  .  .
0 * A = . .  .  .  .   .  .  .
. . . . / .  .   .  v  .  .  .
. v . . S .  .   .  .  >  .  .
. S . . . .  .   .  .  7  @ 10
. . . . . .  .   .  .  . 10  .
```

3d8 難しそうなのでいったん 9 をみる
難しいよ～撤退
24:14

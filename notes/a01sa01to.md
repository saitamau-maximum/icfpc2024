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

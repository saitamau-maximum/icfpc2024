# ICFP Language メモ

ASCII 33 `!` から 126 `~` までの 94 文字を使って書かれる言語。
最初の文字は indicator で、それに続く文字列を body ということにする。
各 token は空白区切り。

## 真偽値

body はなし、 indicator は `T` or `F`

- `T`: true
- `F`: false

## 数値

indicator が `I` かつ body は 94 進数で表現。
例えば、 ASCII 33 `!` は 0、 ASCII 126 `~` は 93。
`I/6` は 1337 (= $14 \times 94 + 21$)

## 文字列

indicator が `S` かつ body は ASCII 文字列。
ASCII 33 - 126 は順番に `abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#$%&'()*+,-./:;<=>?@[\]^_`|~<space><newline>` に対応している。

例として `SB%,,/}Q/2,$_` は `Hello, World!` になる。
よって、 `B` は `H`, `%` は `e`, ... に対応することがわかる。

## 単項演算子

「indicator が `U` かつ body がちょうど 1 文字」のあとに、ほかの ICFP 言語の式が 1 つ続く。

- `-`: マイナス (`U- I$` は `-3`)
- `!`: 論理 NOT (`U! T` は `false`)
- `#`: 文字列 to 数値 (`U# S4%34` は `I4%34` に対応してて、 `13818151`)
- `$`: 数値 to 文字列 (`U$ I4%34` は `S4%34` に対応してて、 `test`)

## 二項演算子

「indicator が `B` かつ body がちょうど 2 文字」のあとに、ほかの ICFP 言語の式が 2 つ続く。

- `+`: 加算 (`B+ I# I$` は $2 + 3 = 5$)
- `-`: 減算 (`B- I$ I#` は $3 - 2 = 1$)
- `*`: 乗算 (`B* I$ I#` は $2 \times 3 = 6$)
- `/`: 除算 (0 の方向へ丸める、 `B/ U- I( I#` は $-7 / 2 = -3$)
- `%`: 剰余 (たぶん `a - a / b * b` と同値、 `B% U- I( I#` は $-7 \% 2 = -1$)
- `<`: 整数比較 (`B< I$ I#` は $3 < 2$ なので `false`)
- `>`: 整数比較 (`B> I$ I#` は $3 > 2$ なので `false`)
- `=`: 等号比較 (数値でも bool でも string でも、 `B= I$ I#` は $3 = 2$ なので `false`)
- `|`: OR (`B| T F` は `true`)
- `&`: AND (`B& T F` は `false`)
- `.`: 文字列連結 (`B. S4% S34` は `4%34` となり、 `test`)
- `T`: 文字列 `y` の最初 `x` 文字を取り出す (`BT I$ S4%34` は `test` の最初 3 文字、 `tes`)
- `D`: 文字列 `y` の最初 `x` 文字を取り除く (`BD I$ S4%34` は `test` の最初 3 文字を取り除き、 `t`)
- `$`: `x`(`y`) を実行

## IF

indicator `?` ・ body なし
3 つの token。

1. 条件
2. 真の場合
3. 偽の場合

```text
? B> I# I$ S9%3 S./
```

は C++ 風に書くと `2 > 3 ? "yes" : "no"` になるらしい。

## ラムダ式

indicator `L` ・ body は integer と同様に解釈できる、仮引数の ID。
indicator `v` は変数で、 body は同上、引数の ID。

```text
B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK
```

は

```text
B$
├ B$
│ ├ L#
│ │ └ L$
│ │   └ v#
│ └ B. SB%,,/ S}Q/2,$_
└ IK
```

という構文木ができて、

```text
B$
├ B$
│ ├ L#
│ │ └ L$
│ │   └ v#              // v2
│ └ B. SB%,,/ S}Q/2,$_
└ IK

↓

B$
├ B$
│ ├ L#
│ │ └ L$ v#              // v3 => v2
│ └ B. SB%,,/ S}Q/2,$_
└ IK

↓

B$
├ B$
│ ├ L# L$ v#             // v2 => (v3 => v2)
│ └ B. SB%,,/ S}Q/2,$_   // "Hello" + " World!"
└ IK

↓

B$
├ B$ L# L$ v# B. SB%,,/ S}Q/2,$_  // (v2 => (v3 => v2))("Hello" + " World!")
└ IK

↓

B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK   // ((v2 => (v3 => v2))("Hello" + " World!"))(42)
```

と解析されるっぽい？

## 式の評価

名前呼び出し方式。変数が使われるときに評価する方式。
つまり、ラムダ式で引数が使用されない場合はその引数が評価されない。

```text
B$ L# B$ L" B+ v" v" B* I$ I# v8
B$ L" B+ v" v" B* I$ I#
B+ B* I$ I# B* I$ I#
B+ I' B* I$ I#
B+ I' I'
I-
```

```text
B$
├ L#
│ └ B$
│   ├ L"
│   │ └ B+ v" v"
│   └ B* I$ I#
└ v8

↓

v# に v8 を代入して、トップの B$ とその子 L#, v8 を消す

B$
├ L"
│ └ B+ v" v"
└ B* I$ I#

↓

v" に B* I$ I# を代入して B$ をなくす

B+
├ B* I$ I#
└ B* I$ I#

↓

B* I$ I# を評価

B+
├ I'
└ B* I$ I#

↓

B* I$ I# を評価

B+
├ I'
└ I'

↓

B+ I' I' を評価

I-
```

## 制限

10_000_000 回の β-簡約まで。これを超えると強制終了。

```text
B$ B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L" L# ? B= v# I! I" B$ L$ B+ B$ v" v$ B$ v" v$ B- v# I" I%
```

は 16 と評価されるが、 109 回の β-簡約が行われるらしい。

## その他の演算子

ほかのものはないと推測されているが、まだあるかもしれない。

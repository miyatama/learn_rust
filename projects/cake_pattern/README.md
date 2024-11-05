# outline

Cake Patternの学習

+ [Rust の DI を考える –– Part 2: Rust における DI の手法の整理](https://techblog.paild.co.jp/entry/2023/06/12/170637)
+ [実戦での Scala: Cake パターンを用いた Dependency Injection (DI)](https://eed3si9n.com/ja/real-world-scala-dependency-injection-di/)
+ [Minimal Cake Pattern 再考](https://qiita.com/tayama0324/items/03ba48d3277079f20500)
+ [Rust のテストのために DI は必要か？](https://qiita.com/yan7010/items/1112722fee9fd8000377)

## Minimal Cake Pattern

ニコニコアカウントで生み出されたインジェクション手法。依存性の宣言と注入の記述の簡潔さがいいらしい。

+ XXX -> トレイト境界(謎)
+ XXXに依存する -> UsesXXX
+ 依存の提供 -> ProvideXXX
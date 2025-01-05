# outline

mockallを使いこなしたい！

知見
+ mockallは特定のimpl traitにもautomockはつかえるけど、基本はtraitにautomock。
+ traitの実装は使う側でやる。同一階層にはおかない。

多分これが正解

+ domain層
  + 各trait 
  + domains定義 
+ repository層
  + 各domain実装 
  + 各repositoryのtrait 
  + repositories定義 
+ usecase層
  + 各repository実装 
  + 各usecaseのtrait 
  + usecases定義 
+ ui層
  + usecase実装 
  + lib 
  + main実装

# reference

+ [[Rust] mockallで単体テスト](https://qiita.com/deepgreenAN/items/1b9887db759bbb96c9b6)
+ [Rc<T>は、参照カウント方式のスマートポインタ](https://doc.rust-jp.rs/book-ja/ch15-04-rc.html)
+ [Rustで快適にテストしたいときに読む記事](https://techblog.paild.co.jp/entry/2023/10/23/100943)
+ crate.io
  + [mockall](https://crates.io/crates/mockall)
+ docs
  + [mockall](https://docs.rs/mockall/latest/mockall/)
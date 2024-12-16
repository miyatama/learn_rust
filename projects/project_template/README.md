# outline

コマンドラインアプリのテンプレート

## Layer

| name | description |
| :----- | :----- |
| ui | ユーザーへのインタラクション等を提供 |
| usecase | 各種ビジネスロジック |
| repository | usecaseへdomainを提供 |
| domain | アトム的なデータアクセッサ |
| util | データ定義やログなど |

utilモジュール

| name | description |
| :----- | :----- |
| log | ログ等の共通ロジック |
| data | User等のアトムデータ定義 |

```mermaid
---
参照関係
---
classDiagram

  usecase <.. ui 
  repository <.. usecase
  domain <.. repository
  util <.. ui

  class ui {
  }
  class usecase {
  }
  class repository {
  }
  class domain {
  }
```

## need

+ 3層構造
+ ログ出力
+ Webアクセス(非同期)
+ ファイル管理
+ 制御ファイル(yaml)
+ 起動パラメタでのdry-run

## ui layer

+ [RustのClapクレートがメチャクチャ良かった話](https://zenn.dev/shinobuy/articles/53aed032fe5977)
+ [Clap の Derive API で列挙型のコマンドラインオプションを実装する](https://zenn.dev/takanori_is/articles/rust-clap-derive-api-arg-enum)
+ [Rust の新しい HTTP サーバーのクレート Axum をフルに活用してサーバーサイドアプリケーション開発をしてみる](https://blog-dry.com/entry/2021/12/26/002649#Dependency-Injection)

## will

+ [x]DI構成をModules形式に変更
+ [x]Domainsのインジェクション
+ [x]Repositories内の全部参照に切り替えたい
+ [x]UseCases内の全部参照に切り替えたい
+ [ ]Domainsの実装
+ [ ]テスト組み込み
+ [ ]setting機能追加

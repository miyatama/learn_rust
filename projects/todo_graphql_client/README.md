# outline

[web_app_todo_graphql](https://github.com/miyatama/learn_rust/tree/main/projects/web_app_todo_graphql)のクライアント

# memo

schema取得([graphql_client_cli](https://github.com/graphql-rust/graphql-client/tree/main/graphql_client_cli))

```shell
cargo install graphql_client_cli
graphql-client introspect-schema http://localhost:3000 --output ./src/schema.json
```

リポジトリは自分はderiveで自動作成してもらう or 下記コマンドでコードを生成する

```shell
graphql-client generate ./src/query.graphql --output-directory ./src --schema-path ./src/schema.json
```

__schemaの情報取得(query)

```json
{
  __schema {
    description
    queryType {
      name
      description
      specifiedByURL
      isOneOf
    }
    mutationType {
      name
      description
      specifiedByURL
      isOneOf
    }
    subscriptionType {
      name
      description
      specifiedByURL
      isOneOf
    }
    types {
      name
      description
      specifiedByURL
      isOneOf
    }
    directives {
      description
    }
  }
}
```

# query

http://localhost:3000で投げられるquery

## hero(by StarWars)

![query_example](./imgs/query_example01.png)

```json
{
  query: hero(episode: "EMPIRE") {
    id
    name
    friends {
      id
    }
    appearsIn
  }
}
```

# errors

## cargo install graphql_clientでエラー

```text
error: there is nothing to install in `graphql_client v0.14.0`, because it has no binaries
`cargo install` is only for installing programs, and can't be used with libraries.
To use a library crate, add it as a dependency to a Cargo project with `cargo add`.
```

cliは下記でインストール

```shell
cargo install graphql_client_cli
```

# reference

+ [GraphQLのクエリを基礎から整理してみた](https://qiita.com/shunp/items/d85fc47b33e1b3a88167)
+ [RustでGitHub GraphQL APIを使ってissue一覧を取得する](https://blog.ymgyt.io/entry/fetch-issues-using-github-graphql-api-in-rust/)
+ [GraphQL の Introspection について - schema.json って何だろう](https://lightbulbcat.hatenablog.com/entry/2018/02/17/174623)
+ github
  + [graphql_client - example](https://github.com/graphql-rust/graphql-client/tree/main/examples)
+ crate.io
  + [graphql_client](https://crates.io/crates/graphql_client)
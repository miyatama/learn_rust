# outline

+ トレースする単位がspan

こんな感じにログが出る

```text
2025-01-03T09:25:28.625136Z  INFO span_1{key="hello"}:func01: about_tracing: func01
2025-01-03T09:25:28.625356Z  INFO span_1{key="hello"}:func01:func_b_01: about_tracing: func_b_01
2025-01-03T09:25:28.625599Z  INFO span_1{key="hello"}: about_tracing: func02
2025-01-03T09:25:28.625718Z  INFO span_1{key="hello"}:func_b_01: about_tracing: func_b_01
```

# problems

## could not find `EnvFilter` in `tracing_subscriber`

公式に下記記述があるので

> env-filter: Enables the EnvFilter type, which implements filtering similar to the env_logger crate. Requires “std”.

Cargo.tomlを変更

```toml
tracing-subscriber = {version = "0.3.19", features = ["std", "env-filter"]}
```

## method not found in `Registry`

発生個所

```text
3    | /     tracing_subscriber::registry()
4    | |         .with(
     | |         -^^^^ method not found in `Registry`
     | |_________|
     |
```

公式に下記記述があるので

> registry: enables the registry module. Enabled by default. Requires “std”.

Cargo.tomlを変更

```toml
tracing-subscriber = {version = "0.3.19", features = ["std", "env-filter", "registry"]}
```

↑だけだとだめ。[tracing/tracing-subscriberでログが出力される仕組みを理解する](https://blog.ymgyt.io/entry/how-tracing-and-tracing-subscriber-write-events/)を元に勉強しなおし。

# reference

+ [公式 - tracing](https://docs.rs/tracing/latest/tracing/)
+ [Rust での tracing](https://blog.ojisan.io/rust-tracing/)
+ [tracing/tracing-subscriberでログが出力される仕組みを理解する](https://blog.ymgyt.io/entry/how-tracing-and-tracing-subscriber-write-events/)
+ [OpenTelemetryに触れてみた](https://zenn.dev/yuta28/articles/what-is-opentelemetry)
+ [公式 - opentelemetry](https://opentelemetry.io/)
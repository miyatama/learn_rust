# What is it?

[Tauri](https://v2.tauri.app/start/)の勉強用プロジェクト


作ったときのログ

```text
✔ Project name · tauri-app
✔ Identifier · com.tauri-app.app
✔ Choose which language to use for your frontend · Rust - (cargo)
✔ Choose your UI template · Leptos - (https://leptos.dev/)

Template created!

Your system is missing dependencies (or they do not exist in $PATH):
╭───────────┬───────────────────────────────────────────────────────────╮
│ Tauri CLI │ Run `cargo install tauri-cli --version '^2.0.0' --locked` │
├───────────┼───────────────────────────────────────────────────────────┤
│ Trunk     │ Run `cargo install trunk --locked`                        │
╰───────────┴───────────────────────────────────────────────────────────╯

Make sure you have installed the prerequisites for your OS: https://tauri.app/start/prerequisites/, then run:
  cd tauri-app
  cargo tauri android init

For Desktop development, run:
  cargo tauri dev

For Android development, run:
  cargo tauri android dev
```

# Tauri + Leptos

This template should help get you started developing with Tauri and Leptos.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).


# problem

## Permission fs:allow-write-text-file not found

[公式のpermission](https://v2.tauri.app/learn/security/using-plugin-permissions/)を学習中に発生

```text
 Permission fs:allow-write-text-file not found, expected one of core:default, core:app:default, core:app:allow-app-hide, core:app:allow-app-show, ...
warning: build failed, waiting for other jobs to finish...
```

[FileSystemのpermission](https://v2.tauri.app/plugin/file-system/#default-permission)

下記コマンド打ってなかった([公式](https://v2.tauri.app/learn/security/using-plugin-permissions/)に記載がない？)

```shell
cargo tauri add fs
```

# reference

+ [TauriとLeptosで作るデスクトップアプリ（1）プロジェクトを作成する](https://zenn.dev/daizutabi/articles/tauri-leptos-01)
+ [Leptosを使ってTauriアプリのフロントエンドもRustで書く](https://zenn.dev/laiso/articles/ab8db73d66623a)
# outline

パフォーマンス計測

Windowsの場合はperfの代わりにdtraceが必要

```shell
cargo install flamegraph
cargo build
# cargo flamegraph --bin ./target/debug/multi_thread.exe
```

```shell
# Windowsだと失敗する
cargo install cargo-profiler
cargo profiler view
```

+ [perfとflamegraphを使ってプロファイリング](https://qiita.com/KentaAdachi/items/5a266984c074d29e2e32)

## フォークジョインパターン

タスクを細かく分割して実施

## 発生した問題

### 2.1.3 スレッドの同期と非同期

#### Mutexを使った同期

```rust
error[E0277]: the trait bound `Mutex<_>: Clone` is not satisfied
  --> src\lesson\basics.rs:75:27
   |
75 |             let counter = Mutex::clone(&counter);
   |                           ^^^^^ the trait `Clone` is not implemented for `Mutex<_>`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `multi_thread` (lib) due to 1 previous error
```

Arcと併せて利用する必要あるのでは感

### 3.2.3 タスクの分割と統合

合計スレッドを分割した時点で`value used here after move`

<details><summary>Code</summary>

```rust
pub fn spilt_and_join() {
    debug!("start fork_join::spilt_and_join");
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let handle1 = thread::spawn(move || {
        let sum: i32 = data[..5].iter().sum();
        sum
    });
    let handle2 = thread::spawn(move || {
        let sum: i32 = data[5..].iter().sum();
        sum
    });
    let result1 = handle1.join().unwrap();
    let result2 = handle2.join().unwrap();
    let final_result = result1 + result2;
    info!("result: {:?}", final_result);
}
```

</details>

Arc, Mutexを使って対応

<details><summary>code</summary>

```rust
pub fn spilt_and_join() {
    debug!("start fork_join::spilt_and_join");
    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    let handle1 = thread::spawn({
        let data = Arc::clone(&data);
        move || {
            let data = data.lock().unwrap();
            let sum: i32 = data[..5].iter().sum();
            sum
        }
    });
    let handle2 = thread::spawn({
        let data = Arc::clone(&data);
        move || {
            let data = data.lock().unwrap();
            let sum: i32 = data[5..].iter().sum();
            sum
        }
    });
    let result1 = handle1.join().unwrap();
    let result2 = handle2.join().unwrap();
    let final_result = result1 + result2;
    info!("result: {:?}", final_result);
}
```

</details>




# reference

+ [Rustでのマルチスレッドプログラミング入門](https://www.amazon.co.jp/Rust%E3%81%A7%E3%81%AE%E3%83%9E%E3%83%AB%E3%83%81%E3%82%B9%E3%83%AC%E3%83%83%E3%83%89%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E5%85%A5%E9%96%80-%E5%AE%89%E5%85%A8%E6%80%A7%E3%81%A8%E3%83%91%E3%83%95%E3%82%A9%E3%83%BC%E3%83%9E%E3%83%B3%E3%82%B9%E3%81%AE%E8%9E%8D%E5%90%88-%E3%83%8D%E3%82%B3-ebook/dp/B0CR48X5QR/ref=sr_1_1?__mk_ja_JP=%E3%82%AB%E3%82%BF%E3%82%AB%E3%83%8A&crid=3AFIJT2IN2HDQ&dib=eyJ2IjoiMSJ9.UU5u14Vp_C8LYfdNgIYbEBaJIYkHTbko--XJxvedGxsTk0B8Rrqx2fiXM2Q5VuFNl4xLh28PTImMqZFeljCA6J087uMZ-jpHO3g064d8DWJ4Lmf1XaIP45TM_MeiA68jwuzWg_oKoQfaJW0hboc_aXXSqXrurpy3w2ofueaBZ6AJ5JrG8wklayF_0ro1A9JZaQchtZQxdjPLe7MmI0DrbEHWnvwkwPmt4ISUBlMGeDE.jCu7wjPFklupAkeRezgp0n3mxgMKzVMhA5ziTeY4uNY&dib_tag=se&keywords=Rust%E3%81%A7%E3%81%AE%E3%83%9E%E3%83%AB%E3%83%81%E3%82%B9%E3%83%AC%E3%83%83%E3%83%89%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E5%85%A5%E9%96%80&qid=1732102765&s=digital-text&sprefix=rust%E3%81%A7%E3%81%AE%E3%83%9E%E3%83%AB%E3%83%81%E3%82%B9%E3%83%AC%E3%83%83%E3%83%89%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E5%85%A5%E9%96%80%2Cdigital-text%2C157&sr=1-1)
+ [Rustにおけるスレッド間でのデータ共有とstd::thread::scope](https://zenn.dev/toru3/articles/ce9232f53c47c8#mutex%E3%82%92%E4%BD%BF%E3%81%A3%E3%81%9F%E3%82%B9%E3%83%AC%E3%83%83%E3%83%89%E9%96%93%E3%81%A7%E3%81%AE%E5%8F%AF%E5%A4%89%E3%81%AA%E3%83%87%E3%83%BC%E3%82%BF%E5%85%B1%E6%9C%89)
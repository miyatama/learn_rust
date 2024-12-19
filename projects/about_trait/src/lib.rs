mod impls;
mod traits;

use impls::{HasConstImpl, SimpleTraitImpl};
use traits::{run_basics, HasConst, SimpleTrait};

pub fn run() {
    println!("# いろんなtraitの使い方");
    simple_trait();
    has_const();
    run_basics();
}

fn simple_trait() {
    println!("## シンプルなトレイト実装");
    let simple_trait_impl = SimpleTraitImpl {};
    simple_trait_impl.method();
    println!("");
}

fn has_const() {
    println!("## 定数の保持もできます。");
    println!("上書きされた定数: {}", HasConstImpl::NUMBER);
    println!("");
}

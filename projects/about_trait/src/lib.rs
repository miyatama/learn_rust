mod impls;
mod traits;

use impls::SimpleTraitImpl;
use traits::SimpleTrait;

pub fn run() {
    println!("いろんなtraitの使い方");
    simple_trait();
}

fn simple_trait() {
    println!("シンプルなトレイト実装");

    let simple_trait_impl = SimpleTraitImpl {};
    simple_trait_impl.method();
}

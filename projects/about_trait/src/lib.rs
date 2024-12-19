mod impls;
mod traits;

use impls::{call_param_trait, FunParamImpl, HasConstImpl, SimpleTraitImpl};
use traits::{run_basics, HasConst, SimpleTrait};

pub fn run() {
    println!("# いろんなtraitの使い方");
    simple_trait();
    has_const();
    run_fun_param();
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

fn book_basic() {
    println!("## 公式の使い方です。謎ですね。");
    run_basics();
    println!("");
}

fn run_fun_param() {
    println!("## パラメタの引数にトレイトも指定できます");
    let fun_param = FunParamImpl {};
    call_param_trait(&fun_param);
    println!("");
}

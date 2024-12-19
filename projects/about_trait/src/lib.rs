mod impls;
mod traits;

use impls::{
    call_param_trait, CallerTraitImpl, CallerTraitImpl2, FunParamImpl, HasConstImpl,
    LogicTraitImpl, LogicTraitImpl2, SimpleTraitImpl,
};
use traits::{run_basics, CallerTrait, HasConst, LogicTrait, SimpleTrait};

pub fn run() {
    println!("# いろんなtraitの使い方");
    simple_trait();
    has_const();
    run_fun_param();
    run_has_trait();
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

fn run_has_trait() {
    println!("## トレイトを構造体の中に保持する事もできます");
    let logic_trait = LogicTraitImpl::new();
    let caller_trait = CallerTraitImpl::new(&logic_trait);
    let value = caller_trait.run();
    println!("caller result: {}", value);

    println!("");
}

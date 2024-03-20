mod count_down;
mod primer;
use futures::executor::block_on;

fn main() {
    println!("start main");
    let future = do_something();
    block_on(future);
    primer::run();
    count_down::run();
    println!("finish main");
}

async fn do_something() {
    println!("Hello, world!");
}

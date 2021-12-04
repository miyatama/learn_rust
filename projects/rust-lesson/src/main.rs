mod vars;
mod stack_heap;
mod ownership;
mod generics;
mod lifetime;
mod structs;
mod enums;
mod traits;
mod error_handling;
mod unit_test;
extern crate lib_demo;

fn main() {
    // println!("Hello, world!");
    // vars::run();
    // vars::sub_a::func_a();
    // vars::sub_b::func_b();
    // hello_stack();
    // stack_heap::run();                
    // ownership::run();
    // generics::run();
    // lifetime::run();
    // structs::run();
    // enums::run();
    // traits::run();
    error_handling::run();
    lib_demo::print_random_number();
}

// fn hello_stack() {
//     let a = 10;
//     let b = 20;
//     println!("{}, {}", a, b);
// }
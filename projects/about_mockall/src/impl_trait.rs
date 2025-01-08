use mockall::predicate::*;
use mockall::*;
use std::fmt::Debug;

struct Foo {}

#[automock]
impl Foo {
    fn foo(&self) -> impl Debug {}
}

pub fn call_impl_trait() {
    let foo = Foo {};
    let debugger = foo.foo();
    println!("call_impl_trait: {:?}", debugger);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut mock = MockFoo::new();
        mock.expect_foo()
            .returning(|| Box::new(String::from("Hello, World!")));
        println!("{:?}", mock.foo());
    }
}

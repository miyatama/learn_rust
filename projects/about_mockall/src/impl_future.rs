use futures::executor;
use mockall::predicate::*;
use mockall::*;
use std::future::{Future, ready};

struct Foo {}
#[automock]
impl Foo {
    fn foo(&self) -> impl Future<Output = i32> {
        async {
            500
        }
    }
}

pub fn call_impl_future() {
    let foo = Foo {};
    let debugger = foo.foo();
    let value = executor::block_on(debugger);
    println!("call_impl_trait: {:?}", value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut mock = MockFoo::new();
        mock.expect_foo().returning(|| Box::pin(ready(42)));
        let debugger = mock.foo();
        let value = executor::block_on(debugger);
        assert_eq!(42, 42);
    }
}

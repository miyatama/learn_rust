use mockall::predicate::*;
use mockall::*;

#[automock]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

pub fn call_matching_multiple_values() {
    struct MyStruct {}
    impl MyTrait for MyStruct {
        fn foo(&self, x: u32) -> u32 {
            x + 100
        }
    }

    let mock = MyStruct {};
    let _value = func01(&mock);
}

fn func01(x: &dyn MyTrait) -> (u32, u32) {
    (x.foo(100), x.foo(101))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut mock = MockMyTrait::new();
        mock.expect_foo().with(eq(100u32)).return_const(200u32);
        mock.expect_foo().with(eq(101u32)).return_const(201u32);
        let expect = (200, 201);
        let actual = func01(&mock);
        assert_eq!(expect, actual);
    }
}

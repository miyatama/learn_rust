use mockall::*;
use mockall::predicate::*;

#[automock]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

pub fn call_call_counts() {
    struct MyStruct {}
    impl MyTrait for MyStruct {
        fn foo(&self, x: u32) -> u32 {
            x + 10
        }
    }
    let mock = MyStruct {};
    let _value = func01(&mock);
}

fn func01(mock: &dyn MyTrait) -> u32 {
    mock.foo(100)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test01() {
        let mut mock = MockMyTrait::new();
        mock.expect_foo()
            .with(eq(100u32))
            .times(1)
            .return_const(101u32);
        mock.expect_foo()
            .with(eq(101u32))
            .never()
            .return_const(102u32);

        let expect = 101u32;
        let actual = func01(&mock);
        assert_eq!(expect, actual);
    }
}

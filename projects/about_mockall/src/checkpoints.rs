use mockall::predicate::*;
use mockall::*;

#[automock]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

pub fn call_checkpoints() {
    struct MyStruct {}
    impl MyTrait for MyStruct {
        fn foo(&self, x: u32) -> u32 {
            x + 100
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
        mock.expect_foo().times(1).return_const(101u32);
        let actual = func01(&mock);
        let expect = 101u32;
        assert_eq!(expect, actual);
        mock.checkpoint();
        mock.expect_foo().times(2).return_const(102u32);
        let actual = func01(&mock);
        let expect = 102u32;
        assert_eq!(expect, actual);
        // panic here.call foo 1 time.
        // mock.checkpoint();
        let actual = func01(&mock);
        let expect = 102u32;
        assert_eq!(expect, actual);
        mock.checkpoint();
    }
}

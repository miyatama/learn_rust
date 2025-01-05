use mockall::predicate::*;
use mockall::*;

#[automock]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

pub fn call_sequence() {
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
        let mut seq = Sequence::new();
        let mut mock01 = MockMyTrait::new();
        mock01
            .expect_foo()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(100u32);

        let mut mock02 = MockMyTrait::new();
        mock02
            .expect_foo()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(200u32);

        let expect = 100u32;
        let actual = func01(&mock01);
        assert_eq!(expect, actual);
        let expect = 200u32;
        let actual = func01(&mock02);
        assert_eq!(expect, actual);

        // failure case
        /*
        let expect = 200u32;
        let actual = func01(&mock02);
        assert_eq!(expect, actual);
        let expect = 100u32;
        let actual = func01(&mock01);
        assert_eq!(expect, actual);
        */
    }
}

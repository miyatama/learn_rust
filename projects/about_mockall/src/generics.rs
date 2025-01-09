use mockall::predicate::*;
use mockall::*;

#[automock]
trait Foo {
    fn foo<T: 'static>(&self, t: T) -> i32;
}

pub fn call_generics() {
    struct FooStruct {}
    impl Foo for FooStruct {
        fn foo<T: 'static>(&self, _t: T) -> i32 {
            2i32
        }
    }
    let foo = FooStruct {};
    let _value = foo.foo(5i16);
    let _value = foo.foo(5i8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut mock = MockFoo::new();
        mock.expect_foo::<i16>().returning(|_| 5i32);
        let expect = 5i32;
        let actual = mock.foo(5i16);
        assert_eq!(expect, actual);

        mock.expect_foo::<i8>().returning(|_| -5i32);
        let expect = -5i32;
        let actual = mock.foo(5i8);
        assert_eq!(expect, actual);
    }
}

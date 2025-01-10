use mockall::predicate::*;
use mockall::*;

struct X<'a>(&'a i32);

#[automock]
trait Foo {
    fn foo<T: 'static>(&self, t: T) -> i32;
}

#[automock]
trait Bar {
    fn bar<'a>(&self, x: X<'a>) -> i32;
}

#[automock]
trait Huu<T> {
    fn huu(&self, t: T) -> i32;
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

    struct BarStruct {}
    impl Bar for BarStruct {
        fn bar<'a>(&self, x: X<'a>) -> i32 {
            *x.0
        }
    }
    let bar = BarStruct {};
    let x_value = 8i32;
    let x = X(&x_value);
    let _value = bar.bar(x);

    struct HuuStruct {}
    impl<T> Huu<T> for HuuStruct {
        fn huu(&self, _t: T) -> i32 {
            8i32
        }
    }
    let huu = HuuStruct {};
    let _value = huu.huu(8u32);
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

    #[test]
    fn test02() {
        let mut mock = MockBar::new();
        mock.expect_bar().withf(|f| *f.0 == 5).return_const(101);
        let x = X(&5);
        assert_eq!(101, mock.bar(x));
    }

    #[test]
    fn test03() {
        let mut mock = MockHuu::new();
        mock.expect_huu().returning(|_t| 101i32);
        assert_eq!(101, mock.huu(300));
    }
}

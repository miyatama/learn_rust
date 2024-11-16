use mockall::predicate::*;
use mockall::*;

// mockall samples
#[automock]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

fn call_with_four(x: &dyn MyTrait) -> u32 {
    x.foo(4)
}

fn do_something() {}

struct NonClone {
    id: u8,
}

#[automock]
pub trait Foo {}

#[automock]
pub trait UseFoo {
    fn foo(&self) -> NonClone;
}

/*
impl<T: Foo> UseFoo for T {
    fn foo(&self) -> NonClone {
        NonClone { id: 1 }
    }
}

#[automock]
pub trait ProvidesFoo {
    type Uses: UseFoo;
    fn get_foo(&self) -> &Self::Uses;
}
 */

pub fn calc_id(user: &dyn UseFoo) -> u8 {
    user.foo().id * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let mut mock = MockUseFoo::new();
        let r = NonClone { id: 100 };
        mock.expect_foo().return_once(move || r);
        let actual = calc_id(&mock);
        let expect = 200;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_call_with_four() {
        let mut mock = MockMyTrait::new();
        mock.expect_foo()
            .with(predicate::eq(4))
            .times(1)
            .returning(|x| x + 1);
        let expect = 5;
        let actual = call_with_four(&mock);
        assert_eq!(expect, actual);
    }
}

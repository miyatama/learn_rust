use mockall::*;
use mockall::predicate::*;

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
        let expect = calc_id(&mock);
        assert_eq!(200, expect);
    }
}

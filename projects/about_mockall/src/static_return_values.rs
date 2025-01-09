use mockall::predicate::*;
use mockall::*;
use std::rc::Rc;

struct NonClone {
    pub value: u32,
}

#[automock]
trait MyTrait {
    fn foo(&self) -> u32;
    fn bar(&self, x: u32, y: u32) -> u32;
    fn get_non_clone(&self) -> NonClone;
    fn rc_foo(&self, x: Rc<u32>) -> Rc<u32>;
}

pub fn call_static_return_values() {
    struct MyStruct {}
    impl MyTrait for MyStruct {
        fn foo(&self) -> u32 {
            5
        }
        fn bar(&self, x: u32, y: u32) -> u32 {
            x + y
        }
        fn get_non_clone(&self) -> NonClone {
            NonClone { value: 5 }
        }
        fn rc_foo(&self, x: Rc<u32>) -> Rc<u32> {
            Rc::new(1000 + *x)
        }
    }
    let trait_struct = MyStruct {};
    let _return_val = func01(&trait_struct);
    let _return_val2 = func02(&trait_struct);
    let _return_val3 = func03(&trait_struct);
}

fn func01(x: &dyn MyTrait) -> u32 {
    x.foo() + x.bar(10, 11)
}

fn func02(x: &dyn MyTrait) -> u32 {
    let val = x.get_non_clone();
    val.value
}

fn func03(x: &dyn MyTrait) -> u32 {
    let param = Rc::new(5);
    let value = x.rc_foo(param);
    *value
}

#[cfg(test)]
fn do_something() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut mock = MockMyTrait::new();
        mock.expect_foo().return_const(10u32);
        mock.expect_bar().returning(|x, y| x + y + 1);
        let actual = func01(&mock);
        let expect = 32u32;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test02() {
        let mut mock = MockMyTrait::new();
        let non_clone = NonClone { value: 100 };
        mock.expect_get_non_clone().return_once(move || non_clone);
        let actual = func02(&mock);
        let expect = 100;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test03() {
        let mut mock = MockMyTrait::new();
        let non_clone = NonClone { value: 100 };
        mock.expect_get_non_clone().return_once(move || {
            do_something();
            non_clone
        });
        let actual = func02(&mock);
        let expect = 100;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test04() {
        let mut mock = MockMyTrait::new();
        let argument = Rc::new(5);
        let r = Rc::new(101);
        /*
        Mock objects are always Send. If you need to use a return type that isn’t, you can use the return_const_st,
        returning_st, or return_once_st, methods.
         */
        mock.expect_rc_foo()
            // .returning(|| r);
            .withf_st(move |x| *x == argument)
            .returning_st(move |_| r.clone());
        let actual = func03(&mock);
        let expect = 101;
        assert_eq!(expect, actual);
    }
}

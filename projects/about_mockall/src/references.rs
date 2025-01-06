use mockall::predicate::*;
use mockall::*;
use std::fmt::Display;

#[automock]
trait MyTrait {
    fn foo(&self, x: &u32) -> u32;
}

struct Thing(u32);

#[automock]
trait Container {
    fn get(&self, i: u32) -> &'static Thing;
    fn get2(&mut self, i: u32) -> &Thing;
    fn get3(&mut self, i: u32) -> &mut Thing;
}

#[automock]
trait Foo {
    fn name(&self) -> &str;
    fn name2(&self) -> &dyn Display;
}

pub fn call_references() {
    struct MyStruct {}
    impl MyTrait for MyStruct {
        fn foo(&self, x: &u32) -> u32 {
            x + 1u32
        }
    }
    let mock = MyStruct {};
    let _value = func01(&mock);
    const THING: Thing = Thing(200);
    struct MyContainer {
        thing: Thing,
    }
    impl Container for MyContainer {
        fn get(&self, i: u32) -> &'static Thing {
            &THING
        }
        fn get2(&mut self, i: u32) -> &Thing {
            self.thing = Thing(i);
            &self.thing
        }
        fn get3(&mut self, i: u32) -> &mut Thing {
            &mut self.thing
        }
    }
    let mut mock = MyContainer { thing: Thing(10) };
    func02(&mock);
    func03(&mut mock);
}

fn func01(mock: &dyn MyTrait) -> u32 {
    let value = 100u32;
    mock.foo(&value)
}

fn func02(mock: &dyn Container) -> u32 {
    mock.get(500).0
}

fn func03(mock: &mut dyn Container) -> u32 {
    mock.get2(500).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut mock = MockMyTrait::new();
        mock.expect_foo()
            .withf(|x: &u32| *x == 100)
            .returning(|x: &u32| *x + 1);
        let expect = 101u32;
        let actual = func01(&mock);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test02() {
        let mut mock = MockContainer::new();
        const THING: Thing = Thing(100);
        mock.expect_get().return_const(&THING);
        let expect = 100;
        let actual = func02(&mock);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test03() {
        let mut mock = MockContainer::new();
        let thing: Thing = Thing(100);
        mock.expect_get2().return_const(thing);
        let expect = 100;
        let actual = func03(&mut mock);
        assert_eq!(expect, actual);
    }

    #[test]
    fn test04() {
        let mut mock = MockContainer::new();
        let thing: Thing = Thing(100);
        mock.expect_get3().return_var(thing);
        mock.get3(0).0 = 50;
        assert_eq!(50, mock.get3(0).0);
    }

    #[test]
    fn test05() {
        let mut mock = MockFoo::new();
        mock.expect_name().return_const("abcd".to_owned());
        assert_eq!("abcd", mock.name());
    }

    #[test]
    fn test06() {
        let mut mock = MockFoo::new();
        mock.expect_name2().return_const(Box::new("abcd"));
        assert_eq!("abcd", format!("{}", mock.name2()));
    }
}

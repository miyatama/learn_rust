use mockall::predicate::*;
use mockall::*;

#[automock]
trait MyTrait {
    fn foo(&self, x: &u32) -> u32;
}

struct Thing(u32);

#[automock]
trait Container {
    fn get(&self, i: u32) -> &'static Thing;
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
    struct MyContainer {}
    impl Container for MyContainer {
        fn get(&self, i: u32) -> &'static Thing {
            &THING
        }
    }
    let mut mock = MyContainer {};
    func02(&mock);
}

fn func01(mock: &dyn MyTrait) -> u32 {
    let value = 100u32;
    mock.foo(&value)
}

fn func02(mock: &dyn Container) -> u32 {
    mock.get(500).0
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
}

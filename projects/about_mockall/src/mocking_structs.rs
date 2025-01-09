use mockall_double::double;

mod thing {
    use mockall::*;

    pub struct Thing {}

    #[automock]
    impl Thing {
        pub fn foo(&self) -> u32 {
            101
        }
    }
}

#[double]
use thing::Thing;

pub fn call_mocking_structs() {
    #[cfg(not(test))]
    let thing = Thing {};
    #[cfg(test)]
    let thing = Thing::default();
    let _value = func01(&thing);
}

fn func01(thing: &Thing) -> u32 {
    thing.foo()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        let mut mock = Thing::default();
        mock.expect_foo().returning(|| 2);
        let expect = 2;
        let actual = func01(&mock);
        assert_eq!(expect, actual);
    }
}

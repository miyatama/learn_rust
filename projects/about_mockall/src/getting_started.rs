use mockall::*;
use mockall::predicate::*;

#[automock]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

pub fn call_getting_started_func() -> u32 {
    struct MyStruct { }
    impl MyTrait for MyStruct {
        fn foo(&self, x: u32) -> u32 {
            x * 2
        }
    }
    let trait_struct = MyStruct{};
    getting_started_func(&trait_struct)
}

fn getting_started_func(x: &dyn MyTrait) -> u32 {
    x.foo(4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getting_started_test() {
        let mut mock = MockMyTrait::new();
        mock
        .expect_foo()
        .with(predicate::eq(4))
        .times(1)
        .returning(|x| x + 1);
    assert_eq!(5, getting_started_func(&mock));
    }
}

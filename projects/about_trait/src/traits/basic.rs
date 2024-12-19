use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
trait Example {
    const CONST_NO_DEFAULT: i32;
    const CONST_WITH_DEFAULT: i32 = 99;
    fn method_without_default(&self) {}
    fn method_with_default(&self) {}
}

trait GenericExample<T> {
    fn len(&self) -> u32;
    fn elt_at(&self, n: u32) -> T;
    fn iter<F>(&self, f: F)
    where
        F: Fn(T);
}

trait TraitMethods {
    fn by_ref(self: &Self) {}
    fn by_ref_Mut(self: &mut Self) {}
    fn by_box(self: Box<Self>) {}
    fn by_rc(self: Rc<Self>) {}
    fn by_arc(self: Arc<Self>) {}
    fn by_pin(self: Pin<&Self>) {}
    fn with_lifetime<'a>(self: &'a Self) {}
    fn nested_pin(self: Pin<Arc<Self>>) {}
}

trait NonDispatchable {
    fn returns(&self) -> Self
    where
        Self: Sized;
    /*
    fn foo()
    where
        Self: Sized;
    fn param(&self, other: Self)
    where
        Self: Sized;
    fn typed<T>(&self, x: T)
    where
        Self: Sized;
     */
}

struct S;
impl NonDispatchable for S {
    fn returns(&self) -> Self
    where
        Self: Sized,
    {
        S
    }
}

pub fn run_basics() {
    let _obj: Box<dyn NonDispatchable> = Box::new(S);
    /*
    obj.returns();
    obj.param(S);
    obj.typed(1);
     */
}

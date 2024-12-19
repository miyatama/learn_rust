use crate::traits::SimpleTrait;

pub struct SimpleTraitImpl {}

impl SimpleTrait for SimpleTraitImpl {
    fn method(&self) {
        println!("SimpleTraitImpl::method");
    }
}

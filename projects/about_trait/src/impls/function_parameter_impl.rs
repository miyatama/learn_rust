use crate::traits::FunParam;

pub struct FunParamImpl {}

impl FunParam for FunParamImpl {
    const ID: u8 = 101u8;
}

pub fn call_param_trait(param: &impl FunParam) {
    // println!("call param trait.param.id is {}", *param.ID);
}

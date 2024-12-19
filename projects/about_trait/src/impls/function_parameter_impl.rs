use crate::traits::FunParam;

pub struct FunParamImpl {}

impl FunParam for FunParamImpl {
    const ID: u8 = 101u8;
    fn get_name(&self) -> String {
        "miyatama".to_string()
    }
}

pub fn call_param_trait(param: &impl FunParam) {
    println!("call param trait name is {}", param.get_name());
}

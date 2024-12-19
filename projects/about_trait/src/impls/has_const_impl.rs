use crate::traits::HasConst;

pub struct HasConstImpl {}

impl HasConst for HasConstImpl {
    const NUMBER: u8 = 100u8;
}

pub trait FunParam {
    const ID: u8 = 100u8;
    fn get_name(&self) -> String;
}

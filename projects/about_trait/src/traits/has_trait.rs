pub trait LogicTrait {
    fn run_logic(&self) -> u8;
}

pub trait CallerTrait {
    fn run(&self) -> u8;
}

pub trait LogicTrait2 {
    fn run_logic(&self) -> u8;
}

pub trait CallerTrait2<'a, L: LogicTrait2> {
    fn get_logic(&self) -> &L;
}

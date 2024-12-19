use crate::traits::{CallerTrait, CallerTrait2, LogicTrait, LogicTrait2};
use std::sync::Arc;

pub struct LogicTraitImpl {
    value: u8,
}

impl LogicTraitImpl {
    pub fn new() -> Self {
        Self { value: 101 }
    }
}

impl LogicTrait for LogicTraitImpl {
    fn run_logic(&self) -> u8 {
        self.value
    }
}

pub struct CallerTraitImpl<'a, L: LogicTrait> {
    main_logic: &'a L,
}

impl<'a, L: LogicTrait> CallerTraitImpl<'a, L> {
    pub fn new(logic: &'a L) -> Self {
        Self { main_logic: logic }
    }
}

impl<'a, L: LogicTrait> CallerTrait for CallerTraitImpl<'a, L> {
    fn run(&self) -> u8 {
        self.main_logic.run_logic()
    }
}

// newで参照を作るよ！
pub struct LogicTrait2Impl {
    value: u8,
}

impl LogicTrait2Impl {
    pub fn new() -> Self {
        Self { value: 201 }
    }
}

impl LogicTrait2 for LogicTrait2Impl {
    fn run_logic(&self) -> u8 {
        self.value
    }
}

pub struct CallerTrait2Impl<'a, L: LogicTrait2> {
    main_logic: Arc<&'a L>,
}

impl<'a, L: LogicTrait2> CallerTrait2Impl<'a, L> {
    pub fn new() -> Self {
        let logic_trait = LogicTrait2Impl::new();
        Self {
            main_logic: Arc::new(&logic_trait),
        }
    }
}

impl<'a, L: LogicTrait2> CallerTrait2<'a, L> for CallerTrait2Impl<'a, L> {
    fn get_logic(&self) -> &L {
        self.main_logic
    }
}

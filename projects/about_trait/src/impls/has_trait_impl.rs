use std::sync::Arc;
use crate::traits::{CallerTrait, CallerTrait2, LogicTrait, LogicTrait2};

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

pub struct CallerTrait2Impl {
    main_logic: Arc<dyn LogicTrait2>,
}

impl CallerTrait2Impl
{
    pub fn new() -> Self {
        let logic_trait = LogicTrait2Impl::new();
        Self {
            main_logic: Arc::new(logic_trait),
        }
    }
}

impl CallerTrait2 for CallerTrait2Impl {
    /*
    fn get_logic(&self) -> LogicTrait2 {
        self.main_logic
    }
     */
    fn get_logic(&self) -> u8{
        5u8
    }
}

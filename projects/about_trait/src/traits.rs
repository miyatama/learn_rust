mod basic;
mod function_parameter;
mod has_const;
mod has_trait;
mod simple_trait;

pub use basic::run_basics;
pub use function_parameter::FunParam;
pub use has_const::HasConst;
pub use has_trait::{CallerTrait, CallerTrait2, LogicTrait, LogicTrait2};
pub use simple_trait::SimpleTrait;

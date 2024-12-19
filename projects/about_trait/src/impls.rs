mod function_parameter_impl;
mod has_const_impl;
mod has_trait_impl;
mod simple_trait_impl;

pub use function_parameter_impl::{call_param_trait, call_param_trait2, FunParamImpl};
pub use has_const_impl::HasConstImpl;
pub use has_trait_impl::{CallerTraitImpl, CallerTrait2Impl, LogicTraitImpl};
pub use simple_trait_impl::SimpleTraitImpl;

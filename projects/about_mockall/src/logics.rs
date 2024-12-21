pub mod entity;
pub mod handlers;
pub mod repository;
pub mod repository_impl;

use mockall_double::double;

#[double]
pub use repository_impl::LimitInMemoryClientRepository;

// doubleの展開誤
/*
#[cfg(not(test))]
pub use repository_impl::LimitInMemoryClientRepository;
#[cfg(test)]
pub use repository_impl::MockLimitInMemoryClientRepository as LimitInMemoryClientRepository;
 */

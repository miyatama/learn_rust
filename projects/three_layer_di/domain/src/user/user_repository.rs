use crate::{MyResult, User};

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait UserRepository {
    fn list(&self) -> Vec<User>;
    fn create(&self, user: User) -> MyResult<()>;
    fn update(&self, user: User) -> MyResult<()>;
}

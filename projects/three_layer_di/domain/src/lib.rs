mod email_address;
mod error;
mod repositories;
mod user;

pub use email_address::EmailAddress;
pub use error::{error_type::MyErrorType, MyError, MyResult};
pub use repositories::Repositories;
pub use user::{
    user_id::UserId,
    user_name::{user_first_name::UserFirstName, user_last_name::UserLastName, UserName},
    user_repository::UserRepository,
    User,
};

#[cfg(feature = "mock")]
pub use user::user_repository::MockUserRepository;

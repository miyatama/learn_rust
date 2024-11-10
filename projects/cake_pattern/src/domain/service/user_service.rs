use crate::domain::entity::User;
use crate::domain::repository::{UsesUserRepository, ProvidesUserRepository};

pub trait UsesUserService: Send + Sync + 'static {
    fn find_user(&self, id: String) -> Result<Option<User>, Box<dyn std::error::Error>>;

    #[warn(dead_code)]
    fn deactivate_user(&self, id: String) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait UserService: ProvidesUserRepository {}

impl<T: UserService> UsesUserService for T {
    fn find_user(&self, id: String) -> Result<Option<User>, Box<dyn std::error::Error>> {
        self.user_repository().find_user(id)
    }

    #[warn(dead_code)]
    fn deactivate_user(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        let user = self.user_repository().find_user(id)?;
        if let Some(mut user) = user {
            user.effective = false;
            self.user_repository().update(user)?;
        };
        Ok(())
    }
}

pub trait ProvidesUserService: Send + Sync + 'static {
    type T: UsesUserService;
    fn user_service(&self) -> &Self::T;
}

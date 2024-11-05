pub trait UserService: ProvidesUserRepository {}

impl<T: UserService> UsesUserService for T {
    fn find_user(&self, id: String) -> Result<Option<User>> {
        self.user_repository().find_user(id)
    }

    fn deactivate_user(&self, id: String) -> Result<()> {
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
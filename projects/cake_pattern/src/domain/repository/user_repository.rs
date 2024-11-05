pub trait UsesUserRepository: Send + Sync + 'static {
    fn find_user(&self, id: String) -> Result<Option<User>>;
    fn update(&self, user: User) -> Result<()>;
}

pub trait UserRepository: ProvidesDatabase{}

impl <T: UserRepository>: UsesUserRepository for T {
    fn find_user(&self, id: String) -> Result<Option<User>> {
        self.database().find_user(id)
    }

    fn update(&self, user: User) -> Result<()> {
        self.database().update(user)
    }
}

pub trait ProvidesUserRepository: Send + Sync + 'static {
    type T: UsesUserRepository;
    fn user_repository(&self) -> &Self::T;
}
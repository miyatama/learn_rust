use crate::domain::data::ProvidesDatabase;
use crate::domain::entity::User;

pub trait UsesUserRepository: Send + Sync + 'static {
    fn find_user(&self, id: String) -> Result<Option<User>, Box<dyn std::error::Error>>;
    fn update(&self, user: User) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait UserRepository: ProvidesDatabase {}

impl<T: UserRepository> UsesUserRepository for T {
    fn find_user(&self, id: String) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let database = self.database();
        database.find(id)
    }

    fn update(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        self.database().update(user)
    }
}

pub trait ProvidesUserRepository: Send + Sync + 'static {
    type T: UsesUserRepository;
    fn user_repository(&self) -> &Self::T;
}

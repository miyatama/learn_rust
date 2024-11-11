use crate::domain::data::{ProvidesDatabase, UsesDatabase};
use crate::domain::entity::User;

pub trait UsesUserRepository: Send + Sync + 'static {
    fn find_user(&self, id: String) -> Result<Option<User>, Box<dyn std::error::Error>>;
    fn update_user(&self, user: User) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait UserRepository: ProvidesDatabase {}

impl<T: UserRepository> UsesUserRepository for T {
    fn find_user(&self, id: String) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let database = self.database();
        database.find(id)
    }

    fn update_user(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        self.database().update(user)
    }
}

pub trait ProvidesUserRepository: Send + Sync + 'static {
    type T: UsesUserRepository;
    fn user_repository(&self) -> &Self::T;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::data::Database;

    // TODO UsesDatabaseのモック挙動
    struct TestProvides {}
    impl Database for TestProvides {}
    impl ProvidesDatabase for TestProvides {
        type T = Self;
        fn database(&self) -> &Self::T {
            self
        }
    }
    impl UserRepository for TestProvides {}
    impl ProvidesUserRepository for TestProvides {
        type T = Self;
        fn user_repository(&self) -> &Self::T {
            self
        }
    }

    #[test]
    fn uses_user_repository_find_user() {
        let provides = TestProvides {};
        let result = provides
            .user_repository()
            .find_user("miyatama".to_string())
            .unwrap()
            .unwrap();
        let expect = User {
            id: "test".to_string(),
            effective: false,
        };
        assert_eq!(expect, result);
    }

    #[test]
    fn uses_user_repository_update() {
        let provides = TestProvides {};
        let user = User {
            id: "test".to_string(),
            effective: false,
        };
        let result = provides.user_repository().update_user(user);
        assert_eq!(result.is_ok(), true);
    }
}

use crate::domain::entity::User;

pub trait UsesDatabase: Send + Sync + 'static {
    fn find(&self, id: String) -> Result<Option<User>, Box<dyn std::error::Error>>;
    fn update(&self, user: User) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait Database: Send + Sync + 'static {}

impl<T: Database> UsesDatabase for T {
    fn find(&self, _id: String) -> Result<Option<User>, Box<dyn std::error::Error>> {
        Ok(Some(User {
            id: "id-a".to_string(),
            effective: true,
        }))
    }

    fn update(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        Ok(println!("updated user: {:?}", user))
    }
}

pub trait ProvidesDatabase: Send + Sync + 'static {
    type T: UsesDatabase;
    fn database(&self) -> &Self::T;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestProvides;
    impl Database for TestProvides {}
    impl ProvidesDatabase for TestProvides {
        type T = Self;
        fn database(&self) -> &Self::T {
            self
        }
    }
    #[test]
    fn test_uses_database_find() {
        let test_provides = TestProvides {};
        let result = test_provides
            .database()
            .find("test".to_string())
            .unwrap()
            .unwrap();
        let expect = User {
            id: "id-a".to_string(),
            effective: true,
        };
        assert_eq!(expect, result);
    }

    #[test]
    fn test_uses_database_update() {
        let test_provides = TestProvides {};
        let user = User {
            id: "test".to_string(),
            effective: false,
        };
        let result = test_provides.database().update(user);
        assert_eq!(result.is_ok(), true);
    }
}

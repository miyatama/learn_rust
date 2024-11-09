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

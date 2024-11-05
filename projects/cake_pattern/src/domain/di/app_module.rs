use crate::data::database::{Database, ProvidesDatabase};
use crate::repository::user_repository::{ProvidesUserRepository, UserRepository};
use crate::service::user_service::{ProvidesUserService, UserService};

pub struct AppModule;

impl AppModule {
    pub fn new() -> Self {
        Self
    }
}

impl Database for AppModule {}
impl UserRepository for AppModule {}
impl UserService for AppModule {}

impl ProvidesDatabase for AppModule {
    type T = Self;
    fn database(&self) -> &Self::T {
        self
    }
}

impl ProvidesUserRepository for AppModule {
    type T = Self;
    fn user_repository(&self) -> &Self::T {
        self
    }
}

impl ProvidesUserService for AppModule {
    type T = Self;
    fn user_service(&self) -> &Self::T {
        self
    }
}

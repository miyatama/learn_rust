use crate::domain::data::{Database, ProvidesDatabase};
use crate::domain::repository::{ProvidesUserRepository, UserRepository};
use crate::domain::service::UserService;
use std::clone::Clone;

#[derive(Clone)]
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

// TODO 何故かmainで実装しないとビルドが通らない
/*
impl ProvidesUserService for AppModule {
    type T = Self;
    fn user_service(&self) -> &Self::T {
        self
    }
}
 */

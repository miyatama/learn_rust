use crate::UserRepository;

pub trait Repositories {
    type UserRepo: UserRepository;
    fn user_repository(&self) -> &Self::UserRepo;
}

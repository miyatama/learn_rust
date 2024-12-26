/*
use crate::entity::Client;
use crate::repository::ClientRepository;
use crate::LimitInMemoryClientRepository;
use std::rc::Rc;
use uuid::Uuid;
*/
use crate::logics::entity::Client;
use crate::logics::repository::ClientRepository;
// use crate::logics::repository_impl::InMemoryClientRepository;
use crate::logics::repository_impl::LimitInMemoryClientRepository;
use std::rc::Rc;
use uuid::Uuid;

pub struct GetClientHandler<T: ClientRepository> {
    client_repo: Rc<T>,
}

impl<T: ClientRepository> GetClientHandler<T> {
    pub fn new(client_repo: Rc<T>) -> Self {
        Self { client_repo }
    }

    pub fn execute(&self, id: Uuid) -> Result<Client, String> {
        let client = self.client_repo.by_id(id).unwrap();
        Ok(client)
    }
}

/*
// ↓こういうのがやりたかったけどむりっぽい
pub struct GetClientHandler2<T: ClientRepository> {
    client_repo: Rc<T>,
}

impl<T: ClientRepository> GetClientHandler2<T> {
    pub fn new() -> Self {
        let repository = InMemoryClientRepository::new();
        Self { client_repo: Rc::new(repository as T) }
    }

    pub fn get_repository(&self) -> Box<ClientRepository> {
        Box::new(self.client_repo)
    }
}
 */

pub struct LimitGetClientHandlerV1 {
    client_repo: Rc<LimitInMemoryClientRepository>,
}

impl LimitGetClientHandlerV1 {
    pub fn new(client_repo: Rc<LimitInMemoryClientRepository>) -> Self {
        Self { client_repo }
    }

    pub fn execute(&self, id: Uuid) -> Result<Client, String> {
        let client = self.client_repo.by_id(id).unwrap();
        Ok(client)
    }
}

pub struct LimitGetClientHandlerV2 {
    client_repo: Rc<LimitInMemoryClientRepository>,
}

impl LimitGetClientHandlerV2 {
    pub fn new() -> Self {
        let client_repo = Rc::new(LimitInMemoryClientRepository::new(10_usize));
        Self { client_repo }
    }

    pub fn execute(&self, id: Uuid) -> Result<Client, String> {
        let client = self.client_repo.by_id(id)?;
        Ok(client)
    }
}

#[cfg(test)]
impl LimitGetClientHandlerV2 {
    #[allow(dead_code)]
    fn set_client_repo(&mut self, new_client_repo: Rc<LimitInMemoryClientRepository>) {
        self.client_repo = new_client_repo;
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::logics::entity::Client;
    use crate::logics::repository::MockClientRepository;
    use fake::{uuid::UUIDv4, Fake, Faker};
    use mockall::predicate;
    use std::rc::Rc;

    #[test]
    fn get_client_handler() {
        // let client = Faker.fake::<Client>();
        let client = Client::new(UUIDv4.fake());
        let id = client.id();

        let mut mock_repo = MockClientRepository::new();
        mock_repo
            .expect_by_id()
            .with(preidcate::eq(id))
            .times(1)
            .return_const(Ok(client.clone()));

        let handler = GetClientHandler::new(Rc::new(mock_repo));
        let client2 = handler.execute().unwrap();

        assert_eq!(client, client2);
    }

    #[test]
    fn limit_get_client_handler_v1() {
        // let client = Faker.fake::<Client>();
        let client = Client::new(UUIDv4.fake());
        let id = client.id();

        // doubleを使っているのでMockLimitInMemoryClientRepositoryを使う必要がない
        // let mut mock_repo = LimitInMemoryClientRepository::new();
        let mut mock_repo = MockLimitInMemoryClientRepository::new();
        mock_repo
            .expect_by_id()
            .with(preidcate::eq(id))
            .times(1)
            .return_const(Ok(client.clone()));

        let handler = LimitGetClientHandlerV1::new(Rc::new(mock_repo));
        let client2 = handler.execute().unwrap();

        assert_eq!(client, client2);
    }

    #[test]
    fn limit_get_client_handler_v2() {
        // let client = Faker.fake::<Client>();
        let client = Client::new(UUIDv4.fake());
        let id = client.id();

        // doubleを使っているのでMockLimitInMemoryClientRepositoryを使う必要がない
        let mut mock_repo = LimitInMemoryClientRepository::new();
        mock_repo
            .expect_by_id()
            .with(preidcate::eq(id))
            .times(1)
            .return_const(Ok(client.clone()));

        let handler = LimitGetClientHandlerV2::new();
        handler.set_client_repo(Rc::new(mock_repo));
        let client2 = handler.execute().unwrap();

        assert_eq!(client, client2);
    }
}

 */

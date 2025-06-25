use crate::{config::cqrs_framework, domain::aggregate::BankAccount, queries::BankAccountView};
use postgres_es::{default_postgress_pool, PostgresCqrs, PostgresViewRepository};
use std::sync::Arc;

#[derive(Clone)]
pub struct ApplicationState {
    pub cqrs: Arc<PostgresCqrs<ImageEdit>>,
    pub image_edit_query: Arc<PostgresViewRepository<ImageEditView, ImageEdit>>,
}

pub async fn new_application_state() -> ApplicationState {
    let pool = default_postgress_pool("postgres:://user:postgres@localhost:5432/demo").await;
    let (cqrs, image_edit_query) = cqrs_framework(pool);
    ApplicationState {
        cqrs: cqrs,
        image_edit_query: image_edit_query,
    }
}

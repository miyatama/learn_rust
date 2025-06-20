use crate::{config::cqrs_framework, domain::aggregate::BankAccount, queries::BankAccountView};
use postgres_es::{default_postgress_pool, PostgresCqrs, PostgresViewRepository};
use std::sync::Arc;

#[derive(Clone)]
pub struct ApplicationState {
    pub cqrs: Arc<PostgresCqrs<BankAccount>>,
    pub account_query: Arc<PostgresViewRepository<BankAccountView, BankAccount>>,
}

pub async fn new_application_state() -> ApplicationState {
    let pool = default_postgress_pool("postgres:://demo_user:demo_pass@localhost:5432/demo").await;
    let (cqrs, account_query) = cqrs_framework(pool);
    ApplicationState {
        cqrs: cqrs,
        account_query: account_query,
    }
}

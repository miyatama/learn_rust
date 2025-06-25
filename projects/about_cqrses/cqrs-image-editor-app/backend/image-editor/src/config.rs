use std::sync::Arc;

use cqrs_es::Query;
use postgres_es::{PostgresCqrs, PostgresViewRepository};
use sqlx::{Pool, Postgres};

use crate::domain::aggregate::ImageEdit;
use crate::queries::{ImageQuery, ImageEditView, SimpleLoggingQuery};
use crate::services::{ImageEditServices, HappyPathImageEditServices};

pub fn cqrs_framework(
    pool: Pool<Postgres>,
) -> (
    Arc<PostgresCqrs<ImageEdit>>,
    Arc<PostgresViewRepository<ImageEditView, ImageEdit>>,
) {
    let simple_query = SimpleLoggingQuery {};
    let image_edit_view_repo = Arc::new(PostgresViewRepository::new("image_edit_query", pool.clone()));
    let mut image_edit_query = ImageQuery::new(image_edit_view_repo.clone());

    image_edit_query.use_error_handler(Box::new(|e| println!("{e}")));

    // Create and return an event-sourced `CqrsFramework`.
    let queries: Vec<Box<dyn Query<ImageEdit>>> =
        vec![Box::new(simple_query), Box::new(image_edit_query)];
    let services = ImageEditServices::new(Box::new(HappyPathImageEditServices));
    (
        Arc::new(postgres_es::postgres_cqrs(pool, queries, services)),
        image_edit_view_repo,
    )
}

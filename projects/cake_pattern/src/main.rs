mod domain;
use self::domain::di::app_module::AppModule;
use actix_web::HttpResponse;
use crate::domain::service::{UsesUserService, ProvidesUserService};

impl ProvidesUserService for AppModule {
    type T = Self;
    fn user_service(&self) -> &Self::T {
        self
    }
}

fn main() {
    // Cake Pattern
    let app = AppModule {};
    let user = app.user_service().find_user("100".to_string());
    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    };
}

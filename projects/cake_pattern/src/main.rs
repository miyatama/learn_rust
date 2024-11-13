mod domain;
use self::domain::di::AppModule;
use self::domain::three_layer::ProvidesSvcB;
use self::domain::two_layer::ServiceImpl;
use crate::domain::service::{ProvidesUserService, UsesUserService};
use actix_web::{get, web, App, HttpResponse, HttpServer};

impl ProvidesUserService for AppModule {
    type T = Self;
    fn user_service(&self) -> &Self::T {
        self
    }
}

#[get("/user")]
async fn get_user(app_module: web::Data<AppModule>) -> HttpResponse {
    let user = app_module.user_service().find_user("100".to_string());
    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    let app = AppModule {};
    HttpServer::new(move || App::new().service(get_user).data(app.clone()))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}

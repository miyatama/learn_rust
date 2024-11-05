mod domain;
use self::domain::di::app_module::AppModule;
use actix_web::HttpResponse;

fn main() {
    // Cake Pattern
    let app = AppModule {};
    let user = app.user_service().find_user(100);
    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    };
}

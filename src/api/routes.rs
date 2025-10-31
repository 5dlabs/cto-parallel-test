use actix_web::{web, HttpResponse};

/// Configure all API routes
///
/// Note: This module depends on `crate::schema` being available (Task 1 dependency).
/// Future tasks will integrate database operations using these schema definitions.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Compile-time assertion that schema module exists (Task 1 dependency)
    let _ = core::any::type_name::<crate::schema::users::table>();
    cfg.service(
        web::scope("/api")
            .service(health_check)
            .service(web::scope("/users").configure(user_routes))
            .service(web::scope("/products").configure(product_routes)),
    );
}

/// Health check endpoint
#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

/// User routes configuration (placeholder for Task 3)
fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(HttpResponse::NotImplemented)));
}

/// Product routes configuration (placeholder for Task 4)
fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(HttpResponse::NotImplemented)));
}

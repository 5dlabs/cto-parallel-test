use actix_web::{web, HttpResponse};

/// Configure all API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
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

/// Placeholder for user routes - will be implemented by Task 3
fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("").route(web::get().to(|| async { HttpResponse::NotImplemented().finish() })),
    );
}

/// Placeholder for product routes - will be implemented by Task 4
fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("").route(web::get().to(|| async { HttpResponse::NotImplemented().finish() })),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new().configure(configure_routes)).await;
        let req = test::TestRequest::get().uri("/api/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_user_routes_not_implemented() {
        let app = test::init_service(App::new().configure(configure_routes)).await;
        let req = test::TestRequest::get().uri("/api/users").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 501);
    }

    #[actix_web::test]
    async fn test_product_routes_not_implemented() {
        let app = test::init_service(App::new().configure(configure_routes)).await;
        let req = test::TestRequest::get().uri("/api/products").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 501);
    }
}

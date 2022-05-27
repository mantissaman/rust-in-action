use actix_web::{HttpServer, App,  web,HttpResponse};


async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("System is healthy")
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
        .route("/health_check", web::get().to(health_check))
    }

    )
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
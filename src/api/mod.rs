//! The API server of Shadow
use actix_web::{middleware, web, App, HttpServer};

mod eth;

/// Run HTTP Server
pub async fn serve(port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/eth/receipt/{tx}").to(eth::receipt))
            .service(web::resource("/eth/proposal").to(eth::proposal))
    })
    .disable_signals()
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
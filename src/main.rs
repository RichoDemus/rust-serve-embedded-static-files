use crate::serve_assets::static_fie;
use actix_web::http::header;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod serve_assets;

#[get("/api/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "application/json"))
        .body(r#"{"msg":"hello"}"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/{filename:.*}", web::get().to(static_fie))
    })
    .bind(("0.0.0.0", 8080))?
    .run();
    println!("Started");
    server.await
}

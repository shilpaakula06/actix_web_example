use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello, Actix Web!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


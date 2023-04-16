use actix_web::{web, App, FromRequest, HttpServer, Responder, HttpResponse};

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn now() -> impl Responder {
    HttpResponse::Ok().body("The time now is: ")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {      
        App::new()
            .route("/hello", web::get().to(hello))
            .route("now", web::get().to(now))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

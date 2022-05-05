use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use subcrate::HOGE;
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(format!("Hello world, {}!", HOGE))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hoi there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let meaningless_error: Result<(), ()> = Result::Err(());
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hoi", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

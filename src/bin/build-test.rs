use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use num_bigint::BigUint;
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
    let _meaningless_error: Result<(), ()> = Result::Err(());
    let _meaningless_bignum: BigUint = BigUint::new(vec![0]);
    let _meaningless_usize: usize = 13;
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hoi", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

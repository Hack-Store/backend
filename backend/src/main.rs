use actix_web::{App, HttpServer, Responder, HttpResponse, post};

#[post("/api/v1/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(echo)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


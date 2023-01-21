use actix_web::{ web, App, HttpRequest, HttpResponse, HttpServer, Responder };

async fn ping() -> impl Responder {
  HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .route("/ping", web::get().to(ping))
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await
}
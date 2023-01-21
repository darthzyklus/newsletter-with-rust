use actix_web::{ web, App, HttpResponse, HttpServer };

async fn ping() -> HttpResponse {
  HttpResponse::Ok().finish()
}

pub async fn run() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .route("/ping", web::get().to(ping))
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await
}
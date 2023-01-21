use std::net::TcpListener;

use actix_web::{ web, App, HttpResponse, HttpServer };
use actix_web::dev::Server;

async fn ping() -> HttpResponse {
  HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
  let server = HttpServer::new(|| {
    App::new()
      .route("/ping", web::get().to(ping))
  })
  .listen(listener)?
  .run();

  Ok(server)
}
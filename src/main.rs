use actix_files;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	println!("Starting webserver");
    HttpServer::new(|| {
        App::new()
          .service(hello)
          .service(actix_files::Files::new("/", "./staticweb/")
          	.redirect_to_slash_directory()
          	.index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

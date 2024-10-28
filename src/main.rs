use actix_files as fs;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Serve all files in the `static/` directory, and set the index file for root
            .service(fs::Files::new("/", "./static").index_file("index.html").disable_listing())
    })
    .bind("0.0.0.0:8080")?  // Bind the server to port 8080
    .run()
    .await
}
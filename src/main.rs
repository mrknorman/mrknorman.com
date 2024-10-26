use actix_files as fs;  // For serving static files
use actix_web::{web, App, HttpServer, HttpResponse, Responder};

// Entry point for the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Serve static files from the `static/` directory (optional)
            .service(fs::Files::new("/static", "./static").show_files_listing())
            // Route for the root page ("/")
            .route("/", web::get().to(home_page))
            // Route for the about page ("/about")
            .route("/about/", web::get().to(about_page))
            // Route for the contact page
            .route("/contact/", web::get().to(contact_page))
    })
    .bind("0.0.0.0:8080")?  // Bind the server to port 8080
    .run()
    .await
}

// Function to serve the home page
async fn home_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../static/index.html"))  // Serve index.html from the root
}

// Function to serve the about page
async fn about_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../static/about/index.html"))  // Serve index.html from /about/
}

// Function to serve the about page
async fn contact_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../static/contact/index.html"))  // Serve index.html from /about/
}
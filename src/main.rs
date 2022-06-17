use actix_web::{HttpServer, App, web, HttpResponse, Responder};
use actix_web::web::Data;
use tera::{Tera, Context};
use serde::Serialize;

#[derive(Serialize)]
struct Template {
    name: String,
    link: String,
    author: String,
    categorie: String,
    description: String,
    image: String,
    date: String,
    tags: String,
}

fn make_data() {
    
}
async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Marketplace");

    let templates = make_data();

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .app_data(Data::new(tera))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

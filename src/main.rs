use actix_web::{HttpServer, App, web, HttpResponse, Responder};
use actix_web::web::Data;
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};
use std::fs;
use std::env;
use serde_json::value::Value;
use serde_json::Map;

#[derive(Serialize, Deserialize)]
struct Template {
    name: String,
    url: String,
    authors: Vec<Map<String, Value>>,
    categories: Vec<String>,
    description: String,
    tags: Vec<String>,
    license: String,
    version: String,
}

fn make_data() -> std::vec::Vec<Template> {
    println!("{}", env::current_dir().unwrap().display());
    let paths = fs::read_dir("data").unwrap();
    println!("{:#?}", paths);
    let folder_content = paths.map(|path| path.unwrap().path());
    let mut templates = Vec::new();
    for path in folder_content {
        println!("{}", path.display());
        let p = path.to_str().unwrap().to_string();

        let json_data = {
            // Load the first file into a string.
            let text = std::fs::read_to_string(p).unwrap();

            // Parse the string into a dynamically-typed JSON structure.
            serde_json::from_str::<Value>(&text).unwrap()
        };

        let template = Template {
            name: json_data["name"].as_str().unwrap().to_string(),
            url: json_data["url"].as_str().unwrap().to_string(),
            authors: json_data["authors"].as_array().unwrap().to_vec().iter().map(|x| x.as_object().unwrap().clone()).collect(),
            categories: json_data["categories"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect(),
            description: json_data["description"].as_str().unwrap().to_string(),
            tags: json_data["tags"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect(),
            license: json_data["license"].as_str().unwrap().to_string(),
            version: json_data["version"].as_str().unwrap().to_string(),
        };

        templates.push(template);


    }
    templates
}

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Marketplace");

    let templates = make_data();

    data.insert("templates", &templates);

    

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

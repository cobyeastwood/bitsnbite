// @date: Updated on Nov 6, 2020
// @name: main.rs
// @desc: server

mod bits;

use mime;
use actix_session::Session;
use actix_files;
use actix_web::http::StatusCode;
use actix_web::{guard, get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use json::JsonValue;

const DIR: &str = "client/build/index.html";

async fn index_rust() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; text/css; charset=utf-8")
        .body(format!("Rust App")))
}

async fn index_react() -> Result<actix_files::NamedFile> {
    let text_css: mime::Mime = "text/css".parse().unwrap();
    Ok(actix_files::NamedFile::set_content_type(actix_files::NamedFile::open("client/build/index.html")?, text_css))
}

async fn index(item: web::Json<bits::Food>) -> HttpResponse {
    bits::Group::Fruit(item.0.clone());
    println!("{:?}", &item);
    HttpResponse::Ok().json(item.0)
}

// Derive 'get' handler
#[get("/home")]
async fn home(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("Req: {:?}", req);

    let mut counter = 1;

    match session.get::<i32>("counter")? {
        Some(count) => {
            println!("SESSION value: {}", count);
            counter = count + 1;
        }
        None => println!("SESSION value: None"),
    }

    session.set("counter", counter)?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../statics/home.html")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(1024))
            .service(web::resource("/rust").route(web::get().to(index_rust)))
            .service(actix_files::Files::new("/static", "client/build/static"))
            .service(web::resource("/api/v1/index").route(web::post().to(index)))
            .default_service(
                web::resource("client/build/index.html")
                    .route(web::get().to(index_react))
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(|| HttpResponse::MethodNotAllowed()),
                    )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Json Documentation to Reference -> https://github.com/actix/examples/blob/master/json/src/main.rs

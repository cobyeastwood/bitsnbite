use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::{get, middleware, App, HttpRequest, HttpResponse, HttpServer, Result};

// Derive 'get' handler
#[get("/home")]
async fn home(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

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
        .body(include_str!("../static/home.html")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().wrap(middleware::Logger::default()).service(home))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Hello World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let host = std::env::var("HOST").unwrap_or(String::from("0.0.0.0"));
    let port = std::env::var("PORT").unwrap_or(String::from("8080"))
        .parse::<u16>().expect("PORT must be a number");

    HttpServer::new(|| 
        App::new()
            .service(index)
    )
        .bind((host, port))?
        .run()
        .await
}

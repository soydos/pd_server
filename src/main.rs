use std::env;
use log::{info};
use env_logger;
use actix_web::{
    guard, web, middleware, App, Responder, HttpServer,
    HttpResponse,
    Result
};
use actix_web::http::{StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct GenericResponse {
    message: String,
}

fn index() -> impl Responder {
    "yo!"
}

fn p404() -> Result<HttpResponse> {

    let body = GenericResponse{
        message: "not found".to_string()
    };

    let json_body = serde_json::to_string(&body).unwrap();
    let response = HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("application/json; charset=utf-8")
        .body(json_body);

    Ok(response)
}

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,pd_server=debug");
    env_logger::init();

    info!("starting up");

    HttpServer::new(|| App::new()
        .wrap(middleware::Logger::default())
        .service(
            web::resource("/").to(index))
        .default_service(
            // 404 for GET request
            web::resource("")
                .route(web::get().to(p404))
                // all requests that are not `GET`
                .route(
                    web::route()
                        .guard(guard::Not(guard::Get()))
                        .to(|| HttpResponse::MethodNotAllowed()),
                ),
        ))
        .bind("0.0.0.0:8080")?
        .run()
}

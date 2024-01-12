/*************************************************************************
    "task_server"
    main.rs
    10/17/23
    © Matthew Bentivegna
*************************************************************************/
mod api;
mod models;
mod repository;
mod date;
mod timer;
mod file;
mod constants;

use std::sync::Arc;
use serde::Serialize;
use actix_web::{
    get,
    web,
    App,
    HttpResponse,
    HttpServer,
    Responder,
    Result
};
use crate::constants::constants as program_constants;
use crate::file::config::config_load;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Task-Server is running".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let options = Arc::new(config_load(program_constants::CONFIG_FILENAME));
    let options_local = Arc::clone(&options);
    let database = web::Data::new(repository::database::Database::new());

    crate::timer::timer::run(database.clone(), options);
    
    HttpServer::new(move ||
        App::new()
            .app_data(database.clone())
            .configure(api::api::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    )
        .bind((options_local.server_ip().as_str(), options_local.server_port()))?
        .run()
        .await
}
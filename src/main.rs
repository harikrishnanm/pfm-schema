#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, middleware, error, http::StatusCode};
use actix_web::middleware::Logger;
use env_logger::Env;
use log::{error, info};
use actix_web_validator::JsonConfig;
use actix_web_validator::error::Error::Validate;
use actix_http::ResponseBuilder;
use dotenv::dotenv;


mod routes;
mod config;
mod schema;

pub struct RequestContext {
    db_pool: config::PgPool,
}

fn handle_err(err: actix_web_validator::error::Error) -> actix_http::error::Error{
    match err {
        Validate(validation_err) => {
            let rs = ResponseBuilder::new(StatusCode::BAD_REQUEST).json(validation_err.clone());
            error::InternalError::from_response(validation_err, rs).into()
        },
        err => {
            error!("Error processing json {}", &err);
            error::InternalError::new(err, StatusCode::BAD_REQUEST).into()
        }
    }
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::from_env(Env::default().default_filter_or("trace")).init();

    info!("########   Starting Platform Router (main)  #########");

    let addr = config::get_server_address();
    let workers = config::get_worker_count();

    let pool = config::get_db().await.expect("Could not get connection pool");

    HttpServer::new(move || {
        App::new()
            .data(
                RequestContext {
                    db_pool: pool.clone(),
                }
            )
            .app_data(
                JsonConfig::default().error_handler(|err, _req| {
                    handle_err(err)
                }
            ))
            .wrap(middleware::Compress::default())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(routes::register)
    }).workers(workers)
        .bind(addr)?
        .run()
        .await
}

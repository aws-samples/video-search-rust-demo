use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;
use lambda_web::{is_running_on_lambda, LambdaError, run_actix_on_lambda};

pub mod handlers;
pub mod store;
pub mod askama_filters;

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let factory = move || {
        App::new()
            .wrap(Logger::default())
            .service(handlers::index::handler)
            .service(handlers::video_detail::handler)
            .service(handlers::api_video_search::handler)
            .service(handlers::api_request_subtitle::handler)
    };

    if is_running_on_lambda() {
        run_actix_on_lambda(factory).await?;
    } else {
        HttpServer::new(factory)
            .bind("127.0.0.1:3000")?
            .run()
            .await?;
    }

    Ok(())
}
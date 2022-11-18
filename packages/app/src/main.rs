/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: MIT-0
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this
 * software and associated documentation files (the "Software"), to deal in the Software
 * without restriction, including without limitation the rights to use, copy, modify,
 * merge, publish, distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
 * INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
 * PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
 * HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
 * OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */
 
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
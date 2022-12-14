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
 
use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use actix_web::{HttpResponse, web};
use actix_web::get;
use actix_web::http::header::{self};
use askama::Template;
use serde::{Deserialize};
use crate::store::{get_video, VideoItem};

#[derive(Template)]
#[template(path = "video-detail.html")]
#[allow(dead_code)]
struct VideoDetailTemplate {
    video: VideoItem,
    content_host: String,
    timing: f32,
}

impl VideoDetailTemplate {
    pub fn new(video: VideoItem, timing: Option<f32>) -> Self {
        VideoDetailTemplate {
            video,
            content_host: dotenv::var("CONTENT_HOST").expect("CONTENT_HOST must be set"),
            timing: timing.unwrap_or(0.01)
        }
    }
}

#[derive(Deserialize)]
pub struct VideoDetailQuery {
    #[serde(rename = "t")]
    timing: Option<f32>
}

#[get("/video/{id}")]
pub async fn handler(id: web::Path<String>, query: web::Query<VideoDetailQuery>) -> actix_web::Result<HttpResponse> {

    let id = id.into_inner();

    let item = get_video(&id).await
        .map_err(|e| ErrorNotFound(e))?;

    let html = VideoDetailTemplate::new(item, query.timing)
        .render()
        .map_err(|e| ErrorInternalServerError(e))?;

    let response = HttpResponse::Ok()
        .content_type(header::ContentType::html())
        .body(html);

    Ok(response)
}

mod filters {
    pub use crate::askama_filters::*;
}
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
 
use actix_web::error::ErrorInternalServerError;
use actix_web::http::header;
use actix_web::HttpResponse;
use askama::Template;
use actix_web::get;
use crate::store::{scan_videos, VideoItem};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    videos: Vec<VideoItem>
}

impl IndexTemplate{
    pub fn new(videos: Vec<VideoItem>) -> Self {
        IndexTemplate{
            videos
        }
    }
}

#[get("/")]
pub async fn handler() -> actix_web::Result<HttpResponse> {
    let videos = scan_videos().await
        .map_err(|e| ErrorInternalServerError(e))?;

    let html = IndexTemplate::new(videos)
        .render()
        .unwrap();

    let response = HttpResponse::Ok()
        .content_type(header::ContentType::html())
        .body(html);

    Ok(response)
}

mod filters {
    pub use crate::askama_filters::content_url_opt;
}
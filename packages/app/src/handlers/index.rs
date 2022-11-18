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
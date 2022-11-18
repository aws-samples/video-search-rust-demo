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

use actix_web::{HttpResponse, web};
use actix_web::post;
use serde::{Deserialize, Serialize};
use crate::store::get_video;

#[derive(Deserialize)]
pub struct RequestSubtitleRequest {
    video_id: String,
    target_lang: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct SubtitleQueueMessage {
    pub video_id: String,
    pub content_language: String,
    pub translate_language: Option<String>
}

impl SubtitleQueueMessage {
    pub fn new(video_id: &str, content_language: &str, translate_language: Option<&str>) -> Self {
        SubtitleQueueMessage {
            video_id: video_id.to_owned(),
            content_language: content_language.to_owned(),
            translate_language: translate_language.map(|s| s.to_owned())
        }
    }
}

#[post("/api/video/subtitle")]
pub async fn handler(req: web::Json<RequestSubtitleRequest>) -> actix_web::Result<HttpResponse> {

    let queue_url = dotenv::var("SUBTITLE_QUEUE_URL")
        .expect("SUBTITLE_QUEUE_URL must be set.");

    let shared_config = aws_config::from_env().load().await;

    if let Ok(video) = get_video(&req.video_id).await {
        let content_language = &video.lang.split("-").collect::<Vec<_>>()[0];
        let msg = SubtitleQueueMessage::new(
            &req.video_id,
            *content_language,
            req.target_lang.as_ref().map(|l| l.as_str()));

        let sqs = aws_sdk_sqs::Client::new(&shared_config);
        sqs.send_message()
            .queue_url(&queue_url)
            .message_body(serde_json::to_string(&msg).unwrap())
            .send()
            .await
            .unwrap();

        Ok(HttpResponse::Created().finish())
    } else {
        Ok(HttpResponse::BadRequest().finish())
    }
}
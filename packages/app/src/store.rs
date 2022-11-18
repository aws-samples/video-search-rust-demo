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
 
use aws_sdk_dynamodb::model::AttributeValue;
use serde_dynamo::{from_item, from_items};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct VideoItem {
    pub id: String,
    pub title: String,
    pub lang: String,
    pub subtitles: Vec<String>,
    pub video_key: String,
    pub thumbnail_key: Option<String>,
}

pub async fn scan_videos() -> Result<Vec<VideoItem>, anyhow::Error> {
    let table_name = dotenv::var("DYNAMODB_TABLE_NAME")
        .expect("DYNAMODB_TABLE_NAME must be set.");

    let shared_config = aws_config::from_env().load().await;
    let dynamodb = aws_sdk_dynamodb::Client::new(&shared_config);

    let output = dynamodb.scan()
        .table_name(table_name)
        .projection_expression("id, title, thumbnail_key, subtitles, lang, video_key")
        .send()
        .await?;

    let items: Vec<VideoItem> = from_items(output.items.unwrap())?;

    Ok(items)
}

pub async fn get_video(id: &str) -> Result<VideoItem, anyhow::Error> {
    let table_name = dotenv::var("DYNAMODB_TABLE_NAME")
        .expect("DYNAMODB_TABLE_NAME must be set.");

    let shared_config = aws_config::from_env().load().await;
    let dynamodb = aws_sdk_dynamodb::Client::new(&shared_config);

    let output = dynamodb.get_item()
        .table_name(table_name)
        .key("id", AttributeValue::S(id.to_owned()))
        .send()
        .await?;

    let item = from_item(output.item.unwrap())?;

    Ok(item)
}
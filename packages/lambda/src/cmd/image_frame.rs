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
 
use std::path::Path;
use aws_sdk_s3::types::ByteStream;
use lambda_runtime::{Error, service_fn, LambdaEvent};
use tokio::fs::File;
use tokio::io::{BufWriter};
use tokio::process::Command;
use lib::index::{ImageFrameEvent};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;

    Ok(())
}

async fn handler(event: LambdaEvent<ImageFrameEvent>) -> Result<(), Error> {
    println!("{:?}", event);

    let bucket_name = dotenv::var("BUCKET_NAME")
        .expect("BUCKET_NAME must be set.");

    let shared_config = aws_config::from_env().load().await;
    let s3 = aws_sdk_s3::Client::new(&shared_config);

    let video_file_path = download_object(&s3, &bucket_name, &event.payload.video_key).await?;
    println!("Video is downloaded at {}", video_file_path);
    let image_file_path = create_frame_image(&video_file_path).await?;
    println!("Image is created at {}", image_file_path);
    upload_object(&s3, &bucket_name, &event.payload.thumbnail_key, &image_file_path).await?;
    println!("file {} is uploader at s3 key {}", &image_file_path, &event.payload.thumbnail_key);

    Ok(())
}

async fn upload_object(client: &aws_sdk_s3::Client, bucket: &str, key: &str, file_path: &str) -> Result<(), Error> {
    let bs = ByteStream::from_path(file_path).await?;
    client.put_object()
        .bucket(bucket)
        .key(key)
        .body(bs)
        .send()
        .await
        .unwrap();

    Ok(())
}

async fn create_frame_image(file_path: &str) -> Result<String, Error> {
    let output_file_path = "/tmp/thumbnail.jpg";
    Command::new("/opt/ffmpeg/bin/ffmpeg")
        .args(&["-ss", "00:00:01", "-i", file_path, "-frames:v", "1", "-q:v", "2", output_file_path])
        .output()
        .await
        .unwrap();

    Ok(output_file_path.to_string())
}

async fn download_object(client: &aws_sdk_s3::Client, bucket: &str, key: &str) -> Result<String, Error> {
    let output = client.get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;

    let file_ext = Path::new(key).extension().and_then(|s| s.to_str()).expect("file extension must be exist");
    let file_path = format!("/tmp/video.{}", file_ext);

    let mut r = output.body.into_async_read();
    let file = File::create(&file_path).await?;
    let mut w = BufWriter::new(file);

    tokio::io::copy(&mut r, &mut w).await?;

    Ok(file_path)
}
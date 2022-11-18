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
 
use aws_lambda_events::event::sns::SnsEvent;
use lambda_runtime::{Error, service_fn, LambdaEvent};
use tantivy::{doc, Term};
use lib::index::IndexTopicMessage;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;

    Ok(())
}

async fn handler(event: LambdaEvent<SnsEvent>) -> Result<(), Error> {

    println!("{:?}", serde_json::to_string(&event.payload).unwrap());

    let mount = dotenv::var("TANTIVY_MOUNT")
        .expect("TANTIVY_MOUNT must be set.");

    for record in event.payload.records {
        let message = record.sns.message;
        let msg = serde_json::from_str::<IndexTopicMessage>(&message)
            .unwrap_or_else(| _| panic!("invalid message: {}", message));

        let schema = lib::tantivy::tantivy_schema(&msg.lang);
        let index = lib::tantivy::tantivy_index(&mount, &msg.lang).unwrap();
        let mut index_writer = index.writer(50_000_000).unwrap();

        let video_id_field = schema.get_field("video_id").unwrap();
        let time_field = schema.get_field("time").unwrap();
        let body_field = schema.get_field("body").unwrap();

        // delete exist video_id docs.
        index_writer.delete_term(Term::from_field_text(video_id_field, &msg.video_id));
        index_writer.commit().unwrap();

        for line in msg.body.lines() {
            if let Some((time, body)) = line.split_once(' ') {
                index_writer.add_document(doc! {
                    video_id_field => msg.video_id.as_str(),
                    time_field => time,
                    body_field => body
                }).unwrap();
            }
        }

        index_writer.commit().unwrap();
    }

    Ok(())
}
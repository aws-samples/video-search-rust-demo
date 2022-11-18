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
 
use aws_lambda_events::event::sqs::SqsEvent;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_s3::types::ByteStream;
use itertools::Itertools;
use lambda_runtime::{Error, service_fn, LambdaEvent};
use serde_dynamo::{from_attribute_value, to_attribute_value};
use lib::index::IndexTopicMessage;
use lib::subtitle::{Subtitle, SubtitleQueueMessage};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;

    Ok(())
}

async fn handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {

    println!("{:?}", serde_json::to_string(&event.payload).unwrap());

    let table_name = dotenv::var("DYNAMODB_TABLE_NAME")
        .expect("DYNAMODB_TABLE_NAME must be set.");
    let bucket_name = dotenv::var("BUCKET_NAME")
        .expect("BUCKET_NAME must be set.");
    let topic_arn = dotenv::var("TOPIC_ARN")
        .expect("TOPIC_ARN must be set.");
    let shared_config = aws_config::from_env()
        .load().await;

    let dynamodb = aws_sdk_dynamodb::Client::new(&shared_config);
    let s3 = aws_sdk_s3::Client::new(&shared_config);
    let translate = aws_sdk_translate::Client::new(&shared_config);
    let sns = aws_sdk_sns::Client::new(&shared_config);

    for record in event.payload.records {
        let body = record.body.expect("message body must be exist");
        let msg = serde_json::from_str::<SubtitleQueueMessage>(&body)
            .unwrap_or_else(|_| panic!("invalid message: {}", body));

        let transcription_key = format!("transcription/{}", msg.video_id);
        let json = load_text_object(&s3, &bucket_name, &transcription_key).await.unwrap();

        let mut subtitle = Subtitle::from_transcribe_output(&json).unwrap();

        let lang = if let Some(target_language) = msg.translate_language {
            subtitle.translate(&translate, &msg.content_language, &target_language).await.unwrap();
            target_language
        } else {
            msg.content_language
        };

        let vtt = subtitle.vtt();
        put_object(&s3, &bucket_name, &format!("subtitle/{}/{}.vtt", msg.video_id, lang), vtt.as_bytes()).await.unwrap();
        update_subtitle(&dynamodb, &table_name, &msg.video_id, &lang).await.unwrap();
        publish_message_to_topic(&sns, &topic_arn, &IndexTopicMessage{
            video_id: msg.video_id.clone(),
            lang,
            body: subtitle.index_body()
        }).await.unwrap();
    }

    Ok(())
}

async fn load_text_object(client: &aws_sdk_s3::Client, bucket: &str, key: &str) -> Result<String, Error> {
    let output = client.get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;

    let bs = output.body.collect().await?.into_bytes();
    let text = std::str::from_utf8(&bs)?;

    Ok(text.to_string())
}

async fn put_object(client: &aws_sdk_s3::Client, bucket: &str, key: &str, content: &[u8]) -> Result<aws_sdk_s3::output::PutObjectOutput, Error> {

    let bs = ByteStream::from(content.to_vec());
    let output = client.put_object()
        .bucket(bucket)
        .key(key)
        .body(bs)
        .send()
        .await?;

    Ok(output)
}

async fn update_subtitle(client: &aws_sdk_dynamodb::Client, table_name: &str, id: &str, lang: &str) -> Result<(), Error> {
    let item_output = client.get_item()
        .table_name(table_name)
        .key("id", AttributeValue::S(id.to_owned()))
        .send()
        .await?;

    if let Some(item) = item_output.item {

        let subtitles = if let Some(v) = item.get("subtitles") {
            let mut ss: Vec<String> = from_attribute_value(v.to_owned()).unwrap();
            ss.push(lang.to_string());

            ss.into_iter().unique().collect()
        } else {
            vec![lang.to_string()]
        };

        client.update_item()
            .table_name(table_name)
            .key("id", AttributeValue::S(id.to_owned()))
            .update_expression("SET subtitles = :subtitles")
            .expression_attribute_values(":subtitles", to_attribute_value(subtitles).unwrap())
            .send()
            .await?;
    }

    Ok(())
}

async fn publish_message_to_topic(client: &aws_sdk_sns::Client, topic_arn: &str, message: &IndexTopicMessage) -> Result<(), Error> {

    client.publish()
        .topic_arn(topic_arn)
        .message(serde_json::to_string(message).unwrap())
        .send()
        .await?;

    Ok(())
}
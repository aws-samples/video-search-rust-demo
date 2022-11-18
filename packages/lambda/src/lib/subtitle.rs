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
 
use aws_sdk_translate::Client;
use serde::{Serialize, Deserialize, Deserializer, de};
use serde_json::Value;
use tokio::io::{AsyncWriteExt, BufReader};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct TranscribeJobOutput {
    #[serde(rename = "jobName")]
    job_name: String,
    #[serde(rename = "accountId")]
    account_id: String,
    status: String,
    results: TranscribeJobResult,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct TranscribeJobResult {
    items: Vec<TranscribeJobTranscriptItem>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct TranscribeJobTranscriptItem {
    r#type: String,
    #[serde(deserialize_with = "de_alternatives")]
    alternatives: String,
    #[serde(default)]
    #[serde(deserialize_with = "de_o_f32_from_str")]
    start_time: Option<f32>,
    #[serde(default)]
    #[serde(deserialize_with = "de_o_f32_from_str")]
    end_time: Option<f32>
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct TranscribeJobTranscriptItemAlternative {
    #[serde(deserialize_with = "de_f32_from_str")]
    confidence: f32,
    content: String
}

fn de_alternatives<'de, D>(deserializer: D) -> Result<String, D::Error>
    where D: Deserializer<'de>
{
    let v = Value::deserialize(deserializer)?;
    let arr = v.as_array().ok_or_else(|| de::Error::custom("alternatives must be array"))?;
    let head = arr.get(0).ok_or_else(|| de::Error::custom("alternatives must have head item"))?;
    let obj = head.as_object().ok_or_else(|| de::Error::custom("alternative must be object"))?;
    let obj_vaule = obj.get("content").ok_or_else(|| de::Error::custom("alternative must have content"))?;
    let content = obj_vaule.as_str().ok_or_else(|| de::Error::custom("content must be string"))?;

    Ok(content.to_string())
}

fn de_f32_from_str<'de, D>(deserializer: D) -> Result<f32, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(de::Error::custom)
}

fn de_o_f32_from_str<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    Ok(s.parse::<f32>().ok())
}

#[derive(Debug)]
pub struct Subtitle {
    items: Vec<SubtitleItem>
}

#[derive(Default, Debug, Clone)]
pub struct SubtitleItem {
    start_time: f32,
    end_time: f32,
    content: String
}

impl Subtitle {
    pub fn from_transcribe_output(json: &str) -> anyhow::Result<Subtitle> {
        let output = serde_json::from_str::<TranscribeJobOutput>(json)?;
        let mut items: Vec<SubtitleItem> = vec![];

        let iter = output.results.items.into_iter();
        let mut item = SubtitleItem::default();
        for it in iter {
            if item.start_time == f32::default() {
                item.start_time = it.start_time.unwrap_or(0f32);
                item.end_time = it.end_time.unwrap_or(0f32);
                item.content = it.alternatives;
            } else if it.alternatives.as_str() != "." && it.alternatives.as_str() != "?" {
                item.content = format!("{} {}", item.content, it.alternatives);
                if let Some(et) = it.end_time {
                    item.end_time = et;
                }
            } else {
                item.content = format!("{}{}", item.content, it.alternatives);
                items.push(item.clone());
                item = SubtitleItem::default();
            }
        }

        if item.start_time != f32::default() {
            items.push(item);
        }

        Ok(Subtitle { items })
    }

    pub async fn translate(&mut self, client: &Client,
                           source_language_code: &str,
                           target_language_code: &str) -> anyhow::Result<()> {
        let mut items: Vec<SubtitleItem> = vec![];
        for item in self.items.iter() {
            let mut item = item.clone();
            let translate = client.translate_text()
                .source_language_code(source_language_code)
                .target_language_code(target_language_code)
                .text(&item.content)
                .send()
                .await?;
            item.content = translate.translated_text.unwrap_or_default();
            items.push(item);
        }
        self.items = items;
        Ok(())
    }

    pub async fn save_as_srt(&self, output_path: &str) -> Result<(), tokio::io::Error> {
        let file = tokio::fs::File::create(output_path).await?;
        let mut w = BufReader::new(file);

        for (i, item) in self.items.iter().enumerate() {
            w.write_all(format!("{}\n", i + 1).as_bytes()).await?;
            w.write_all(format!("{} --> {}\n",
                                Self::time_format(item.start_time, ","),
                                Self::time_format(item.end_time, ",")).as_bytes()).await?;
            w.write_all(item.content.as_bytes()).await?;
            w.write_all(b"\n\n").await?;
        }

        w.flush().await?;
        Ok(())
    }

    pub async fn save_as_vtt(&self, output_path: &str) -> Result<(), tokio::io::Error> {
        let file = tokio::fs::File::create(output_path).await?;
        let mut w = BufReader::new(file);

        w.write_all("WEBVTT\n\n".as_bytes()).await?;

        for (_i, item) in self.items.iter().enumerate() {
            w.write_all(format!("{} --> {}\n",
                                Self::time_format(item.start_time, "."),
                                Self::time_format(item.end_time, ".")).as_bytes()).await?;
            w.write_all(item.content.as_bytes()).await?;
            w.write_all(b"\n\n").await?;
        }

        w.flush().await?;
        Ok(())
    }

    pub fn vtt(&self) -> String {
        let mut vtt = String::new();

        vtt.push_str("WEBVTT\n\n");

        for (_i, item) in self.items.iter().enumerate() {
            vtt.push_str(&format!("{} --> {}\n",
                                  Self::time_format(item.start_time, "."),
                                  Self::time_format(item.end_time, ".")));
            vtt.push_str(&item.content);
            vtt.push_str("\n\n");
        }

        vtt
    }

    pub fn index_body(&self) -> String {
        let mut body = String::new();
        for (_i, item) in self.items.iter().enumerate() {
            body.push_str(&format!("{} {}\n",
                                   &Self::time_format(item.start_time, "."),
                                   &item.content));
        }

        body
    }

    fn time_format(t: f32, d: &str) -> String {
        let mut t = t;
        let h = (t as i32) / 3600;
        t -= (h as f32) * 3600_f32;
        let m = (t as i32) / 60;
        t -= (m as f32) * 60_f32;
        let s = t as i32;
        t -= s as f32;
        let ms = (t  * 1000_f32) as i32;

        format!("{:02}:{:02}:{:02}{}{:03}", h, m, s, d, ms)
    }
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
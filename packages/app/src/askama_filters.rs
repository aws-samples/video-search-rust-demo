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

pub fn content_url_opt(key: &Option<String>) -> askama::Result<String> {
    let host = dotenv::var("CONTENT_HOST").expect("CONTENT_HOST must be set.");
    if let Some(s) = key {
        Ok(format!("//{}/{}", host, s))
    } else {
        // default thumbnail
        Ok("".to_string())
    }
}

pub fn content_url(key: &str) -> askama::Result<String> {
    let host = dotenv::var("CONTENT_HOST").expect("CONTENT_HOST must be set.");
    Ok(format!("//{}/{}", host, key))
}

pub fn second_format(seconds: &u32) -> askama::Result<String> {

    if seconds == &0 {
        return Ok("".to_string());
    }

    let mm = seconds / 60;
    let ss = seconds - (mm * 60);

    Ok(format!("{}:{:02}", mm, ss))
}
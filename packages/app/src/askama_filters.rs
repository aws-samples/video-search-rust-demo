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
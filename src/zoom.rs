use chrono::Utc;
use frank_jwt;
use reqwest;
use serde_json;
use std::env;

pub fn create_meeting() -> String {
    let user_id = env::var("ZOOM_USER_ID").expect("ZOOM_USER_ID not set");
    let default_url = env::var("ZOOM_DEFAULT_URL").expect("ZOOM_DEFAULT_URL not set").to_string();

    let url = format!("https://api.zoom.us/v2/users/{}/meetings", user_id);
    let body = json!({
        "topic": "Incident",
        "type": 1,
        "settings": {
            "auto_recording": "cloud",
            "join_before_host": true,
            "mute_upon_entry": false
        }
    });

    let client = reqwest::Client::new();
    let response_result = client.post(&url)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", make_jwt()))
        .json(&body)
        .send();

    match response_result {
        Ok(mut response) => {
            let json_result: reqwest::Result<serde_json::Value> = response.json();
            match json_result {
                Ok(json) => {
                    match json["join_url"].as_str() {
                        Some(join_url) => join_url.to_string(),
                        None => default_url,
                    }
                },
                Err(_) => default_url,
            }
        },
        Err(_) => default_url,
    }
}

fn make_jwt() -> String {
    let key = env::var("ZOOM_API_KEY").expect("ZOOM_API_KEY not set");
    let secret = env::var("ZOOM_API_SECRET").expect("ZOOM_API_SECRET not set");

    let header = json!({
        "alg": "HS256",
        "typ": "JWT"
    });
    let payload = json!({
        "iss": key,
        "exp": Utc::now().timestamp_millis()
    });

    frank_jwt::encode(header, &secret, &payload, frank_jwt::Algorithm::HS256).unwrap()
}

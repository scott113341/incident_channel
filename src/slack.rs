use std::env;

extern crate slack_api;

pub fn create_channel(channel_name: &str) -> Result<slack_api::Channel, String> {
    let client = slack_api::default_client().unwrap();

    let request = slack_api::channels::CreateRequest {
        name: channel_name,
        validate: Some(true),
    };

    let response = slack_api::channels::create(&client, &token(), &request);
    match response {
        Ok(c_res) => {
            match c_res.channel {
                Some(channel) => Ok(channel),
                None => Err("Something went wrong".to_string()),
            }
        },
        Err(c_err) => Err(format!("{:?}", c_err)),
    }
}

pub fn send_info(channel_id: &str, user_id: &str, user_name: &str, zoom_url: &str) {
    let client = slack_api::default_client().unwrap();

    // Send message with info
    let attachments = json!([{
        "fields": [
            {
                "title": "Incident Bridge",
                "value": zoom_url,
                "short": false
            },
            {
                "title": "On-Call Schedules",
                "value": env::var("PAGERDUTY_URL").expect("PAGERDUTY_URL not set."),
                "short": false
            }
        ]
    }]).to_string();
    let request = slack_api::chat::PostMessageRequest {
        channel: &channel_id,
        text: &format!("This incident channel was created by <@{}|{}>.", user_id, user_name),
        attachments: Some(&attachments),
        ..slack_api::chat::PostMessageRequest::default()
    };

    slack_api::chat::post_message(&client, &token(), &request);
    ()
}

fn token() -> String {
    env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN not set.")
}

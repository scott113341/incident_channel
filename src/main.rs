#![feature(plugin)]
#![feature(custom_derive)]
#![feature(dbg_macro)]
#![plugin(rocket_codegen)]

mod slack;
mod zoom;

extern crate chrono;
extern crate frank_jwt;
extern crate rocket;
#[macro_use]
extern crate serde_json;

use chrono::Utc;
use rocket::request::LenientForm;
use std::thread;

#[derive(Debug, FromForm)]
struct SlashCommand {
    command: String,
    text: String,
    response_url: String,
    user_id: String,
    user_name: String,
}

#[post("/", data = "<slash_command>")]
fn index(slash_command: LenientForm<SlashCommand>) -> String {
    let cmd = slash_command.get();

    // Format channel name
    let date = Utc::today().format("%Y%m%d").to_string();
    let channel_name = match cmd.text.chars().count() {
        0 => format!("{}-incident", date),
        _ => format!("{}-{}", date, cmd.text.replace(" ", "-")),
    };

    // Create incident channel
    let channel = slack::create_channel(&channel_name);
    match channel {
        Ok(channel) => {
            let channel_id = channel.id.unwrap();
            let channel_name = channel.name.unwrap();
            let user_id = cmd.user_id.clone();
            let user_name = cmd.user_name.clone();

            let res = format!("Created incident channel <#{}|{}>", channel_id, channel_name);

            thread::spawn(move || {
                let zoom_url = zoom::create_meeting();
                slack::send_info(&channel_id, &user_id, &user_name, &zoom_url);
            });

            res
        },
        Err(err) => format!("Error creating channel: {}", err),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

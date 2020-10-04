mod commands;
mod shared;
mod helpers;
mod core;

use serenity::http::Http;
use serenity::prelude::*;
use std::collections::HashSet;
use crate::shared::insert_shared_data;
use crate::core::{Handler, get_framework};

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Expected 'DISCORD_TOKEN' in the environment variables.");

    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .framework(get_framework(bot_id, owners))
        .await
        .expect("Failed to create discord client.");

    insert_shared_data(&client).await;

    if let Err(err) = client.start().await {
        println!("Failed to start the bot: {:?}", err);
    }
}

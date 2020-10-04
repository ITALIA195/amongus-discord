use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        voice::VoiceState,
        id::{GuildId, UserId}
    },
    framework::standard::{
        CommandResult,
        StandardFramework,
        macros::hook
    },
    prelude::*
};
use crate::shared::MutedUsersTracker;
use crate::commands::MUTING_GROUP;
use std::collections::HashSet;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Successfully connected as {}", ready.user.name);
    }

    async fn voice_state_update(&self, ctx: Context, _: Option<GuildId>, old: Option<VoiceState>, new: VoiceState) {
        let data = ctx.data.read().await;
        let muted_users = data.get::<MutedUsersTracker>().expect("Expected MutedUsersTracker in TypeMap.");

        // Left 
    }
}

#[hook]
async fn after(ctx: &Context, msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => {
            println!("Command '{}' by '{}'", command_name, msg.author.name);
            match msg.delete(ctx).await {
                Ok(()) => println!("Deleted caller message"),
                Err(why) => println!("Failed to delete caller message: {:?}", why)
            };
        },
        Err(why) => println!("Command '{}' returned error {:?}", command_name, why)
    }
}

pub struct Handler;

pub fn get_framework(bot_id: UserId, owners: HashSet<UserId>) -> StandardFramework {
    StandardFramework::new()
        .configure(|c| c
            .prefix("a")
            .on_mention(Some(bot_id))
            .owners(owners))
        .after(after)
        .group(&MUTING_GROUP)
}

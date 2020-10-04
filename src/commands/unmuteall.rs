use crate::helpers::{set_mute, set_mute_channel_members};
use serenity::{
    model::{
        guild::{Guild, Member},
        voice::{VoiceState},
        id::{ChannelId},
        channel::Message,
    },
    framework::standard::{
        CommandResult,
        Args,
        macros::command
    },
    utils::MessageBuilder,
    prelude::*
};

#[command]
#[description("Unmutes everyone in the channel you are in")]
#[aliases("u")]
pub async fn unmuteall(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let guild: Guild = match msg.guild_id {
        Some(id) => ctx.cache.guild(id).await.expect("Guild is not cached"),
        None => {
            if let Err(err) = msg.channel_id.say(&ctx.http, "This command can only be used in servers.").await {
                println!("Error sending message: {:?}", err);
            }
            return Ok(());
        }
    };

    let caller_voice_state: &VoiceState = match guild.voice_states.get(&msg.author.id) {
        Some(voice_state) => voice_state,
        None => {
            if let Err(why) = msg.channel_id.say(&ctx.http, "You aren't in a voice channel").await {
                println!("Error sending message: {:?}", why);
            }
            return Ok(());
        }
    };

    for (user_id, voice_state) in &guild.voice_states {
        if voice_state.channel_id != caller_voice_state.channel_id {
            continue;
        }

        let member: Member = match ctx.cache.member(guild.id, user_id).await {
            Some(member) => member,
            None => {
                println!("Failed to access member {:?} cache", user_id);
                let reply = MessageBuilder::new()
                    .push("Failed to fetch ")
                    .mention(user_id)
                    .push("'s guild member data")
                    .build();
                if let Err(why) = msg.channel_id.say(&ctx.http, &reply).await {
                    println!("Error sending message: {:?}", why);
                }
                return Ok(());
            }
        };

        if let Err(err) = set_mute(ctx, &member, false).await {
            println!("Error while unmuting {}: {:?}", member.user.name, err);
            let reply = MessageBuilder::new()
                .push("I don't have the permission to unmute ")
                .mention(&member.user)
                .build();
            if let Err(why) = msg.channel_id.say(&ctx.http, &reply).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    
    Ok(())
}

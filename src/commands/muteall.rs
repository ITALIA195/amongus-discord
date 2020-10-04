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
#[description("Mutes everyone in the channel you are in")]
#[aliases("m")]
pub async fn muteall(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let guild: Guild = match msg.guild_id {
        Some(id) => ctx.cache.guild(id).await.expect("Expected Guild to be cached."),
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

    let channel_id: ChannelId = caller_voice_state.channel_id.expect("Expected Caller to be in a voice channel");
    set_mute_channel_members(ctx, guild, channel_id, true).await?;
    
    Ok(())
}

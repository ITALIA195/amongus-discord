use serenity::{
    model::{
        id::{UserId, ChannelId},
        guild::{Guild, Member}
    },
    builder::EditMember,
    prelude::*
};
use crate::shared::MutedUsersTracker;

pub enum MuteError {
    MissingMutePermission,
    __Nonexhaustive
}

pub async fn set_mute(ctx: &Context, member: &Member, mute: bool) -> Result<(), SerenityError> {
    member.edit(&ctx.http, |edit: &mut EditMember| {
        edit.mute(mute);
        edit
    }).await?;

    let mut data = ctx.data.write().await;
    let muted_users: &mut Vec<UserId> = data.get_mut::<MutedUsersTracker>().expect("Expected MutedUsersTracker in TypeMap.");
    match mute {
        true => muted_users.push(member.user.id),
        false => muted_users.retain(|&id| id != member.user.id)
    }

    Ok(())
}

pub async fn set_mute_channel_members(ctx: &Context, guild: Guild, channel_id: ChannelId, mute: bool) -> Result<(), SerenityError> {
    let users_in_channel = guild.voice_states.iter()
        .filter(|&(_, state)| state.channel_id == Some(channel_id))
        .map(|(user_id, _)| user_id);

    for user_id in users_in_channel {
        let member: Member = guild.member(ctx, user_id).await.expect("Expected user to be in Guild.");
        set_mute(ctx, &member, mute).await?;
    }

    Ok(())
}
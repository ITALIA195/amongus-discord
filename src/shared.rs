use serenity::prelude::*;
use serenity::model::id::UserId;
use serenity::model::id::ChannelId;

pub struct MutedChannelsTracker;
pub struct MutedUsersTracker;

impl TypeMapKey for MutedUsersTracker {
    type Value = Vec<UserId>;
}

impl TypeMapKey for MutedChannelsTracker {
    type Value = Vec<ChannelId>;
}

pub async fn insert_shared_data(client: &Client) {
    let mut data = client.data.write().await;

    data.insert::<MutedUsersTracker>(Vec::default());
    data.insert::<MutedChannelsTracker>(Vec::default());
}
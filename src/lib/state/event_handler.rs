use super::State;
use async_trait::async_trait;
use twilight_model::gateway::payload::{
    BanAdd, BanRemove, ChannelCreate, ChannelDelete, ChannelPinsUpdate, ChannelUpdate, GuildCreate,
    GuildDelete, GuildEmojisUpdate, GuildIntegrationsUpdate, GuildUpdate, InviteCreate,
    InviteDelete,
};

macro_rules! impl_handler_trait {
    ($($fn:ident: [$($fn_args:ty),*];)*) => {
            #[async_trait]
            #[allow(unused_variables)]
            pub trait EventHandler: Send + Sync {
                $(
                    async fn $fn(&self, state: &State, $(_: $fn_args)*) -> super::super::GenericResult<()> {
                        Ok(())
                    }
                )*
            }
    }
}

impl_handler_trait! {
    ban_add: [BanAdd];
    ban_remove: [BanRemove];
    channel_create: [ChannelCreate];
    channel_delete: [ChannelDelete];
    channel_pins_update: [ChannelPinsUpdate];
    channel_update: [ChannelUpdate];
    gateway_heartbeat: [u64];
    gateway_heartbeat_ack: [];
    gateway_hello: [u64];
    gateway_invalidate_session: [bool];
    gateway_reconnect: [];
    gift_code_update: [];
    guild_create: [Box<GuildCreate>];
    guild_delete: [Box<GuildDelete>];
    guild_emojis_update: [GuildEmojisUpdate];
    guild_integrations_update: [GuildIntegrationsUpdate];
    guild_update: [Box<GuildUpdate>];
    invite_create: [Box<InviteCreate>];
    invite_delete: [InviteDelete];
    
}

// #[async_trait]
// pub trait EventHandler: Send + Sync {
//     async fn ban_add(&self, state: &State, ban: BanAdd) -> super::super::GenericResult<()> {
//         Ok(())
//     }

//     async fn ban_remove(&self, state: &State, ban: BanRemove) -> super::super::GenericResult<()> {
//         Ok(())
//     }
// }

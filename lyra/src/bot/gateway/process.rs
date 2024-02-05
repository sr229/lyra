use std::sync::Arc;

use twilight_gateway::{Event, Latency, MessageSender};

use super::{model::Process, LastCachedStates};
use crate::bot::{core::model::BotState, error::gateway::ProcessResult};

pub async fn process(
    bot: Arc<BotState>,
    event: Event,
    states: LastCachedStates,
    latency: Latency,
    sender: MessageSender,
) -> ProcessResult {
    match event {
        Event::Ready(ref e) => bot.as_ready_context(e).process().await,
        Event::GuildCreate(ref e) => bot.as_guild_create_context(e).process().await,
        Event::GuildDelete(ref e) => bot.as_guild_delete_context(e).process().await,
        Event::InteractionCreate(e) => {
            bot.into_interaction_create_context(e, latency, sender)
                .process()
                .await
        }
        Event::VoiceStateUpdate(e) => {
            bot.into_voice_state_update_context(e, states, sender)
                .process()
                .await
        }
        _ => Ok(()),
    }
}

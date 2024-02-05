use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::bot::{
    command::{
        macros::{caut, hid, out},
        model::{BotSlashCommand, SlashCommand},
        Ctx,
    },
    error::command::Result as CommandResult,
};

/// Shows the bot's latency.
#[derive(CreateCommand, CommandModel)]
#[command(name = "ping")]
pub struct Ping;

impl BotSlashCommand for Ping {
    async fn run(self, mut ctx: Ctx<SlashCommand>) -> CommandResult {
        if let Some(latency) = ctx.latency().average() {
            out!(format!("🏓 Pong! `({}ms)`", latency.as_millis()), ctx);
        } else {
            caut!(
                "Cannot calculate the ping at the moment, try again later.",
                ctx
            );
        }
    }
}

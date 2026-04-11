#[allow(dead_code)]
mod xp_processor;

#[allow(dead_code)]
mod db_manager;
#[allow(dead_code)]
mod slash_commands;

use clap::Parser;
use serenity::all::{CreateInteractionResponseMessage, GuildId, Interaction};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event. This is called whenever a new message is received.
    //
    // Event handlers are dispatched through a threadpool, and so multiple events can be
    // dispatched simultaneously.
    async fn message(&self, _ctx: Context, msg: Message) {
        xp_processor::run(&msg);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "bonus" => Some(slash_commands::bonus::run(&command.data.options())),
                "flush" => Some(slash_commands::flush::run(&command.data.options())),
                "add_channel" => Some(slash_commands::add_channel::run(&command.data.options())),
                "remove_channel" => {
                    Some(slash_commands::remove_channel::run(&command.data.options()))
                }
                "is_xp_channel" => Some(slash_commands::is_rp::run(&command.data.options())),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = serenity::all::CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
    // a READY payload is sent by Discord. This payload contains data like the current user's guild
    // Ids, current user data, private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let ecks_guild_id: GuildId = GuildId::new(416678673538613283);
        ecks_guild_id
            .set_commands(
                &ctx.http,
                vec![
                    slash_commands::bonus::register(),
                    slash_commands::flush::register(),
                    slash_commands::is_rp::register(),
                    slash_commands::add_channel::register(),
                    slash_commands::remove_channel::register(),
                ],
            )
            .await
            .unwrap();
    }
}

/// Simple bot to track xp of users
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Token to authenticate with discord
    #[arg(short, long)]
    token: String,

    /// File path of the sqlite3 database
    #[arg(short, long, default_value = "test-db.db3")]
    database: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    db_manager::initialize_db(&args.database);

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(&args.token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

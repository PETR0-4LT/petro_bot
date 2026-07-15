#[allow(dead_code)]
mod xp_processor;

#[allow(dead_code)]
mod db_manager;
#[allow(dead_code)]
mod slash_commands;

use clap::Parser;
use serenity::all::{CommandInteraction, CreateInteractionResponseFollowup, CreateInteractionResponseMessage, GuildId, Interaction};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

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
                "bonus" => slash_commands::bonus::run(&command.data.options()),
                "flush" => slash_commands::flush::run(&command.data.options()),
                "add_channel" => slash_commands::add_channel::run(&command.data.options()),
                "remove_channel" => slash_commands::remove_channel::run(&command.data.options()),
                "is_xp_channel" => slash_commands::is_rp::run(&command.data.options()),
                "leaderboard" => slash_commands::leaderboard::run(&command.data.options()),
                "flush_range" => slash_commands::flush_range::run(&command.data.options()),
                _ => slash_commands::DisplayType::Text("not implemented :(".to_string())
            };

            let data = match content
            {
                // string array resposne type (sending multiple sepparate messages to one command) is the only case that needs a different behaviour.
                // it uses an early return. Everything else uses the bottom send_response() call. This is ugly. I could've avoided it but
                // I didn't want to change the send_response() 'data' param to be a vector, because most would be single-element; Nor add another layer to the DisplayType enum for Single/Multiple. 
                slash_commands::DisplayType::StringArr(arr) => 
                {
                    send_messages(&ctx, &command, arr).await;
                    return; 
                }
                slash_commands::DisplayType::Text(src) => CreateInteractionResponseMessage::new().content(src),
                slash_commands::DisplayType::EmbedArr(arr) => CreateInteractionResponseMessage::new().embeds(arr), // embedArr doesn't have the same problem because it's alr. supported
                slash_commands::DisplayType::Embed(val) => CreateInteractionResponseMessage::new().embed(val)
            };
            // BEHAVIOUR FOR ALL EXCEPT STRINGARR
            send_response(&ctx, &command, data).await;
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
                    slash_commands::leaderboard::register(),
                    slash_commands::flush_range::register()
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


async fn send_response(ctx : &Context, command : &CommandInteraction, data : CreateInteractionResponseMessage)
{
    let builder = serenity::all::CreateInteractionResponse::Message(data);
    if let Err(why) = command.create_response(&ctx.http, builder).await {
        println!("Cannot respond to slash command: {why}");
    }
}
async fn send_messages(ctx : &Context, command : &CommandInteraction, string_arr : Vec<String>)
{
    let mut iter = string_arr.into_iter();
    let init_response =  CreateInteractionResponseMessage::new().content(iter.next().unwrap());
    send_response(ctx, command, init_response).await;
    for i in iter
    {
        let builder = CreateInteractionResponseFollowup::new().content(i);
        if let Err(why) = command.create_followup(&ctx.http, builder).await {
            println!("Cannot respond to slash command: {why}");
        }
    }
}
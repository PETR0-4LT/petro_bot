use crate::db_manager;
use serenity::all::{CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue};

pub fn run(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::Channel(channel),
        ..
    }) = options.first()
    {
        if db_manager::channels::query_is_rp(&channel.id.to_string()) {
            format!(
                "{} is in the list of XP channels.",
                channel.name.as_ref().unwrap()
            )
        } else {
            "I have NEVER heard of this channel in my LIFE vro".to_string()
        }
    } else {
        "Please provide a valid channel".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("is_xp_channel")
        .description("Checks if a given channel to the list of XP channels")
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::Channel,
                "channel",
                "channel to query",
            )
            .required(true),
        )
}

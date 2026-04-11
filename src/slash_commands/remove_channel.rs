use crate::db_manager;
use serenity::all::{
    CreateCommand, CreateCommandOption, Permissions, ResolvedOption, ResolvedValue,
};

pub fn run(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::Channel(channel),
        ..
    }) = options.first()
    {
        db_manager::channels::delete(&channel.id.to_string());
        format!(
            "{} removed from the list of XP channels.",
            channel.name.as_ref().unwrap()
        )
    } else {
        "Please provide a valid channel".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("remove_channel")
        .description("Removes a given channel to the list of XP channels")
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::Channel,
                "channel",
                "channel to add to db",
            )
            .required(true),
        )
        .default_member_permissions(Permissions::MANAGE_ROLES)
}

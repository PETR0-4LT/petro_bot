use serenity::all::{CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue};

use crate::{db_manager, slash_commands::DisplayType};

pub fn run(options: &[ResolvedOption]) -> DisplayType {
    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _),
        ..
    }) = options.first()
    {
        DisplayType::Text(
        format!(
            "{}'s bonus xp is {}",
            user.tag(),
            db_manager::users::query_xp(&user.id.to_string())
        ))
    } else {
        DisplayType::Text("Please provide a valid user".to_string())
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("bonus")
        .description("View the bonus XP accumulated by a given user")
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::User,
                "user",
                "User to query",
            )
            .required(true),
        )
}

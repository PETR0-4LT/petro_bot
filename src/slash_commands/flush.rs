use crate::{db_manager, slash_commands::DisplayType};
use serenity::all::{
    CreateCommand, CreateCommandOption, Permissions, ResolvedOption, ResolvedValue
};

pub fn run(options: &[ResolvedOption]) -> DisplayType {
    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _),
        ..
    }) = options.first()
    {
        let xp = db_manager::users::query_xp(&user.id.to_string());
        db_manager::users::set_zero(&user.id.to_string());
        DisplayType::Text(format!("Reset {}'s bonus xp of {} to 0.", user.tag(), xp))
    } else {
        DisplayType::Text("Please provide a valid user".to_string())
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("flush")
        .description("Resets a given user's bonus xp")
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::User,
                "user",
                "User to reset",
            )
            .required(true),
        )
        .default_member_permissions(Permissions::MANAGE_ROLES)
}

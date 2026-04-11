use crate::db_manager;
use serenity::all::{
    CreateCommand, CreateCommandOption, Permissions, ResolvedOption, ResolvedValue,
};

pub fn run(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _),
        ..
    }) = options.first()
    {
        let xp = db_manager::users::query_xp(&user.id.to_string());
        db_manager::users::delete(&user.id.to_string());
        format!("Reset {}'s bonus xp of {} to 0.", user.tag(), xp)
    } else {
        "Please provide a valid user".to_string()
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

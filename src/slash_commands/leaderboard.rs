use crate::{db_manager, slash_commands::DisplayType};
use serenity::all::{
    CreateCommand, CreateCommandOption, CreateEmbed, ResolvedOption, ResolvedValue
};

pub fn run(options: &[ResolvedOption]) -> DisplayType {
    // command is /leaderboard [cutoff]; the cutoff argument is optional
    // here min_xp = cutoff, if cutoff is provided. Otherwise it is 0.
    let min_xp = match options.first()
    {
        Some(option) => match option.value
        {
            ResolvedValue::Integer(val) => val,
            _ => 0
        },
        None => 0 
    };
    
    let users = db_manager::users::query_users(min_xp as u32);
    let mut embed = CreateEmbed::new().title("Leaderboard");
    for (i, (user_id, xp)) in users.iter().enumerate()
    {
        embed = embed.field((i + 1).to_string(), format!(" <@{}>  - {} \n", user_id, xp), true);
    }
    DisplayType::Embed(embed)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("leaderboard")
        .description("Lists all users of XP > [CUTOFF]")
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::Integer,
                "cutoff",
                "Minimum XP to be displayed",
            )
            .required(false)
        )
}

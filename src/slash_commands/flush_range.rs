use serenity::all::{CreateCommand, CreateCommandOption, Permissions, ResolvedOption, ResolvedValue};

use crate::{db_manager, slash_commands::DisplayType};

pub fn run(options: &[ResolvedOption]) -> DisplayType {
    if let Some(ResolvedOption {
        value: ResolvedValue::Integer(min_xp),
        ..
    }) = options.first()
    {
        // second argument tells wheter the formatting should be fenced or plaintext. Default is false (plaintext)
        let fenced = match options.get(1)
        {
            Some(option) => match option.value
            {
                ResolvedValue::Boolean(p_bool) => p_bool,
                _ => false
            },
            None => false 
        };

        let users = db_manager::users::flush_users(*min_xp as u32);
        // iterating through the vector again just to build another array with its formatted contents
        // that is then iterated over again in the dispatcher, is really inefficient
        // But this command should only be ran once every couple of weeks and for fairly small arrays, so its no biggie
        let mut arr = Vec::<String>::new(); 
        arr.push(format!("Reset XP of {} users", users.len()));
        if fenced == true
        {
            for i in users
            {
                arr.push(format!("```!give-xp <@{}> {}```", i.0, i.1));
            }
        }
        else // IT WAS SO GOOD I EVEN WROTE IT TWICE
        {
            for i in users
            {
                arr.push(format!("!give-xp <@{}> {}", i.0, i.1));
            }               
        }
        DisplayType::StringArr(arr)
    } else {
        DisplayType::Text("Can't do that mate".to_string())
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("flush_range")
        .description("Resets XP of all users above [cutoff] and prints a copy-pastable list")
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::Integer,
                "cutoff",
                "Minimum XP to be purged",
            )
            .required(true)
        )
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::Boolean,
                "fenced",
                "Should the messages be 'fenced' (doesn't ping, easier to copy-paste on PC, harder on mobile)",
            )
            .required(true)
        )
        .default_member_permissions(Permissions::MANAGE_ROLES)

}
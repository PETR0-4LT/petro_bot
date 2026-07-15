use serenity::all::CreateEmbed;

pub mod add_channel;
pub mod bonus;
pub mod flush;
pub mod is_rp;
pub mod remove_channel;
pub mod leaderboard;
pub mod flush_range;

pub enum DisplayType
{
    Embed(CreateEmbed),
    EmbedArr(Vec<CreateEmbed>),
    Text(String),
    StringArr(Vec<String>),
}

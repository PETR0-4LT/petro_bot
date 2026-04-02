use serenity::{all::Message};

use crate::db_manager;

pub fn message_is_in_rp(src : &Message) -> bool
{  
    db_manager::channels::query_is_rp(&src.channel_id.to_string())
}
pub fn get_bonus(src : &str) -> u32
{
    src.len() as u32 / 8 // DECIDE EXACT RATE LATER
}
 
pub fn run(src : &Message)
{
    if message_is_in_rp(src)
    {
        let bonus = get_bonus(&src.content);
        if bonus >= 15
        {
            db_manager::users::update_or_insert(&src.author.id.to_string(), bonus);
        }
    }
}
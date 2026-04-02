use super::get_connection;
use super::DB;

use std::sync::{MutexGuard};
use rusqlite::{Connection};

fn insert(id : &str, bonus_xp : u32, connection : &MutexGuard<Connection>)
{
    let sql = "INSERT INTO users (user_id, bonus_xp) VALUES (:id, :bonus_xp)";
    connection.execute(&sql, &[(":id", id), (":bonus_xp", &bonus_xp.to_string())]).unwrap();
}

fn exists(id : &str, connection : &MutexGuard<Connection>) -> bool
{
    let sql = "SELECT user_id FROM users WHERE user_id = :id";
    let mut stmt = connection.prepare(&sql).unwrap();
    let mut rows = stmt.query(&[(":id", &id)]).unwrap();
    rows.next().unwrap().is_some()
}

// returns TRVE if there exists a user with the given user_id in the users db
pub fn query_exists(id : &str) -> bool
{
    let conn = get_connection!();
    exists(id, &conn)
}

pub fn query_xp(id : &str) -> u32
{
    let conn = get_connection!();
    let sql = "SELECT bonus_xp FROM users WHERE user_id = :id";
    let mut stmt = conn.prepare(&sql).unwrap();
    let mut rows = stmt.query(&[(":id", &id)]).unwrap();

    match rows.next().unwrap()
    {
        None => 
        {
            return 0;
        }
        Some(row) =>
        {
            return row.get(0).unwrap();
        }
    }
}

pub fn delete_all()
{
    let conn = get_connection!();
    let sql = "DELETE FROM users WHERE 1 = 1";
    conn.execute(sql, ()).unwrap();
}

pub fn delete(id : &str)
{
    let conn = get_connection!();
    let sql = "DELETE FROM users WHERE user_id = :id";
    conn.execute(sql, &[(":id", &id)]).unwrap();
}

pub fn push_new(id : &str, bonus_xp : u32)
{
    let conn = get_connection!();
    insert(id, bonus_xp, &conn);
}

pub fn update_or_insert(id : &str, bonus_xp : u32)
{
    let conn = get_connection!();
    if !exists(id, &conn)
    {
        insert(id, bonus_xp, &conn);
    }
    else 
    {
        //update
        let update_sql = "UPDATE users SET bonus_xp = bonus_xp + :bonus WHERE user_id = :id";
        conn.execute(update_sql, &[(":id", id), (":bonus", &bonus_xp.to_string())]).unwrap();
    }
}
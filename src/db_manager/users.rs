use super::DB;
use super::get_connection;

use rusqlite::Connection;
use std::sync::MutexGuard;

fn insert(id: &str, bonus_xp: u32, connection: &MutexGuard<Connection>) {
    let sql = "INSERT INTO users (user_id, bonus_xp) VALUES (:id, :bonus_xp)";
    connection
        .execute(sql, &[(":id", id), (":bonus_xp", &bonus_xp.to_string())])
        .unwrap();
}

fn exists(id: &str, connection: &MutexGuard<Connection>) -> bool {
    let sql = "SELECT user_id FROM users WHERE user_id = :id";
    let mut stmt = connection.prepare(sql).unwrap();
    let mut rows = stmt.query(&[(":id", &id)]).unwrap();
    rows.next().unwrap().is_some()
}

// returns an array of users and their xp whose bonus xp is greater than min_xp
fn get_users(min_xp: u32, connection: &MutexGuard<Connection>) -> Vec<(String, u32)>
{
    let sql = "SELECT user_id, bonus_xp FROM users WHERE bonus_xp > :min_xp ORDER BY bonus_xp DESC";
    let mut stmt = connection.prepare(sql).unwrap();
    
    let rows = stmt.query_map( 
        &[(":min_xp", &min_xp)], 
        |row| 
        {
            let user_id: i64 = row.get(0)?;
            let bonus_xp: i64 = row.get(1)?;
            Ok((user_id.to_string(), bonus_xp as u32))
        }
    ).unwrap();
    rows.collect::<Result<_, _>>().unwrap() // I hate rust so much its unreal
}

// returns TRVE if there exists a user with the given user_id in the users db
pub fn query_exists(id: &str) -> bool {
    let conn = get_connection!();
    exists(id, &conn)
}

pub fn query_xp(id: &str) -> u32 {
    let conn = get_connection!();
    let sql = "SELECT bonus_xp FROM users WHERE user_id = :id";
    let mut stmt = conn.prepare(sql).unwrap();
    let mut rows = stmt.query(&[(":id", &id)]).unwrap();

    match rows.next().unwrap() {
        None => 0,
        Some(row) => row.get(0).unwrap(),
    }
}

// returns a vector of tuples (user_id, bonus_xp), where user's xp > min_xp
pub fn query_users(min_xp: u32) -> Vec<(String, u32)>
{
    let conn = get_connection!();
    get_users(min_xp, &conn)
}

pub fn flush_users(min_xp: u32) -> Vec<(String, u32)>
{
    let conn = get_connection!();
    let ret = get_users(min_xp, &conn);

    let sql = "UPDATE users SET bonus_xp = 0 WHERE bonus_xp >= :min_xp";
    conn.execute(sql, &[(":min_xp", &min_xp)]).unwrap();
    ret
}

pub fn delete_all() {
    let conn = get_connection!();
    let sql = "DELETE FROM users WHERE 1 = 1";
    conn.execute(sql, ()).unwrap();
}

pub fn delete(id: &str) {
    let conn = get_connection!();
    let sql = "DELETE FROM users WHERE user_id = :id";
    conn.execute(sql, &[(":id", &id)]).unwrap();
}

pub fn push_new(id: &str, bonus_xp: u32) {
    let conn = get_connection!();
    insert(id, bonus_xp, &conn);
}

pub fn set_zero(id: &str) {
    let conn = get_connection!();
    let update_sql = "UPDATE users SET bonus_xp = 0 WHERE user_id = :id";
        conn.execute(
            update_sql,
            &[(":id", id)]
        )
        .unwrap();
}

pub fn update_or_insert(id: &str, bonus_xp: u32) {
    let conn = get_connection!();
    if !exists(id, &conn) {
        insert(id, bonus_xp, &conn);
    } else {
        let update_sql = "UPDATE users SET bonus_xp = bonus_xp + :bonus WHERE user_id = :id";
        conn.execute(
            update_sql,
            &[(":id", id), (":bonus", &bonus_xp.to_string())],
        )
        .unwrap();
    }
}

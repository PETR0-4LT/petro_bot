use std::sync::MutexGuard;

use rusqlite::Connection;

use super::DB;
use super::get_connection;

fn exists(id: &str, connection: &MutexGuard<Connection>) -> bool {
    let sql = "SELECT channel_id FROM rp_channels WHERE channel_id = :id";
    // make the query duh
    let mut stmt = connection.prepare(&sql).unwrap();
    let mut rows = stmt.query(&[(":id", &id)]).unwrap();
    rows.next().unwrap().is_some()
}

// returns true if the given ChannelID is an "rp_channel" (in the rp_channels table)
pub fn query_is_rp(id: &str) -> bool {
    // steal the database or something I don't fucking KNOW dude.
    let conn = get_connection!();
    exists(id, &conn)
}

pub fn insert(id: &str) {
    let conn = get_connection!();
    if exists(id, &conn) {
        return;
    }
    let sql = "INSERT INTO rp_channels (channel_id) VALUES (:id)";
    conn.execute(sql, &[(":id", &id)])
        .expect("insertion failed somehow");
}
pub fn delete(id: &str) {
    let conn = get_connection!();
    // double delete doesn't make the bot shit itself so no need to check :)
    /*if !exists(id, &conn)
    {
        return;
    }*/
    let sql = "DELETE FROM rp_channels WHERE channel_id = :id";
    conn.execute(sql, &[(":id", &id)]).unwrap();
}

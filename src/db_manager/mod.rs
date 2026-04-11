use rusqlite::Connection;
use std::sync::OnceLock;

pub mod channels;
pub mod users;

// only way I can make the db connection a global variable is with this fuckass mutex I don't realy know how to use.
// I can't otherwise pass the connection as parameter to functions because I can't change the message handler's parameter list as to pass it along

static DB: OnceLock<std::sync::Mutex<Connection>> = OnceLock::new();

pub fn initialize_db() {
    let conn = Connection::open("test-db.db3").expect("DB open failed");
    DB.set(std::sync::Mutex::new(conn))
        .expect("DB already initialized");
}

// far from the proper way to do this but idc
macro_rules! get_connection {
    () => {
        DB.get().expect("i hate rust").lock().unwrap()
    };
}
pub(crate) use get_connection;

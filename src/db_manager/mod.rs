use rusqlite::Connection;
use std::sync::OnceLock;

pub mod channels;
pub mod users;

static DB: OnceLock<std::sync::Mutex<Connection>> = OnceLock::new();

pub fn initialize_db(path: &str) {
    let conn = Connection::open(path).expect("DB open failed");
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

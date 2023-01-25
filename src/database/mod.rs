pub mod user;

pub use mysql::*;
pub use user::add_user;
pub use user::get_users;

static mut POOL: Option<Pool> = None;

pub fn connect() {
    let url = "mysql://root:root@localhost:3306/rust-test";
    unsafe {
        POOL = Some(Pool::new(url).expect("❌ Could not connect to database!"));
        println!("\n\r🔗 Connected to database.");
    }
}

fn get_connection() -> PooledConn {
    unsafe {
        if POOL.is_none() {
            panic!("Fist connect to database");
        }
        let conn = POOL
            .clone()
            .unwrap()
            .get_conn()
            .expect("❌ Sorry no database connection...");
        return conn;
    }
}

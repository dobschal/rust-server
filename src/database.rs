pub mod database {
    use mysql::prelude::*;
    use mysql::*;

    use crate::entities::entities::User;

    static mut POOL: Option<Pool> = None;

    pub fn connect() -> std::result::Result<(), Box<dyn std::error::Error>> {
        println!("Connect to database...");
        let url = "mysql://root:root@localhost:3306/rust-test";
        unsafe {
            POOL = Some(Pool::new(url)?);
            println!("Connected to database.")
        }
        return Ok(());
    }

    pub fn get_users() -> Vec<User> {
        println!("Request database...");
        let users = get_connection()
            .query_map("SELECT id, name from user", |(id, name)| User { id, name })
            .expect("Could not get users...");
        println!("Users: {:?}", users);
        return users;
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
                .expect("Sorry no database connection...");
            return conn;
        }
    }
}

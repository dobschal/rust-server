pub mod database {
    use mysql::prelude::*;
    use mysql::*;

    use crate::entities::entities::User;

    pub fn request_database() -> std::result::Result<Vec<User>, Box<dyn std::error::Error>> {
        println!("Request database...");
        let url = "mysql://root:root@localhost:3306/rust-test";
        let pool = Pool::new(url)?;

        let mut conn = pool.get_conn()?;

        let users = conn.query_map("SELECT id, name from user", |(id, name)| User { id, name })?;

        println!("Users: {:?}", users);

        return Ok(users);
    }
}

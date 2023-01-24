pub mod entities {

    #[derive(Debug, PartialEq, Eq)]
    pub struct User {
        pub id: i32,
        pub name: Option<String>,
    }
}

# Rust Server

Simple server that connects to a MySQL database and returns HTML or JSON.

## Get Started
Follow these steps to get in running:

### Database

Run a MySQL database `rust-test` on `localhost:3306` with user `root` and password `root` (or change params in `src/database/mod.rs`).
Import the the SQL table:
```sql
CREATE TABLE `user` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=latin1;

INSERT INTO `user` (`id`, `name`) VALUES
(1, 'Sascha'),
(2, 'You');
```

### Build and Run
Install Cargo and run in terminal:
```bash
cargo build # fetches necessary dependencies before build automatically
cargo run # will run the server on http://localhost:7878
```

### Test
Open a browser (or Postman) and call (GET):
> http://localhost:7878/users

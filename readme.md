# Rust Server

Simple server that connects to a MySQL database and returns static files (HTML, CSS, Javascript, Images, ...) or JSON.

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
# fetches necessary dependencies before build/run automatically
# will run the server on http://localhost:7878
cargo run 
```

### Try it
Open a browser (or Postman) and call (GET):
> http://localhost:7878/users

## Extend

In order to extend this implementation, just add new controller methods to the `src/controller` module.
Expose the new method in `src/controller/mod.rs`.

Then register the method in the `src/main.rs`.

This implementation is based on the `mysql` and `serde` module. Everything should come with rust.

## Known Issues

- [ ] Multi Threading is not implemented
- [ ] The 404 error should be in JSON or HTML
- [ ] All JSON REST API paths shoulb start with `/api/v1`
- [ ] URL query and params handling is not implemented
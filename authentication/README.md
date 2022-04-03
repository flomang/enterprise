### auth

A basic auth api. Features:
* login
* logout
* register
* invite

### migrations

This project uses diesel to manage the DB.

To create migrations:
```
diesel migration generate create_tables
```

To run migrations:
```
diesel setup
diesel migration run 
diesel migration revert
```

To create migrations:
```
diesel migration generate create_tables
```
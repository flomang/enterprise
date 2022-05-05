### auth

A basic auth api. Features:
* login
* logout
* register
* invite

### stack
* actix-web
* actix-identity
* diesel

### diesel migrations

To create migrations:
```
diesel migration generate create_tables
```

To run migrations:
```
diesel setup
```
or
```
diesel migration run 
```
or
```
diesel migration revert
```
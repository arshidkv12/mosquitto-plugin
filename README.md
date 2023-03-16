
# Mosquitto Dynamic Security Plugin

Mosquitto authentication and ACL (Access control list) checking plugin. It is working based on MYSQL database.
## Configuration

#### How to install?

Download plugin.so file and load .conf file as follows
```mosquitto -c mosquitto.conf```

The mosquitto.conf file
```plugin path/to/plugin.so```

#### Configure database

Download mosquitto.sql file and import to your mysql by phpmyadmin. Then create .env file and add database credentials.

``` 
DB_HOST=localhost
DB_PORT=3306
DB_DATABASE=mosquitto
DB_USERNAME=root
DB_PASSWORD="Pass"
```

## Documentation

Authentication is working based on MYSQL users table. This plugin is checking username and password. Add username and password in users table.

ACL: Add access details acls table.


| column  | Value  | Permission |
| ------- | ------ | ---------- |
| rw      | 1      |    READ    |
| rw      | 2      |   WRITE    |
| rw      | 4      | SUBSCRIBE  |
| rw      | 8      | UNSUBSCRIBE|



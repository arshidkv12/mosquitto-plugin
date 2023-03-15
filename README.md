
# Mosquitto Dynamic Security Plugin

Mosquitto authentication and ACL (Access control list (ACL) checking plugin. It is working based on MYSQL database.
## Configuration

#### How to install?

Download plugin.so file and load .conf file as follows
```mosquitto -c mosquitto.conf```

#### Configure database

Download mosquitto.sql file and import to your mysql by phpmyadmin. Then create .env file and add database credentials.

```DB_HOST=localhost
DB_PORT=3306
DB_DATABASE=mosquitto
DB_USERNAME=root
DB_PASSWORD="Pass"```

## Documentation

Authentication is working based on MYSQL users table. This plugin is checking username and password. Add username and password in users table.

ACL: Add access detils acls table.

rw column 1 for MOSQ_ACL_READ permission
rw column 2 for MOSQ_ACL_WRITE permission
rw column 4 for MOSQ_ACL_SUBSCRIBE permission
rw column 8 for MOSQ_ACL_UNSUBSCRIBE permission



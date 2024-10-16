
# Mosquitto Dynamic Security Plugin

Mosquitto authentication and ACL (Access control list) checking plugin. It is working based on MYSQL database.
## Configuration

#### How to install?

Download **mosquitto-auth.so** file and load .conf file as follows
```mosquitto -c mosquitto.conf```

**[Download](https://phpbolt.com/wp-content/uploads/2023/03/mosquitto-auth.zip)**

The mosquitto.conf file

```plugin path/to/mosquitto-auth.so```

Feel free to PM: https://ciphercoin.com/contact. 
I can help you to configure. 

#### Configure the MySQL Database

Download mosquitto.sql file and import it to your mysql by phpmyadmin. Then create `.env` file in root folder (mosquitto executing folder) and add database credentials.

``` 
DB_HOST=localhost
DB_PORT=3306
DB_DATABASE=mosquitto
DB_USERNAME=root
DB_PASSWORD="Pass"
```

## Documentation

Authentication is working based on MYSQL users table. This plugin is checking username and password. Add username and password in the users table.

ACL: Add access details acls table.


| column  | Value  | Permission |
| ------- | ------ | ---------- |
| rw      | 1      |    READ    |
| rw      | 2      |   WRITE    |
| rw      | 4      | SUBSCRIBE  |
| rw      | 8      | UNSUBSCRIBE|




## How To Test? 
Download [mosquitto.sql](https://github.com/arshidkv12/mosquitto-plugin/blob/main/mosquitto.sql) then import it by phpmyadmin. 

Step 1: ```mosquitto -c mosquitto.conf```

Step 2: Open the new terminal then enter `mosquitto_sub -t "test/su" -u arshid -P Pass@123`
 
Step 3: Open the new terminal then enter `mosquitto_pub -t "test/su" -m "This is a message !" -u arshid -P Pass@123` 

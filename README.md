
# Mosquitto Dynamic Security (Mosquitto Auth Plugin)

Mosquitto authentication and ACL (Access control list) checking plugin. It is working based on MYSQL database.

Feel free to email me at info@mailmug.net — I’m available to customize `mosquitto-plugin` for you.

## Configuration

#### How to install?

Download **mosquitto-auth.so** file and load .conf file as follows
```mosquitto -c mosquitto.conf```

**[Download](https://phpbolt.com/wp-content/uploads/2023/03/mosquitto-auth.zip)**

Move ```mosquitto-auth/linux-64/mosquitto_auth.so``` to ```/etc/mosquitto/mosquitto_auth.so```


```sudo chown mosquitto:mosquitto /etc/mosquitto/mosquitto_auth.so```    
```sudo chmod 755 /etc/mosquitto/mosquitto_auth.so```

Then edit `mosquitto.conf` file from `/etc/mosquitto/`

Add the following code in **/etc/mosquitto/mosquitto.conf** file.

### The mosquitto.conf file

```
plugin /etc/mosquitto/mosquitto-auth.so
listener 1883
```

### How to edit the environment (.env) file location? 
Edit systemd Linux services environment .env file location. 
Enter the following code in the terminal.

```sudo systemctl edit mosquitto```

```
[Service]
EnvironmentFile=/etc/mosquitto/.env
```

#### Configure the MySQL Database

Download **mosquitto.sql** file and import it to your mysql by phpmyadmin. Then add database credentials in **/etc/mosquitto/.env** file.

``` 
DB_HOST=localhost
DB_PORT=3306
DB_DATABASE=mosquitto
DB_USERNAME=root
DB_PASSWORD="Pass"
```

Feel free to PM: https://ciphercoin.com/contact. 
I can help you to configure. 
Skype: arshidkv12 
Discord : arshidkv12

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

Step 1:  `service mosquitto restart`

Step 2: Open the new terminal then enter `mosquitto_sub -t "test/su" -u arshid -P Pass@123`
 
Step 3: Open the new terminal then enter `mosquitto_pub -t "test/su" -m "This is a message !" -u arshid -P Pass@123` 

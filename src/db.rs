use mysql::{ Pool };
use dotenv::dotenv;
use std::{ env };

#[allow(dead_code)]

pub fn connect() -> Pool {
    dotenv().ok();

    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let db_port = env::var("DB_PORT").expect("DB_PORT host must be set");
    let db_database = env::var("DB_DATABASE").expect("DB_DATABASE host must be set");
    let db_username = env::var("DB_USERNAME").expect("DB_USERNAME host must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD host must be set");
  
    let url:String  = format!("mysql://{}:{}@{}:{}/{}", 
                        db_username, db_password, db_host, db_port, db_database 
                      );
    let pool = Pool::new(url.as_str() ).unwrap();
    pool
  }
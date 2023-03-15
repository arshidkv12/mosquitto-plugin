use mysql::{ Pool, PooledConn };
use dotenv::dotenv;
use std::{ env };
use std::sync::Mutex;
use lazy_static;


lazy_static::lazy_static! {
    static ref POOL: Mutex<Pool> = Mutex::new(Pool::new( get_database_url().as_str() ).unwrap());
}


fn get_database_url() -> String {
    dotenv().ok();
    let db_host : String = env::var("DB_HOST").expect("DB_HOST must be set");
    let db_port : String = env::var("DB_PORT").expect("DB_PORT must be set");
    let db_database : String = env::var("DB_DATABASE").expect("DB_DATABASE must be set");
    let db_username : String = env::var("DB_USERNAME").expect("DB_USERNAME must be set");
    let db_password : String = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
   
    let db_url:String = format!("mysql://{}:{}@{}:{}/{}", 
                db_username.as_str(), db_password.as_str(), db_host.as_str(), db_port.as_str(), db_database.as_str() 
              );
    
    db_url
}

pub fn get_conn() -> PooledConn {
  POOL.lock().unwrap().get_conn().unwrap()
}

pub fn get_pool() -> Pool {
  POOL.lock().unwrap().clone()
}


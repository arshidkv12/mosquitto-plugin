use mysql::{Pool, PooledConn, params::Params};
use dotenv::dotenv;
use std::env;


pub fn connect() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  
    let url = "mysql://root:Pass@123@localhost/mosquitto";
    let pool = Pool::new(url).unwrap();
    pool
  }
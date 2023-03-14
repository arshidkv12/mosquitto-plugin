#[path = "db.rs"]
mod db;
#[path = "constants.rs"]
mod constants;

use std::ffi::{c_int, c_void};
use mosquitto_sys::{
    mosquitto_evt_basic_auth, 
    mosquitto_client_username,
    mosquitto_evt_acl_check
};
use mysql::{ Pool, params };
use mysql::prelude::Queryable;
use constants::*;

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    id: u64,
    username: String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Acl {
    id: u64,
    username: String,
    topic: String,
    rw: u64,
}


pub extern "C" fn basic_auth_callback(
    _event: c_int, 
    event_data: *mut c_void, 
    _userdata: *mut c_void
) -> c_int{
    unsafe{
        let ed =  &mut *(event_data as *mut mosquitto_evt_basic_auth);
        // let username = std::ffi::CStr::from_ptr(ed.username);
        let _username = mosquitto_client_username(ed.client);
        let username = std::ffi::CStr::from_ptr(_username);
        let password = std::ffi::CStr::from_ptr(ed.password);

        let pool: Pool = db::connect();
        let mut conn = pool.get_conn().expect("Failed to get database connection");
        let query = "
                    SELECT id, username FROM users 
                    WHERE username = :username AND password = :password
                "; 

        let result = conn.exec_map(
            query,params!{ 
                "username" => username.to_str().unwrap(),
                "password" => password.to_str().unwrap()
            },
            |(id, username)| {
                User { id, username }
            },
        );


        match result {
            Ok(val) => {
                if val.len() > 0 { 
                    return MOSQ_ERR_SUCCESS;
                }else{ 
                    return MOSQ_ERR_AUTH;
                }
            },
            Err(_err) => {
                return MOSQ_ERR_AUTH;
            }
        }
        
    }

}


pub extern "C" fn acl_check_callback(
    _event: c_int, 
    event_data: *mut c_void, 
    _userdata: *mut c_void
) -> c_int{
    unsafe{
        let ed =  &mut *(event_data as *mut mosquitto_evt_acl_check);
        let _username = mosquitto_client_username(ed.client);
        let username = std::ffi::CStr::from_ptr(_username); 
        let topic = std::ffi::CStr::from_ptr(ed.topic); 

        let pool: Pool = db::connect();
        let mut conn = pool.get_conn().expect("Failed to get database connection");
        let query = " 
                    SELECT id, username, topic, rw FROM acls 
                    WHERE username = :username AND topic = :topic
                    LIMIT 1
                ";
        let result = conn.exec_map(
            query,params!{ 
                "username" => username.to_str().unwrap(),
                "topic" => topic.to_str().unwrap()
            },
            |(id, username, topic, rw )| {
                Acl { id, username, topic, rw }
            },
        );

        let _f  = result.as_ref().to_owned();
        match _f {
            Ok(v) =>{
                eprintln!("+++==={:?}", v[0].username);
            },
            Err(_err) => {
            }
        }
        
        match result {
            Ok(val) => {
                if val.len() > 0 { 
                    MOSQ_ERR_SUCCESS
                }else{ 
                    MOSQ_ERR_ACL_DENIED
                }
            },
            Err(_err) => {
                MOSQ_ERR_ACL_DENIED
            }
        }
        
    }

}

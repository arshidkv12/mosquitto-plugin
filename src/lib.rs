use std::ffi::{c_int, c_void};
use mosquitto_sys::{
     mosquitto_callback_register, 
     mosquitto_callback_unregister, 
     mosquitto_plugin_id_t, 
     mosquitto_opt, 
     mosquitto_evt_basic_auth
};
use mysql::{ Pool, PooledConn, params };
use mysql::prelude::Queryable;

mod db;

const  MOSQ_ERR_SUCCESS : c_int = 0;
const  MOSQ_EVT_BASIC_AUTH : c_int = 3;
const  MOSQ_ERR_AUTH : c_int = 11;
static mut MOSQ_PID: *mut mosquitto_plugin_id_t = std::ptr::null_mut();

#[derive(Debug)]
struct User {
    id: u64,
    username: String,
}

#[no_mangle]
pub extern "C" fn mosquitto_plugin_version(
    _supported_version_count: c_int,
    _supported_versions: *const c_int,
) ->  c_int {
    5
}

extern "C" fn a_basic_auth_callback(
    _event: c_int, 
    event_data: *mut c_void, 
    _userdata: *mut c_void
) -> c_int{
    unsafe{
        let ed =  &mut *(event_data as *mut mosquitto_evt_basic_auth);
        let username = std::ffi::CStr::from_ptr(ed.username);
        let pool: Pool = db::connect();
        let mut conn = pool.get_conn().expect("Failed to get database connection");
        let query = "SELECT id, username FROM users WHERE username = :username";
        let result = conn.exec_map(
            query,params!{ "username" => username.to_bytes() },
            |(id, username)| {
                User { id, username }
            },
        );
        eprintln!("+++{}", username.to_str().unwrap());
        eprintln!("++++====+{:?}", result);
        
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



#[no_mangle]
pub extern "C" fn mosquitto_plugin_init(
    identifier: *mut mosquitto_plugin_id_t,
    _userdata: *mut *mut c_void,
    _options: *const mosquitto_opt,
    _option_count: c_int,
) -> c_int {

    return unsafe{ 
        MOSQ_PID = identifier;
        mosquitto_callback_register(identifier, MOSQ_EVT_BASIC_AUTH, Some(a_basic_auth_callback), std::ptr::null(), std::ptr::null_mut())
    }
}

#[no_mangle]
pub extern  "C" fn  mosquitto_plugin_cleanup(
    _user_data: *mut c_void, 
    _opts: *const mosquitto_opt, 
    _opt_count: c_int
) -> c_int {
    return unsafe {
        mosquitto_callback_unregister(MOSQ_PID, MOSQ_EVT_BASIC_AUTH, Some(a_basic_auth_callback), std::ptr::null())
    }
}



mod db;
mod auth;
mod constants;

use std::ffi::{c_int, c_void};
use mosquitto_sys::{
     mosquitto_callback_register, 
     mosquitto_callback_unregister, 
     mosquitto_plugin_id_t, 
     mosquitto_opt
};
use constants::{
    MOSQ_ERR_SUCCESS,
    MOSQ_EVT_BASIC_AUTH,
    MOSQ_EVT_ACL_CHECK
};
use auth::{
    basic_auth_callback, 
    acl_check_callback
};


static mut MOSQ_PID: *mut mosquitto_plugin_id_t = std::ptr::null_mut();


#[no_mangle]
pub extern "C" fn mosquitto_plugin_version(
    _supported_version_count: c_int,
    _supported_versions: *const c_int,
) ->  c_int {
    5
}




#[no_mangle]
pub extern "C" fn mosquitto_plugin_init(
    identifier: *mut mosquitto_plugin_id_t,
    _userdata: *mut *mut c_void,
    _options: *const mosquitto_opt,
    _option_count: c_int,
) -> c_int {

    unsafe{ 
        MOSQ_PID = identifier;
        mosquitto_callback_register(identifier, MOSQ_EVT_BASIC_AUTH, Some(basic_auth_callback), std::ptr::null(), std::ptr::null_mut());
        mosquitto_callback_register(identifier, MOSQ_EVT_ACL_CHECK, Some(acl_check_callback), std::ptr::null(), std::ptr::null_mut());
    }
    MOSQ_ERR_SUCCESS
}

#[no_mangle]
pub extern  "C" fn  mosquitto_plugin_cleanup(
    _user_data: *mut c_void, 
    _opts: *const mosquitto_opt, 
    _opt_count: c_int
) -> c_int {
    unsafe {
        mosquitto_callback_unregister(MOSQ_PID, MOSQ_EVT_BASIC_AUTH, Some(basic_auth_callback), std::ptr::null());
        mosquitto_callback_unregister(MOSQ_PID, MOSQ_EVT_ACL_CHECK, Some(acl_check_callback), std::ptr::null());
    }
    MOSQ_ERR_SUCCESS
}

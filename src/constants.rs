use std::ffi::{c_int};

#[allow(dead_code)]
pub const  MOSQ_ERR_SUCCESS : c_int = 0;
pub const  MOSQ_EVT_ACL_CHECK : c_int = 2;
pub const  MOSQ_EVT_BASIC_AUTH : c_int = 3;
pub const  MOSQ_ERR_AUTH : c_int = 11;
pub const  MOSQ_ERR_ACL_DENIED : c_int = 12;

pub const MOSQ_ACL_NONE : c_int =  0;
pub const MOSQ_ACL_READ : c_int =  1;
pub const MOSQ_ACL_WRITE : c_int =  2;
pub const MOSQ_ACL_SUBSCRIBE : c_int =  4;
pub const MOSQ_ACL_UNSUBSCRIBE : c_int =  8;
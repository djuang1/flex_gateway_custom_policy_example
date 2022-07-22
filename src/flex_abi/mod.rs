
use std::ptr::{null_mut};

type Bytes = Vec<u8>;

#[repr(C)]
enum Status {
    Ok = 0,
    NotFound = 1,
    BadArgument = 2,
    ParseFailure = 4,
    Empty = 7,
    CasMismatch = 8,
    InternalFailure = 10,
}

extern "C" {
    fn proxy_call_foreign_function(
        name_data: *const u8,
        name_size: usize,
        args_data: *const u8,
        args_size: usize,
        return_data: *mut *mut u8,
        return_size: *mut usize,
    ) -> Status;
}

fn call(
    name: &String,
    args: &[String]
) -> Result<Option<Bytes>, Status> {
    let mut return_data: *mut u8 = null_mut();
    let mut return_size: usize = 0;

    let arg = args.join("|");

    unsafe {
        match proxy_call_foreign_function(
            name.as_ptr(),
            name.len(),
            arg.as_ptr(),
            arg.len(),
            &mut return_data,
            &mut return_size,
        ) {
            Status::Ok => {
                if !return_data.is_null() {
                    Ok(Some(Vec::from_raw_parts(
                        return_data,
                        return_size,
                        return_size,
                    )))
                } else {
                    Ok(None)
                }
            }
            Status::NotFound => Ok(None),
            status => panic!("unexpected status: {}", status as u32),
        }
    }
}

fn log(level: String, msg: String) {
    call(&String::from("flex_log"), &[level, msg]);
}

pub fn log_debug(msg: String) {
    log(String::from("debug"), msg)
}

pub fn log_info(msg: String) {
    log(String::from("info"), msg)
}

pub fn log_warn(msg: String) {
    log(String::from("warn"), msg)
}

pub fn log_error(msg: String) {
    log(String::from("error"), msg)
}

pub fn service_create(name: String, ns: String, uri: String) -> bool {
	if call(&"flex_service_create".to_string(), &[name, ns, uri]).is_ok() {
        true
    } else {
        false
    }
}

pub fn get_env(name: String) -> Option<String> {
	match call(&"flex_get_env".to_string(), &[name]) {
        Ok(None) => None,
        Ok(Some(v)) => {
            match String::from_utf8(v) {
                Ok(s) => Some(s),
                Err(_) => None
            }
        }
        Err(_) =>None
    }
}

use std::ffi::{c_char, CStr, CString};

pub use reqwest;

fn c(s: String) -> *const c_char {
    CString::new(s.clone()).unwrap().into_raw()
}

unsafe fn rs(s: *const c_char) -> String {
    CStr::from_ptr(s)
        .to_str()
        .expect("failed to convert C const char* to rust")
        .to_string()
}

#[no_mangle]
pub extern "C" fn get() {
    println!("Hello world!\n");
}

#[repr(C)]
pub struct GET {
    pub success: bool,
    pub content: *const c_char,
}

#[no_mangle]
pub extern "C" fn testget(url: *const c_char) -> GET {
    unsafe {
        match reqwest::blocking::get(rs(url)) {
            Ok(body) => GET {
                success: true,
                content: c(body.text().unwrap()),
            },
            Err(_) => GET {
                success: false,
                content: c("".to_string()),
            },
        }
    }
}
pub extern "C" fn testpost(url: *const c_char, mes: *const c_char) {
    unsafe {
        (reqwest::blocking::Client::new())
            .post(rs(url))
            .body(rs(mes))
            .send()
            .unwrap();
    }
}

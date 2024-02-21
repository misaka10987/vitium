use std::ffi::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use reqwest;

unsafe fn r(c : *const c_char) -> String{
    unsafe{ CStr::from_ptr(c).to_str().expect("failed to convert C const char* to rust String").to_string() }
}
fn c(r : String) -> *const c_char{
    CString::new(r.clone()).unwrap().into_raw()
}

#[repr(C)]
pub struct Conj{
    pub gotta : i32,
    pub resp : *const c_char
}

fn errana(e : reqwest::Error) -> Conj{
    Conj{
        gotta : 20000 + e.status(),
        resp : c(e.url())
    }
}

#[no_mangle]
pub extern "C" fn get(url : *const c_char) -> Conj{
    unsafe{
        let txt = reqwest::blocking::get(r(url));
        if txt.is_ok(){
            Conj{
                gotta : 20200,
                resp : c(txt.unwrap().text().unwrap())
            }
        }
        else{
            errana(txt.unwrap_err())
        }
    }
}

#[no_mangle]
pub extern "C" fn post(url : *const c_char,mes : *const c_char) -> Conj{
    unsafe {
        let res = reqwest::blocking::Client::new().post(r(url)).body(r(mes)).send();
        if res.is_ok(){
            Conj{
                gotta : 20201,
                resp : c("The message has sent sucessfully.".to_string())
            }
        }
        else {
            errana(res.unwrap_err())
        }
    }
}
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

#[no_mangle]
pub extern "C" fn get(url : *const c_char) -> Conj{
    unsafe{
        let txt = reqwest::blocking::get(r(url));
        if txt.is_ok(){
            Conj{
                gotta : 20200,
                resp : c(txt.unwrap().text().unwrap())
            }
        }else{
            let e = txt.err();
            if e.is_some() {
                Conj{
                    gotta : 20599,
                    resp : c("time out".to_string())
                }
            }else if e.is_none(){
                Conj{
                    gotta : 20303,
                    resp : c("a".to_string())
                }
            }
            else{
                Conj{
                    gotta : 1,
                    resp : c("Not OK".to_string())
                }
            }
            
        }
    }
}
pub extern "C" fn post(url : *const c_char,mes : *const c_char){
    unsafe {reqwest::blocking::Client::new().post(r(url)).body(r(mes)).send().unwrap();}
}
// use serde_json::to_string as json;
// use std::ffi::CString;
// use vitium_common::request::Req;

#[repr(C)]
pub enum TestEnum {
    A(TestStruct),
}

#[repr(C)]
pub struct TestStruct {
    pub a: i32,
}

#[no_mangle]
pub extern "C" fn test_fn(a: i32, b: i32) -> TestEnum {
    TestEnum::A(TestStruct { a: a + b })
}

// #[no_mangle]
// pub extern "C" fn req_se(req: Req) -> *const i8 {
//     let cstr=CString::new(json(&req).unwrap()).unwrap();
//     cstr.as_ptr()
// }

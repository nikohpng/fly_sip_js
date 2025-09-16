#![deny(clippy::all)]
mod fly_sip;
use napi_derive::napi;
use std::ffi::CString;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
pub fn fly_sip_init_sdk_js(internal: String, 
        external: String, 
        app_id: String, 
        app_key: String, 
        username: String, 
        password: String, 
        uesr_id: String, 
        is_https: i32) -> i32 {
  let internal_c =  CString::new(internal).unwrap();
  let external_c = CString::new(external).unwrap();
  let app_id_c= CString::new(app_id).unwrap();
  let app_key_c = CString::new(app_key).unwrap();
  let username_c = CString::new(username).unwrap();
  let password_c = CString::new(password).unwrap();
  let user_id_c = CString::new(uesr_id).unwrap();
  unsafe {
    return fly_sip::fly_sip_init_sdk(internal_c.as_ptr(),
       external_c.as_ptr(),
        app_id_c.as_ptr(), 
        app_key_c.as_ptr(), 
        username_c.as_ptr(), 
        password_c.as_ptr(), 
        user_id_c.as_ptr(), 
        is_https)
  }
}

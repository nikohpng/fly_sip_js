use std::os::raw::c_char;

#[link(name = "libfly_sip")]
unsafe extern "C" {
    pub unsafe fn fly_sip_init_sdk(internal: *const c_char, 
        external: *const c_char, 
        appId: *const c_char, 
        appKey: *const c_char, 
        userName: *const c_char, 
        password: *const c_char, 
        uesrId: *const c_char, 
        isHttps: i32) -> i32;
}
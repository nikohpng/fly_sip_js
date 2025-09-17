use std::os::raw::c_char;

#[cfg(target_os = "macos")]
#[link(name = "FlySip", kind = "framework")]
extern "C" {
    pub fn fly_sip_init_sdk(
        internal: *const c_char,
        external: *const c_char,
        appId: *const c_char,
        appKey: *const c_char,
        userName: *const c_char,
        password: *const c_char,
        userId: *const c_char,
        isHttps: i32,
    ) -> i32;
}

#[cfg(target_os = "windows")]
#[link(name = "libfly_sip")]
extern "C" {
    pub fn fly_sip_init_sdk(
        internal: *const c_char,
        external: *const c_char,
        appId: *const c_char,
        appKey: *const c_char,
        userName: *const c_char,
        password: *const c_char,
        userId: *const c_char,
        isHttps: i32,
    ) -> i32;
}
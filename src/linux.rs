use crate::User;
use std::ffi::CStr;

impl User {
    pub fn current() -> Self {
        let mut passwd: libc::passwd = unsafe { std::mem::zeroed() };
        let mut result: *mut libc::passwd = std::ptr::null_mut();
        let mut buffer = vec![0 as libc::c_char; 1024];
        loop {
            let status = unsafe {
                libc::getpwuid_r(
                    libc::geteuid(),
                    &mut passwd,
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    &mut result,
                )
            };
            if status == libc::ERANGE {
                buffer.resize(buffer.len() * 2, 0);
                continue;
            }
            break;
        }
        if result.is_null() {
            return Self::new("", "");
        }
        let username = unsafe { CStr::from_ptr(passwd.pw_name) }
            .to_string_lossy()
            .into_owned();
        let full_name = unsafe { CStr::from_ptr(passwd.pw_gecos) }
            .to_string_lossy()
            .split(',')
            .next()
            .unwrap_or_default()
            .to_string();
        Self::new(username, full_name)
    }
}

use crate::User;
use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

impl User {
    /// Returns the current user, using `getpwuid_r` for the effective user
    /// ID.
    ///
    /// The username comes from the passwd entry's `pw_name` field, the home
    /// directory from `pw_dir`, and the ID from `pw_uid`. The full name
    /// comes from the first comma-separated field of `pw_gecos` (the
    /// `finger`-style GECOS convention: `"Full Name,Room,Work
    /// Phone,Home Phone"`). If that field is empty, [`User::full_name`]
    /// falls back to the username.
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
            return Self::new("", "", "", "");
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
        Self::new(
            username.clone(),
            if full_name.is_empty() {
                username
            } else {
                full_name
            },
            PathBuf::from(OsStr::from_bytes(
                unsafe { CStr::from_ptr(passwd.pw_dir) }.to_bytes(),
            )),
            passwd.pw_uid.to_string(),
        )
    }
}

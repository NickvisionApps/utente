use crate::User;
use core::ffi::c_void;
use std::path::PathBuf;
use windows::Win32::Foundation::{HLOCAL, LocalFree};
use windows::Win32::NetworkManagement::NetManagement::{
    NetApiBufferFree, NetUserGetInfo, USER_INFO_10,
};
use windows::Win32::Security::Authentication::Identity::{GetUserNameExW, NameDisplay};
use windows::Win32::Security::Authorization::ConvertSidToStringSidW;
use windows::Win32::Security::{LookupAccountNameW, PSID, SID_NAME_USE};
use windows::Win32::System::WindowsProgramming::GetUserNameW;
use windows::core::{HSTRING, PCWSTR, PWSTR};

impl User {
    /// Returns the current user.
    ///
    /// The username comes from `GetUserNameW`. The full name is looked up
    /// via `GetUserNameExW` with the `NameDisplay` format, which requires
    /// the account to be domain-joined; if that fails or returns an empty
    /// string, `NetUserGetInfo` (level 10) is used instead, which reads the
    /// full name from the local user database and works for local/workgroup
    /// accounts. If neither lookup produces a non-empty full name,
    /// [`User::full_name`] falls back to the username. The home directory
    /// comes from the `USERPROFILE` environment variable. The ID is the
    /// user's SID, looked up via `LookupAccountNameW` and formatted as a
    /// string with `ConvertSidToStringSidW`.
    pub fn current() -> Self {
        let mut username_buffer = [0u16; 257];
        let mut username_len = username_buffer.len() as u32;
        unsafe { GetUserNameW(Some(PWSTR(username_buffer.as_mut_ptr())), &mut username_len) }.ok();
        let username =
            String::from_utf16_lossy(&username_buffer[..username_len.saturating_sub(1) as usize]);
        let mut display_buffer = [0u16; 1024];
        let mut display_len = display_buffer.len() as u32;
        let display_name = unsafe {
            GetUserNameExW(
                NameDisplay,
                Some(PWSTR(display_buffer.as_mut_ptr())),
                &mut display_len,
            )
        }
        .then(|| String::from_utf16_lossy(&display_buffer[..display_len as usize]))
        .filter(|s| !s.is_empty());
        let full_name = display_name.or_else(|| {
            let mut info: *mut u8 = std::ptr::null_mut();
            let status = unsafe {
                NetUserGetInfo(
                    None::<&PCWSTR>,
                    &HSTRING::from(username.as_str()),
                    10,
                    &mut info,
                )
            };
            if status != 0 {
                return None;
            }
            let full_name = unsafe {
                (*(info as *const USER_INFO_10))
                    .usri10_full_name
                    .to_string()
            }
            .ok()
            .filter(|s| !s.is_empty());
            unsafe { NetApiBufferFree(Some(info as *const c_void)) };
            full_name
        });
        let mut sid_buffer = [0u8; 256];
        let mut sid_size = sid_buffer.len() as u32;
        let mut domain_buffer = [0u16; 256];
        let mut domain_size = domain_buffer.len() as u32;
        let mut sid_use = SID_NAME_USE::default();
        let id = unsafe {
            LookupAccountNameW(
                None::<&PCWSTR>,
                &HSTRING::from(username.as_str()),
                Some(PSID(sid_buffer.as_mut_ptr() as *mut c_void)),
                &mut sid_size,
                Some(PWSTR(domain_buffer.as_mut_ptr())),
                &mut domain_size,
                &mut sid_use,
            )
        }
        .ok()
        .and_then(|_| {
            let mut sid_string = PWSTR::null();
            unsafe {
                ConvertSidToStringSidW(
                    PSID(sid_buffer.as_mut_ptr() as *mut c_void),
                    &mut sid_string,
                )
            }
            .ok()?;
            let string = unsafe { sid_string.to_string() }.ok();
            unsafe { LocalFree(Some(HLOCAL(sid_string.0 as *mut c_void))) };
            string
        })
        .unwrap_or_default();
        Self::new(
            username.clone(),
            full_name.unwrap_or(username),
            PathBuf::from(std::env::var("USERPROFILE").unwrap_or_default()),
            id,
        )
    }
}

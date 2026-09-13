use crate::User;
use core::ffi::c_void;
use windows::Win32::NetworkManagement::NetManagement::{
    NetApiBufferFree, NetUserGetInfo, USER_INFO_10,
};
use windows::Win32::Security::Authentication::Identity::{GetUserNameExW, NameDisplay};
use windows::Win32::System::WindowsProgramming::GetUserNameW;
use windows::core::{HSTRING, PCWSTR, PWSTR};

impl User {
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
        Self::new(username.clone(), full_name.unwrap_or(username))
    }
}

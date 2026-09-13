use crate::User;
use objc2_foundation::{NSFullUserName, NSHomeDirectory, NSUserName};
use std::path::PathBuf;

impl User {
    /// Returns the current user, using `NSUserName`, `NSFullUserName`, and
    /// `NSHomeDirectory` from `Foundation`, plus `geteuid` for the user ID.
    ///
    /// If the account has no full name set, [`User::full_name`] falls back
    /// to the username.
    pub fn current() -> Self {
        let username = NSUserName().to_string();
        let full_name = NSFullUserName().to_string();
        Self::new(
            username.clone(),
            if full_name.is_empty() {
                username
            } else {
                full_name
            },
            PathBuf::from(NSHomeDirectory().to_string()),
            unsafe { libc::geteuid() }.to_string(),
        )
    }
}

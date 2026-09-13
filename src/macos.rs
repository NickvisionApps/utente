use crate::User;
use objc2_foundation::{NSFullUserName, NSUserName};

impl User {
    /// Returns the current user, using `NSUserName` and `NSFullUserName`
    /// from `Foundation`.
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
        )
    }
}

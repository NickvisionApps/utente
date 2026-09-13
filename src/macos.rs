use crate::User;
use objc2_foundation::{NSFullUserName, NSUserName};

impl User {
    pub fn current() -> Self {
        Self::new(NSUserName().to_string(), NSFullUserName().to_string())
    }
}

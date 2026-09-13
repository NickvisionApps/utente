use crate::User;
use objc2_foundation::{NSFullUserName, NSUserName};

impl User {
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

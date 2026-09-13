use crate::User;

impl User {
    pub fn current() -> Self {
        Self::new("", "")
    }
}

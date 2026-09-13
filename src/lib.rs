#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
compile_error!("armoire only supports Windows, macOS, and Linux");

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

pub struct User {
    username: String,
    full_name: String,
}

impl User {
    fn new(username: impl Into<String>, full_name: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            full_name: full_name.into(),
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn full_name(&self) -> &str {
        &self.full_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_current() {
        let user = User::current();
        assert!(!user.username().is_empty());
        assert!(!user.full_name().is_empty());
    }
}

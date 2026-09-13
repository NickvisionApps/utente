//! Cross-platform retrieval of the current operating system user's username,
//! full (display) name, home directory, and stable identifier.
//!
//! The core type is [`User`], obtained via [`User::current`].
//!
//! # Example
//!
//! ```
//! use utente::User;
//!
//! let user = User::current();
//! assert!(!user.username().is_empty());
//! assert!(!user.full_name().is_empty());
//! ```
//!
//! # Platform support
//!
//! Utente supports Windows, macOS, and Linux targets. See [`User::current`]
//! for the OS APIs used on each platform.

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
compile_error!("utente only supports Windows, macOS, and Linux");

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The current operating system user's username, full (display) name, home
/// directory, and stable identifier.
///
/// # Examples
///
/// ```
/// use utente::User;
///
/// let user = User::current();
/// assert!(!user.username().is_empty());
/// assert!(!user.full_name().is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    username: String,
    full_name: String,
    home_directory: PathBuf,
    id: String,
}

impl User {
    fn new(
        username: impl Into<String>,
        full_name: impl Into<String>,
        home_directory: impl Into<PathBuf>,
        id: impl Into<String>,
    ) -> Self {
        Self {
            username: username.into(),
            full_name: full_name.into(),
            home_directory: home_directory.into(),
            id: id.into(),
        }
    }

    /// Returns the user's username (short login name).
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Returns the user's full (display) name.
    ///
    /// This is never empty: on every supported platform, [`User::current`]
    /// falls back to the username if no full name is set for the account.
    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    /// Returns the user's home directory.
    pub fn home_directory(&self) -> &Path {
        &self.home_directory
    }

    /// Returns a stable identifier for the user: the UID on Linux and
    /// macOS, or the SID (e.g. `"S-1-5-21-..."`) on Windows.
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_current() {
        let user = User::current();
        println!("{:?}", user);
        assert!(!user.username().is_empty());
        assert!(!user.full_name().is_empty());
        assert!(user.home_directory().exists());
        assert!(!user.id().is_empty())
    }
}

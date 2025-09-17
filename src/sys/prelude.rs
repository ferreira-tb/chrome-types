pub use super::Browser;
pub use super::storage::*;

#[cfg(feature = "firefox")]
pub use super::BROWSER;

#[cfg(not(feature = "firefox"))]
pub use super::CHROME;

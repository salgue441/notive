//! Event handlers.

pub mod navigation;
pub mod window;

#[cfg(test)]
mod tests;

pub use navigation::handle_navigation;

// Re-export for tests
#[cfg(test)]
pub use navigation::{is_oauth_url, should_open_externally};

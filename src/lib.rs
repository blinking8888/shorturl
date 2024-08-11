//! `shorturl`: URL Shortener Service

/// Main Application interface
pub mod app;
pub mod database;
pub mod error;
pub mod shorturl;

/// Make it convenient for the user to use
pub use app::{App, Config};
pub use database::Database;

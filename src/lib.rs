//! `shorturl`: URL Shortener Service

/// Main Application interface
pub mod app;
mod database;
mod shorturl;

/// Make it convenient for the user to use
pub use app::{App, Config};

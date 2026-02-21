// Public surface for integration and e2e tests.
// The binary entry point (main.rs) owns the TUI plumbing; this crate exposes
// only the modules needed for testing without pulling in the terminal setup.
pub mod api;
pub mod app;
pub mod cli;
pub mod config;
pub mod models;
pub mod ui;

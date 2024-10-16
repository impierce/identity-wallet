pub mod command;
pub mod error;
pub mod persistence;
pub mod state;
pub mod stronghold;
pub mod subject;

// Re-exports
pub use oid4vc::{oid4vc_core, oid4vc_manager, oid4vci, oid4vp, siopv2};

// This folder is where the main backend rust code lives together with all the business logic.
// The folder state is where our appstate and it's features are defined, completely according to the redux pattern.
// The command.rs holds the functions through which the front and backend comminicate using actions and reducers.
// The error.rs defines our app_error types, implemented throughout the code using the thiserror crate.
// The persistence.rs is where we define our app persistence functions.
// The stronghold.rs is where we implement the stronghold library for our app, which is used to store sensitive data.

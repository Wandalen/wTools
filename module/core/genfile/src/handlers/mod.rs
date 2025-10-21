//! Command handlers for genfile CLI
//!
//! This module contains the execution logic for all commands.
//! Each handler receives a `VerifiedCommand` and `ExecutionContext`,
//! executes the operation using `genfile_core`, and returns `OutputData` or `ErrorData`.

mod shared_state;

pub mod archive;
pub mod file;
pub mod parameter;
pub mod value;
pub mod content;
pub mod materialize;
pub mod pack;
pub mod info;

#![cfg_attr( doc, doc = include_str!("../spec.md") )]

//! Claude Code CLI Launcher
//!
//! General-purpose library for executing Claude Code CLI commands
//! programmatically with full parameter control.
//!
//! # Quick Start
//!
//! ```no_run
//! use claude_runner_core::ClaudeCommand;
//!
//! let cmd = ClaudeCommand::new()
//!   .with_working_directory( "/tmp/work" )
//!   .with_max_output_tokens( 200_000 );
//!
//! let output = cmd.execute()?;
//! println!( "{}", output.stdout );
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Architecture
//!
//! - **Single execution point:** All commands go through `execute()`
//! - **Builder pattern:** Configuration via chainable `with_*()` methods
//! - **Private fields:** Can't construct with struct literals
//! - **No session logic:** Pure execution, no state management
//!
//! # Migration: Old Way Impossible
//!
//! This crate enforces the new builder pattern at compile time. Old patterns
//! from the deprecated API are impossible:
//!
//! ## Old Pattern 1: Factory Method (Doesn't Exist)
//!
//! ```compile_fail
//! use claude_runner_core::ClaudeCommand;
//!
//! // ERROR: generate() method doesn't exist
//! let cmd = ClaudeCommand::generate("/tmp", "msg", 1000, Strategy::Fresh);
//! ```
//!
//! ## Old Pattern 2: Direct Construction (Fields Private)
//!
//! ```compile_fail
//! use claude_runner_core::ClaudeCommand;
//! use std::path::PathBuf;
//!
//! // ERROR: fields are private
//! let cmd = ClaudeCommand {
//!   working_directory: Some(PathBuf::from("/tmp")),
//!   max_output_tokens: Some(200_000),
//!   continue_conversation: false,
//!   message: None,
//!   args: vec![],
//! };
//! ```
//!
//! ## New Pattern: Builder Only
//!
//! ```no_run
//! use claude_runner_core::ClaudeCommand;
//!
//! // the ONLY way to construct ClaudeCommand
//! let output = ClaudeCommand::new()
//!   .with_working_directory( "/tmp" )
//!   .with_max_output_tokens( 200_000 )
//!   .execute()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! See [spec.md](../spec.md) for complete documentation.

mod command;
mod types;

pub use crate::command::ClaudeCommand;
pub use crate::types::{ ActionMode, ExecutionOutput, LogLevel };

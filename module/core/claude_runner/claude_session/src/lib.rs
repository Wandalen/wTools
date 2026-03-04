//! Session storage management for Claude Code.
//!
//! This crate provides session storage path resolution and continuation detection
//! for Claude Code conversations, enabling isolated and resumable AI-assisted
//! development sessions.
//!
//! # Core Concepts
//!
//! ## Session Isolation
//!
//! Sessions are isolated in hyphen-prefixed directories (`-{name}/`) that are
//! automatically excluded from git. Each session has its own conversation history
//! and working directory context.
//!
//! ## Session Detection
//!
//! Claude Code v2.0+ tracks sessions by execution directory in `~/.claude/projects/`.
//! The crate provides storage-based detection to determine if a session exists.
//!
//! ## Separation of Concerns
//!
//! This crate handles ONLY session storage management. For Claude Code execution,
//! use the `claude_runner_core` crate which provides a builder pattern API.
//!
//! # Modules
//!
//! - [`session`]: Session directory management and lifecycle control
//! - [`detection`]: Claude Code session existence detection
//!
//! # Examples
//!
//! ## Create Session Directory
//!
//! ```
//! use claude_session::{ SessionManager, Strategy };
//! use std::path::PathBuf;
//!
//! // Create session manager
//! let sessions_dir = PathBuf::from( "./" );
//! let mgr = SessionManager::new( &sessions_dir );
//!
//! // Ensure session directory exists
//! let session_dir = mgr.ensure_session( "debug", Strategy::Resume )?;
//!
//! // NOTE: For execution, use claude_runner_core::ClaudeCommand
//! // See claude_runner_core crate documentation for execution examples.
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ## Check Session Existence
//!
//! ```
//! use claude_session::check_session_exists;
//! use std::path::PathBuf;
//!
//! let session_dir = PathBuf::from( "/tmp/my-session" );
//! if check_session_exists( &session_dir )
//! {
//!   println!( "Session has conversation history" );
//! }
//! ```

#![warn(missing_docs)]

pub mod session;
pub mod detection;

pub use session::{ SessionManager, Strategy };
pub use detection::{ check_session_exists, get_claude_storage_path };

//! # claude_storage_core - Claude Code Storage Access Library
//!
//! Pure library for structured read/write access to Claude Code's filesystem-based storage.
//!
//! ## Overview
//!
//! Claude Code stores conversations in a filesystem-based "database" at `~/.claude/`,
//! using JSONL (JSON Lines) files for conversation history and directory hierarchies
//! for project organization. This crate provides safe, structured access to that storage.
//!
//! **Zero dependencies**: This library has no runtime dependencies for fast compilation
//! and minimal attack surface. All parsing is hand-written using `std` only.
//!
//! ## Architecture
//!
//! Claude Code's storage is filesystem-native:
//! - **Projects**: Directories in `~/.claude/projects/`
//! - **Sessions**: JSONL files within project directories
//! - **Entries**: Individual JSON objects (one per line in JSONL files)
//!
//! ## Core Types
//!
//! - [`Storage`]: Entry point for all storage operations
//! - [`Project`]: Represents a project (UUID or path-based)
//! - [`Session`]: Represents a conversation session (JSONL file)
//! - [`Entry`]: Individual conversation entry (user or assistant message)
//!
//! ## Safety Guarantees
//!
//! - **Append-only**: Write operations only append to JSONL files (no modification/deletion)
//! - **Atomic writes**: Uses temp file + rename pattern
//! - **Format validation**: All reads validate JSONL structure
//! - **Path encoding**: Automatic encoding/decoding of filesystem paths
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use claude_storage_core::{ Storage, ProjectId };
//!
//! fn main() -> claude_storage_core::Result< () >
//! {
//!   // List all projects
//!   let storage = Storage::new()?;
//!   for project in storage.list_projects()?
//!   {
//!     println!( "Project: {:?}", project.id() );
//!
//!     // List sessions within project
//!     for mut session in project.sessions()?
//!     {
//!       println!( "  Session: {}", session.id() );
//!       println!( "  Entries: {}", session.entries()?.len() );
//!     }
//!   }
//!   Ok( () )
//! }
//! ```
//!
//! ## Extraction Context
//!
//! This is the core library extracted from the monolithic `claude_storage` crate (2025-11-29).
//! The CLI functionality remains in the `claude_storage` crate, which depends on this core library.

#![deny( missing_docs )]
#![warn( rust_2018_idioms )]

mod path;
mod error;
mod entry;
mod session;
mod project;
mod storage;
mod json;
pub mod stats;
mod filter;
mod search;
mod export;

pub use path::{ encode_path, decode_path };
pub use error::{ Error, Result };
pub use entry::
{
  Entry,
  EntryType,
  MessageContent,
  UserMessage,
  AssistantMessage,
  ContentBlock,
  ThinkingMetadata,
};
pub use session::Session;
pub use project::{ Project, ProjectId };
pub use storage::Storage;
pub use json::{ JsonValue, parse_json };
pub use stats::{ SessionStats, GlobalStats, ProjectStats };
pub use filter::{ StringMatcher, SessionFilter, ProjectFilter };
pub use search::{ SearchFilter, SearchMatch };
pub use export::{ ExportFormat, export_session, export_session_to_file };

//! # claude_storage - CLI Tool for Claude Code Storage
//!
//! Command-line interface for exploring and analyzing Claude Code's filesystem-based storage.
//!
//! ## Overview
//!
//! This crate provides a CLI tool for querying Claude Code's conversation storage at `~/.claude/`.
//! It wraps the `claude_storage_core` library with an interactive REPL and one-shot command interface.
//!
//! **Extraction context**: This is the CLI-focused crate after extracting core library functionality
//! to `claude_storage_core` (2025-11-29). All storage access logic now lives in the core library;
//! this crate provides the command-line interface.
//!
//! ## Usage
//!
//! ### REPL mode (interactive):
//! ```bash
//! cargo run --features cli
//! > .status
//! > .list target::projects
//! > exit
//! ```
//!
//! ### One-shot mode (scripting):
//! ```bash
//! cargo run --features cli -- .status
//! cargo run --features cli -- .count target::projects
//! ```
//!
//! ## CLI Commands
//!
//! - `.status` - Show storage statistics (projects, sessions, entries, tokens)
//! - `.list` - List projects or sessions with optional filtering
//! - `.show` - Display details about a specific session
//! - `.count` - Fast counting of entries, sessions, or projects
//!
//! ## Library API
//!
//! For programmatic access to Claude Code storage, use the `claude_storage_core` crate directly:
//!
//! ```rust,no_run
//! use claude_storage_core::{ Storage, ProjectId };
//!
//! fn main() -> claude_storage_core::Result< () >
//! {
//!   let storage = Storage::new()?;
//!   for project in storage.list_projects()?
//!   {
//!     println!( "Project: {:?}", project.id() );
//!   }
//!   Ok( () )
//! }
//! ```

#![deny( missing_docs )]
#![warn( rust_2018_idioms )]

// Re-export core library types for convenience
pub use claude_storage_core::
{
  encode_path,
  decode_path,
  Error,
  Result,
  Entry,
  EntryType,
  MessageContent,
  UserMessage,
  AssistantMessage,
  ContentBlock,
  ThinkingMetadata,
  Session,
  Project,
  ProjectId,
  Storage,
  JsonValue,
  parse_json,
  SessionStats,
  GlobalStats,
  ProjectStats,
  stats,
};

#[cfg( feature = "cli" )]
pub mod cli;

#[cfg( feature = "cli" )]
pub use cli::parse_project_parameter;

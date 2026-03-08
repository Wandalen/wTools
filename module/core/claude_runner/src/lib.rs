//! `claude_runner` — command schema constants for `.claude` unilang commands.
//!
//! This library provides **command definition constants only** — the YAML file path
//! for `.claude` and `.claude.help` command schemas for unilang registration.
//!
//! ## Architecture
//!
//! ```text
//! wplan runner → claude_runner binary → dream_agent::routines::claude_routine
//!                                           → dream_agent::execute_claude()
//!                                               → subprocess: claude_runner (cli binary)
//!                                                   → claude binary
//! ```
//!
//! The `claude_runner` binary (runner plugin) routes unilang `.claude` commands to
//! `dream_agent::routines`, which executes Claude Code via subprocess boundary.
//! This lib has **zero willbe dependencies** — it is a pure constants crate.
//!
//! ## Registering commands in other binaries
//!
//! **Build-time (PHF):** Point `build.rs` at [`COMMANDS_YAML`].
//!
//! **Runtime:** Use `MultiYamlAggregator` with [`COMMANDS_YAML`].

/// Absolute path to this crate's unilang command definitions YAML.
///
/// Use in `build.rs` for compile-time aggregation or at runtime for dynamic registration.
pub const COMMANDS_YAML : &str = concat!( env!( "CARGO_MANIFEST_DIR" ), "/claude.commands.yaml" );

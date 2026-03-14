//! `claude_runner` — CLI binary + command schema constants for `.claude` unilang commands.
//!
//! This crate has two roles:
//!
//! 1. **Library** — exports [`COMMANDS_YAML`], the path to the `.claude` command schema,
//!    used by `dream` (willbe CLI) for compile-time PHF registration via `build.rs`.
//!
//! 2. **Binary** (`claude_runner`) — CLI invoked as a subprocess by `dream_agent`.
//!    Accepts `--flag value` argv and executes Claude Code via `claude_runner_core`.
//!
//! ## Two consumers, two roles
//!
//! ```text
//! dream (willbe CLI, build.rs)
//!   aggregates: claude_runner::COMMANDS_YAML → registers .claude + .claude.help in PHF map
//!
//! dream_agent (willbe lib)
//!   spawns subprocess: claude_runner --message X --dir Y ...
//!     → argv_to_unilang_tokens() → ClaudeCommand builder → claude subprocess
//! ```
//!
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

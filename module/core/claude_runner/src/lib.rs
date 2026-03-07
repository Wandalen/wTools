//! `claude_runner` — standalone AI command binary and reusable library.
//!
//! Provides `.claude` and `.claude.help` command implementations.
//! Can be used as a library by other willbe binaries, or invoked
//! directly as the `claude_runner` CLI binary (runner plugin).
//!
//! ## Architecture
//!
//! ```text
//! claude_runner CLI  →  claude_routine  →  dream_agent::execute_claude  →  ClaudeCommand
//! wplan (runtime)    →  spawn claude_runner binary (via runners.yaml config)
//! ```
//!
//! ## Runner Plugin System
//!
//! `claude_runner` is registered in `$WPLAN_CONFIG_DIR/runners.yaml` as the default runner.
//! When dream/wish receives `.claude` commands, it spawns this binary as a subprocess.
//! Planning orchestration (multi-dir, `work_dir` expansion) is handled by wplan itself
//! before invoking this binary.
//!
//! ## Registering commands in other binaries
//!
//! **Build-time (PHF):** Point `build.rs` at [`COMMANDS_YAML`].
//!
//! **Runtime:** Use `MultiYamlAggregator` with [`COMMANDS_YAML`].

#[ cfg( feature = "enabled" ) ]
pub mod routines;

/// Absolute path to this crate's unilang command definitions YAML.
///
/// Use in `build.rs` for compile-time aggregation or at runtime for dynamic registration.
pub const COMMANDS_YAML : &str = concat!( env!( "CARGO_MANIFEST_DIR" ), "/claude.commands.yaml" );

// Include compile-time generated static commands for library scope.
// Defines StaticCommandDefinition type used by build_command_registry_from.
#[ cfg( feature = "enabled" ) ]
include!( concat!( env!( "OUT_DIR" ), "/static_commands.rs" ) );

#[ cfg( feature = "enabled" ) ]
use unilang::{ CommandRegistry, CommandDefinition, VerifiedCommand, ExecutionContext, OutputData, ErrorData };

/// Build command registry with `claude_runner`'s AI routines.
///
/// Maps `.claude` and `.claude.help` commands to their routine functions.
/// Called from `main.rs` with the compile-time generated `AGGREGATED_COMMANDS`.
#[ cfg( feature = "enabled" ) ]
#[ must_use ]
#[ inline ]
pub fn build_command_registry_from(
  commands : &'static phf::Map< &'static str, &'static StaticCommandDefinition >
) -> CommandRegistry
{
  type RoutineFn = fn( VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData >;

  let routines : phf::Map< &'static str, RoutineFn > = phf::phf_map!
  {
    ".claude"      => routines::claude_routine,
    ".claude.help" => routines::claude_help_routine,
  };

  #[ allow( deprecated ) ]
  let mut registry = CommandRegistry::new();

  for ( name, static_cmd ) in commands.entries()
  {
    if let Some( &routine ) = routines.get( *name )
    {
      let cmd : CommandDefinition = ( *static_cmd ).into();

      #[ allow( deprecated ) ]
      if let Err( e ) = registry.command_add_runtime( &cmd, Box::new( routine ) )
      {
        if *name != ".help"
        {
          eprintln!( "WARNING: Failed to register routine for {name}: {e}" );
        }
      }
    }
    else
    {
      eprintln!( "WARNING: No routine found for command: {name}" );
    }
  }

  registry
}

/// Execute command with `claude_runner` registry (re-export from wplan).
#[ cfg( feature = "enabled" ) ]
pub use wplan::execute_with_registry;

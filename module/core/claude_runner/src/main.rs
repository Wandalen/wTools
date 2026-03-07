//! `claude_runner` CLI - AI assistance via Claude Code
//!
//! Standalone binary providing `.claude` and `.claude.help` commands.
//! Registered as a runner plugin via `$WPLAN_CONFIG_DIR/runners.yaml`
//! and invoked by wplan's runner plugin system.
//!
//! ## Usage
//!
//! Direct invocation (via wplan runner fallback):
//! ```text
//! claude_runner .claude command::"fix the bug"
//! claude_runner .claude.help
//! ```
//!
//! Via wplan runner plugin:
//! ```text
//! dream .claude command::"fix the bug"    # wplan spawns claude_runner
//! dream .plan.claude command::"task"      # wplan routes via runner plugin
//! ```

// Include compile-time generated static commands (.claude, .claude.help)
include!( concat!( env!( "OUT_DIR" ), "/static_commands.rs" ) );

fn main()
{
  let registry = claude_runner::build_command_registry_from( &AGGREGATED_COMMANDS );
  let args : Vec< String > = std::env::args().collect();
  claude_runner::execute_with_registry( registry, args );
}

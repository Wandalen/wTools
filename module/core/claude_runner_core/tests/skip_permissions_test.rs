//! Tests for `with_skip_permissions()` builder method
//!
//! Verifies the `--dangerously-skip-permissions` flag is correctly
//! added or omitted based on the builder configuration.

use claude_runner_core::ClaudeCommand;

#[test]
fn with_skip_permissions_true_adds_flag()
{
  let cmd = ClaudeCommand::new()
    .with_skip_permissions( true );

  let built = cmd.build_command_for_test();
  let args : Vec< _ > = built.get_args().collect();

  assert!
  (
    args.iter().any( |a| *a == "--dangerously-skip-permissions" ),
    "Expected --dangerously-skip-permissions flag, got: {args:?}"
  );
}

#[test]
fn with_skip_permissions_false_no_flag()
{
  let cmd = ClaudeCommand::new()
    .with_skip_permissions( false );

  let built = cmd.build_command_for_test();
  let args : Vec< _ > = built.get_args().collect();

  assert!
  (
    !args.iter().any( |a| *a == "--dangerously-skip-permissions" ),
    "Should NOT have --dangerously-skip-permissions flag, got: {args:?}"
  );
}

#[test]
fn default_skip_permissions_is_false()
{
  let cmd = ClaudeCommand::new();

  let built = cmd.build_command_for_test();
  let args : Vec< _ > = built.get_args().collect();

  assert!
  (
    !args.iter().any( |a| *a == "--dangerously-skip-permissions" ),
    "Default should NOT have --dangerously-skip-permissions flag, got: {args:?}"
  );
}

#[test]
fn skip_permissions_appears_before_custom_args()
{
  let cmd = ClaudeCommand::new()
    .with_skip_permissions( true )
    .with_arg( "--custom-flag" );

  let built = cmd.build_command_for_test();
  let args : Vec< _ > = built.get_args().collect();

  let skip_pos = args.iter().position( |a| *a == "--dangerously-skip-permissions" );
  let custom_pos = args.iter().position( |a| *a == "--custom-flag" );

  assert!( skip_pos.is_some(), "Missing --dangerously-skip-permissions" );
  assert!( custom_pos.is_some(), "Missing --custom-flag" );
  assert!
  (
    skip_pos.unwrap() < custom_pos.unwrap(),
    "--dangerously-skip-permissions should appear before custom args"
  );
}

#[test]
fn skip_permissions_last_call_wins()
{
  let cmd = ClaudeCommand::new()
    .with_skip_permissions( true )
    .with_skip_permissions( false );

  let built = cmd.build_command_for_test();
  let args : Vec< _ > = built.get_args().collect();

  assert!
  (
    !args.iter().any( |a| *a == "--dangerously-skip-permissions" ),
    "Last call (false) should win, got: {args:?}"
  );
}

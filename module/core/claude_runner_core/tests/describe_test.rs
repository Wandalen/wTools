//! Tests for `describe()` and `describe_env()` command inspection methods
//!
//! Verifies that the human-readable command representation and environment
//! variable listing accurately reflect the configured builder state.
//!
//! ## Test Coverage Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `describe_shows_cd_with_working_directory` | Working dir set | `/tmp/project` | First line is `cd /tmp/project` | ✅ |
//! | `describe_without_working_directory_no_cd` | No working dir | none | Starts with `claude` | ✅ |
//! | `describe_shows_skip_permissions_flag` | `skip_permissions=true` | `with_skip_permissions(true)` | Contains `--dangerously-skip-permissions` | ✅ |
//! | `describe_omits_skip_permissions_when_false` | `skip_permissions=false` | `with_skip_permissions(false)` | Does not contain `--dangerously-skip-permissions` | ✅ |
//! | `describe_shows_continuation_flag` | continue=true | `with_continue_conversation(true)` | Contains ` -c` | ✅ |
//! | `describe_shows_message` | message set | `"hello world"` | Contains `"hello world"` | ✅ |
//! | `describe_escapes_quotes_in_message` | message with double-quotes | `say "hello"` | `\"hello\"` in output | ✅ |
//! | `describe_escapes_backslash_in_message` | message with backslash | `back\slash` | `back\\slash` in output | ✅ |
//! | `describe_message_backslash_before_quote` | message with `\"` | `say \"world\"` | Correct escaped output | ✅ |
//! | `describe_message_with_newline_embeds_literally` | message with newline | `"line1\nline2"` | Newline embedded in output | ✅ |
//! | `describe_message_with_single_quote` | message with single quote | `it's fine` | Single quote preserved | ✅ |
//! | `describe_working_directory_with_spaces_cd_unquoted` | path with spaces | `/path with spaces` | Unquoted cd line (human-readable) | ✅ |
//! | `describe_system_prompt_shows_raw_arg` | `system_prompt` set | `"help me"` | `--system-prompt` in args | ✅ |
//! | `describe_api_key_appears_in_plaintext` | `api_key` set | `"sk-ant-123"` | Key appears unmasked | ✅ |
//! | `describe_shows_custom_args` | custom args | `--model claude-opus-4-5` | Args in output | ✅ |
//! | `describe_full_command` | all options | full set | Correct 2-line format | ✅ |
//! | `describe_env_shows_tier1_defaults` | default state | `ClaudeCommand::new()` | All tier-1 vars | ✅ |
//! | `describe_env_omits_unset_tier2_tier3` | default state | `ClaudeCommand::new()` | No tier-2/3 vars | ✅ |
//! | `describe_env_shows_set_tier2` | tier-2 set | `action_mode`, `log_level`, temp | Env vars present | ✅ |
//! | `describe_env_shows_set_tier3` | tier-3 set | sandbox, session, `top_p`, `top_k` | Env vars present | ✅ |
//! | `describe_env_overridden_tier1` | tier-1 overridden | custom token/timeout | Overridden values | ✅ |
//! | `describe_env_lines_are_newline_separated` | format | `ClaudeCommand::new()` | Each line has `=` | ✅ |
//!
//! ## Corner Cases Covered
//!
//! - ✅ Message with backslash: escaped to `\\` for shell correctness
//! - ✅ Message with double-quote: escaped to `\"`
//! - ✅ Message with backslash-before-doublequote: `\"` → `\\\"` (bug-fix-describe-backslash-escape)
//! - ✅ Message with newline: embedded literally (known behavior, human-readable only)
//! - ✅ Message with single quote: preserved as-is (valid inside double-quoted shell)
//! - ✅ Working directory with spaces: `cd` line unquoted (human-readable, not shell-safe)
//! - ✅ `system_prompt`: appears as raw CLI args (no quoting)
//! - ✅ `api_key`: appears in plaintext (no masking)
//!
//! ## Lessons Learned (Bugs Fixed)
//!
//! - **2025-02-28 (issue-describe-backslash-escape):** `describe()` only escaped `"` but not `\`.
//!   Messages containing `\"` produced `\\"` in describe output, which shell parses as closing
//!   the double-quoted string early, producing wrong argument splitting.
//!   Root cause: `msg.replace('"', "\\\"")` applied without first escaping backslashes.
//!   Prevention: Always escape `\` before `"` when building double-quoted shell strings.

use claude_runner_core::{ ClaudeCommand, ActionMode, LogLevel };

// ============================================================================
// describe() tests
// ============================================================================

#[test]
fn describe_shows_cd_with_working_directory()
{
  let desc = ClaudeCommand::new()
    .with_working_directory( "/tmp/project" )
    .describe();

  assert!( desc.starts_with( "cd /tmp/project\n" ) );
}

#[test]
fn describe_without_working_directory_no_cd()
{
  let desc = ClaudeCommand::new().describe();

  assert!( !desc.contains( "cd " ) );
  assert!( desc.starts_with( "claude" ) );
}

#[test]
fn describe_shows_skip_permissions_flag()
{
  let desc = ClaudeCommand::new()
    .with_skip_permissions( true )
    .describe();

  assert!( desc.contains( "--dangerously-skip-permissions" ) );
}

#[test]
fn describe_omits_skip_permissions_when_false()
{
  let desc = ClaudeCommand::new()
    .with_skip_permissions( false )
    .describe();

  assert!( !desc.contains( "--dangerously-skip-permissions" ) );
}

#[test]
fn describe_shows_continuation_flag()
{
  let desc = ClaudeCommand::new()
    .with_continue_conversation( true )
    .describe();

  assert!( desc.contains( " -c" ) );
}

#[test]
fn describe_shows_message()
{
  let desc = ClaudeCommand::new()
    .with_message( "hello world" )
    .describe();

  assert!( desc.contains( "\"hello world\"" ) );
}

#[test]
fn describe_escapes_quotes_in_message()
{
  let desc = ClaudeCommand::new()
    .with_message( "say \"hello\"" )
    .describe();

  assert!( desc.contains( "\\\"hello\\\"" ) );
}

#[test]
fn describe_shows_custom_args()
{
  let desc = ClaudeCommand::new()
    .with_arg( "--model" )
    .with_arg( "claude-opus-4-5" )
    .describe();

  assert!( desc.contains( "--model claude-opus-4-5" ) );
}

#[test]
fn describe_full_command()
{
  let desc = ClaudeCommand::new()
    .with_working_directory( "/home/user/proj" )
    .with_skip_permissions( true )
    .with_continue_conversation( true )
    .with_message( "fix bug" )
    .describe();

  let lines : Vec< &str > = desc.lines().collect();
  assert_eq!( lines.len(), 2 );
  assert_eq!( lines[ 0 ], "cd /home/user/proj" );
  assert!( lines[ 1 ].starts_with( "claude --dangerously-skip-permissions" ) );
  assert!( lines[ 1 ].contains( "-c" ) );
  assert!( lines[ 1 ].ends_with( "\"fix bug\"" ) );
}

// ============================================================================
// describe() corner case tests - message escaping
// ============================================================================

/// Reproduces `describe()` producing malformed shell output for messages containing `\"`
/// (backslash followed by double-quote). Before the fix, `msg.replace('"', "\\\"")` only
/// escaped double-quotes but left backslashes un-doubled, causing shell to misparse the
/// resulting `\\"` as a closing double-quote.
///
/// ## Root Cause
///
/// `describe()` used `msg.replace('"', "\\\"")` which escapes `"` → `\"` but does not
/// first double backslashes. When message contains `\"`, the result is `\\"` which in
/// a double-quoted shell string means: escaped-backslash=`\` + closing-quote. This
/// breaks the argument boundary, splitting one arg into three pieces.
///
/// ## Why Not Caught Initially
///
/// The existing test `describe_escapes_quotes_in_message` used `say "hello"` (no
/// backslash before quote). This passed because replacing `"` → `\"` is sufficient
/// when no backslash precedes the quote. The edge case requires BOTH a backslash AND
/// a doublequote in the same message, which was not tested.
///
/// ## Fix Applied
///
/// Changed `describe()` to first escape `\` → `\\`, then `"` → `\"`:
/// `msg.replace('\\', "\\\\").replace('"', "\\\"")`. This produces `\\\"` for `\"`
/// which shell correctly interprets as literal-backslash + literal-doublequote.
///
/// ## Prevention
///
/// When producing double-quoted shell strings from arbitrary content, always escape
/// `\` before `"`. The two-step replacement
/// `replace('\\', "\\\\").replace('"', "\\\"")`
/// is the correct approach. Single-step replacement of only `"` is insufficient.
///
/// ## Pitfall to Avoid
///
/// Never apply shell quoting by only escaping double-quotes. Any backslash in the
/// content will remain unescaped, and a backslash immediately before a double-quote
/// `\"` in the raw content will corrupt the resulting shell string representation.
// test_kind: bug_reproducer(issue-describe-backslash-escape)
#[test]
fn describe_message_backslash_before_quote()
{
  // Message contains backslash followed immediately by double-quote: say \"world\"
  let msg = "say \\\"world\\\"";  // Rust string: say \"world\"
  let desc = ClaudeCommand::new()
    .with_message( msg )
    .describe();

  // After fix: `\` → `\\`, `"` → `\"`, so `\"` becomes `\\\"` in output.
  // The describe output should contain `\\\"world\\\"` (Rust string: `\\\"world\\\"`)
  // which is shell: `\\\"world\\\"` = backslash + `"` + world + backslash + `"`
  assert!(
    desc.contains( "\\\\\\\"world\\\\\\\"" ),
    "describe() must escape backslashes before double-quotes: expected `\\\\\\\"` in output, got: {desc:?}"
  );
  // Also verify the message still appears correctly quoted
  assert!( desc.contains( "\"say" ), "describe() must wrap message in double-quotes" );
}

#[test]
fn describe_escapes_backslash_in_message()
{
  // Standalone backslash in message must be doubled
  let desc = ClaudeCommand::new()
    .with_message( "back\\slash" )  // Rust string: back\slash
    .describe();

  // After fix: `\` → `\\`, so output should contain `back\\slash`
  // (Rust string: `back\\slash`, actual chars: b,a,c,k,\,\,s,l,a,s,h)
  assert!(
    desc.contains( "back\\\\slash" ),  // Rust string `back\\\\slash` = actual `back\\slash`
    "Backslash in message must be doubled in describe() output, got: {desc:?}"
  );
}

#[test]
fn describe_message_with_newline_embeds_literally()
{
  // Message with newline: describe() embeds the newline literally.
  // This is known behavior: describe() is human-readable only (FR-21), not shell-safe.
  // A message with newline produces a describe() output that spans multiple lines.
  let desc = ClaudeCommand::new()
    .with_message( "line1\nline2" )
    .describe();

  // Both parts appear in the output (newline embedded)
  assert!( desc.contains( "line1" ), "First part must appear in describe()" );
  assert!( desc.contains( "line2" ), "Second part must appear in describe()" );
  // The describe output itself contains a newline inside the message fragment
  let msg_part = desc.split( "claude " ).nth( 1 ).unwrap_or( "" );
  assert!( msg_part.contains( '\n' ), "Newline is embedded literally in describe() output (known behavior)" );
}

#[test]
fn describe_message_with_single_quote()
{
  // Single quotes are valid inside double-quoted shell strings; no escaping needed.
  let desc = ClaudeCommand::new()
    .with_message( "it's working" )
    .describe();

  assert!( desc.contains( "it's working" ), "Single quotes pass through unchanged in describe()" );
}

#[test]
fn describe_working_directory_with_spaces_cd_unquoted()
{
  // describe() produces a human-readable cd line without quoting the path.
  // This is known behavior per FR-21 (human-readable, not shell-safe).
  // Actual execution (build_command) uses cmd.current_dir() which is shell-safe.
  let desc = ClaudeCommand::new()
    .with_working_directory( "/home/user/my project" )
    .describe();

  // The cd line is produced without quotes around the path
  let first_line = desc.lines().next().unwrap_or( "" );
  assert_eq!( first_line, "cd /home/user/my project",
    "describe() produces unquoted cd line for paths with spaces (human-readable behavior)" );
}

#[test]
fn describe_system_prompt_shows_raw_arg()
{
  // with_system_prompt() adds --system-prompt + value to args.
  // describe() shows them as raw args without quoting the value.
  let desc = ClaudeCommand::new()
    .with_system_prompt( "You are helpful" )
    .describe();

  assert!( desc.contains( "--system-prompt" ), "system prompt flag must appear in describe()" );
  assert!( desc.contains( "You are helpful" ), "system prompt value must appear in describe()" );
}

#[test]
fn describe_api_key_appears_in_plaintext()
{
  // with_api_key() adds --api-key + value to args; no masking in describe().
  // This is by design: describe() shows the full command for debug/dry-run purposes.
  let desc = ClaudeCommand::new()
    .with_api_key( "sk-ant-secret" )
    .describe();

  assert!( desc.contains( "--api-key" ), "api_key flag must appear in describe()" );
  assert!( desc.contains( "sk-ant-secret" ), "api_key value appears in plaintext in describe() (no masking)" );
}

// ============================================================================
// describe_env() tests
// ============================================================================

#[test]
fn describe_env_shows_tier1_defaults()
{
  let env = ClaudeCommand::new().describe_env();

  assert!( env.contains( "CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000" ) );
  assert!( env.contains( "CLAUDE_CODE_BASH_TIMEOUT=3600000" ) );
  assert!( env.contains( "CLAUDE_CODE_BASH_MAX_TIMEOUT=7200000" ) );
  assert!( env.contains( "CLAUDE_CODE_AUTO_CONTINUE=true" ) );
  assert!( env.contains( "CLAUDE_CODE_TELEMETRY=false" ) );
}

#[test]
fn describe_env_omits_unset_tier2_tier3()
{
  let env = ClaudeCommand::new().describe_env();

  assert!( !env.contains( "CLAUDE_CODE_AUTO_APPROVE_TOOLS" ) );
  assert!( !env.contains( "CLAUDE_CODE_ACTION_MODE" ) );
  assert!( !env.contains( "CLAUDE_CODE_LOG_LEVEL" ) );
  assert!( !env.contains( "CLAUDE_CODE_TEMPERATURE" ) );
  assert!( !env.contains( "CLAUDE_CODE_SANDBOX_MODE" ) );
  assert!( !env.contains( "CLAUDE_CODE_SESSION_DIR" ) );
  assert!( !env.contains( "CLAUDE_CODE_TOP_P" ) );
  assert!( !env.contains( "CLAUDE_CODE_TOP_K" ) );
}

#[test]
fn describe_env_shows_set_tier2()
{
  let env = ClaudeCommand::new()
    .with_action_mode( ActionMode::Allow )
    .with_log_level( LogLevel::Debug )
    .with_temperature( 0.5 )
    .describe_env();

  assert!( env.contains( "CLAUDE_CODE_ACTION_MODE=allow" ) );
  assert!( env.contains( "CLAUDE_CODE_LOG_LEVEL=debug" ) );
  assert!( env.contains( "CLAUDE_CODE_TEMPERATURE=0.5" ) );
}

#[test]
fn describe_env_shows_set_tier3()
{
  let env = ClaudeCommand::new()
    .with_sandbox_mode( false )
    .with_session_dir( "/tmp/sessions" )
    .with_top_p( 0.9 )
    .with_top_k( 50 )
    .describe_env();

  assert!( env.contains( "CLAUDE_CODE_SANDBOX_MODE=false" ) );
  assert!( env.contains( "CLAUDE_CODE_SESSION_DIR=/tmp/sessions" ) );
  assert!( env.contains( "CLAUDE_CODE_TOP_P=0.9" ) );
  assert!( env.contains( "CLAUDE_CODE_TOP_K=50" ) );
}

#[test]
fn describe_env_overridden_tier1()
{
  let env = ClaudeCommand::new()
    .with_max_output_tokens( 50_000 )
    .with_bash_timeout_ms( 1_000 )
    .describe_env();

  assert!( env.contains( "CLAUDE_CODE_MAX_OUTPUT_TOKENS=50000" ) );
  assert!( env.contains( "CLAUDE_CODE_BASH_TIMEOUT=1000" ) );
}

#[test]
fn describe_env_lines_are_newline_separated()
{
  let env = ClaudeCommand::new().describe_env();
  let lines : Vec< &str > = env.lines().collect();

  // At minimum tier 1 defaults: tokens, bash_timeout, bash_max_timeout, auto_continue, telemetry
  assert!( lines.len() >= 5, "Expected at least 5 env lines, got {}", lines.len() );

  for line in &lines {
    assert!( line.contains( '=' ), "Each line should be NAME=VALUE, got: {line}" );
  }
}

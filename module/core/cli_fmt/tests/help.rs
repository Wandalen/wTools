//! CLI help template rendering tests.
//!
//! Tests for `CliHelpTemplate`, `CliHelpStyle`, `CliHelpData`, and related types
//! under the `cli_help_template` feature flag in `cli_fmt`.
//!
//! ## Test Matrix
//!
//! | # | Scenario | Config | Expected |
//! |---|----------|--------|----------|
//! | T01 | 2 groups × 2 cmds, 2 opts, 2 examples | default style, tty_detect=false | column alignment enforced; no ANSI |
//! | T02 | same data | tty_detect=false | no ANSI escape sequences |
//! | T03 | same data, explicit tty_detect=false | tty_detect=false | same as T01 |
//! | T04 | cmd_name_width=10, 11-char name | custom style | name not truncated |
//! | T05 | empty options vec | default style | no "Options:" section emitted |
//! | T06 | empty examples vec | default style | no "Examples:" section emitted |
//! | T07 | single group, single cmd | default style, tty_detect=false | `Usage: {binary}` header, `Commands:` header, group+cmd appear; no ANSI |
//! | T08 | struct construction only | N/A | CliHelpStyle::default() field values match print_usage() |
//! | T09 | ExampleEntry with desc=Some vs None | tty_detect=false | desc=Some renders `# text` inline; desc=None renders no `#` |

use cli_fmt::help::*;

// ── helpers ───────────────────────────────────────────────────────────────────

fn two_group_data() -> CliHelpData
{
  CliHelpData
  {
    binary  : "myapp".into(),
    tagline : "example tool".into(),
    groups  : vec!
    [
      CommandGroup
      {
        name    : "Group A".into(),
        entries : vec!
        [
          CommandEntry { name : "cmd-one".into(),   desc : "first command".into()  },
          CommandEntry { name : "cmd-two".into(),   desc : "second command".into() },
        ],
      },
      CommandGroup
      {
        name    : "Group B".into(),
        entries : vec!
        [
          CommandEntry { name : "cmd-three".into(), desc : "third command".into()  },
          CommandEntry { name : "cmd-four".into(),  desc : "fourth command".into() },
        ],
      },
    ],
    options  : vec!
    [
      OptionEntry { name : "format::text|json".into(), desc : "Output format".into() },
      OptionEntry { name : "dry::bool".into(),         desc : "Dry-run".into()        },
    ],
    examples : vec!
    [
      ExampleEntry { invocation : "myapp cmd-one".into(), desc : Some( "run one".into() ) },
      ExampleEntry { invocation : "myapp cmd-two".into(), desc : None                     },
    ],
  }
}

fn no_tty_style() -> CliHelpStyle
{
  CliHelpStyle { tty_detect : false, ..CliHelpStyle::default() }
}

// ── T01 ─ column alignment ────────────────────────────────────────────────────

/// T01: With `tty_detect=false`, command names are left-padded to `cmd_name_width`
/// (20) and option names are padded to `opt_name_width` (18). No ANSI codes emitted.
///
/// Column widths: cmd = `cmd_name_width`(20) + `col_gap`(2) = 22 chars total;
/// opt = `opt_name_width`(18) + `col_gap`(2) = 20 chars total.
#[ test ]
fn test_column_alignment()
{
  let tmpl = CliHelpTemplate::new( no_tty_style(), two_group_data() );
  let out  = tmpl.render();

  // "cmd-one" (7 chars) padded to 20 → 13 trailing spaces before col_gap
  assert!(
    out.contains( "cmd-one             " ),
    "cmd-one must be padded to 20 chars (cmd_name_width), got:\n{out}",
  );
  // "dry::bool" (9 chars) padded to 18 → 9 trailing spaces before col_gap
  assert!(
    out.contains( "dry::bool         " ),
    "dry::bool must be padded to 18 chars (opt_name_width), got:\n{out}",
  );
  assert!(
    !out.contains( "\x1b[" ),
    "no ANSI codes in no-TTY mode, got:\n{out}",
  );
}

// ── T02 ─ no ANSI codes ───────────────────────────────────────────────────────

/// T02: When `tty_detect=false`, the rendered string must contain zero ANSI
/// escape sequences (regex `\x1b[` must not appear).
#[ test ]
fn test_no_ansi_codes()
{
  let tmpl = CliHelpTemplate::new( no_tty_style(), two_group_data() );
  let out  = tmpl.render();
  assert!(
    !out.contains( "\x1b[" ),
    "ANSI escape sequences must be absent with tty_detect=false, got:\n{out}",
  );
}

// ── T03 ─ explicit tty_detect=false ──────────────────────────────────────────

/// T03: Explicitly constructing `CliHelpStyle { tty_detect: false, .. }` is
/// equivalent to T01 — no ANSI, groups and commands visible.
#[ test ]
fn test_explicit_tty_detect_false()
{
  let style = CliHelpStyle { tty_detect : false, ..CliHelpStyle::default() };
  let tmpl  = CliHelpTemplate::new( style, two_group_data() );
  let out   = tmpl.render();
  assert!(
    !out.contains( "\x1b[" ),
    "explicit tty_detect=false must suppress all ANSI codes, got:\n{out}",
  );
  assert!( out.contains( "Group A" ), "group header must appear in output, got:\n{out}" );
  assert!( out.contains( "cmd-one" ), "command name must appear in output, got:\n{out}" );
}

// ── T04 ─ name not truncated when wider than cmd_name_width ──────────────────

/// T04: `cmd_name_width` is a minimum padding width, not a hard truncation
/// limit. An 11-char name with `cmd_name_width=10` must appear intact.
#[ test ]
fn test_name_not_truncated()
{
  let style = CliHelpStyle { cmd_name_width : 10, tty_detect : false, ..CliHelpStyle::default() };
  let data  = CliHelpData
  {
    binary  : "app".into(),
    tagline : "test".into(),
    groups  : vec![ CommandGroup
    {
      name    : "G".into(),
      entries : vec![ CommandEntry { name : "eleven-char".into(), desc : "desc".into() } ],
    }],
    options  : vec![],
    examples : vec![],
  };
  let out = CliHelpTemplate::new( style, data ).render();
  assert!(
    out.contains( "eleven-char" ),
    "11-char name must not be truncated when cmd_name_width=10, got:\n{out}",
  );
}

// ── T05 ─ no Options section when options is empty ────────────────────────────

/// T05: When `CliHelpData::options` is empty, the "Options:" header must not
/// appear in the rendered output.
#[ test ]
fn test_no_options_section()
{
  let mut data  = two_group_data();
  data.options  = vec![];
  let out       = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!(
    !out.contains( "Options:" ),
    "Options section must be absent when options vec is empty, got:\n{out}",
  );
}

// ── T06 ─ no Examples section when examples is empty ─────────────────────────

/// T06: When `CliHelpData::examples` is empty, the "Examples:" header must not
/// appear in the rendered output.
#[ test ]
fn test_no_examples_section()
{
  let mut data  = two_group_data();
  data.examples = vec![];
  let out       = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!(
    !out.contains( "Examples:" ),
    "Examples section must be absent when examples vec is empty, got:\n{out}",
  );
}

// ── T07 ─ single group: binary name, group header, command visible ────────────

/// T07: The rendered output contains `"Usage: {binary}"` and `"Commands:"` —
/// verifying the structured header format specified in `feature/002` and `api/002`.
/// Group header and command name appear in the body. No ANSI codes with `tty_detect=false`.
#[ test ]
fn test_single_group_binary_name()
{
  let data = CliHelpData
  {
    binary  : "myapp".into(),
    tagline : "a single-group tool".into(),
    groups  : vec![ CommandGroup
    {
      name    : "Cmds".into(),
      entries : vec![ CommandEntry { name : "run".into(), desc : "run it".into() } ],
    }],
    options  : vec![],
    examples : vec![],
  };
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!( out.contains( "Usage: myapp" ), "Usage: header with binary must appear, got:\n{out}" );
  assert!( out.contains( "Commands:" ),    "Commands: header must appear, got:\n{out}"          );
  assert!( out.contains( "Cmds" ),         "group header must appear in output, got:\n{out}"    );
  assert!( out.contains( "run" ),          "command name must appear in output, got:\n{out}"    );
  assert!(
    !out.contains( "\x1b[" ),
    "no ANSI codes with tty_detect=false, got:\n{out}",
  );
}

// ── T08 ─ CliHelpStyle::default() field values ───────────────────────────────

/// T08: `CliHelpStyle::default()` must produce the same layout constants as the
/// hardcoded `print_usage()` in `claude_profile/src/lib.rs`.
#[ test ]
fn test_style_default_fields()
{
  let s = CliHelpStyle::default();
  assert_eq!( s.cmd_indent,     4  );
  assert_eq!( s.cmd_name_width, 20 );
  assert_eq!( s.grp_indent,     2  );
  assert_eq!( s.opt_indent,     2  );
  assert_eq!( s.opt_name_width, 18 );
  assert_eq!( s.col_gap,        2  );
  assert_eq!( s.example_indent, 2  );
}

// ── T09 ─ ExampleEntry.desc renders inline when Some ─────────────────────────

/// T09: When `ExampleEntry.desc` is `Some(text)`, the rendered output appends
/// `  # {text}` on the same line as the invocation.
///
/// When `desc` is `None`, the invocation line has no `#` annotation.
///
/// # Root Cause
/// `emit_examples()` previously ignored `ExampleEntry.desc` entirely, making the
/// `desc: Option<String>` field a silent no-op despite being documented as
/// "Optional annotation line appended after the invocation."
///
/// # Why Not Caught
/// The `two_group_data()` fixture used `desc: Some("run one")` but no test
/// asserted that "run one" appeared in the rendered output, so the silent drop
/// went undetected across all T01–T08 tests.
///
/// # Fix Applied
/// `emit_examples()` now branches on `ex.desc`: `Some(text)` → appends `  # {text}`
/// after the invocation on the same line; `None` → emits the invocation unchanged.
///
/// # Prevention
/// Any new `Option`-typed field on a data struct must have at least one test
/// asserting the `Some` branch is visible in rendered output.
///
/// # Pitfall
/// Forgetting to test `Some` paths on `Option` fields in renderers leads to
/// documented-but-broken API contracts that only surface during manual testing.
#[ test ]
fn test_example_desc_rendered()
{
  let out = CliHelpTemplate::new( no_tty_style(), two_group_data() ).render();

  // First example has desc=Some("run one") — must appear as `# run one` on its line.
  let first_line = out.lines()
    .find( |l| l.contains( "myapp cmd-one" ) )
    .unwrap_or_default();
  assert!(
    first_line.contains( "# run one" ),
    "ExampleEntry with desc=Some must render '# run one' on the invocation line, got:\n{first_line:?}",
  );

  // Second example has desc=None — its line must not contain `#`.
  let second_line = out.lines()
    .find( |l| l.contains( "myapp cmd-two" ) )
    .unwrap_or_default();
  assert!(
    !second_line.contains( '#' ),
    "ExampleEntry with desc=None must not render '#' on the invocation line, got:\n{second_line:?}",
  );
}

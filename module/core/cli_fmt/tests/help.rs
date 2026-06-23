#![ cfg( feature = "cli_help_template" ) ]

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
//! | T10 | CliHelpStyle::default() color fields | struct construction only | all 5 color fields + tty_detect match documented API defaults |
//! | T11 | groups: vec![], one option | default style, tty_detect=false | render succeeds; binary/tagline present; no group text |
//! | T12 | opt_name_width=10, 12-char opt name | custom style | option name not truncated |
//! | T13 | CliHelpStyle::default() (tty_detect=true), non-TTY process | default style | no ANSI codes in output — TTY probe returns false under nextest |
//! | T14 | Cargo.toml content | string check | `"data_fmt"` absent — AC-4 regression guard |
//! | T-A01 | custom usage_lines, empty usage_lines | tty_detect=false | custom lines replace default header; empty preserves default |
//! | T-A02 | non-empty arguments, empty arguments | tty_detect=false | Arguments: section renders/omits |
//! | T-A03 | option_groups with one group | tty_detect=false | named group header and entries appear |
//! | T-A04 | empty option_groups, non-empty options | tty_detect=false | legacy Options: section renders |
//! | T-A05 | both options and option_groups non-empty | tty_detect=false | option_groups renders; options suppressed |
//! | T-A06 | two groups with different name lengths | tty_detect=false | per-group independent column padding |
//! | T-A07 | CliHelpData::default() | N/A | all fields empty; no panic |
//! | T-A08 | compile_fail doctest | N/A | exhaustive CliHelpData literal rejected by #[non_exhaustive] |
//! | T-A09 | example construction pattern | tty_detect=false | non-empty output from default + field assignment |
//! | T-B01 | usage_lines with 3 entries | tty_detect=false | all 3 lines render indented; default header absent |
//! | T-B02 | arguments with 2 different-length names | tty_detect=false | short name padded to longest name length |
//! | T-B03 | CommandGroup with empty entries | tty_detect=false | group header renders; no entries emitted; no panic |
//! | T-B04 | fully empty CliHelpData | tty_detect=false | render succeeds; Usage: and Commands: headers present |
//! | T-B05 | ExampleEntry with desc=Some("") | tty_detect=false | `# ` marker rendered; no content after it |
//! | T-B06 | OptionGroup with empty entries | tty_detect=false | group header silently skipped; no panic |
//! | T-B07 | empty-entry OptionGroup + non-empty options | tty_detect=false | options suppressed; empty group silent |
//! | T-B08 | both arguments and groups non-empty | tty_detect=false | Arguments: section appears before first group |
//! | T-B09 | two examples in declaration order | tty_detect=false | first declared example at lower position than second |
//! | T-B10 | tagline and usage line | tty_detect=false | blank line separates usage from tagline; usage appears first |
//! | T-B11 | col_gap=4, cmd_name_width=7, one command | tty_detect=false | 4-space gap between padded name column and description (FT-31) |
//! | T-B12 | cmd_indent=2, cmd_name_width=3, one command | tty_detect=false | 2-space leading indent instead of default 4 (FT-32) |

use cli_fmt::help::*;

// ── helpers ───────────────────────────────────────────────────────────────────

fn two_group_data() -> CliHelpData
{
  let mut data = CliHelpData::default();
  data.binary = "myapp".into();
  data.tagline = "example tool".into();
  data.groups = vec!
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
  ];
  data.options = vec!
  [
    OptionEntry { name : "format::text|json".into(), desc : "Output format".into() },
    OptionEntry { name : "dry::bool".into(),         desc : "Dry-run".into()        },
  ];
  data.examples = vec!
  [
    ExampleEntry { invocation : "myapp cmd-one".into(), desc : Some( "run one".into() ) },
    ExampleEntry { invocation : "myapp cmd-two".into(), desc : None                     },
  ];
  data
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
  let mut data = CliHelpData::default();
  data.binary = "app".into();
  data.tagline = "test".into();
  data.groups = vec![ CommandGroup
  {
    name    : "G".into(),
    entries : vec![ CommandEntry { name : "eleven-char".into(), desc : "desc".into() } ],
  }];
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
  let mut data = CliHelpData::default();
  data.binary = "myapp".into();
  data.tagline = "a single-group tool".into();
  data.groups = vec![ CommandGroup
  {
    name    : "Cmds".into(),
    entries : vec![ CommandEntry { name : "run".into(), desc : "run it".into() } ],
  }];
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
// BUG-007 task/bug/closed/007_example_desc_silent_drop.md — desc field ignored in emit_examples
// test_kind: bug_reproducer(BUG-007)
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

// ── T10 ─ CliHelpStyle::default() color fields and tty_detect ─────────────────

/// T10: All 6 previously untested `CliHelpStyle::default()` fields — 5 ANSI color
/// codes plus `tty_detect` — must match the values documented in `api/002_help_api.md`.
///
/// T08 only covers the 7 layout fields (indents, widths, gap). T10 covers the
/// remaining 6 fields that form the API contract for color and TTY behaviour.
#[ test ]
fn test_style_color_defaults()
{
  let s = CliHelpStyle::default();
  assert_eq!( s.color_tagline, "\x1b[1m",           "color_tagline must be bold ANSI code"          );
  assert_eq!( s.color_group,   "\x1b[33m\x1b[1m",  "color_group must be yellow+bold ANSI codes"    );
  assert_eq!( s.color_option,  "\x1b[1;36m",        "color_option must be bold cyan ANSI code"      );
  assert_eq!( s.color_example, "\x1b[2m",            "color_example must be dim ANSI code"           );
  assert_eq!( s.color_reset,   "\x1b[0m",            "color_reset must be ANSI reset sequence"       );
  assert!( s.tty_detect,                              "tty_detect must be true by default"            );
}

// ── T11 ─ empty groups vec renders without panic ──────────────────────────────

/// T11: `CliHelpData` with `groups: vec![]` must render without panic.
/// The binary name and tagline must appear; no group-specific content emitted.
#[ test ]
fn test_empty_groups()
{
  let mut data = CliHelpData::default();
  data.binary = "app".into();
  data.tagline = "test tool".into();
  data.options = vec![ OptionEntry { name : "verbose::bool".into(), desc : "Enable verbose".into() } ];
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!( !out.is_empty(),        "render with empty groups must return non-empty string"  );
  assert!( out.contains( "app" ),       "binary name must appear in output, got:\n{out}"   );
  assert!( out.contains( "test tool" ), "tagline must appear in output, got:\n{out}"        );
}

// ── T12 ─ opt_name_width is minimum padding, not truncation limit ─────────────

/// T12: `opt_name_width` is a minimum padding width, not a hard truncation limit.
/// A 12-char option name with `opt_name_width=10` must appear intact in output.
///
/// Mirrors T04 which tests the same property for `cmd_name_width`.
#[ test ]
fn test_opt_name_not_truncated()
{
  let style = CliHelpStyle { opt_name_width : 10, tty_detect : false, ..CliHelpStyle::default() };
  let mut data = CliHelpData::default();
  data.binary = "app".into();
  data.tagline = "test".into();
  data.options = vec![ OptionEntry { name : "format::json".into(), desc : "format specifier".into() } ];
  let out = CliHelpTemplate::new( style, data ).render();
  assert!(
    out.contains( "format::json" ),
    "12-char option name must not be truncated when opt_name_width=10, got:\n{out}",
  );
}

// ── T13 ─ tty_detect=true suppresses ANSI in non-TTY test environment ─────────

/// T13 (FT-10): `CliHelpStyle::default()` has `tty_detect=true`. Under nextest
/// the process stdout is not a TTY, so the TTY probe returns false and all ANSI
/// codes must be suppressed — same observable result as `tty_detect=false`.
///
/// This is the only test that exercises the TTY-probe code path through
/// `CliHelpStyle::default()` rather than an explicit `tty_detect=false` override.
#[ test ]
fn test_tty_detect_true_suppresses_ansi_in_non_tty()
{
  let out = CliHelpTemplate::new( CliHelpStyle::default(), two_group_data() ).render();
  assert!(
    !out.contains( "\x1b[" ),
    "tty_detect=true in non-TTY test environment must suppress ANSI codes, got:\n{out}"
  );
}

// ── T14 ─ data_fmt crate is not a dependency ──────────────────────────────────

/// T14 (FT-11): `cli_fmt` must not list `data_fmt` as a dependency.
/// The help renderer uses only `strs_tools` primitives for string manipulation.
/// This is a regression guard for AC-4 of `docs/feature/002_cli_help_template.md`.
#[ test ]
fn test_no_data_fmt_dependency()
{
  let cargo = include_str!( "../Cargo.toml" );
  assert!(
    !cargo.contains( "data_fmt" ),
    "cli_fmt must not depend on data_fmt — uses strs_tools primitives only"
  );
}

// ── T-A01 ─ usage_lines replaces default header ──────────────────────────────

/// T-A01 (FT-12, FT-13): When `usage_lines` is non-empty, the custom lines
/// replace the hardcoded `"Usage: {binary} <command>"` header. When empty
/// (default), the original header emits unchanged.
#[ test ]
fn test_usage_lines()
{
  let mut data = CliHelpData::default();
  data.binary = "clr".into();
  data.tagline = "runner".into();
  data.usage_lines = vec![ "clr <command>".into() ];
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!(
    out.contains( "  clr <command>" ),
    "custom usage_lines must appear indented, got:\n{out}",
  );
  assert!(
    !out.contains( "Usage: clr" ),
    "default Usage: header must NOT appear when usage_lines is set, got:\n{out}",
  );

  let mut data = CliHelpData::default();
  data.binary = "clr".into();
  data.tagline = "runner".into();
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!(
    out.contains( "Usage: clr" ),
    "default Usage: header must appear when usage_lines is empty, got:\n{out}",
  );
}

// ── T-A02 ─ arguments section renders when non-empty ─────────────────────────

/// T-A02 (FT-14, FT-15): When `arguments` is non-empty, the `"Arguments:"`
/// section renders. When empty (default), no `"Arguments:"` header appears.
#[ test ]
fn test_arguments_section()
{
  let mut data = CliHelpData::default();
  data.binary = "app".into();
  data.tagline = "test".into();
  data.arguments = vec![ OptionEntry { name : "<MSG>".into(), desc : "Message to send".into() } ];
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!(
    out.contains( "Arguments:" ),
    "Arguments: header must appear when arguments is non-empty, got:\n{out}",
  );
  assert!(
    out.contains( "  <MSG>  Message to send" ),
    "argument entry must render with content-driven padding, got:\n{out}",
  );

  let mut data = CliHelpData::default();
  data.binary = "app".into();
  data.tagline = "test".into();
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!(
    !out.contains( "Arguments:" ),
    "Arguments: header must NOT appear when arguments is empty, got:\n{out}",
  );
}

// ── T-A03 ─ option_groups renders named sections ─────────────────────────────

/// T-A03 (FT-16, AP-7): Named option groups render with `"{name}:"` header
/// and entries with content-driven padding.
#[ test ]
fn test_option_groups_render()
{
  let mut data = CliHelpData::default();
  data.binary = "app".into();
  data.tagline = "test".into();
  data.option_groups = vec!
  [
    OptionGroup
    {
      name    : "RUNNER OPTIONS".into(),
      entries : vec![ OptionEntry { name : "--flag".into(), desc : "A flag".into() } ],
    },
  ];
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!(
    out.contains( "RUNNER OPTIONS:" ),
    "option group header must appear, got:\n{out}",
  );
  assert!(
    out.contains( "  --flag  A flag" ),
    "option group entry must render with padding, got:\n{out}",
  );
}

// ── T-A04 ─ empty option_groups preserves legacy Options section ─────────────

/// T-A04 (FT-17): When `option_groups` is empty and `options` is non-empty,
/// the legacy `"Options:"` section renders (backward compat).
#[ test ]
fn test_option_groups_empty_backward_compat()
{
  let mut data = CliHelpData::default();
  data.binary = "app".into();
  data.tagline = "test".into();
  data.options = vec![ OptionEntry { name : "--opt".into(), desc : "desc".into() } ];
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!(
    out.contains( "Options:" ),
    "legacy Options: section must appear when option_groups is empty, got:\n{out}",
  );
  assert!(
    out.contains( "--opt" ),
    "legacy option entry must appear, got:\n{out}",
  );
}

// ── T-A05 ─ option_groups non-empty suppresses legacy options ────────────────

/// T-A05 (FT-18): When `option_groups` is non-empty, the legacy `options`
/// field is suppressed — its entries do not appear and `"Options:"` is absent.
#[ test ]
fn test_option_groups_suppresses_options()
{
  let mut data = CliHelpData::default();
  data.binary = "app".into();
  data.tagline = "test".into();
  data.options = vec![ OptionEntry { name : "--old".into(), desc : "old opt".into() } ];
  data.option_groups = vec!
  [
    OptionGroup
    {
      name    : "NEW".into(),
      entries : vec![ OptionEntry { name : "--new".into(), desc : "new opt".into() } ],
    },
  ];
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!(
    out.contains( "NEW:" ),
    "option group header must appear, got:\n{out}",
  );
  assert!(
    out.contains( "  --new  new opt" ),
    "option group entry must appear, got:\n{out}",
  );
  assert!(
    !out.contains( "--old" ),
    "legacy option entry must NOT appear when option_groups is non-empty, got:\n{out}",
  );
  assert!(
    !out.contains( "Options:" ),
    "Options: header must NOT appear when option_groups is non-empty, got:\n{out}",
  );
}

// ── T-A06 ─ per-group independent column padding ─────────────────────────────

/// T-A06 (FT-19): Each option group computes its own column width from its
/// own entries — group A's padding is NOT inflated by group B's longer names.
#[ test ]
fn test_option_groups_independent_padding()
{
  let mut data = CliHelpData::default();
  data.binary = "app".into();
  data.tagline = "test".into();
  data.option_groups = vec!
  [
    OptionGroup
    {
      name    : "GROUP A".into(),
      entries : vec!
      [
        OptionEntry { name : "--aa".into(), desc : "flag a".into() },
        OptionEntry { name : "--bb".into(), desc : "flag b".into() },
      ],
    },
    OptionGroup
    {
      name    : "GROUP B".into(),
      entries : vec!
      [
        OptionEntry { name : "--longer-name".into(), desc : "a long flag".into() },
      ],
    },
  ];
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!(
    out.contains( "  --aa  flag a" ),
    "group A entry must use group A's own max_len (4), not group B's (13), got:\n{out}",
  );
  assert!(
    out.contains( "  --bb  flag b" ),
    "group A second entry must use same group-local padding, got:\n{out}",
  );
  assert!(
    out.contains( "  --longer-name  a long flag" ),
    "group B entry must use group B's own max_len (13), got:\n{out}",
  );
}

// ── T-A07 ─ CliHelpData::default() constructs with empty fields ──────────────

/// T-A07 (FT-20, AP-8): `CliHelpData::default()` constructs without panic.
/// All Vec fields are empty; binary and tagline are empty strings.
#[ test ]
fn test_cli_help_data_default()
{
  let data = CliHelpData::default();
  assert!( data.binary.is_empty(),        "binary must be empty"        );
  assert!( data.tagline.is_empty(),       "tagline must be empty"       );
  assert!( data.groups.is_empty(),        "groups must be empty"        );
  assert!( data.options.is_empty(),       "options must be empty"       );
  assert!( data.examples.is_empty(),      "examples must be empty"      );
  assert!( data.usage_lines.is_empty(),   "usage_lines must be empty"   );
  assert!( data.arguments.is_empty(),     "arguments must be empty"     );
  assert!( data.option_groups.is_empty(), "option_groups must be empty" );
}

// ── T-B01 ─ multiple usage_lines all render indented ─────────────────────────

/// T-B01: When `usage_lines` contains 3 entries, each appears on its own indented
/// line. The default `"Usage: {binary} <command>"` header is absent entirely.
#[ test ]
fn test_multiple_usage_lines()
{
  let mut data = CliHelpData::default();
  data.binary = "app".into();
  data.tagline = "tool".into();
  data.usage_lines = vec!
  [
    "app <command>".into(),
    "app --help".into(),
    "app <command> [options]".into(),
  ];
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!(
    out.contains( "  app <command>\n" ),
    "first custom usage line must render indented, got:\n{out}",
  );
  assert!(
    out.contains( "  app --help\n" ),
    "second custom usage line must render indented, got:\n{out}",
  );
  assert!(
    out.contains( "  app <command> [options]\n" ),
    "third custom usage line must render indented, got:\n{out}",
  );
  assert!(
    !out.contains( "Usage: app" ),
    "default Usage: header must not appear when usage_lines is non-empty, got:\n{out}",
  );
}

// ── T-B02 ─ arguments multi-entry column padding uses longest name ─────────────

/// T-B02: When `arguments` contains entries with different name lengths, all entries
/// are padded to the longest name's length — not each entry's own length.
#[ test ]
fn test_arguments_multi_entry_padding()
{
  let mut data = CliHelpData::default();
  data.binary = "app".into();
  data.tagline = "test".into();
  data.arguments = vec!
  [
    OptionEntry { name : "<A>".into(),         desc : "short arg".into()  },
    OptionEntry { name : "<LONG-ARGUMENT>".into(), desc : "long arg".into() },
  ];
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  // "<A>" is 3 chars; "<LONG-ARGUMENT>" is 15 chars; max_len = 15
  // "<A>" padded to 15 → "<A>            " then "  short arg"
  assert!(
    out.contains( "  <A>              short arg" ),
    "short argument name must be padded to match longest name (15 chars), got:\n{out}",
  );
  assert!(
    out.contains( "  <LONG-ARGUMENT>  long arg" ),
    "longest argument name must appear with no extra padding, got:\n{out}",
  );
}

// ── T-B03 ─ CommandGroup with empty entries vec renders only its header ────────

/// T-B03: A `CommandGroup` with an empty `entries` vec must render its group
/// header without panicking. No command entries appear beneath it.
#[ test ]
fn test_command_group_empty_entries()
{
  let mut data = CliHelpData::default();
  data.binary = "app".into();
  data.tagline = "test".into();
  data.groups = vec!
  [
    CommandGroup { name : "EMPTY GROUP".into(), entries : vec![] },
    CommandGroup
    {
      name    : "FULL GROUP".into(),
      entries : vec![ CommandEntry { name : "cmd".into(), desc : "a command".into() } ],
    },
  ];
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!( out.contains( "EMPTY GROUP" ), "empty-entries group header must appear, got:\n{out}" );
  assert!( out.contains( "FULL GROUP" ),  "non-empty group header must appear, got:\n{out}"     );
  assert!( out.contains( "cmd" ),         "command from non-empty group must appear, got:\n{out}" );
}

// ── T-B04 ─ render with entirely empty CliHelpData is infallible ──────────────

/// T-B04: `CliHelpTemplate::render()` must not panic when given a fully default
/// (empty) `CliHelpData`. The output must still contain the `"Usage:"` and
/// `"Commands:"` structural headers.
#[ test ]
fn test_render_empty_data_infallible()
{
  let data = CliHelpData::default();
  let out  = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!( !out.is_empty(),          "render of empty data must produce non-empty string"       );
  assert!( out.contains( "Usage:" ), "Usage: header must appear even with empty data, got:\n{out}" );
  assert!( out.contains( "Commands:" ), "Commands: header must appear even with empty data, got:\n{out}" );
}

// ── T-B05 ─ ExampleEntry desc=Some("") renders empty annotation marker ────────

/// T-B05: When `ExampleEntry.desc` is `Some("")` (empty string), the rendered
/// line appends `  # ` (the annotation marker with trailing space but no text).
/// This documents that the annotation is rendered whenever `Some` is present,
/// regardless of whether the inner string is non-empty.
#[ test ]
fn test_example_empty_desc_some_renders_marker()
{
  let mut data = CliHelpData::default();
  data.binary = "app".into();
  data.tagline = "test".into();
  data.examples = vec!
  [
    ExampleEntry { invocation : "app cmd".into(), desc : Some( "".into() ) },
  ];
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  let line = out.lines()
    .find( |l| l.contains( "app cmd" ) )
    .unwrap_or_default();
  assert!(
    line.contains( "# " ),
    "ExampleEntry with desc=Some(\"\") must render '# ' marker, got:\n{line:?}",
  );
}

// ── T-B06 ─ OptionGroup with empty entries is silently skipped ────────────────

/// T-B06: An `OptionGroup` whose `entries` vec is empty is silently skipped —
/// its section header (`"{name}:"`) is NOT emitted. No panic occurs.
///
/// This documents the contract: groups without entries produce no visible output.
#[ test ]
fn test_option_group_empty_entries_skipped()
{
  let mut data = CliHelpData::default();
  data.binary = "app".into();
  data.tagline = "test".into();
  data.option_groups = vec!
  [
    OptionGroup { name : "EMPTY SECTION".into(), entries : vec![] },
  ];
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!(
    !out.contains( "EMPTY SECTION:" ),
    "OptionGroup with empty entries must not emit its header, got:\n{out}",
  );
}

// ── T-B07 ─ empty-entry OptionGroup suppresses legacy options (footgun) ────────

/// T-B07: When `option_groups` is non-empty (even if all groups have empty
/// `entries`), the legacy `options` field is suppressed. Neither `"Options:"`
/// nor the option entries appear in the output.
///
/// This is the documented suppression rule: `option_groups` vec non-empty →
/// suppress `options`. Users who accidentally provide an empty-entry group
/// will lose their `options` silently. Add entries or remove the empty group.
#[ test ]
fn test_empty_option_group_suppresses_legacy_options()
{
  let mut data = CliHelpData::default();
  data.binary = "app".into();
  data.tagline = "test".into();
  data.options = vec![ OptionEntry { name : "--verbose".into(), desc : "verbosity".into() } ];
  data.option_groups = vec![ OptionGroup { name : "EMPTY GROUP".into(), entries : vec![] } ];
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  // option_groups is non-empty (vec has 1 group) → options suppressed
  assert!(
    !out.contains( "Options:" ),
    "Options: must be suppressed when option_groups is non-empty, got:\n{out}",
  );
  assert!(
    !out.contains( "--verbose" ),
    "--verbose must not appear when option_groups suppresses legacy options, got:\n{out}",
  );
  // The empty-entry group itself must also be silent
  assert!(
    !out.contains( "EMPTY GROUP:" ),
    "empty-entry OptionGroup header must not appear, got:\n{out}",
  );
}

// ── T-B08 ─ arguments section appears before command groups in output ──────────

/// T-B08: When both `arguments` and `groups` are non-empty, the `"Arguments:"`
/// section appears BEFORE the first command group entry in the output.
///
/// Verified by comparing string positions of "Arguments:" and the group header.
#[ test ]
fn test_arguments_before_groups_in_output()
{
  let mut data = CliHelpData::default();
  data.binary = "app".into();
  data.tagline = "test".into();
  data.arguments = vec![ OptionEntry { name : "<FILE>".into(), desc : "input file".into() } ];
  data.groups = vec![ CommandGroup
  {
    name    : "FILE OPS".into(),
    entries : vec![ CommandEntry { name : "read".into(), desc : "read a file".into() } ],
  }];
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  let pos_args = out.find( "Arguments:" ).expect( "Arguments: must appear in output" );
  let pos_grp  = out.find( "FILE OPS" ).expect( "Group header must appear in output" );
  assert!(
    pos_args < pos_grp,
    "Arguments: section must appear before group entries, but got Arguments at {pos_args}, group at {pos_grp}",
  );
}

// ── T-A09 ─ examples/basic_usage.rs smoke test ───────────────────────────────

/// T-A09: `examples/basic_usage.rs` compiles with `CliHelpData::default()`
/// and field assignment under `#[non_exhaustive]`. The example's compilation
/// is validated by `cargo test --examples --all-features`.
///
/// This smoke test verifies the same construction pattern produces non-empty
/// output.
#[ test ]
fn test_examples_compile()
{
  let mut data = CliHelpData::default();
  data.binary = "mytool".into();
  data.tagline = "A sample CLI tool.".into();
  data.groups = vec![ CommandGroup
  {
    name    : "Ops".into(),
    entries : vec![ CommandEntry { name : "run".into(), desc : "Run it".into() } ],
  }];
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!( !out.is_empty(), "example construction pattern must produce non-empty output" );
}

// ── T-B09 ─ examples render in declaration order ─────────────────────────────

/// T-B09 (FT-29): When `examples` contains multiple entries, the rendered output
/// preserves declaration order — the first declared example appears at a lower
/// byte offset than the second.
#[ test ]
fn test_examples_declaration_order()
{
  let mut data = CliHelpData::default();
  data.binary = "app".into();
  data.tagline = "test".into();
  data.examples = vec!
  [
    ExampleEntry { invocation : "app cmd-a".into(), desc : None },
    ExampleEntry { invocation : "app cmd-b".into(), desc : None },
  ];
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  let pos_a = out.find( "app cmd-a" ).expect( "first example must appear in output" );
  let pos_b = out.find( "app cmd-b" ).expect( "second example must appear in output" );
  assert!(
    pos_a < pos_b,
    "first declared example must appear before second in rendered output (pos_a={pos_a}, pos_b={pos_b})"
  );
}

// ── T-B10 ─ tagline separated from usage line by blank line ──────────────────

/// T-B10 (FT-30): The tagline appears after the usage line, with a blank line
/// (`"\n\n"`) as the structural separator between them.
#[ test ]
fn test_tagline_blank_line_separator()
{
  let mut data = CliHelpData::default();
  data.binary  = "myapp".into();
  data.tagline = "My helpful tool".into();
  let out = CliHelpTemplate::new( no_tty_style(), data ).render();
  assert!(
    out.contains( "My helpful tool" ),
    "tagline must appear in rendered output, got:\n{out}"
  );
  assert!(
    out.contains( "\n\n" ),
    "a blank line must separate usage line from tagline, got:\n{out:?}"
  );
  let pos_usage   = out.find( "Usage: myapp" ).expect( "Usage: header must appear in output" );
  let pos_tagline = out.find( "My helpful tool" ).expect( "tagline must appear in output" );
  assert!(
    pos_usage < pos_tagline,
    "usage line must appear before tagline (pos_usage={pos_usage}, pos_tagline={pos_tagline})"
  );
}

// ── T-B11 ─ custom col_gap spacing ───────────────────────────────────────────

/// T-B11 (FT-31): `col_gap=4` produces 4 spaces between the padded name column
/// and the description text. With `cmd_name_width=7` and `cmd_indent=4` (default),
/// a command named `"cmd-one"` (7 chars, exact fit) renders as:
/// `"    cmd-one    do one thing"` — 4-indent, 7-name, 4-gap.
///
/// Contrasts with the default `col_gap=2` which would produce
/// `"    cmd-one  do one thing"`.
#[ test ]
fn test_col_gap_custom()
{
  let style = CliHelpStyle { col_gap : 4, cmd_name_width : 7, tty_detect : false, ..CliHelpStyle::default() };
  let mut data = CliHelpData::default();
  data.groups = vec!
  [
    CommandGroup
    {
      name    : "CMDS".into(),
      entries : vec![ CommandEntry { name : "cmd-one".into(), desc : "do one thing".into() } ],
    },
  ];
  let out = CliHelpTemplate::new( style, data ).render();
  assert!(
    out.contains( "    cmd-one    do one thing" ),
    "FT-31: col_gap=4 must produce 4 spaces between padded name column and description, got:\n{out}",
  );
}

// ── T-B12 ─ custom cmd_indent leading indent ──────────────────────────────────

/// T-B12 (FT-32): `cmd_indent=2` produces 2-space leading indent for command lines.
/// With `cmd_name_width=3` and `col_gap=2` (default), a command named `"run"` (3 chars,
/// exact fit) renders as `"  run  run the app"` — 2-indent, 3-name, 2-gap.
///
/// Contrasts with the default `cmd_indent=4` which would produce
/// `"    run  run the app"`.
#[ test ]
fn test_cmd_indent_custom()
{
  let style = CliHelpStyle { cmd_indent : 2, cmd_name_width : 3, tty_detect : false, ..CliHelpStyle::default() };
  let mut data = CliHelpData::default();
  data.groups = vec!
  [
    CommandGroup
    {
      name    : "CMDS".into(),
      entries : vec![ CommandEntry { name : "run".into(), desc : "run the app".into() } ],
    },
  ];
  let out = CliHelpTemplate::new( style, data ).render();
  assert!(
    out.contains( "  run  run the app" ),
    "FT-32: cmd_indent=2 must produce 2-space leading indent instead of default 4, got:\n{out}",
  );
}

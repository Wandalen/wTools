//! Tests for the `include_md_section!` proc-macro.
//!
//! ## Covered spec cases
//!
//! | Spec | Case | Assertion |
//! |------|------|-----------|
//! | `docs/feature/002_section_extraction.md` | Top-level section | Heading + nested subsections included |
//! | `docs/invariant/004_section_extraction_rules.md` | Level-aware boundary | H2 subsection stops at next H1 |
//! | `docs/invariant/004_section_extraction_rules.md` | First occurrence wins | Duplicate heading returns first |
//! | `docs/feature/002_section_extraction.md` | Subsection extraction | H2 stops at sibling H2 |
//! | `docs/invariant/004_section_extraction_rules.md` | Case-sensitive match | Wrong case → compile error |
//! | `docs/invariant/002_compile_time_errors.md` | Missing file | Compile-time error |
//! | `docs/invariant/002_compile_time_errors.md` | Heading not found | Compile-time error |
//! | `docs/api/002_include_md_section.md` | No arguments | Compile-time error |
//! | `docs/api/002_include_md_section.md` | One argument | Compile-time error |
//! | `docs/api/002_include_md_section.md` | Three arguments | Compile-time error |
//! | `docs/feature/002_section_extraction.md` | Code block (backtick) | Heading inside backtick fence not a boundary (BUG-005) |
//! | `docs/feature/002_section_extraction.md` | Code block (tilde) | Heading inside tilde fence not a boundary (BUG-005) |
//! | `docs/feature/002_section_extraction.md` | Empty section body | Section with no body returns heading + blank line |
//! | `docs/invariant/004_section_extraction_rules.md` | ATX no-space not heading | ##NoSpace line is plain content |
//! | `docs/invariant/004_section_extraction_rules.md` | Setext not heading | Underline-style heading is plain content |

#![ cfg( feature = "enabled" ) ]

use include_md ::include_md_section;
use std ::path ::PathBuf;

// ------------------------------------------------------------------ positive

/// Top-level section includes all nested subsections until the next same-level heading.
#[ test ]
fn top_level_section_includes_nested_subsections()
{
  let section = include_md_section!( "tests/fixture/multi_section.md", "# Introduction" );
  let expected = concat!(
    "# Introduction\n",
    "\n",
    "Welcome to the test fixture.\n",
    "\n",
    "## Overview\n",
    "\n",
    "This subsection is under Introduction.\n",
    "\n",
    "### Detail\n",
    "\n",
    "A nested subsection.\n",
    "\n",
  );
  assert_eq!( section, expected, "# Introduction must include all nested subsections" );
}

/// Subsection extraction stops at the next sibling or parent heading.
#[ test ]
fn subsection_stops_at_next_equal_or_higher_level_heading()
{
  let section = include_md_section!( "tests/fixture/multi_section.md", "## Overview" );
  let expected = concat!(
    "## Overview\n",
    "\n",
    "This subsection is under Introduction.\n",
    "\n",
    "### Detail\n",
    "\n",
    "A nested subsection.\n",
    "\n",
  );
  assert_eq!( section, expected, "## Overview must end before # Usage (higher level)" );
}

/// Immediate sibling H2 correctly terminates extraction of an H2 section.
#[ test ]
fn h2_section_stops_at_sibling_h2()
{
  let section = include_md_section!( "tests/fixture/multi_section.md", "## Installation" );
  let expected = concat!(
    "## Installation\n",
    "\n",
    "Run cargo add.\n",
    "\n",
  );
  assert_eq!( section, expected, "## Installation must stop before ## Configuration" );
}

/// When a heading appears more than once, the first occurrence is returned.
#[ test ]
fn first_occurrence_wins_for_duplicate_heading()
{
  let section = include_md_section!( "tests/fixture/multi_section.md", "# Notes" );
  let expected = concat!(
    "# Notes\n",
    "\n",
    "Final section with no subsections.\n",
    "\n",
  );
  assert_eq!( section, expected, "first # Notes must be returned, not the second" );
  assert!
  (
    !section.contains( "second Notes section" ),
    "second # Notes occurrence must not appear in result",
  );
}

/// Section with no subsections returns only its own content.
#[ test ]
fn section_with_no_subsections_returns_own_content_only()
{
  // The last section has no sub-headings and no following heading — reaches EOF.
  // Since the fixture's second # Notes is the final section, extract it via the
  // fixture's second section content by checking ## Configuration.
  let section = include_md_section!( "tests/fixture/multi_section.md", "## Configuration" );
  let expected = concat!(
    "## Configuration\n",
    "\n",
    "Set options here.\n",
    "\n",
  );
  assert_eq!( section, expected, "## Configuration must stop before # Notes" );
}

/// `bug_reproducer(BUG-005)` — Heading inside a backtick fenced code block must not terminate section.
///
/// ## Bug: BUG-005 — `code_block_heading_not_a_boundary`
///
/// **Root Cause:** `extract_section` had no `in_code_block` state. A line like `## x` inside
/// a fenced block was passed to `heading_level()` unconditionally, producing `Some(2)`, which
/// satisfied `level <= target_level` and caused premature `break`.
///
/// **Why Not Caught:** No fixture contained a fenced code block with a heading-like line at
/// the same depth as the target section. All prior fixtures used plain-text sections only.
///
/// **Fix Applied:** Added `in_code_block: bool` local in `extract_section`; toggled on lines
/// starting with ` ``` ` or `~~~`; heading boundary check skipped while flag is true.
///
/// **Prevention:** Always track fenced code block state when scanning markdown for headings.
/// Any new test fixture for section extraction should include at least one code-fenced section.
///
/// **Pitfall:** Fenced code blocks can contain heading-like lines. Heading detection without
/// fence-state tracking produces silent wrong-value truncation — the macro still compiles.
#[ test ]
fn code_block_heading_not_a_boundary()
{
  let section = include_md_section!( "tests/fixture/edge_cases.md", "## H2 With Code Block" );
  let expected = concat!(
    "## H2 With Code Block\n",
    "\n",
    "Content before code block.\n",
    "\n",
    "```rust\n",
    "## this looks like H2 but is code\n",
    "```\n",
    "\n",
    "Content after code block.\n",
    "\n",
  );
  assert_eq!(
    section,
    expected,
    "heading inside backtick code block must not terminate section extraction (BUG-005)",
  );
}

/// `bug_reproducer(BUG-005)` — Heading inside a tilde fenced code block must not terminate section.
///
/// Tilde fence variant of the same code block boundary bug (BUG-005).
/// See `code_block_heading_not_a_boundary` for full root cause analysis.
#[ test ]
fn tilde_fence_heading_not_a_boundary()
{
  let section = include_md_section!( "tests/fixture/edge_cases.md", "## H2 With Tilde Fence" );
  let expected = concat!(
    "## H2 With Tilde Fence\n",
    "\n",
    "Content before tilde.\n",
    "\n",
    "~~~python\n",
    "## tilde fence heading\n",
    "~~~\n",
    "\n",
    "Content after tilde.\n",
    "\n",
  );
  assert_eq!(
    section,
    expected,
    "heading inside tilde code block must not terminate section extraction (BUG-005)",
  );
}

/// Empty section body: a section with no body lines returns the heading plus the trailing blank line.
#[ test ]
fn empty_section_body_returns_heading_and_blank()
{
  // `# Empty Section` in edge_cases.md is immediately followed by a blank line then the next H1.
  let section = include_md_section!( "tests/fixture/edge_cases.md", "# Empty Section" );
  assert_eq!(
    section,
    "# Empty Section\n\n",
    "section with no body must return heading plus the blank line before the next heading",
  );
}

/// ATX heading without a space separator (`##NoSpace`) is plain content, not a section boundary.
#[ test ]
fn atx_no_space_not_a_heading_boundary()
{
  let section = include_md_section!( "tests/fixture/edge_cases.md", "# ATX No Space Section" );
  let expected = concat!(
    "# ATX No Space Section\n",
    "\n",
    "##NotAHeading here.\n",
    "\n",
    "Normal text.\n",
    "\n",
  );
  assert_eq!(
    section,
    expected,
    "##NoSpace (no space after hashes) is not a valid ATX heading and must be treated as content",
  );
}

/// Setext-style underline headings (`===` or `---` beneath text) are not recognized as ATX
/// headings and must not terminate section extraction.
#[ test ]
fn setext_heading_not_a_boundary()
{
  // `# Setext Style Section` is the last section in edge_cases.md; it contains setext underlines.
  let section = include_md_section!( "tests/fixture/edge_cases.md", "# Setext Style Section" );
  let expected = concat!(
    "# Setext Style Section\n",
    "\n",
    "Normal text.\n",
    "\n",
    "Setext Text\n",
    "==========\n",
    "\n",
    "More content after setext.\n",
  );
  assert_eq!(
    section,
    expected,
    "setext-style heading underlines must not be mistaken for ATX headings or section boundaries",
  );
}

// ------------------------------------------------------------------ compile-fail helpers

/// Spawn `cargo check` on `code` with `features` enabled on `include_md`.
/// Returns `true` when compilation fails — the expected outcome for compile-fail cases.
fn check_compile_fails( code : &str, features : &[ &str ] ) -> bool
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  let target_root = PathBuf ::from( manifest_dir ).join( "target" ).join( "compile_fail_tests_section_extraction" );
  let src_dir = target_root.join( format!( "src_{}", std ::process ::id() ) );
  std ::fs ::create_dir_all( src_dir.join( "src" ) ).expect( "create temp src dir" );

  let features_str = features
    .iter()
    .map( | f | format!( "\"{f}\"" ) )
    .collect ::< Vec< _ > >()
    .join( ", " );

  let cargo_toml = format!(
    "[package]\n\
     name = \"compile_fail_check\"\n\
     version = \"0.1.0\"\n\
     edition = \"2021\"\n\
     \n\
     [dependencies]\n\
     include_md = {{ path = \"{manifest_dir}\", default-features = false, features = [{features_str}] }}\n",
  );

  std ::fs ::write( src_dir.join( "Cargo.toml" ), &cargo_toml ).expect( "write temp Cargo.toml" );
  std ::fs ::write( src_dir.join( "src/main.rs" ), code ).expect( "write temp main.rs" );

  let cargo = std ::env ::var( "CARGO" ).unwrap_or_else( | _ | "cargo".to_owned() );
  let output = std ::process ::Command ::new( &cargo )
    .args
    ( [
      "check",
      "--manifest-path",
      &src_dir.join( "Cargo.toml" ).to_string_lossy(),
    ] )
    .env( "CARGO_TARGET_DIR", target_root.join( "build" ) )
    .output()
    .expect( "cargo check invocation failed" );

  let _ = std ::fs ::remove_dir_all( &src_dir );
  !output.status.success()
}

// ------------------------------------------------------------------ compile-fail

/// Non-existent file is rejected at compile time.
#[ test ]
fn missing_file_is_compile_error()
{
  assert!(
    check_compile_fails
    (
      "fn main() { let _ = include_md ::include_md_section!( \"does_not_exist.md\", \"# X\" ); }",
      &[ "enabled" ],
    ),
    "include_md_section! must not compile when the file does not exist",
  );
}

/// Heading absent from the file is rejected at compile time.
#[ test ]
fn heading_not_found_is_compile_error()
{
  // Pass the absolute path to the fixture so the proc-macro can find it from the
  // temp project's CARGO_MANIFEST_DIR, regardless of where the temp dir lives.
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  let abs_fixture = format!( "{manifest_dir}/tests/fixture/multi_section.md" );
  let code = format!(
    "fn main() {{ let _ = include_md ::include_md_section!( \"{abs_fixture}\", \"## DoesNotExist__XYZ\" ); }}"
  );
  assert!(
    check_compile_fails( &code, &[ "enabled" ] ),
    "include_md_section! must not compile when the heading is not found",
  );
}

/// Wrong heading case is rejected at compile time (case-sensitive matching).
#[ test ]
fn wrong_case_heading_is_compile_error()
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  let abs_fixture = format!( "{manifest_dir}/tests/fixture/multi_section.md" );
  // Fixture has "# Introduction" (capital I); lowercase should not match.
  let code = format!(
    "fn main() {{ let _ = include_md ::include_md_section!( \"{abs_fixture}\", \"# introduction\" ); }}"
  );
  assert!(
    check_compile_fails( &code, &[ "enabled" ] ),
    "include_md_section! must not compile when heading case does not match",
  );
}

/// Zero arguments is rejected at compile time.
#[ test ]
fn no_args_is_compile_error()
{
  assert!(
    check_compile_fails
    (
      "fn main() { let _ = include_md ::include_md_section!(); }",
      &[ "enabled" ],
    ),
    "include_md_section! must not compile with no arguments",
  );
}

/// One argument (missing heading) is rejected at compile time.
#[ test ]
fn one_arg_is_compile_error()
{
  assert!(
    check_compile_fails
    (
      "fn main() { let _ = include_md ::include_md_section!( \"fixture/multi_section.md\" ); }",
      &[ "enabled" ],
    ),
    "include_md_section! must not compile with only one argument",
  );
}

/// Three arguments is rejected at compile time.
#[ test ]
fn three_args_is_compile_error()
{
  assert!(
    check_compile_fails
    (
      "fn main() { let _ = include_md ::include_md_section!( \"a.md\", \"# X\", \"extra\" ); }",
      &[ "enabled" ],
    ),
    "include_md_section! must not compile with three arguments",
  );
}

/// File exceeding 10 MB is rejected at compile time via the size check in the proc-macro.
#[ test ]
fn oversized_file_is_compile_error()
{
  use std ::io ::Write;

  let tmp_dir = std ::env ::temp_dir().join
  (
    format!( "include_md_section_oversized_{}", std ::process ::id() )
  );
  std ::fs ::create_dir_all( &tmp_dir ).expect( "create temp dir" );
  let big_file = tmp_dir.join( "big.md" );

  {
    let mut f = std ::fs ::File ::create( &big_file ).expect( "create big file" );
    // Write just over 10 MB so the content.len() check fires inside the proc-macro.
    let chunk = vec![ b'a'; 65_536 ];
    let mut written = 0_usize;
    while written < 10_000_001
    {
      let n = ( 10_000_001 - written ).min( chunk.len() );
      f.write_all( &chunk[ ..n ] ).expect( "write chunk" );
      written += n;
    }
  }

  // Use an absolute path; include_md_section! resolves relative to CARGO_MANIFEST_DIR,
  // but an absolute path is always unambiguous regardless of the temp project's root.
  let abs = big_file.to_string_lossy().into_owned();
  let code = format!
  (
    "fn main() {{ let _ = include_md ::include_md_section!( \"{abs}\", \"# Heading\" ); }}"
  );
  let result = check_compile_fails( &code, &[ "enabled" ] );
  let _ = std ::fs ::remove_dir_all( &tmp_dir );
  assert!( result, "include_md_section! must not compile when the file exceeds 10 MB" );
}

/// File with invalid UTF-8 bytes is rejected at compile time via `fs::read_to_string`.
#[ test ]
fn invalid_utf8_is_compile_error()
{
  let tmp_dir = std ::env ::temp_dir().join
  (
    format!( "include_md_section_utf8_{}", std ::process ::id() )
  );
  std ::fs ::create_dir_all( &tmp_dir ).expect( "create temp dir" );
  let invalid_file = tmp_dir.join( "invalid.bin" );
  // 0xFF is never valid in UTF-8; `read_to_string` will fail, causing a compile-time error.
  std ::fs ::write( &invalid_file, [ 0xFF_u8, 0xFE, 0x00 ] ).expect( "write invalid UTF-8" );

  let abs = invalid_file.to_string_lossy().into_owned();
  let code = format!
  (
    "fn main() {{ let _ = include_md ::include_md_section!( \"{abs}\", \"# Heading\" ); }}"
  );
  let result = check_compile_fails( &code, &[ "enabled" ] );
  let _ = std ::fs ::remove_dir_all( &tmp_dir );
  assert!( result, "include_md_section! must not compile with an invalid UTF-8 file" );
}

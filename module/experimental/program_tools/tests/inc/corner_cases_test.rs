//! Corner case tests for `program_tools` data structures.
//!
//! Validates builder API behavior across edge cases, empty inputs, large data,
//! special characters, and various structural configurations.
//!
//! ## Test Categories
//!
//! - **SOURCE Corner Cases**: Empty fields, large data, special characters
//! - **PROGRAM Corner Cases**: Zero sources, single source, three sources, insertion order, duplicates, manifest field
//! - **PLAN Corner Cases**: Minimal configurations, `run_options` stored vs `None`
//! - **DEBUG Trait**: Formatting validation
//! - **NAMESPACE**: Import path validation
//! - **EXPLICIT Parameters**: Default vs explicit behavior
//! - **`CapturedOutput` Corner Cases**: Default values, lossy UTF-8, empty needle, clone independence, assertion no-panics
//! - **`RunOptions` Corner Cases**: Sentinel defaults, clone independence, Debug formatting

#[ allow( unused_imports ) ]
use super::*;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// SOURCE Corner Cases
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test SOURCE-001: Source with empty `file_path`.
///
/// Validates that Source builder accepts empty string for `file_path` field
/// without panic or validation errors.
#[ test ]
fn source_empty_file_path()
{
  use the_module::program;

  let source = program::Source::former()
    .file_path( "" )
    .data( "fn main() {}" )
    .form();

  assert_eq!( source.file_path, "" );
  assert_eq!( source.data, "fn main() {}" );
}

/// Test SOURCE-002: Source with empty data.
///
/// Validates that Source builder accepts empty string for data field
/// without panic or validation errors.
#[ test ]
fn source_empty_data()
{
  use the_module::program;

  let source = program::Source::former()
    .file_path( "main.rs" )
    .data( "" )
    .form();

  assert_eq!( source.file_path, "main.rs" );
  assert_eq!( source.data, "" );
}

/// Test SOURCE-003: Source with both fields empty.
///
/// Validates that Source builder accepts empty strings for all fields
/// without panic or validation errors.
#[ test ]
fn source_both_fields_empty()
{
  use the_module::program;

  let source = program::Source::former()
    .file_path( "" )
    .data( "" )
    .form();

  assert_eq!( source.file_path, "" );
  assert_eq!( source.data, "" );
}

/// Test SOURCE-004: Source with large data (1MB string).
///
/// Validates that Source builder handles large data payloads without
/// memory issues or performance degradation.
#[ test ]
fn source_large_data()
{
  use the_module::program;

  let large_data = "// comment\n".repeat( 50_000 ); // ~550KB
  let expected_len = large_data.len();

  let source = program::Source::former()
    .file_path( "large.rs" )
    .data( &large_data )
    .form();

  assert_eq!( source.file_path, "large.rs" );
  assert_eq!( source.data.len(), expected_len );
  assert!( source.data.starts_with( "// comment\n" ) );
}

/// Test SOURCE-005: Source with special characters (Unicode, newlines, tabs).
///
/// Validates that Source builder preserves exact Unicode characters,
/// escape sequences, and special characters without modification.
#[ test ]
fn source_special_characters()
{
  use the_module::program;

  let source = program::Source::former()
    .file_path( "src/模块.rs" ) // Unicode Chinese
    .data( "fn main() {\n\tprintln!(\"🦀 Rust\");\n}" )
    .form();

  assert_eq!( source.file_path, "src/模块.rs" );
  assert_eq!( source.data, "fn main() {\n\tprintln!(\"🦀 Rust\");\n}" );
  assert!( source.data.contains( "🦀" ) ); // Verify emoji preserved
  assert!( source.data.contains( '\t' ) ); // Verify tab preserved
  assert!( source.data.contains( '\n' ) ); // Verify newline preserved
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// PROGRAM Corner Cases
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test PROGRAM-001: Program with zero sources (empty program).
///
/// Validates that Program builder constructs valid empty program with
/// zero sources in the collection.
#[ test ]
fn program_zero_sources()
{
  use the_module::program;

  let program = program::Program::former().form();

  assert_eq!( program.source.len(), 0 );
  assert!( program.source.is_empty() );
}

/// Test PROGRAM-002: Program with single source.
///
/// Validates that Program builder correctly constructs program with
/// exactly one source file.
#[ test ]
fn program_single_source()
{
  use the_module::program;

  let program = program::Program::former()
    .source()
      .file_path( "main.rs" )
      .data( "fn main() {}" )
      .end()
    .form();

  assert_eq!( program.source.len(), 1 );
  assert_eq!( program.source[ 0 ].file_path, "main.rs" );
  assert_eq!( program.source[ 0 ].data, "fn main() {}" );
}

/// Test PROGRAM-004: Program with duplicate file paths.
///
/// Validates that Program builder accepts multiple sources with identical
/// `file_path` values without validation errors (no uniqueness constraint).
#[ test ]
fn program_duplicate_file_paths()
{
  use the_module::program;

  let program = program::Program::former()
    .source()
      .file_path( "main.rs" )
      .data( "// version 1" )
      .end()
    .source()
      .file_path( "main.rs" ) // Duplicate path
      .data( "// version 2" )
      .end()
    .form();

  assert_eq!( program.source.len(), 2 );
  assert_eq!( program.source[ 0 ].file_path, "main.rs" );
  assert_eq!( program.source[ 1 ].file_path, "main.rs" );
  assert_eq!( program.source[ 0 ].data, "// version 1" );
  assert_eq!( program.source[ 1 ].data, "// version 2" );
}

/// Test PROGRAM-003: Program with three sources.
///
/// Validates that Program builder correctly accumulates three sources
/// and all are accessible by index.
#[ test ]
fn program_three_sources()
{
  use the_module::program;

  let program = program::Program::former()
    .source()
      .file_path( "a.rs" )
      .data( "// a" )
      .end()
    .source()
      .file_path( "b.rs" )
      .data( "// b" )
      .end()
    .source()
      .file_path( "c.rs" )
      .data( "// c" )
      .end()
    .form();

  assert_eq!( program.source.len(), 3 );
  assert_eq!( program.source[ 0 ].file_path, "a.rs" );
  assert_eq!( program.source[ 1 ].file_path, "b.rs" );
  assert_eq!( program.source[ 2 ].file_path, "c.rs" );
}

/// Test PROGRAM-005: Insertion order preserved across three sources.
///
/// Validates that the source collection retains the order in which sources
/// were added via successive `.source()` builder calls.
#[ test ]
fn program_insertion_order_preserved()
{
  use the_module::program;

  let program = program::Program::former()
    .source()
      .file_path( "first.rs" )
      .data( "// first" )
      .end()
    .source()
      .file_path( "second.rs" )
      .data( "// second" )
      .end()
    .source()
      .file_path( "third.rs" )
      .data( "// third" )
      .end()
    .form();

  assert_eq!( program.source[ 0 ].file_path, "first.rs" );
  assert_eq!( program.source[ 1 ].file_path, "second.rs" );
  assert_eq!( program.source[ 2 ].file_path, "third.rs" );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// PLAN Corner Cases
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test PLAN-001: Minimal plan (program with zero sources).
///
/// Validates that Plan builder constructs valid plan containing empty program
/// with zero sources.
#[ test ]
fn plan_minimal_with_empty_program()
{
  use the_module::program;

  let plan = program::Plan::former()
    .program()
      // No sources added
      .end()
    .form();

  assert_eq!( plan.program.source.len(), 0 );
  assert!( plan.program.source.is_empty() );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// DEBUG Trait Validation
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test DEBUG-001: Debug formatting for all structs.
///
/// Validates that Source, Program, and Plan all implement Debug trait
/// and format without panic across various configurations.
#[ test ]
fn debug_trait_all_structs()
{
  use the_module::program;

  let source = program::Source::former()
    .file_path( "test.rs" )
    .data( "code" )
    .form();

  let program = program::Program::former()
    .source()
      .file_path( "main.rs" )
      .data( "fn main() {}" )
      .end()
    .form();

  let plan = program::Plan::former()
    .program()
      .source()
        .file_path( "lib.rs" )
        .data( "pub fn test() {}" )
        .end()
      .end()
    .form();

  // All Debug formatting should succeed without panic
  let source_debug = format!( "{source:?}" );
  let program_debug = format!( "{program:?}" );
  let plan_debug = format!( "{plan:?}" );

  // Verify non-empty debug output
  assert!( !source_debug.is_empty() );
  assert!( !program_debug.is_empty() );
  assert!( !plan_debug.is_empty() );

  // Verify struct names appear in debug output
  assert!( source_debug.contains( "Source" ) );
  assert!( program_debug.contains( "Program" ) );
  assert!( plan_debug.contains( "Plan" ) );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// NAMESPACE Validation
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test NAMESPACE-001: Exposed module namespace accessibility.
///
/// Validates that Source, Program, and Plan types are accessible via
/// `the_module::program` import path (the exposed module).
#[ test ]
fn namespace_exposed_module_imports()
{
  // Validates compilation: if it compiles, module namespace is correct.

  use the_module::program;

  let _source: program::Source = program::Source::former()
    .file_path( "test.rs" )
    .data( "code" )
    .form();

  let _program: program::Program = program::Program::former()
    .form();

  let _plan: program::Plan = program::Plan::former()
    .program()
      .end()
    .form();
}

/// Test NAMESPACE-002: Prelude namespace accessibility.
///
/// Validates that Source, Program, and Plan types are accessible via
/// `program_tools::prelude::*` import path.
#[ test ]
fn namespace_prelude_imports()
{
  // Note: This test validates compilation, not runtime behavior.
  // If it compiles, prelude exports are correct.

  use the_module::prelude::*;

  let _source: Source = Source::former()
    .file_path( "test.rs" )
    .data( "code" )
    .form();

  let _program: Program = Program::former()
    .form();

  let _plan: Plan = Plan::former()
    .program()
      .end()
    .form();

  // Compilation success validates prelude namespace
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// EXPLICIT Parameters
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test EXPLICIT-001: Default `form()` equivalent to explicit `form()`.
///
/// Validates that calling `.form()` without parameters produces same result
/// as any future explicit parameter variations.
#[ test ]
fn explicit_form_default_equivalence()
{
  use the_module::program;

  let source1 = program::Source::former()
    .file_path( "test.rs" )
    .data( "code" )
    .form();

  let source2 = program::Source::former()
    .file_path( "test.rs" )
    .data( "code" )
    .form();

  // Both should produce identical structures
  assert_eq!( source1.file_path, source2.file_path );
  assert_eq!( source1.data, source2.data );
}

/// Test EXPLICIT-002: Explicit parameter specification.
///
/// Validates that all builder methods accept explicit parameters and
/// produce expected field values.
#[ test ]
fn explicit_parameters_all_fields()
{
  use the_module::program;

  let source = program::Source::former()
    .file_path( "explicit.rs" )
    .data( "explicit data" )
    .form();

  assert_eq!( source.file_path, "explicit.rs" );
  assert_eq!( source.data, "explicit data" );

  let program = program::Program::former()
    .source()
      .file_path( "file1.rs" )
      .data( "data1" )
      .end()
    .source()
      .file_path( "file2.rs" )
      .data( "data2" )
      .end()
    .form();

  assert_eq!( program.source.len(), 2 );
  assert_eq!( program.source[ 0 ].file_path, "file1.rs" );
  assert_eq!( program.source[ 1 ].file_path, "file2.rs" );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// CapturedOutput Corner Cases
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test CAPTURED-001: `CapturedOutput::default()` has zero `exit_status` and empty buffers.
///
/// Validates that the derived Default impl initialises all fields to their
/// zero values — no magic defaults are silently applied.
#[ test ]
fn captured_output_default_has_zero_values()
{
  use the_module::CapturedOutput;

  let out = CapturedOutput::default();
  assert_eq!( out.exit_status, 0 );
  assert!( out.stdout.is_empty() );
  assert!( out.stderr.is_empty() );
}

/// Test CAPTURED-002: `stdout_str` on invalid UTF-8 bytes uses lossy replacement.
///
/// Bytes 0xFF and 0xFE are not valid UTF-8. `from_utf8_lossy` replaces them
/// with the Unicode replacement character (U+FFFD) rather than panicking.
#[ test ]
fn captured_output_lossy_utf8_stdout()
{
  use the_module::CapturedOutput;

  // 0xFF is never valid UTF-8; lossy decode must not panic
  let out = CapturedOutput { exit_status : 0, stdout : vec![ 0xFF, 0xFE ], stderr : vec![] };
  let decoded = out.stdout_str();
  // The string should be non-empty (replacement chars) and must not contain the raw bytes
  assert!( !decoded.is_empty(), "lossy decode should produce a non-empty string" );
  // Replacement char U+FFFD appears for each invalid byte sequence
  assert!( decoded.contains( '\u{FFFD}' ), "expected replacement char for invalid UTF-8" );
}

/// Test CAPTURED-003: `stderr_str` on invalid UTF-8 bytes uses lossy replacement.
#[ test ]
fn captured_output_lossy_utf8_stderr()
{
  use the_module::CapturedOutput;

  let out = CapturedOutput { exit_status : 1, stdout : vec![], stderr : vec![ 0xC0, 0x80 ] };
  let decoded = out.stderr_str();
  assert!( !decoded.is_empty() );
  assert!( decoded.contains( '\u{FFFD}' ) );
}

/// Test CAPTURED-004: `stdout_contains` with an empty needle always returns true.
///
/// This is Rust's standard `str::contains` semantics — every string contains
/// the empty string.  Documents this edge case explicitly so callers are not
/// surprised.
#[ test ]
fn captured_output_stdout_contains_empty_needle()
{
  use the_module::CapturedOutput;

  let out = CapturedOutput { exit_status : 0, stdout : b"hello".to_vec(), stderr : vec![] };
  assert!( out.stdout_contains( "" ), "empty needle must always match (str::contains semantics)" );

  let empty_out = CapturedOutput { exit_status : 0, stdout : vec![], stderr : vec![] };
  assert!( empty_out.stdout_contains( "" ), "empty needle on empty stdout must also match" );
}

/// Test CAPTURED-005: `stderr_eq` returns true for exact match, false otherwise.
///
/// Mirrors the coverage in PRED-001 but for stderr, which was previously untested.
#[ test ]
fn captured_output_stderr_eq_exact_match()
{
  use the_module::CapturedOutput;

  let out = CapturedOutput
  {
    exit_status : 1,
    stdout : vec![],
    stderr : b"error\n".to_vec(),
  };
  assert!( out.stderr_eq( "error\n" ) );
  assert!( !out.stderr_eq( "error" ) );   // missing newline
  assert!( !out.stderr_eq( "other\n" ) ); // different content
}

/// Test CAPTURED-006: `assert_exit_ok` does not panic when exit status is zero.
///
/// Verifies the success path of the assertion method — panics only on non-zero.
#[ test ]
fn captured_output_assert_exit_ok_no_panic()
{
  use the_module::CapturedOutput;

  let out = CapturedOutput { exit_status : 0, stdout : vec![], stderr : vec![] };
  out.assert_exit_ok(); // must not panic
}

/// Test CAPTURED-007: `assert_stdout_empty` and `assert_stderr_empty` do not panic on empty buffers.
#[ test ]
fn captured_output_assert_empty_no_panic()
{
  use the_module::CapturedOutput;

  let out = CapturedOutput { exit_status : 0, stdout : vec![], stderr : vec![] };
  out.assert_stdout_empty(); // must not panic
  out.assert_stderr_empty(); // must not panic
}

/// Test CAPTURED-008: `CapturedOutput` implements `Clone` and produces an independent copy.
///
/// Mutating the clone must not affect the original.
#[ test ]
fn captured_output_clone_is_independent()
{
  use the_module::CapturedOutput;

  let original = CapturedOutput { exit_status : 42, stdout : b"hi\n".to_vec(), stderr : vec![] };
  let mut cloned = original.clone();
  cloned.exit_status = 0;
  cloned.stdout.clear();

  assert_eq!( original.exit_status, 42 );
  assert_eq!( original.stdout, b"hi\n" );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// RunOptions Corner Cases
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test RUNOPTS-001: `RunOptions::default()` has sensible sentinel values.
///
/// Empty strings act as "use default" sentinels — they are replaced at run
/// time by the runner's `effective_*` helpers. Validates the contract.
#[ test ]
fn run_options_default_sentinel_values()
{
  use the_module::RunOptions;

  let opts = RunOptions::default();
  assert_eq!( opts.build_profile, "" );
  assert_eq!( opts.cargo_path, "" );
  assert_eq!( opts.edition, "" );
  assert_eq!( opts.package_name, "" );
  assert!( opts.target_dir.is_none() );
  assert!( opts.timeout_ms.is_none() );
  assert!( opts.features.is_empty() );
  assert!( opts.env_vars.is_empty() );
  assert!( opts.capture, "capture must default to true" );
  assert!( opts.cleanup, "cleanup must default to true" );
}

/// Test RUNOPTS-002: `RunOptions` implements `Clone` and produces an independent copy.
#[ test ]
fn run_options_clone_is_independent()
{
  use the_module::RunOptions;

  let original = RunOptions { timeout_ms : Some( 5_000 ), capture : false, ..Default::default() };
  let mut cloned = original.clone();
  cloned.timeout_ms = None;
  cloned.capture = true;

  assert_eq!( original.timeout_ms, Some( 5_000 ) );
  assert!( !original.capture );
}

/// Test RUNOPTS-003: `RunOptions` implements `Debug` and formats without panic.
#[ test ]
fn run_options_debug_formatting()
{
  use the_module::RunOptions;

  let opts = RunOptions::default();
  let formatted = format!( "{opts:?}" );
  assert!( !formatted.is_empty() );
  assert!( formatted.contains( "RunOptions" ) );
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Program manifest field
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Test PROGRAM-006: Program manifest field stores and retrieves custom TOML.
///
/// When `manifest` is `Some`, the runner uses it verbatim instead of generating
/// a default. This test validates the builder sets the field correctly.
#[ test ]
fn program_manifest_field_stored()
{
  use the_module::program;

  let toml = "[package]\nname = \"custom\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n";
  let prog = program::Program::former()
    .manifest( toml.to_string() )
    .form();

  assert_eq!( prog.manifest.as_deref(), Some( toml ) );
}

/// Test PROGRAM-007: Program manifest field is `None` by default (runner generates it).
#[ test ]
fn program_manifest_defaults_to_none()
{
  use the_module::program;

  let prog = program::Program::former().form();
  assert!( prog.manifest.is_none() );
}

/// Test PLAN-002: Plan with `run_options` set stores the options.
#[ test ]
fn plan_with_run_options_stored()
{
  use the_module::{ program, RunOptions };

  let opts = RunOptions { timeout_ms : Some( 1_000 ), capture : false, ..Default::default() };
  let plan = program::Plan::former()
    .program()
      .end()
    .run_options( opts )
    .form();

  let stored = plan.run_options.expect( "run_options must be Some when explicitly set" );
  assert_eq!( stored.timeout_ms, Some( 1_000 ) );
  assert!( !stored.capture );
}

/// Test PLAN-003: Plan without `run_options` has `None` (runner uses Default).
#[ test ]
fn plan_without_run_options_is_none()
{
  use the_module::program;

  let plan = program::Plan::former()
    .program()
      .end()
    .form();

  assert!( plan.run_options.is_none() );
}

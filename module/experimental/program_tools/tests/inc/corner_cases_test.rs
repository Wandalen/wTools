//! Corner case tests for `program_tools` data structures.
//!
//! Validates builder API behavior across edge cases, empty inputs, large data,
//! special characters, and various structural configurations.
//!
//! ## Test Categories
//!
//! - **SOURCE Corner Cases**: Empty fields, large data, special characters
//! - **PROGRAM Corner Cases**: Zero sources, single source, three sources, insertion order, duplicates
//! - **PLAN Corner Cases**: Minimal configurations
//! - **DEBUG Trait**: Formatting validation
//! - **NAMESPACE**: Import path validation
//! - **EXPLICIT Parameters**: Default vs explicit behavior

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

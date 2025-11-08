//!
//! Bug reproduction and documentation for publish command PathBuf casting panic.
//!
//! # Bug Description
//!
//! When running `will .publish dry:0` (or any argument without space after colon),
//! the command panics with:
//! ```text
//! thread 'main' panicked at module\core\wca\src\ca\grammar\types.rs:152:3:
//! Unknown cast variant. Got `List([String("dry:0")])` and try to cast to `std ::path ::PathBuf`
//! ```
//!
//! # Root Cause
//!
//! The publish command's subject is defined as `Type::List(Type::String, ',')` in the
//! command grammar (see `src/command/mod.rs:23`), meaning it expects a comma-separated
//! list of path strings. However, `src/command/publish.rs:41` incorrectly attempts to
//! cast this List directly to a PathBuf using `.get_owned::<PathBuf>(0)`.
//!
//! When the user provides an argument like "dry:0" without proper spacing ("dry: 0"),
//! the wca parser doesn't recognize it as a property (which requires " : " pattern)
//! and instead treats it as a subject argument, wrapping it in a List per the command
//! definition. The subsequent attempt to cast this List to PathBuf triggers the panic
//! because wca's type system has no conversion from List to PathBuf.
//!
//! The issue is a type mismatch in the command implementation:
//! - Command grammar defines subject as `List<String>` (correct)
//! - Line 52 correctly retrieves it as `Vec<String>` (correct)
//! - Line 41 incorrectly tries to retrieve it as `PathBuf` (bug)
//!
//! # Why Not Caught
//!
//! This bug was not caught earlier because:
//!
//! 1. **No integration tests for command parsing**: Existing publish tests focus on
//!    the action layer logic (staleness detection, validation), not command-line
//!    argument parsing and type casting.
//!
//! 2. **Type system gap**: Rust's type system cannot catch this at compile time because
//!    `get_owned::<T>()` uses runtime type casting with a generic parameter. The mismatch
//!    between the command definition (List) and the cast target (PathBuf) is only
//!    detectable at runtime.
//!
//! 3. **Inconsistent usage patterns**: The same `args[0]` is accessed twice with different
//!    type expectations (PathBuf at line 41, Vec<String> at line 52). The PathBuf cast
//!    at line 41 was never actually needed - it's only used for display formatting via
//!    `.display()`, which could be achieved by formatting the Vec<String> directly.
//!
//! 4. **Property vs subject ambiguity**: Tests didn't cover malformed property syntax
//!    like "dry:0" (missing space) which causes the parser to misclassify the input
//!    as a subject rather than a property.
//!
//! # Fix Applied
//!
//! Remove the incorrect PathBuf cast at line 41 of `src/command/publish.rs`. The
//! `args_line` variable should be constructed from the properly-typed `patterns`
//! variable (which correctly gets `Vec<String>` from args[0]), or simply remove
//! the redundant `args_line` entirely if it's not needed.
//!
//! The fix involves:
//! 1. Removing the PathBuf cast at line 41
//! 2. Either constructing args_line from patterns, or removing args_line if unused
//! 3. Ensuring all access to args[0] uses the correct Vec<String> type
//!
//! # Prevention
//!
//! To prevent similar issues in the future:
//!
//! 1. **Add command-level integration tests**: Create tests that invoke commands through
//!    the wca parser layer, not just the action layer. This catches type mismatches
//!    between command definitions and implementations.
//!
//! 2. **Type safety at command definition**: Consider adding compile-time verification
//!    that command implementations access arguments with types matching the grammar
//!    definition. This could be achieved through procedural macros or builder patterns.
//!
//! 3. **Single source of truth for argument access**: Each command argument should be
//!    retrieved exactly once and stored in a properly-typed variable, avoiding multiple
//!    casts with different target types.
//!
//! 4. **Property syntax validation**: Add tests for common malformed input patterns
//!    like "prop:value" (missing space) to ensure graceful error messages instead of
//!    panics.
//!
//! # Pitfall
//!
//! The critical pitfall here is **inconsistent type expectations for the same argument**.
//! When a command argument is defined with a specific type in the grammar, ALL access
//! to that argument must use the matching type. Using different types (PathBuf vs Vec<String>)
//! for the same argument creates a runtime bomb that only detonates with specific input.
//!
//! Additional pitfall: **wca's generic get_owned::<T>() hides type mismatches**. Unlike
//! traditional function calls where type errors are caught at compile time, this generic
//! method defers type checking to runtime, making it easy to accidentally request the
//! wrong type without compiler warnings.
//!
//! **Lesson**: When working with dynamic command parsing frameworks, establish strict
//! conventions for argument access patterns and validate them through integration tests
//! that exercise the full parsing pipeline.

use super :: *;

// test_kind: bug_reproducer(issue-publish-pathbuf-cast)

#[ test ]
#[ should_panic( expected = "Unknown cast variant. Got `List([String(\"dry:0\")])` and try to cast to `std ::path ::PathBuf`" ) ]
fn publish_with_malformed_property_pathbuf_cast_panics()
{
  // This test demonstrates the panic that occurred in the original buggy code
  // when running `will .publish dry:0` where "dry:0" (without space) is
  // treated as a subject argument and then incorrectly cast to PathBuf

  use wca :: { CommandsAggregator, Type };

  // Recreate the BUGGY pattern - attempting to cast List subject to PathBuf
  let ca = CommandsAggregator ::former()
  .command( "publish" )
   .hint( "publish package" )
   .subject()
    .hint( "Paths to packages" )
    .kind( Type ::List( Type ::String.into(), ',' ) )
    .optional( true )
    .end()
   .property( "dry" )
    .hint( "Dry run flag" )
    .kind( Type ::Bool )
    .optional( true )
    .end()
   .routine( | o: wca ::VerifiedCommand | -> error_tools ::Result< () >
   {
    // BUGGY CODE: This demonstrates the original issue at publish.rs:41
    // Attempting to cast List to PathBuf causes panic
    let _result: std ::path ::PathBuf = o.args.get_owned( 0 )
     .unwrap_or_else( || std ::path ::PathBuf ::from( "" ) );

    Ok( () )
   })
   .end()
  .perform();

  let args = vec![ ".publish".to_string(), "dry:0".to_string() ];

  // This WILL panic, which is what we expect to demonstrate the bug
  ca.perform( args ).ok();
}

#[ test ]
fn publish_with_malformed_property_correct_type_handling()
{
  // This test demonstrates the CORRECT approach - using Vec<String>
  // to match the List command definition, which prevents the panic

  use wca :: { CommandsAggregator, Type };

  let ca = CommandsAggregator ::former()
  .command( "publish" )
   .hint( "publish package" )
   .subject()
    .hint( "Paths to packages" )
    .kind( Type ::List( Type ::String.into(), ',' ) )
    .optional( true )
    .end()
   .property( "dry" )
    .hint( "Dry run flag" )
    .kind( Type ::Bool )
    .optional( true )
    .end()
   .routine( | o: wca ::VerifiedCommand | -> error_tools ::Result< () >
   {
    // CORRECT CODE: Get Vec<String> matching the List definition
    let patterns: Vec< String > = o.args.get_owned( 0 )
     .unwrap_or_else( || vec![ "./".into() ] );

    // Now we can safely work with the patterns
    assert_eq!( patterns, vec![ "dry:0" ] );

    Ok( () )
   })
   .end()
  .perform();

  let args = vec![ ".publish".to_string(), "dry:0".to_string() ];

  // This should succeed without panic
  let result = ca.perform( args );
  assert!( result.is_ok(), "Should successfully handle subject argument with correct type" );
}

#[ test ]
fn publish_args_type_consistency()
{
  // This test verifies that args[0] is consistently accessed with the correct type
  // matching the command definition (List/Vec, not PathBuf)

  use wca :: { CommandsAggregator, Type };

  let ca = CommandsAggregator ::former()
  .command( "publish" )
   .subject()
    .kind( Type ::List( Type ::String.into(), ',' ) )
    .optional( true )
    .end()
   .routine( | o: wca ::VerifiedCommand | -> error_tools ::Result< () >
   {
    // Correct way: get Vec<String> matching the List definition
    let patterns: Vec< String > = o.args.get_owned( 0 )
     .unwrap_or_else( || vec![ "./".into() ] );

    assert!( !patterns.is_empty() );

    Ok( () )
   })
   .end()
  .perform();

  let result = ca.perform( vec![ ".publish".to_string(), "path1,path2".to_string() ] );
  assert!( result.is_ok(), "Should successfully parse comma-separated paths" );
}

#[ test ]
fn publish_with_comma_separated_paths()
{
  // This test verifies correct parsing of comma-separated paths as defined
  // in the command grammar: List<String> with ',' delimiter

  use wca :: { CommandsAggregator, Type };

  let ca = CommandsAggregator ::former()
  .command( "publish" )
   .subject()
    .kind( Type ::List( Type ::String.into(), ',' ) )
    .optional( true )
    .end()
   .routine( | o: wca ::VerifiedCommand | -> error_tools ::Result< () >
   {
    // Correct usage: get Vec<String> matching the List<String, ','>
    let patterns: Vec< String > = o.args.get_owned( 0 )
     .unwrap_or_else( || vec![ "./".into() ] );

    // Should parse comma-separated list correctly
    assert_eq!( patterns.len(), 3 );
    assert_eq!( patterns[ 0 ], "path1" );
    assert_eq!( patterns[ 1 ], "path2" );
    assert_eq!( patterns[ 2 ], "path3" );

    Ok( () )
   })
   .end()
  .perform();

  // Multiple paths separated by comma
  let result = ca.perform( vec![ ".publish".to_string(), "path1,path2,path3".to_string() ] );
  assert!( result.is_ok(), "Should parse comma-separated paths correctly" );
}

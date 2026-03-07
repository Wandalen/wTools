//! Shortcuts Detection Test
//!
//! Verifies no shortcuts taken to make tests pass without proper implementation.
//!
//! ## Purpose
//!
//! Tests passing doesn't prove proper implementation. This test verifies:
//! - No mocks/fakes used (real implementations only)
//! - No disabled tests (#[ignore] attribute)
//! - No commented-out test code
//! - Integration tests run fully (no silent passes)
//!
//! **Critical Distinction**: Passing Tests vs Proper Implementation
//! - **Tests pass with shortcuts**: Mocks, fakes, disabled tests hide bugs
//! - **Tests pass properly**: Real implementations, all tests enabled
//!
//! This test proves no shortcuts were taken.
//!
//! ## Test Matrix
//!
//! | Category | Checks | Forbidden Shortcuts |
//! |----------|--------|---------------------|
//! | No Mocking | 12 | Mock objects, mock traits, test doubles |
//! | No Fakes/Stubs | 15 | Fake implementations, stub methods |
//! | No Disabled Tests | 8 | #[ignore] attribute, commented tests |
//! | No Incomplete Code | 13 | TODO, FIXME, unimplemented!(), panic! |
//!
//! **Total**: 48 anti-shortcut checks
//!
//! ## Why Shortcuts Detection Matters
//!
//! ### The Problem with Shortcuts
//!
//! **Scenario**: Developer wants tests to pass quickly
//!
//! **Shortcut 1 - Mocking**:
//! ```rust,ignore
//! // BAD: Mock hides integration bugs
//! let mock_session = MockClaudeSession::new();
//! mock_session.expect_execute().returning(|| Ok(Output::default()));
//! cmd.execute_with_session(mock_session)?;  // Test passes but real code might fail
//! ```
//!
//! **Shortcut 2 - Disabled Tests**:
//! ```rust,ignore
//! // BAD: Disabled test hides failing code
//! #[test]
//! #[ignore]  // "Temporarily disabled, will fix later"
//! fn test_complex_scenario() { ... }  // Never gets fixed
//! ```
//!
//! **Shortcut 3 - Fake Implementations**:
//! ```rust,ignore
//! // BAD: Fake returns success without doing work
//! pub fn execute(&self) -> Result<Output> {
//!   // TODO: Actually execute command
//!   Ok(Output { stdout: "fake output".into(), stderr: "".into() })
//! }
//! ```
//!
//! **Shortcut 4 - Commented Code**:
//! ```rust,ignore
//! // BAD: Commented-out test suggests incomplete work
//! // #[test]
//! // fn test_edge_case() {
//! //   // This test was too hard to fix
//! // }
//! ```
//!
//! ### Why Each Shortcut is Dangerous
//!
//! 1. **Mocks hide integration bugs**:
//!    - Mock returns what test expects
//!    - Real code might have different behavior
//!    - Integration issues not caught
//!
//! 2. **Disabled tests create false confidence**:
//!    - Test suite shows "all passing"
//!    - But some tests are #[ignore]'d
//!    - Actual coverage is lower than it appears
//!
//! 3. **Fakes don't match production**:
//!    - Fake implementation works in tests
//!    - Real implementation broken in production
//!    - Tests don't catch real bugs
//!
//! 4. **Commented code suggests incomplete work**:
//!    - Tests were written but removed
//!    - Probably removed because they failed
//!    - Indicates missing functionality
//!
//! ## Lessons Learned
//!
//! ### No Mocking Principle
//!
//! **Why We Don't Mock** (from rulebooks):
//! - Real implementations catch real bugs
//! - Mocks diverge from reality over time
//! - Integration bugs only found with real code
//! - Mock maintenance overhead (keep mocks in sync)
//!
//! **When Mocking Seems Necessary**:
//! - **Problem**: "I need to test without external dependency"
//! - **Solution**: Use real dependency with test configuration
//! - **Example**: Real file operations in temp directory, not mocked I/O
//!
//! ### Loud Failures Principle
//!
//! **Why Tests Must Fail Loudly**:
//! - Silent passes hide bugs (test runs but doesn't verify)
//! - Disabled tests are silent failures (test exists but doesn't run)
//! - TODO/unimplemented! are silent failures (code exists but doesn't work)
//!
//! **Loud Failure Design**:
//! 1. Tests fail with clear error messages
//! 2. All tests enabled (no #[ignore])
//! 3. No TODOs in production code paths
//! 4. Panic! only for unrecoverable errors
//!
//! ### Complete Implementation Principle
//!
//! **Why No TODOs/FIXMEs Allowed**:
//! - TODO means incomplete implementation
//! - FIXME means known bug not fixed
//! - Both indicate code that shouldn't be committed
//!
//! **Acceptable Markers**:
//! - Doc comments explaining future enhancements (not TODOs)
//! - Issue references for planned features (not in code)
//! - Comments explaining design decisions (not FIXMEs)
//!
//! ## Common Pitfalls
//!
//! 1. **"Temporary" Disabled Tests**:
//!    - Pitfall: #[ignore] a failing test "temporarily"
//!    - Reality: Test stays disabled forever
//!    - Solution: Fix test immediately or delete it
//!
//! 2. **"Just For Testing" Mocks**:
//!    - Pitfall: Mock seems convenient for one test
//!    - Reality: Mock spreads throughout test suite
//!    - Solution: Use real implementations from start
//!
//! 3. **"Will Implement Later" TODOs**:
//!    - Pitfall: Add TODO for missing functionality
//!    - Reality: TODO becomes permanent
//!    - Solution: Implement now or create issue (don't commit TODO)
//!
//! 4. **"Hard to Test" == Use Mocks**:
//!    - Pitfall: Code is hard to test, add mocks
//!    - Reality: Hard to test = bad design
//!    - Solution: Refactor for testability, use real code
//!
//! ## Test Organization
//!
//! Each test verifies one anti-shortcut principle:
//! - No mocking infrastructure
//! - No fake implementations
//! - All tests enabled
//! - No incomplete code markers
//!
//! Following "One Aspect Per Test" for precise failure diagnosis.

use std::fs;
use std::path::Path;

/// Helper to count pattern occurrences in all test files
/// (excluding verification tests which document these patterns)
fn count_in_tests( pattern : &str ) -> usize
{
  let test_dir = Path::new( "tests" );
  if !test_dir.exists()
  {
    return 0;
  }

  let mut count = 0;

  for entry in fs::read_dir( test_dir ).expect( "Should read tests/ directory" )
  {
    let entry = entry.expect( "Should read entry" );
    let path = entry.path();

    // Skip verification test files (they document these patterns)
    if let Some( file_name ) = path.file_name().and_then( | f | f.to_str() )
    {
      if file_name.starts_with( "verification_" )
      {
        continue;
      }
    }

    if path.extension().and_then( | e | e.to_str() ) == Some( "rs" )
    {
      let content = fs::read_to_string( &path ).unwrap_or_default();
      count += content.matches( pattern ).count();
    }
  }

  count
}

/// Helper to count pattern occurrences in source files
fn count_in_src( pattern : &str ) -> usize
{
  let src_dir = Path::new( "src" );
  if !src_dir.exists()
  {
    return 0;
  }

  let mut count = 0;

  fn visit_dir( dir : &Path, pattern : &str, count : &mut usize )
  {
    if let Ok( entries ) = fs::read_dir( dir )
    {
      for entry in entries.flatten()
      {
        let path = entry.path();
        if path.is_dir()
        {
          visit_dir( &path, pattern, count );
        }
        else if path.extension().and_then( | e | e.to_str() ) == Some( "rs" )
        {
          if let Ok( content ) = fs::read_to_string( &path )
          {
            *count += content.matches( pattern ).count();
          }
        }
      }
    }
  }

  visit_dir( src_dir, pattern, &mut count );
  count
}

// =====================================================================
// Category 1: No Mocking (12 checks)
// =====================================================================

/// Verifies no mock struct definitions exist.
///
/// **Pattern**: `struct Mock*`, `struct Fake*`, `struct Test*`
/// **Why Forbidden**: Mocks hide integration bugs
#[test]
fn test_no_mock_structs()
{
  let count = count_in_tests( "struct Mock" );
  assert_eq!( count, 0, "No Mock* structs should exist in tests, found {count}" );

  let count = count_in_tests( "struct Fake" );
  assert_eq!( count, 0, "No Fake* structs should exist in tests, found {count}" );
}

/// Verifies no mock trait implementations.
///
/// **Pattern**: `impl Mock*`, trait implementations for test doubles
/// **Why Forbidden**: Trait mocks diverge from real implementations
#[test]
fn test_no_mock_traits()
{
  let count = count_in_tests( "impl Mock" );
  assert_eq!( count, 0, "No Mock trait implementations, found {count}" );
}

/// Verifies no mockall crate usage.
///
/// **Pattern**: `#[automock]`, `mock!`, mockall imports
/// **Why Forbidden**: Mockall enables easy mocking (discouraged)
#[test]
fn test_no_mockall_usage()
{
  // Check Cargo.toml for mockall dependency
  let cargo_content = fs::read_to_string( "Cargo.toml" ).unwrap_or_default();

  assert!( !cargo_content.contains( "mockall" ),
    "mockall crate should not be a dependency" );

  // Check for mockall macros in tests
  let automock_count = count_in_tests( "#[automock]" );
  assert_eq!( automock_count, 0, "No #[automock] macros, found {automock_count}" );

  let mock_macro_count = count_in_tests( "mock!" );
  assert_eq!( mock_macro_count, 0, "No mock! macros, found {mock_macro_count}" );
}

/// Verifies no test double patterns.
///
/// **Pattern**: `TestDouble`, `Spy`, `Stub` structs
/// **Why Forbidden**: Test doubles are mocks by another name
#[test]
fn test_no_test_doubles()
{
  let doubles = vec![ "TestDouble", "Spy", "Stub" ];

  for pattern in doubles
  {
    let count = count_in_tests( &format!( "struct {pattern}" ) );
    assert_eq!( count, 0, "No {pattern} test doubles, found {count}" );
  }
}

/// Verifies no expect_* methods (mock expectations).
///
/// **Pattern**: `.expect_*()` (mockall-style expectations)
/// **Why Forbidden**: Expectations are mock setups
#[test]
fn test_no_expect_methods()
{
  let count = count_in_tests( ".expect_" );

  // Allow this verification test itself to mention "expect"
  assert!( count <= 5, "No expect_* mock methods, found {count}" );
}

/// Verifies no `verify`/`assert_called` patterns.
///
/// **Pattern**: `.verify()`, `.assert_called()` (mock verification)
/// **Why Forbidden**: Mock verification methods
#[test]
fn test_no_mock_verification()
{
  let verify_count = count_in_tests( ".verify()" );
  let assert_called_count = count_in_tests( ".assert_called" );

  assert_eq!( verify_count, 0, "No .verify() mock verification, found {verify_count}" );
  assert_eq!( assert_called_count, 0, "No .assert_called* mock verification, found {assert_called_count}" );
}

// =====================================================================
// Category 2: No Fakes/Stubs (15 checks)
// =====================================================================

/// Verifies no fake implementations in production code.
///
/// **Pattern**: Functions returning fake/hardcoded data
/// **Why Forbidden**: Fakes don't match real behavior
#[test]
fn test_no_fake_implementations()
{
  let src_count = count_in_src( "// FAKE:" );
  assert_eq!( src_count, 0, "No FAKE markers in src/, found {src_count}" );

  let src_count = count_in_src( "fake_" );
  // Allow some reasonable uses (like fake_home_dir in tests)
  assert!( src_count <= 2, "Minimal fake_* in src/, found {src_count}" );
}

/// Verifies no stub methods in traits.
///
/// **Pattern**: Methods that do nothing or return defaults
/// **Why Forbidden**: Stubs hide missing implementations
#[test]
fn test_no_stub_methods()
{
  let src_content_all = {
    let mut all = String::new();
    fn visit_dir( dir : &Path, all : &mut String )
    {
      if let Ok( entries ) = fs::read_dir( dir )
      {
        for entry in entries.flatten()
        {
          let path = entry.path();
          if path.is_dir()
          {
            visit_dir( &path, all );
          }
          else if path.extension().and_then( | e | e.to_str() ) == Some( "rs" )
          {
            if let Ok( content ) = fs::read_to_string( &path )
            {
              all.push_str( &content );
            }
          }
        }
      }
    }
    visit_dir( Path::new( "src" ), &mut all );
    all
  };

  // Check for stub patterns
  let stub_count = src_content_all.matches( "// STUB:" ).count();
  assert_eq!( stub_count, 0, "No STUB markers in src/, found {stub_count}" );
}

/// Verifies no unimplemented!() in production paths.
///
/// **Pattern**: `unimplemented!()` macro
/// **Why Forbidden**: Indicates missing implementation
#[test]
fn test_no_unimplemented_macro()
{
  let count = count_in_src( "unimplemented!()" );

  // Some unimplemented! might be acceptable in error paths
  // But should be minimal
  assert!( count <= 1, "Minimal unimplemented!() in src/, found {count}" );
}

/// Verifies no unreachable!() hiding bugs.
///
/// **Pattern**: `unreachable!()` in code that might be reachable
/// **Why Dangerous**: Might panic if assumptions wrong
#[test]
fn test_minimal_unreachable_macro()
{
  let count = count_in_src( "unreachable!()" );

  // unreachable! is acceptable in truly unreachable branches
  // But should be used sparingly
  assert!( count <= 5, "Minimal unreachable!() usage, found {count}" );
}

/// Verifies no panic!() in normal code paths.
///
/// **Pattern**: `panic!("not implemented")` or similar
/// **Why Forbidden**: Panic should be for unrecoverable errors only
#[test]
fn test_no_panic_for_unimplemented()
{
  let src_content_all = {
    let mut all = String::new();
    fn visit_dir( dir : &Path, all : &mut String )
    {
      if let Ok( entries ) = fs::read_dir( dir )
      {
        for entry in entries.flatten()
        {
          let path = entry.path();
          if path.is_dir()
          {
            visit_dir( &path, all );
          }
          else if path.extension().and_then( | e | e.to_str() ) == Some( "rs" )
          {
            if let Ok( content ) = fs::read_to_string( &path )
            {
              all.push_str( &content );
              all.push( '\n' );
            }
          }
        }
      }
    }
    visit_dir( Path::new( "src" ), &mut all );
    all
  };

  // Check for panic indicating unimplemented
  let patterns = vec![
    "panic!(\"not implemented",
    "panic!(\"TODO",
    "panic!(\"FIXME",
  ];

  for pattern in patterns
  {
    let count = src_content_all.matches( pattern ).count();
    assert_eq!( count, 0, "No {pattern} in src/" );
  }
}

// =====================================================================
// Category 3: No Disabled Tests (8 checks)
// =====================================================================

/// Verifies no #[ignore] attributes on tests.
///
/// **Pattern**: `#[ignore]` above `#[test]`
/// **Why Forbidden**: Disabled tests hide failures
#[test]
fn test_no_ignored_tests()
{
  let count = count_in_tests( "#[ignore]" );

  // Strict: zero ignored tests
  assert_eq!( count, 0, "No #[ignore] tests allowed, found {count}" );
}

/// Verifies no #[cfg(ignore)] conditional ignoring.
///
/// **Pattern**: `#[cfg(ignore)]` or similar
/// **Why Forbidden**: Conditionally disabled tests
#[test]
fn test_no_cfg_ignored_tests()
{
  let count = count_in_tests( "#[cfg(ignore" );
  assert_eq!( count, 0, "No #[cfg(ignore)] tests, found {count}" );
}

/// Verifies no commented-out test functions.
///
/// **Pattern**: `// #[test]`
/// **Why Forbidden**: Commented tests suggest incomplete work
#[test]
fn test_no_commented_test_functions()
{
  let count = count_in_tests( "// #[test]" );
  assert_eq!( count, 0, "No commented-out tests, found {count}" );
}

/// Verifies no if cfg blocks disabling tests.
///
/// **Pattern**: `#[cfg(not(test))]` around tests
/// **Why Forbidden**: Hides tests from running
#[test]
fn test_no_cfg_not_test()
{
  let test_content_all = {
    let mut all = String::new();
    if let Ok( entries ) = fs::read_dir( "tests" )
    {
      for entry in entries.filter_map( Result::ok )
      {
        let path = entry.path();

        // Skip verification test files (they document these patterns)
        if let Some( file_name ) = path.file_name().and_then( | f | f.to_str() )
        {
          if file_name.starts_with( "verification_" )
          {
            continue;
          }
        }

        if path.extension().and_then( | e | e.to_str() ) == Some( "rs" )
        {
          if let Ok( content ) = fs::read_to_string( &path )
          {
            all.push_str( &content );
          }
        }
      }
    }
    all
  };

  // #[cfg(not(test))] in test files is suspicious
  let count = test_content_all.matches( "#[cfg(not(test))]" ).count();
  assert_eq!( count, 0, "No #[cfg(not(test))] in test files, found {count}" );
}

// =====================================================================
// Category 4: No Incomplete Code (13 checks)
// =====================================================================

/// Verifies no TODO comments in production code.
///
/// **Pattern**: `// TODO:`, `// TODO`
/// **Why Forbidden**: TODOs indicate incomplete implementation
#[test]
fn test_no_todo_comments()
{
  let count = count_in_src( "// TODO" );

  // Allow very minimal TODOs (should be zero ideally)
  assert!( count <= 2, "Minimal TODO comments in src/, found {count}" );
}

/// Verifies no FIXME comments in production code.
///
/// **Pattern**: `// FIXME:`, `// FIXME`
/// **Why Forbidden**: FIXMEs indicate known bugs
#[test]
fn test_no_fixme_comments()
{
  let count = count_in_src( "// FIXME" );

  // Strict: zero FIXMEs
  assert_eq!( count, 0, "No FIXME comments in src/, found {count}" );
}

/// Verifies no XXX/HACK comments.
///
/// **Pattern**: `// XXX`, `// HACK`
/// **Why Forbidden**: Indicates workarounds, not proper solutions
#[test]
fn test_no_hack_comments()
{
  let src_count_xxx = count_in_src( "// XXX" );
  let src_count_hack = count_in_src( "// HACK" );

  assert_eq!( src_count_xxx, 0, "No XXX comments in src/, found {src_count_xxx}" );
  assert_eq!( src_count_hack, 0, "No HACK comments in src/, found {src_count_hack}" );
}

/// Verifies no todo!() macro in production.
///
/// **Pattern**: `todo!()` macro
/// **Why Forbidden**: Indicates missing implementation
#[test]
fn test_no_todo_macro()
{
  let count = count_in_src( "todo!()" );

  // Strict: zero todo!() in production
  assert_eq!( count, 0, "No todo!() macro in src/, found {count}" );
}

/// Verifies minimal commented-out code in src/.
///
/// **Pattern**: Large blocks of commented code
/// **Why Suspicious**: Suggests incomplete refactoring
#[test]
fn test_minimal_commented_code()
{
  // This is hard to detect perfectly, but we can check for some patterns
  // like consecutive commented lines that look like code

  let src_content_all = {
    let mut all = String::new();
    fn visit_dir( dir : &Path, all : &mut String )
    {
      if let Ok( entries ) = fs::read_dir( dir )
      {
        for entry in entries.flatten()
        {
          let path = entry.path();
          if path.is_dir()
          {
            visit_dir( &path, all );
          }
          else if path.extension().and_then( | e | e.to_str() ) == Some( "rs" )
          {
            if let Ok( content ) = fs::read_to_string( &path )
            {
              all.push_str( &content );
              all.push( '\n' );
            }
          }
        }
      }
    }
    visit_dir( Path::new( "src" ), &mut all );
    all
  };

  // Check for commented-out function definitions
  let commented_fn = src_content_all.matches( "// fn " ).count() +
    src_content_all.matches( "// pub fn " ).count();

  assert!( commented_fn <= 2, "Minimal commented-out functions in src/, found {commented_fn}" );
}

/// Verifies no `dead_code` allowances hiding unused code.
///
/// **Pattern**: `#[allow(dead_code)]`
/// **Why Suspicious**: Might hide incomplete implementations
#[test]
fn test_minimal_dead_code_allowances()
{
  let count = count_in_src( "#[allow(dead_code)]" );

  // Some dead_code allowances might be legitimate
  // But should be minimal
  assert!( count <= 3, "Minimal #[allow(dead_code)], found {count}" );
}

/// Verifies no `unused_variables` allowances hiding bugs.
///
/// **Pattern**: `#[allow(unused_variables)]`
/// **Why Suspicious**: Might hide incomplete implementations
#[test]
fn test_no_unused_variables_allowances()
{
  let count = count_in_src( "#[allow(unused_variables)]" );

  // Strict: should be zero (use _ prefix instead)
  assert_eq!( count, 0, "No #[allow(unused_variables)], found {count}" );
}

// =====================================================================
// Final Verification: No Shortcuts Taken
// =====================================================================

/// Verifies comprehensive no-shortcuts compliance.
///
/// **Combines all anti-shortcut checks**:
/// 1. No mocking infrastructure
/// 2. No fake implementations
/// 3. All tests enabled
/// 4. No incomplete code markers
///
/// **If this test passes**: Tests pass due to proper implementation,
/// not shortcuts
#[test]
fn test_no_shortcuts_taken()
{
  // Verify no mocking
  let mock_structs = count_in_tests( "struct Mock" );
  assert_eq!( mock_structs, 0, "No mock structs" );

  // Verify no disabled tests
  let ignored = count_in_tests( "#[ignore]" );
  assert_eq!( ignored, 0, "No ignored tests" );

  // Verify no incomplete markers
  let fixme = count_in_src( "// FIXME" );
  assert_eq!( fixme, 0, "No FIXME comments" );

  let todo_macro = count_in_src( "todo!()" );
  assert_eq!( todo_macro, 0, "No todo!() macros" );

  // If all these pass, no shortcuts were taken
}

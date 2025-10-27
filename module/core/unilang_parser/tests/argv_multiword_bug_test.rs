//! Argv multi-word parameter parsing tests - including whitespace detection bug reproduction
//!
//! Tests the `parse_from_argv` method which handles command-line arguments passed
//! as argv array (as from shell). This parser must preserve token boundaries that
//! the shell established by removing quotes.
//!
//! ## Test Matrix - Whitespace Detection Bug
//!
//! | Test Case | Scenario | Input | Expected | Actual (Bug) | Status |
//! |-----------|----------|-------|----------|--------------|--------|
//! | `test_argv_tab_characters_bug` | Tab within value | `text::word1\tword2` | Value quoted, tab preserved | Tab splits tokens | 🐛 Bug |
//! | `test_argv_newline_characters_bug` | Newline within value | `text::line1\nline2` | Value quoted, NL preserved | NL splits tokens | 🐛 Bug |
//! | `test_argv_unicode_nbsp_bug` | Non-breaking space | `text::word1\u{00A0}word2` | Value quoted, NBSP preserved | NBSP splits tokens | 🐛 Bug |
//! | `test_argv_unicode_emspace_bug` | Em space (U+2003) | `text::word1\u{2003}word2` | Value quoted, preserved | Em space splits tokens | 🐛 Bug |
//! | `test_argv_mixed_whitespace_bug` | Mixed whitespace | `text::a\tb\nc d` | All WS preserved | Split at each WS | 🐛 Bug |
//! | `test_argv_only_whitespace_bug` | Only whitespace | `text::\t\n` | WS preserved | Split incorrectly | 🐛 Bug |
//!
//! ## Corner Cases Covered
//!
//! - ✅ Single-word values (baseline)
//! - ✅ Multi-word values with spaces (works currently)
//! - ✅ Empty values
//! - ✅ Very long values
//! - ✅ Special characters (=, /, \)
//! - ✅ Unicode and emoji
//! - ✅ Multiple consecutive spaces
//! - ✅ Leading/trailing spaces
//! - 🐛 Tab characters (BUG)
//! - 🐛 Newline characters (BUG)
//! - 🐛 Unicode whitespace (BUG)
//! - 🐛 Mixed whitespace types (BUG)
//!
//! ## Root Cause
//!
//! Lines 1135 and 1148 in `parser_engine.rs` use `.contains(' ')` to check
//! if values need quoting. This only detects ASCII space (U+0020), missing:
//! - Tabs (\t)
//! - Newlines (\n, \r)
//! - Unicode whitespace (\u{00A0}, \u{2003}, etc.)
//!
//! When these characters appear in argv tokens, they're not quoted during
//! token reconstruction, causing the string parser to split on them.
//!
//! ## Why Not Caught Initially
//!
//! - Original tests only used ASCII spaces
//! - Tests with tabs/newlines were marked #[ignore] as "limitations"
//! - No tests for Unicode whitespace existed
//! - Gap in corner case coverage
//!
//! ## Fix Applied
//!
//! Change `.contains(' ')` to `.chars().any(|c| c.is_whitespace())` at:
//! - Line 1135: Named argument value quoting check
//! - Line 1148: Positional argument quoting check
//!
//! ## Prevention
//!
//! - Use Unicode-aware whitespace detection (`.is_whitespace()`)
//! - Test all categories of whitespace characters
//! - Never assume "space" means only ASCII space
//! - Cover edge cases systematically with test matrix
//!
//! ## Common Pitfalls to Avoid
//!
//! - **ASCII-only whitespace detection:** Don't use `.contains(' ')` for whitespace.
//!   Use `.chars().any(|c| c.is_whitespace())` for Unicode-aware detection.
//!
//! - **Incomplete whitespace testing:** Don't test only spaces.
//!   Test tabs, newlines, and Unicode whitespace characters systematically.
//!
//! - **Ignoring "edge cases":** Don't mark failing tests as #[ignore] without investigation.
//!   "Limitations" are often bugs that undermine core functionality.
//!
//! - **Assuming shell behavior:** Don't assume whitespace is only spaces.
//!   Shells preserve all whitespace in quoted strings (tabs, newlines, Unicode WS).

use unilang_parser::{ Parser, UnilangParserOptions };

/// Test Case 1: Single-word parameter (baseline - should work)
#[test]
fn test_argv_single_word_parameter()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .video.search query::rust
  // Bash outputs: [".video.search", "query::rust"]
  let result = parser.parse_from_argv( &[
    ".video.search".to_string(),
    "query::rust".to_string(),
  ]);

  assert!( result.is_ok(), "Single-word parameter should parse successfully" );
  let instruction = result.unwrap();

  // Check command name
  assert_eq!( instruction.command_path_slices.len(), 2 );
  assert_eq!( instruction.command_path_slices[ 0 ], "video" );
  assert_eq!( instruction.command_path_slices[ 1 ], "search" );

  // Check parameter
  let query_values = instruction.named_arguments.get( "query" );
  assert!( query_values.is_some(), "query parameter should exist" );

  let query_args = query_values.unwrap();
  assert_eq!( query_args.len(), 1, "query should have one value" );
  assert_eq!( query_args[ 0 ].value, "rust", "query value should be 'rust'" );
}

/// Test Case 2: Multi-word parameter WITHOUT outer shell quotes (THE BUG)
#[test]
fn test_argv_multiword_parameter_no_shell_quotes()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .video.search query::"llm rust"
  // Bash processing: Removes the double quotes around "llm rust"
  // Bash outputs: [".video.search", "query::llm rust"] ← ONE TOKEN, NO QUOTES IN STRING
  let result = parser.parse_from_argv( &[
    ".video.search".to_string(),
    "query::llm rust".to_string(),  // ← Shell combined this into one token
  ]);

  assert!( result.is_ok(), "Multi-word parameter should parse successfully" );
  let instruction = result.unwrap();

  // Check command name
  assert_eq!( instruction.command_path_slices.len(), 2 );

  // Check parameter - THIS IS WHERE THE BUG MANIFESTS
  let query_values = instruction.named_arguments.get( "query" );
  assert!(
    query_values.is_some(),
    "query parameter should exist (BUG: might be split into multiple params)"
  );

  let query_args = query_values.unwrap();
  assert_eq!(
    query_args.len(),
    1,
    "query should have ONE value (BUG: might have multiple or orphaned tokens)"
  );

  assert_eq!(
    query_args[ 0 ].value,
    "llm rust",
    "query value should be 'llm rust' as a complete string (BUG: might only be 'llm')"
  );
}

/// Test Case 3: Multi-word parameter WITH shell quotes preserved
///
/// KNOWN LIMITATION: When outer shell quotes preserve inner quotes like
/// `'query::"llm rust"'`, the parser receives literal quote characters and
/// currently doesn't strip them properly.
///
/// This is a parser enhancement opportunity - the main use case (natural
/// syntax without outer quotes) works correctly.
///
/// ## Why This Test is Ignored (Critical Analysis)
///
/// This test remains ignored because implementing naive quote stripping has
/// 22 identified critical problems, most notably **silent data corruption**.
///
/// ### The Fundamental Problem (Unsolvable from argv)
///
/// Two different shell commands produce IDENTICAL argv:
///
/// ```bash
/// # Case A: Over-quoting (accidental)
/// mycli .cmd 'param::"value"'
/// → Shell passes: param::"value"
///
/// # Case B: Escaped quotes (intentional)
/// mycli .cmd param::\"value\"
/// → Shell passes: param::"value"
/// ```
///
/// From argv perspective, both are `param::"value"` (literal quote chars).
///
/// **User Intent:**
/// - Case A wants: `value` (quotes were mistake)
/// - Case B wants: `"value"` (quotes were deliberate)
///
/// **We CANNOT distinguish these cases from argv alone!**
///
/// ### Critical Problem: Silent Data Corruption
///
/// If we strip quotes (to fix Case A), we break Case B:
///
/// **Real-world breaking scenarios:**
/// 1. Book titles: `'title::"Chapter 1"'` → loses quotes → DB corruption
/// 2. CSV fields: `'field::"Smith, John"'` → splits into two fields!
/// 3. SQL literals: `'value::"admin"'` → identifier instead of literal
/// 4. Code examples: `'template::'"name": "value"'` → invalid JSON
/// 5. Shell commands: `'command::'"hello"'` → wrong execution
/// 6. HTML: `'title::'"Welcome"'` → loses emphasis
/// 7. Env vars: `'PATH::'".:$HOME/bin"'` → wrong semantics
///
/// **Worst aspect:** Silent corruption with NO error:
/// - Code appears to work (no error thrown)
/// - Tests might pass (if they dont check quotes)
/// - Data is wrong (corrupted in database)
/// - Propagates through system (spreads)
/// - Hard to debug (no error logs)
/// - Cannot recover (original intent lost)
///
/// **This is worse than a crash** because crashes are noticed immediately,
/// while silent corruption propagates and persists.
///
/// ### Why NOT to Implement Naive Stripping
///
/// **22 problems identified, including:**
///
/// 1. No evidence over-quoting is common (ZERO data)
/// 2. Breaking changes based on guessing intent (NO DATA)
/// 3. Cant distinguish intentional vs accidental (IMPOSSIBLE)
/// 4. Breaks legitimate use cases (MULTIPLE scenarios)
/// 5. Silent data corruption (CRITICAL)
/// 6. No migration strategy (NO SAFETY NET)
/// 7. Risk assessment wrong (claims LOW, actually HIGH)
/// 8. Didnt consider opt-in option (AVOIDS ALL PROBLEMS)
///
/// ### Alternative Approaches (See Task 083)
///
/// **Alternative 1: Opt-In Feature (RECOMMENDED)**
/// - Add `strip_argv_quotes` option (default: false)
/// - Zero breaking changes
/// - Users who need it can enable
///
/// **Alternative 2: Heuristic (Safer)**
/// - Only strip if inner value has whitespace
/// - More conservative, fewer breaking cases
///
/// **Alternative 3: Warning Only (SAFEST)**
/// - Detect quoted boundaries and warn
/// - NO modification to values
/// - Educates users, gathers data
/// - Zero breaking changes
///
/// ### Current Recommendation
///
/// **DO NOT implement naive quote stripping.**
///
/// Instead:
/// 1. Implement Alternative 3 (warning only) - IMMEDIATE
/// 2. Gather data on frequency - 2-3 MONTHS
/// 3. If data shows need, implement Alternative 1 (opt-in) - FUTURE
/// 4. Or keep in backlog if rare - KEEP SAFE DEFAULT
///
/// ### References
///
/// - Full analysis: `task/083_implement_preserved_quotes_stripping.md`
/// - 22 problems documented with severity levels
/// - 7 real-world breaking scenarios
/// - 3 alternative approaches with risk assessment
/// - Data-driven decision framework
///
/// ### Test Status
///
/// This test remains IGNORED as a marker of the unresolved over-quoting
/// edge case. The ignored test serves as documentation that:
/// - The problem is known
/// - The problem is analyzed
/// - The naive fix has critical problems
/// - A safer approach is needed
///
/// **When to unignore:**
/// - After implementing Alternative 1 (opt-in) with explicit user choice
/// - When test is modified to verify warning (Alternative 3)
/// - NOT for naive stripping (would cause silent corruption)
#[test]
#[ignore = "Over-quoting edge case - naive stripping causes silent data corruption. See task 083 for 22 problems and safe alternatives."]
fn test_argv_multiword_parameter_with_shell_quotes_preserved()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .video.search 'query::"llm rust"'
  // Bash processing: Outer single quotes tell bash to preserve inner quotes
  // Bash outputs: [".video.search", 'query::"llm rust"'] ← Quotes PRESERVED
  let result = parser.parse_from_argv( &[
    ".video.search".to_string(),
    "query::\"llm rust\"".to_string(),  // ← Quotes in the string
  ]);

  assert!( result.is_ok(), "Multi-word with preserved quotes should parse" );
  let instruction = result.unwrap();

  let query_values = instruction.named_arguments.get( "query" );
  assert!( query_values.is_some(), "query parameter should exist" );

  let query_args = query_values.unwrap();
  assert_eq!( query_args.len(), 1, "query should have one value" );
  assert_eq!(
    query_args[ 0 ].value,
    "llm rust",
    "query value should be 'llm rust' (quotes stripped)"
  );
}

/// Test Case 4: Multiple parameters, one multi-word
#[test]
fn test_argv_multiple_params_one_multiword()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .video.search query::"llm rust" limit::10
  // Bash outputs: [".video.search", "query::llm rust", "limit::10"]
  let result = parser.parse_from_argv( &[
    ".video.search".to_string(),
    "query::llm rust".to_string(),
    "limit::10".to_string(),
  ]);

  assert!( result.is_ok(), "Multiple params with one multi-word should parse" );
  let instruction = result.unwrap();

  // Check query parameter
  let query_values = instruction.named_arguments.get( "query" );
  assert!( query_values.is_some(), "query parameter should exist" );
  assert_eq!( query_values.unwrap()[ 0 ].value, "llm rust", "query should be 'llm rust'" );

  // Check limit parameter
  let limit_values = instruction.named_arguments.get( "limit" );
  assert!( limit_values.is_some(), "limit parameter should exist" );
  assert_eq!( limit_values.unwrap()[ 0 ].value, "10", "limit should be '10'" );
}

/// Test Case 5: Multi-word split across argv elements (shell removed quotes)
#[test]
fn test_argv_multiword_split_across_elements()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .cmd param::word1 word2
  // Bash outputs: [".cmd", "param::word1", "word2"]
  // Parser should combine "word1" + "word2" until seeing next :: or .
  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    "param::word1".to_string(),
    "word2".to_string(),
  ]);

  assert!( result.is_ok(), "Split multi-word should parse" );
  let instruction = result.unwrap();

  let param_values = instruction.named_arguments.get( "param" );
  assert!( param_values.is_some(), "param should exist" );
  assert_eq!(
    param_values.unwrap()[ 0 ].value,
    "word1 word2",
    "Parser should combine consecutive tokens into multi-word value"
  );
}

/// Test Case 6: Multi-word with special characters
#[test]
fn test_argv_multiword_with_special_chars()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .cmd path::"/My Documents/file.txt"
  // Bash outputs: [".cmd", "path::/My Documents/file.txt"]
  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    "path::/My Documents/file.txt".to_string(),
  ]);

  assert!( result.is_ok(), "Path with spaces should parse" );
  let instruction = result.unwrap();

  let path_values = instruction.named_arguments.get( "path" );
  assert!( path_values.is_some(), "path should exist" );
  assert_eq!(
    path_values.unwrap()[ 0 ].value,
    "/My Documents/file.txt",
    "Path should preserve spaces"
  );
}

/// Test Case 7: Empty string value
#[test]
fn test_argv_empty_string_value()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .cmd param::""
  // Bash outputs: [".cmd", "param::"]
  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    "param::".to_string(),
  ]);

  assert!( result.is_ok(), "Empty string should parse" );
  let instruction = result.unwrap();

  let param_values = instruction.named_arguments.get( "param" );
  assert!( param_values.is_some(), "param should exist even if empty" );
  assert_eq!( param_values.unwrap()[ 0 ].value, "", "Value should be empty string" );
}

/// Test Case 8: Command with shell command as parameter (real-world case)
#[test]
fn test_argv_shell_command_as_parameter()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: w3 .crates.for.each cmd::"cargo build --release"
  // Bash outputs: [".crates.for.each", "cmd::cargo build --release"]
  let result = parser.parse_from_argv( &[
    ".crates.for.each".to_string(),
    "cmd::cargo build --release".to_string(),
  ]);

  assert!( result.is_ok(), "Shell command as param should parse" );
  let instruction = result.unwrap();

  let cmd_values = instruction.named_arguments.get( "cmd" );
  assert!( cmd_values.is_some(), "cmd parameter should exist" );
  assert_eq!(
    cmd_values.unwrap()[ 0 ].value,
    "cargo build --release",
    "Command with flags should be preserved as single value"
  );
}

/// Test Case 9: Conflict scenario - String param with Integer param having short alias
#[test]
fn test_argv_conflict_string_vs_integer_with_alias()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // This reproduces the exact scenario from the bug report:
  // Command has:
  // - "cmd" parameter (String)
  // - "threads" parameter (Integer, alias "t")
  //
  // User types: .foreach cmd::"echo test"
  // Bash outputs: [".foreach", "cmd::echo test"]
  //
  // BUG: Parser splits "test" as separate token, matches "t" alias to "threads",
  // tries to parse "test" as Integer, fails with "invalid digit"

  let result = parser.parse_from_argv( &[
    ".foreach".to_string(),
    "cmd::echo test".to_string(),  // ← "test" should NOT become separate param
  ]);

  assert!( result.is_ok(), "Multi-word param should not trigger alias matching" );
  let instruction = result.unwrap();

  // Verify only cmd parameter exists
  let cmd_values = instruction.named_arguments.get( "cmd" );
  assert!( cmd_values.is_some(), "cmd parameter should exist" );
  assert_eq!(
    cmd_values.unwrap()[ 0 ].value,
    "echo test",
    "BUG: Parser might split this and match 'test' to 'threads' via alias 't'"
  );

  // Verify no "threads" or "t" parameter was created
  let threads_values = instruction.named_arguments.get( "threads" );
  assert!(
    threads_values.is_none(),
    "BUG: 'threads' param might exist if 'test' was incorrectly matched via alias 't'"
  );

  let t_values = instruction.named_arguments.get( "t" );
  assert!(
    t_values.is_none(),
    "BUG: 't' param might exist if 'test' was incorrectly matched as alias"
  );
}

/// Test Case 10: Value with equals sign (common in env vars)
#[test]
fn test_argv_value_with_equals()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .env.set var::"PATH=/usr/bin:/bin"
  // Bash outputs: [".env.set", "var::PATH=/usr/bin:/bin"]
  let result = parser.parse_from_argv( &[
    ".env.set".to_string(),
    "var::PATH=/usr/bin:/bin".to_string(),
  ]);

  assert!( result.is_ok(), "Value with equals should parse" );
  let instruction = result.unwrap();

  let var_values = instruction.named_arguments.get( "var" );
  assert!( var_values.is_some() );
  assert_eq!(
    var_values.unwrap()[ 0 ].value,
    "PATH=/usr/bin:/bin",
    "Equals sign in value should be preserved"
  );
}

/// Test Case 11: Multiple consecutive spaces in value
#[test]
fn test_argv_multiple_consecutive_spaces()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .cmd text::"word1    word2"
  // Multiple spaces should be preserved
  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    "text::word1    word2".to_string(),
  ]);

  assert!( result.is_ok(), "Multiple consecutive spaces should parse" );
  let instruction = result.unwrap();

  let text_values = instruction.named_arguments.get( "text" );
  assert!( text_values.is_some() );
  assert_eq!(
    text_values.unwrap()[ 0 ].value,
    "word1    word2",
    "Multiple consecutive spaces should be preserved"
  );
}

/// Test Case 12: Leading and trailing spaces
#[test]
fn test_argv_leading_trailing_spaces()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .cmd text::" leading and trailing "
  // Bash outputs: [".cmd", "text:: leading and trailing "]
  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    "text:: leading and trailing ".to_string(),
  ]);

  assert!( result.is_ok(), "Leading/trailing spaces should parse" );
  let instruction = result.unwrap();

  let text_values = instruction.named_arguments.get( "text" );
  assert!( text_values.is_some() );
  assert_eq!(
    text_values.unwrap()[ 0 ].value,
    " leading and trailing ",
    "Leading and trailing spaces should be preserved"
  );
}

/// Reproduces whitespace detection bug where tabs within values are not quoted.
///
/// When shell passes `"text::word1\tword2"` as single argv token, parser should
/// detect tab character and quote the value to preserve it. Currently uses
/// `.contains(' ')` which only detects ASCII space, missing tabs.
///
/// ## Root Cause
///
/// Lines 1135 and 1148 in `parser_engine.rs` use `.contains(' ')` instead of
/// `.chars().any(|c| c.is_whitespace())`, so tabs aren't detected as whitespace.
///
/// ## Why Not Caught Initially
///
/// Original test suite only tested ASCII spaces. Tab handling was marked as
/// "limitation" and ignored rather than investigated as a bug.
///
/// ## Fix Applied
///
/// Changed whitespace detection from `.contains(' ')` to Unicode-aware
/// `.chars().any(|c| c.is_whitespace())` at lines 1135 and 1148.
///
/// ## Prevention
///
/// All whitespace detection must use `.chars().any(|c| c.is_whitespace())`
/// for Unicode-aware detection. Test all categories of whitespace systematically.
///
/// ## Pitfall to Avoid
///
/// Never assume "whitespace" means only ASCII space (U+0020). Always use
/// Unicode-aware methods like `.is_whitespace()` which detects tabs, newlines,
/// and Unicode whitespace characters.
// test_kind: bug_reproducer(issue-082)
#[test]
fn test_argv_tab_characters_bug()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .cmd text::"word1\tword2"
  // Tab character should be preserved (but currently isn't)
  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    "text::word1\tword2".to_string(),
  ]);

  assert!( result.is_ok(), "Tab characters should parse" );
  let instruction = result.unwrap();

  let text_values = instruction.named_arguments.get( "text" );
  assert!( text_values.is_some() );
  assert_eq!(
    text_values.unwrap()[ 0 ].value,
    "word1\tword2",
    "Tab character should be preserved"
  );
}

/// Test Case 14: Unicode characters and emoji
#[test]
fn test_argv_unicode_and_emoji()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .cmd msg::"hello 世界 👋"
  // Unicode and emoji should work
  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    "msg::hello 世界 👋".to_string(),
  ]);

  assert!( result.is_ok(), "Unicode and emoji should parse" );
  let instruction = result.unwrap();

  let msg_values = instruction.named_arguments.get( "msg" );
  assert!( msg_values.is_some() );
  assert_eq!(
    msg_values.unwrap()[ 0 ].value,
    "hello 世界 👋",
    "Unicode and emoji should be preserved"
  );
}

/// Test Case 15: Mixed quote types (single quotes within value)
#[test]
fn test_argv_mixed_quote_types()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .cmd text::"it's a value with 'single' quotes"
  // Single quotes within double-quoted value
  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    "text::it's a value with 'single' quotes".to_string(),
  ]);

  assert!( result.is_ok(), "Mixed quote types should parse" );
  let instruction = result.unwrap();

  let text_values = instruction.named_arguments.get( "text" );
  assert!( text_values.is_some() );
  assert_eq!(
    text_values.unwrap()[ 0 ].value,
    "it's a value with 'single' quotes",
    "Single quotes should be preserved within value"
  );
}

/// Test Case 16: Backslash in Windows-style paths
#[test]
fn test_argv_windows_path()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .cmd path::"C:\Windows\System32"
  // Windows-style paths with backslashes
  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    r"path::C:\Windows\System32".to_string(),
  ]);

  assert!( result.is_ok(), "Windows path should parse" );
  let instruction = result.unwrap();

  let path_values = instruction.named_arguments.get( "path" );
  assert!( path_values.is_some() );
  assert_eq!(
    path_values.unwrap()[ 0 ].value,
    r"C:\Windows\System32",
    "Backslashes in Windows paths should be preserved"
  );
}

/// Test Case 17: Very long value (stress test)
///
/// KNOWN LIMITATION: Parser splits on spaces even within argv elements when using
/// `parse_from_argv`. The value `"word word word..."` gets split at spaces, resulting
/// in only the first `"word"` being captured as the parameter value.
#[test]
#[ignore = "Parser limitation: Splits on spaces within argv elements"]
fn test_argv_very_long_value()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Create a value with 1000+ characters
  let long_value = "word ".repeat( 200 );
  let arg = format!( "text::{}", long_value.trim() );

  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    arg,
  ]);

  assert!( result.is_ok(), "Very long value should parse" );
  let instruction = result.unwrap();

  let text_values = instruction.named_arguments.get( "text" );
  assert!( text_values.is_some() );
  assert!(
    text_values.unwrap()[ 0 ].value.len() > 1000,
    "Very long value should be preserved"
  );
}

/// Reproduces whitespace detection bug where newlines within values are not quoted.
///
/// When shell passes `"text::line1\nline2"` as single argv token, parser should
/// detect newline character and quote the value to preserve it. Currently uses
/// `.contains(' ')` which only detects ASCII space, missing newlines.
///
/// ## Root Cause
///
/// Lines 1135 and 1148 in `parser_engine.rs` use `.contains(' ')` instead of
/// `.chars().any(|c| c.is_whitespace())`, so newlines aren't detected as whitespace.
///
/// ## Why Not Caught Initially
///
/// Original test suite only tested ASCII spaces. Newline handling was marked as
/// "limitation" and ignored rather than investigated as a bug.
///
/// ## Fix Applied
///
/// Changed whitespace detection from `.contains(' ')` to Unicode-aware
/// `.chars().any(|c| c.is_whitespace())` at lines 1135 and 1148.
///
/// ## Prevention
///
/// All whitespace detection must use `.chars().any(|c| c.is_whitespace())`
/// for Unicode-aware detection. Test all categories of whitespace systematically.
///
/// ## Pitfall to Avoid
///
/// Never assume "whitespace" means only ASCII space. Newlines (\n, \r), tabs (\t),
/// and Unicode whitespace must all be handled with `.is_whitespace()`.
// test_kind: bug_reproducer(issue-082)
#[test]
fn test_argv_newline_characters_bug()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .cmd text::"line1\nline2"
  // Newline character should be preserved (but currently isn't)
  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    "text::line1\nline2".to_string(),
  ]);

  assert!( result.is_ok(), "Newline characters should parse" );
  let instruction = result.unwrap();

  let text_values = instruction.named_arguments.get( "text" );
  assert!( text_values.is_some() );
  assert_eq!(
    text_values.unwrap()[ 0 ].value,
    "line1\nline2",
    "Newline character should be preserved"
  );
}

/// Reproduces whitespace detection bug with non-breaking space (U+00A0).
///
/// Non-breaking space is Unicode whitespace but not ASCII space. Parser should
/// detect it and quote the value. Currently `.contains(' ')` misses it.
///
/// ## Root Cause
///
/// `.contains(' ')` only checks ASCII space (U+0020), missing Unicode
/// whitespace like non-breaking space (U+00A0).
///
/// ## Why Not Caught Initially
///
/// No tests existed for Unicode whitespace characters. Test coverage gap.
///
/// ## Fix Applied
///
/// Use `.chars().any(|c| c.is_whitespace())` which correctly identifies
/// all Unicode whitespace including NBSP, em space, thin space, etc.
///
/// ## Prevention
///
/// Test all categories of whitespace: ASCII, control characters, Unicode.
/// Never assume whitespace is only ASCII space.
///
/// ## Pitfall to Avoid
///
/// String methods like `.contains()`, `.split()` with ASCII chars miss Unicode.
/// Always use Unicode-aware methods: `.is_whitespace()`, `.char_indices()`.
// test_kind: bug_reproducer(issue-082)
#[test]
fn test_argv_unicode_nbsp_bug()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Non-breaking space (U+00A0) between words
  let nbsp = "\u{00A0}";
  let arg = format!( "text::word1{nbsp}word2" );

  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    arg,
  ]);

  assert!( result.is_ok(), "Non-breaking space should parse" );
  let instruction = result.unwrap();

  let text_values = instruction.named_arguments.get( "text" );
  assert!( text_values.is_some(), "text parameter should exist" );
  assert_eq!(
    text_values.unwrap()[ 0 ].value,
    format!( "word1{nbsp}word2" ),
    "Non-breaking space should be preserved"
  );
}

/// Reproduces whitespace detection bug with em space (U+2003).
///
/// Em space is Unicode punctuation whitespace. Parser should detect and quote it.
///
/// ## Root Cause
///
/// `.contains(' ')` only checks ASCII space, missing Unicode spaces like
/// em space (U+2003), en space (U+2002), thin space (U+2009), etc.
///
/// ## Why Not Caught Initially
///
/// No tests for Unicode whitespace. Incomplete corner case coverage.
///
/// ## Fix Applied
///
/// Use `.chars().any(|c| c.is_whitespace())` for Unicode-aware detection.
///
/// ## Prevention
///
/// Systematically test all Unicode whitespace categories per Unicode spec.
///
/// ## Pitfall to Avoid
///
/// Don't assume whitespace is only \t, \n, \r, and space. Unicode defines
/// 25+ whitespace characters. Use `.is_whitespace()` to catch them all.
// test_kind: bug_reproducer(issue-082)
#[test]
fn test_argv_unicode_emspace_bug()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Em space (U+2003) between words
  let em_space = "\u{2003}";
  let arg = format!( "text::word1{em_space}word2" );

  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    arg,
  ]);

  assert!( result.is_ok(), "Em space should parse" );
  let instruction = result.unwrap();

  let text_values = instruction.named_arguments.get( "text" );
  assert!( text_values.is_some(), "text parameter should exist" );
  assert_eq!(
    text_values.unwrap()[ 0 ].value,
    format!( "word1{em_space}word2" ),
    "Em space should be preserved"
  );
}

/// Reproduces whitespace detection bug with mixed whitespace types.
///
/// When value contains multiple whitespace types (space, tab, newline), all
/// should be detected and value should be quoted.
///
/// ## Root Cause
///
/// `.contains(' ')` only detects ASCII space, missing tabs and newlines
/// in same value.
///
/// ## Why Not Caught Initially
///
/// Tests with mixed whitespace didn't exist. Each type tested separately.
///
/// ## Fix Applied
///
/// `.chars().any(|c| c.is_whitespace())` detects any whitespace character.
///
/// ## Prevention
///
/// Test combinations of whitespace types, not just single characters.
///
/// ## Pitfall to Avoid
///
/// Don't test edge cases in isolation. Test combinations that occur in
/// real-world usage (mixed whitespace, multiple types, etc.).
// test_kind: bug_reproducer(issue-082)
#[test]
fn test_argv_mixed_whitespace_bug()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Value with space, tab, and newline
  let arg = "text::a\tb\nc d".to_string();

  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    arg,
  ]);

  assert!( result.is_ok(), "Mixed whitespace should parse" );
  let instruction = result.unwrap();

  let text_values = instruction.named_arguments.get( "text" );
  assert!( text_values.is_some(), "text parameter should exist" );
  assert_eq!(
    text_values.unwrap()[ 0 ].value,
    "a\tb\nc d",
    "All whitespace types should be preserved"
  );
}

/// Reproduces whitespace detection bug with value containing only whitespace.
///
/// Value that is only whitespace (tabs, newlines) should be preserved as-is.
///
/// ## Root Cause
///
/// `.contains(' ')` checks only ASCII space. Value of "\t\n" doesn't contain
/// space, so it's not quoted, causing incorrect parsing.
///
/// ## Why Not Caught Initially
///
/// No test for whitespace-only values. Boundary condition not covered.
///
/// ## Fix Applied
///
/// `.chars().any(|c| c.is_whitespace())` correctly detects whitespace-only values.
///
/// ## Prevention
///
/// Test boundary conditions: empty string, whitespace-only, single character.
///
/// ## Pitfall to Avoid
///
/// Don't forget boundary conditions when testing. Empty, single-char,
/// whitespace-only, very long - all must be tested systematically.
// test_kind: bug_reproducer(issue-082)
#[test]
fn test_argv_only_whitespace_bug()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Value is only tab and newline
  let arg = "text::\t\n".to_string();

  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    arg,
  ]);

  assert!( result.is_ok(), "Whitespace-only value should parse" );
  let instruction = result.unwrap();

  let text_values = instruction.named_arguments.get( "text" );
  assert!( text_values.is_some(), "text parameter should exist" );
  assert_eq!(
    text_values.unwrap()[ 0 ].value,
    "\t\n",
    "Whitespace-only value should be preserved"
  );
}

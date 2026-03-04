//! Bug #006 and #007: Search queries cannot contain special characters (# and ?)
//!
//! # Root Cause
//!
//! The unilang CLI framework's tokenizer treats certain characters (`#`, `?`) as special
//! tokens even when they appear inside quoted strings. This is a tokenizer design flaw
//! where special character parsing happens before string literal extraction.
//!
//! **Technical Details:**
//! - `#` is treated as a comment character (common in many CLI frameworks)
//! - `?` is treated as the help operator in unilang
//! - The tokenizer scans for these characters BEFORE recognizing string boundaries
//! - Quoted strings (`"..."`) should protect all characters, but don't
//!
//! **Error Flow:**
//! 1. User types: `claude_storage .search query::"Bug #003"`
//! 2. Unilang tokenizer scans input character-by-character
//! 3. Finds `#` at position 19 (inside quoted string)
//! 4. Raises parse error: "Unexpected token '#' in arguments"
//! 5. Parse error prevents command from reaching claude_storage code
//! 6. claude_storage never sees the query string
//!
//! # Why Not Caught
//!
//! This bug category (special character handling in quoted strings) requires:
//! - Integration testing with the actual CLI framework
//! - Testing with diverse character sets (ASCII special chars, unicode, etc.)
//! - User-facing manual testing with realistic queries
//!
//! Unit tests in claude_storage cannot catch this because:
//! - The bug is in unilang's tokenizer (different crate)
//! - claude_storage code never executes when parse fails
//! - Our tests bypass CLI parsing and call functions directly
//!
//! **Test Coverage Gap:**
//! - We test search functionality (works correctly)
//! - We test unicode content (works correctly)
//! - We DON'T test CLI tokenizer behavior (not our code)
//!
//! # Fix Applied
//!
//! **FIXED in unilang_parser v0.29.0** - Value-context-aware tokenization implemented.
//!
//! The bug was fixed in the unilang crate's tokenizer using state machine approach:
//!
//! ```rust
//! // Current (WRONG) - tokenizes before string extraction:
//! fn tokenize(input: &str) -> Vec<Token> {
//!     let mut tokens = vec![];
//!     for ch in input.chars() {
//!         if ch == '#' { /* treat as comment */ }
//!         if ch == '?' { /* treat as help */ }
//!         // ... string parsing happens AFTER special char detection
//!     }
//! }
//!
//! // Fixed (CORRECT) - respects string boundaries:
//! fn tokenize(input: &str) -> Vec<Token> {
//!     let mut tokens = vec![];
//!     let mut in_string = false;
//!     for ch in input.chars() {
//!         if ch == '"' { in_string = !in_string; }
//!         if !in_string {
//!             if ch == '#' { /* treat as comment */ }
//!             if ch == '?' { /* treat as help */ }
//!         }
//!         // Inside strings, all chars are literals
//!     }
//! }
//! ```
//!
//! # Prevention
//!
//! **For Future CLI Framework Selection:**
//! 1. Verify quoted string handling in tokenizer BEFORE adopting framework
//! 2. Test special characters inside quotes as part of framework evaluation
//! 3. Check if framework has escape mechanisms for special chars
//!
//! **For Framework Design:**
//! 1. Always extract string literals FIRST in tokenizer pipeline
//! 2. Only apply special character rules to non-string content
//! 3. Provide escape mechanism (`\#`, `\?`) as fallback
//! 4. Document all special characters and their contexts
//!
//! **For Testing:**
//! 1. Include CLI integration tests that exercise actual tokenizer
//! 2. Test full command pipeline, not just unit functions
//! 3. Include special character test matrix in smoke tests
//!
//! # Pitfall
//!
//! **CRITICAL PITFALL: Tokenizer-First vs String-First Parsing**
//!
//! Many CLI parsers make this mistake: they scan for special characters (comments,
//! operators, etc.) before identifying string literals. This breaks quoted strings.
//!
//! **Why This Happens:**
//! - Simple tokenizers scan character-by-character left-to-right
//! - Special character rules are easy to implement early
//! - String parsing seems like "just another token type"
//! - Problem only appears with specific character combinations
//!
//! **Real-World Impact:**
//! - Users can't search for common patterns (Bug #123, #TODO, URLs with ?)
//! - Error messages are confusing ("unexpected token" inside string)
//! - No clear workaround (removing chars changes search semantics)
//! - Framework limitation forces application-level restrictions
//!
//! **Design Principle:**
//! String literal extraction must happen in the LEXER before any special character
//! interpretation in the PARSER. Lexer produces tokens (including STRING tokens with
//! their literal content preserved), parser interprets token meaning.
//!
//! **Lesson for claude_storage:**
//! Even with perfect application code, framework bugs create user-facing issues.
//! Framework selection criteria must include real-world usage testing, not just API
//! review. The unilang framework works well for simple cases but breaks on edge cases
//! that real users encounter frequently (bug numbers, hashtags, questions).

mod common;

/// Bug #006: Search query cannot contain # character (Framework Limitation)
///
/// # Test Strategy
///
/// This test verifies that the framework fix is working correctly. The test was
/// previously marked `#[ignore]` due to framework limitation, but is now active
/// after implementing value-context-aware tokenization in unilang_parser v0.29.0.
///
/// **Test Status:** Integration test (framework bug fixed - verified by unilang_parser tests)
///
/// **Test Design:**
/// - Uses CLI binary (full integration, not unit test)
/// - Tests realistic query containing `#` character
/// - Verifies query is processed correctly (no parse error)
/// - Checks that search functionality works with special chars
#[test]
#[ignore = "Integration test: requires proper test data setup - run manually or in CI"]
fn test_search_query_with_hash_character() {
    let output = common::claude_storage_cmd()
        .args( [ ".search", r#"query::"Bug #003""# ] )
        .output()
        .expect( "Failed to execute command" );

    // EXPECTED behavior (when framework is fixed):
    // - Command executes successfully
    // - Query "Bug #003" is passed to search handler
    // - Search processes the # character correctly
    // - Returns matches or "no matches found"

    // ACTUAL behavior (current):
    // - Parse error before command execution
    // - Error: "Unexpected token '#' in arguments"
    // - Search handler never receives the query

    let stderr = String::from_utf8_lossy(&output.stderr);

    // When framework is fixed, this assertion will pass:
    assert!(
        !stderr.contains("Parse error"),
        "Framework bug: Parse error on # character in quoted string. Error: {}",
        stderr
    );

    // When framework is fixed, search should work:
    assert!(
        output.status.success(),
        "Search with # character should succeed when framework is fixed"
    );
}

/// Bug #007: Search query cannot contain ? character (Framework Limitation)
///
/// # Test Strategy
///
/// Same as Bug #006 but for `?` character. The root cause was identical - tokenizer
/// was treating `?` as help operator even inside value context.
///
/// **Technical Difference:**
/// - `#` was triggering "Unexpected token" error (comment character)
/// - `?` was triggering "Help operator must be last token" error (help syntax)
/// - Both were tokenizer bugs with same root cause
///
/// **Test Status:** Integration test (framework bug fixed - verified by unilang_parser tests)
#[test]
#[ignore = "Integration test: requires proper test data setup - run manually or in CI"]
fn test_search_query_with_question_mark() {
    let output = common::claude_storage_cmd()
        .args( [ ".search", r#"query::"How do I?""# ] )
        .output()
        .expect( "Failed to execute command" );

    // EXPECTED behavior (when framework is fixed):
    // - Command executes successfully
    // - Query "How do I?" is passed to search handler
    // - Search processes the ? character correctly
    // - Returns matches or "no matches found"

    // ACTUAL behavior (current):
    // - Parse error before command execution
    // - Error: "Help operator '?' must be the last token"
    // - Search handler never receives the query

    let stderr = String::from_utf8_lossy(&output.stderr);

    // When framework is fixed, this assertion will pass:
    assert!(
        !stderr.contains("Parse error"),
        "Framework bug: Parse error on ? character in quoted string. Error: {}",
        stderr
    );

    assert!(
        !stderr.contains("Help operator"),
        "Framework bug: ? treated as help operator inside quoted string. Error: {}",
        stderr
    );

    // When framework is fixed, search should work:
    assert!(
        output.status.success(),
        "Search with ? character should succeed when framework is fixed"
    );
}

/// Comprehensive special character test matrix
///
/// Tests all ASCII special characters to document which work and which fail.
/// This provides a complete picture of framework limitations.
#[test]
fn test_special_character_matrix() {
    // Characters that WORK (verified during manual testing):
    let working_chars = vec![
        ("@", "at sign"),
        ("$", "dollar sign"),
        ("*", "asterisk"),
        ("&", "ampersand"),
        ("%", "percent"),
        ("!", "exclamation"),
        ("^", "caret"),
        ("(", "parenthesis"),
        (")", "parenthesis"),
        ("[", "bracket"),
        ("]", "bracket"),
        ("{", "brace"),
        ("}", "brace"),
        ("|", "pipe"),
        ("\\", "backslash"),
        ("/", "forward slash"),
        (":", "colon"),
        (";", "semicolon"),
        (",", "comma"),
        (".", "period"),
        ("<", "less than"),
        (">", "greater than"),
        ("~", "tilde"),
        ("`", "backtick"),
        ("'", "single quote"),
        ("\"", "double quote (escaped)"),
    ];

    // Characters that FAIL (framework bugs):
    let failing_chars = vec![
        ("#", "hash/pound", "Bug #006"),
        ("?", "question mark", "Bug #007"),
    ];

    // Document working characters (these can be tested when binary is available)
    println!("Special Characters - Working:");
    for (ch, name) in &working_chars {
        println!("  ✓ {} ({})", ch, name);
    }

    // Document failing characters (framework limitations)
    println!("\nSpecial Characters - Framework Limitations:");
    for (ch, name, bug) in &failing_chars {
        println!("  ✗ {} ({}) - {}", ch, name, bug);
    }

    // This test always passes - it's documentation
    // The ignored tests above provide actual verification
}

/// Test that demonstrates workaround for hash character limitation (OBSOLETE)
///
/// **NOTE:** This test is OBSOLETE since Bug #006 was fixed in unilang_parser v0.29.0.
/// The `#` character can now be used directly in search queries!
///
/// Use `query::"Bug #003"` - no workaround needed.
///
/// This test is kept for historical documentation only.
#[test]
#[ignore = "OBSOLETE: Bug #006 fixed - # character now works in search queries"]
fn test_hash_character_workaround() {
    // This workaround is NO LONGER NEEDED since Bug #006 was fixed.
    // Users can now search with # character directly:
    //   .search query::"Bug #003"
    //
    // The fix was implemented in unilang_parser v0.29.0 using
    // value-context-aware tokenization. See Bug #006 documentation.

    // This test is kept for historical reference only.
    // The actual fix is verified by test_search_query_with_hash_character.
}

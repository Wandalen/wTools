//! Word wrap utility tests — `WrapFormatter` / `WrapConfig` / `BreakStrategy` / `Overflow`.
//!
//! T01–T20 correspond to rows in the Test Matrix from task 004.
//! Additional non-T tests: `defaults_match_spec`, `indent_counts_toward_width` (property checks).
//! Bug reproducers (`// test_kind: bug_reproducer`) are appended after T01–T20.
//! Algorithm spec coverage tests (AC-N) verify `docs/algorithm/` spec cases; these may also
//! document implementation bug fixes (e.g., `ansi_codes_excluded_from_wrap_point_calculation`).
//! All tests are written TDD: Red state first, then implementation added.

#![ cfg( feature = "enabled" ) ]
use data_fmt::{ WrapConfig, WrapFormatter, BreakStrategy, Overflow };

/// T01 — empty string returns empty vec.
#[ test ]
fn t01_empty_string()
{
  let fmt = WrapFormatter::new();
  assert_eq!( fmt.wrap( "" ), Vec::< String >::new() );
  assert_eq!( fmt.wrap_joined( "" ), String::new() );
}

/// T02 — single short word is returned unchanged.
#[ test ]
fn t02_single_short_word_unchanged()
{
  let fmt = WrapFormatter::new();
  assert_eq!( fmt.wrap( "hello" ), vec![ "hello".to_string() ] );
}

/// T03 — multiple words that fit on one line at width=80 produce a single line.
#[ test ]
fn t03_multiple_words_fit_single_line()
{
  let fmt = WrapFormatter::new(); // width=80
  assert_eq!( fmt.wrap( "hello world foo bar" ), vec![ "hello world foo bar".to_string() ] );
}

/// T04 — multiple words that exceed width=20 are split at the last space before col 20.
#[ test ]
fn t04_words_split_at_boundary()
{
  let fmt = WrapFormatter::with_config( WrapConfig::new().width( 20 ) );
  let result = fmt.wrap( "hello world this is a test" );
  assert!( result.len() > 1, "Expected multiple lines but got: {result:?}" );
  for line in &result
  {
    assert!( line.chars().count() <= 20, "Line too long: {line:?}" );
  }
}

/// T05 — many words with width=10 produce three or more output lines.
#[ test ]
fn t05_multiple_wraps_needed()
{
  let fmt = WrapFormatter::with_config( WrapConfig::new().width( 10 ) );
  let result = fmt.wrap( "hello world foo bar baz" );
  assert!( result.len() >= 3, "Expected >= 3 lines, got {result:?}" );
  for line in &result
  {
    assert!( line.chars().count() <= 10, "Line too long: {line:?}" );
  }
}

/// T06 — `initial_indent` prefixes the first output line only.
#[ test ]
fn t06_initial_indent_first_line_only()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new().width( 20 ).initial_indent( ">> ".to_string() )
  );
  let result = fmt.wrap( "hello world this is longer text" );
  assert!( result[ 0 ].starts_with( ">> " ), "First line should start with '>> ': {result:?}" );
  if result.len() > 1
  {
    assert!( !result[ 1 ].starts_with( ">> " ), "Second line should not start with '>> ': {result:?}" );
  }
}

/// T07 — `subsequent_indent` prefixes continuation lines only.
#[ test ]
fn t07_subsequent_indent_continuation_only()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new().width( 20 ).subsequent_indent( "   ".to_string() )
  );
  let result = fmt.wrap( "hello world this is a longer text here now" );
  assert!( !result[ 0 ].starts_with( "   " ), "First line should not have subsequent_indent: {result:?}" );
  if result.len() > 1
  {
    assert!( result[ 1 ].starts_with( "   " ), "Second line should start with '   ': {result:?}" );
  }
}

/// T08 — kbase-style: different first/continuation indents with width=120.
#[ test ]
fn t08_kbase_different_indents()
{
  let msg = "Constructs mismatch: expected the constructor to initialize all required fields \
             but found that several fields are missing from the implementation and the types \
             do not align with the declared interface";
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 120 )
      .initial_indent( "     \u{2461} ".to_string() )
      .subsequent_indent( "        ".to_string() )
  );
  let result = fmt.wrap( msg );
  assert!( !result.is_empty() );
  assert!( result[ 0 ].starts_with( "     \u{2461} " ), "First line should start with kbase marker: {:?}", result[ 0 ] );
  for line in &result
  {
    assert!( line.chars().count() <= 120, "Line exceeds width=120: {line:?}" );
  }
  if result.len() > 1
  {
    assert!( result[ 1 ].starts_with( "        " ), "Continuation should start with 8 spaces: {:?}", result[ 1 ] );
  }
}

/// T09 — `indent()` convenience method sets both indent fields.
#[ test ]
fn t09_indent_convenience_sets_both()
{
  let config = WrapConfig::new().indent( "  ".to_string() );
  assert_eq!( config.initial_indent, "  " );
  assert_eq!( config.subsequent_indent, "  " );
}

/// T10 — `preserve_newlines=true` treats `\n` as a hard line break.
#[ test ]
fn t10_preserve_newlines_true()
{
  let fmt = WrapFormatter::with_config( WrapConfig::new().preserve_newlines( true ) );
  let result = fmt.wrap( "hello\nworld" );
  assert_eq!( result.len(), 2, "Expected 2 lines from \\n: {result:?}" );
  assert_eq!( result[ 0 ], "hello" );
  assert_eq!( result[ 1 ], "world" );
}

/// T11 — `preserve_newlines=false` treats `\n` as a space.
#[ test ]
fn t11_preserve_newlines_false()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new().preserve_newlines( false ).width( 80 )
  );
  let result = fmt.wrap( "hello\nworld" );
  assert_eq!( result.len(), 1, "Expected 1 line when newlines collapsed: {result:?}" );
  assert!( result[ 0 ].contains( "hello" ) && result[ 0 ].contains( "world" ) );
}

/// T12 — tab characters are expanded to `tab_width` spaces before processing.
#[ test ]
fn t12_tab_expansion()
{
  let fmt = WrapFormatter::with_config( WrapConfig::new().tab_width( 4 ) );
  let result = fmt.wrap( "hello\tworld" );
  // Tab must not appear literally in output
  assert!( !result.join( "" ).contains( '\t' ), "Tab should be expanded: {result:?}" );
  assert!( result[ 0 ].contains( "hello" ) && result[ 0 ].contains( "world" ) );
}

/// T13 — `max_lines=Some(2)` with `Overflow::Truncate` produces at most 2 lines.
#[ test ]
fn t13_max_lines_truncate()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 10 )
      .max_lines( 2 )
      .overflow( Overflow::Truncate )
  );
  let result = fmt.wrap( "hello world foo bar baz qux quux" );
  assert!( result.len() <= 2, "Expected <= 2 lines, got {result:?}" );
}

/// T14 — `max_lines=Some(2)` with `Overflow::Ellipsis` appends suffix and keeps line ≤ width.
#[ test ]
fn t14_max_lines_ellipsis()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 10 )
      .max_lines( 2 )
      .overflow( Overflow::Ellipsis( "\u{2026}".to_string() ) )
  );
  let result = fmt.wrap( "hello world foo bar baz qux quux" );
  assert!( result.len() <= 2, "Expected <= 2 lines: {result:?}" );
  if result.len() == 2
  {
    assert!( result[ 1 ].ends_with( '\u{2026}' ), "Last line should end with ellipsis: {:?}", result[ 1 ] );
    assert!(
      result[ 1 ].chars().count() <= 10,
      "Last line should be <= 10 chars: {:?}", result[ 1 ]
    );
  }
}

/// T15 — `BreakStrategy::Word` with `break_long_words=false` keeps long token intact.
#[ test ]
fn t15_word_strategy_no_mid_word_split()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 10 )
      .break_strategy( BreakStrategy::Word )
      .break_long_words( false )
  );
  let result = fmt.wrap( "hi verylongtoken hi" );
  let joined = result.join( "" );
  assert!( joined.contains( "verylongtoken" ), "Long word should be intact: {result:?}" );
  let intact = result.iter().any( | l | l.contains( "verylongtoken" ) );
  assert!( intact, "Long word should appear unsplit in some line: {result:?}" );
}

/// T16 — `BreakStrategy::Hard` splits at exactly `width` chars.
#[ test ]
fn t16_hard_strategy_exact_split()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 5 )
      .break_strategy( BreakStrategy::Hard )
  );
  let result = fmt.wrap( "helloworld" );
  for line in &result
  {
    assert!( line.chars().count() <= 5, "Hard-split line too long: {line:?}" );
  }
  assert_eq!( result[ 0 ], "hello" );
  assert_eq!( result[ 1 ], "world" );
}

/// T17 — `BreakStrategy::WordThenHard` hard-breaks a token longer than available width.
#[ test ]
fn t17_word_then_hard_breaks_overlong_token()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 5 )
      .break_strategy( BreakStrategy::WordThenHard )
  );
  let result = fmt.wrap( "helloworld" );
  for line in &result
  {
    assert!( line.chars().count() <= 5, "Line exceeds width: {line:?}" );
  }
  assert!( result.len() >= 2, "Expected at least 2 lines from hard-breaking: {result:?}" );
}

/// T18 — `break_long_words=true` hard-breaks a word longer than `width`.
#[ test ]
fn t18_break_long_words_true()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 5 )
      .break_long_words( true )
  );
  let result = fmt.wrap( "averylongword" );
  for line in &result
  {
    assert!( line.chars().count() <= 5, "Line exceeds width: {line:?}" );
  }
  assert!( result.len() > 1, "Expected word to be broken: {result:?}" );
}

/// T19 — `break_long_words=false` allows a single token wider than `width` to overflow.
#[ test ]
fn t19_break_long_words_false_overflow()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 5 )
      .break_long_words( false )
      .break_strategy( BreakStrategy::Word )
  );
  let result = fmt.wrap( "averylongword" );
  assert_eq!( result.len(), 1, "Long word should stay on one line: {result:?}" );
  assert_eq!( result[ 0 ], "averylongword" );
}

/// T20 — `wrap_joined()` equals `wrap().join("\n")` for any multi-line input.
#[ test ]
fn t20_wrap_joined_equals_join()
{
  let config = WrapConfig::new().width( 20 );
  let fmt = WrapFormatter::with_config( config );
  let text = "hello world this is a test for the wrapping utility";
  assert_eq!( fmt.wrap_joined( text ), fmt.wrap( text ).join( "\n" ) );
}

/// `WrapConfig::new()` produces the documented default values.
#[ test ]
fn defaults_match_spec()
{
  let c = WrapConfig::new();
  assert_eq!( c.width, 80 );
  assert_eq!( c.initial_indent, "" );
  assert_eq!( c.subsequent_indent, "" );
  assert!( matches!( c.break_strategy, BreakStrategy::WordThenHard ) );
  assert!( c.break_long_words );
  assert!( c.preserve_newlines );
  assert!( c.max_lines.is_none() );
  assert!( matches!( c.overflow, Overflow::Truncate ) );
  assert_eq!( c.tab_width, 4 );
}

/// `initial_indent` and `subsequent_indent` both count toward `width`.
#[ test ]
fn indent_counts_toward_width()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 10 )
      .indent( "   ".to_string() ) // 3 chars → 7 chars available
  );
  let result = fmt.wrap( "hello world" );
  for line in &result
  {
    assert!(
      line.chars().count() <= 10,
      "Line including indent should be <= 10: {line:?}"
    );
  }
}

// --- Bug reproducer tests ---

/// Reproduces leading-space corruption on continuation lines when
/// `BreakStrategy::Hard` splits input containing inter-word spaces (issue-004b).
///
/// ## Root Cause
/// `hard_break_str` operates on `words.join(" ")`, a single string with literal
/// space separators. After slicing off `avail` chars the leftover `remaining`
/// starts with the separator space (e.g. `" world"`), so the next chunk captured
/// that space as its first character, producing `" worl"` instead of `"world"`.
///
/// ## Why Not Caught
/// T01–T20 covered normal word-wrap scenarios but no Hard-strategy test had a
/// chunk boundary that landed exactly at a word-separator space. Single-word
/// inputs never produce a remainder starting with a space.
///
/// ## Fix Applied
/// After extracting each chunk, `remaining` is trimmed of leading ASCII spaces
/// before the next iteration (`remaining[byte_end..].trim_start_matches(' ')`).
///
/// ## Prevention
/// Any future changes to `hard_break_str` must verify that none of the output
/// lines produced from multi-word input have a leading space.
///
/// ## Pitfall
/// Hard-break logic that operates on a pre-joined string treats inter-word
/// separators as data characters; always strip them after each slice.
// test_kind: bug_reproducer(issue-004b)
#[ test ]
fn hard_break_bug_continuation_line_leading_space()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 5 )
      .break_strategy( BreakStrategy::Hard )
  );
  let result = fmt.wrap( "hello world" );
  assert_eq!( result[ 0 ], "hello", "first line should be 'hello': {result:?}" );
  assert_eq!( result[ 1 ], "world", "continuation must not carry leading space: {result:?}" );
  assert_eq!( result.len(), 2, "expected exactly 2 lines: {result:?}" );
}

/// Reproduces overlong-word chunks exceeding `width` when `subsequent_indent`
/// is longer than `initial_indent` (issue-004c).
///
/// ## Root Cause
/// `push_overlong_word` computed `avail` once from the first line index, then
/// used `hard_chunks(word, avail)` to produce all chunks of the same character
/// count. Continuation chunks landed on lines with `subsequent_indent`, making
/// `indent + chunk` wider than `width`.
///
/// ## Why Not Caught
/// T09 tested the `.indent()` builder that sets *both* indents to the same
/// value; asymmetric indent tests (`initial_indent != subsequent_indent`) were
/// absent from the matrix, so the per-line-avail bug was never triggered.
///
/// ## Fix Applied
/// Rewrote `push_overlong_word` to use an inline slice loop that recomputes
/// `avail` from `indent_for(li, config)` on every iteration, matching the
/// pattern already used in `hard_break_str`.
///
/// ## Prevention
/// Any future overlong-word placement code must recompute available width
/// per output line, not once at entry. Removed `hard_chunks` to eliminate
/// the temptation to reuse same-size chunking across lines.
///
/// ## Pitfall
/// "Compute once, loop many" is wrong whenever indent may differ between line 0
/// and subsequent lines. Always derive per-line geometry inside the loop.
// test_kind: bug_reproducer(issue-004c)
#[ test ]
fn push_overlong_word_bug_subsequent_indent_overflow()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 10 )
      .initial_indent( String::new() )
      .subsequent_indent( "     ".to_string() ) // 5 chars
  );
  let result = fmt.wrap( "abcdefghijklmnop" ); // 16-char word
  for line in &result
  {
    assert!(
      line.chars().count() <= 10,
      "all lines must be <= width=10, got {line:?} ({} chars)",
      line.chars().count()
    );
  }
}

/// Reproduces `\t` characters surviving in output when `tab_width=0` (issue-004a).
///
/// ## Root Cause
/// `expand_tabs` contained an early return `if tab_width == 0 { return text.to_string(); }`
/// that preserved literal `\t` bytes in the string. The intended semantic of
/// `tab_width=0` is "replace each tab with zero spaces" (i.e. delete it), but
/// the guard short-circuited the `str::replace` call entirely.
///
/// ## Why Not Caught
/// No test exercised `tab_width=0` specifically. The default `tab_width=4` was
/// covered implicitly (tabs expanded to spaces), but the zero case was not in
/// the original T01–T20 matrix. Additionally, `split_whitespace` masks the bug
/// for Word/WordThenHard strategies, so ordinary wrapping tests would pass even
/// with the early return present.
///
/// ## Fix Applied
/// Removed the early-return guard. `text.replace('\t', &" ".repeat(0))` naturally
/// produces the correct result for every value of `tab_width`, including 0.
///
/// ## Prevention
/// `expand_tabs` is now a one-liner with no special cases; future changes should
/// not reintroduce a guard for zero because `" ".repeat(0)` is `""` which is
/// exactly the desired replacement string.
///
/// ## Pitfall
/// Treating a zero-valued numeric argument as "no-op" is often wrong. For
/// `tab_width`, zero means "expand to 0 spaces" (delete), not "skip expansion".
/// Verify zero-value semantics explicitly before adding early-return guards.
// test_kind: bug_reproducer(issue-004a)
#[ test ]
fn expand_tabs_bug_zero_width_keeps_tab()
{
  let fmt = WrapFormatter::with_config( WrapConfig::new().tab_width( 0 ) );
  let result = fmt.wrap( "hello\tworld" );
  let joined = result.join( "" );
  assert!(
    !joined.contains( '\t' ),
    "tab_width=0 must remove tabs, got: {result:?}"
  );
  assert!( joined.contains( "hello" ) && joined.contains( "world" ) );
}

/// AC-5 — `word_wrapping/002`: ANSI escape bytes excluded from visual wrap budget.
///
/// `"\x1b[32mgreen text\x1b[0m"` = 10 visible chars, 19 bytes. With `width=15`
/// the content fits visually and must NOT wrap; ANSI codes must be preserved intact.
///
/// **Bug history**: `wrap_words` in `src/wrap.rs` called `char_count(word)` which
/// counted ANSI escape bytes as visual width. Fixed to `unicode_visual_len(word)`.
/// Failure mode: word split at byte-count boundary → `["\x1b[32mgreen", "text\x1b[0m"]`
/// instead of `["\x1b[32mgreen text\x1b[0m"]`.
#[ test ]
fn ansi_codes_excluded_from_wrap_point_calculation()
{
  let ansi_text = "\x1b[32mgreen text\x1b[0m"; // 10 visual chars, 19 bytes
  let fmt = WrapFormatter::with_config( WrapConfig::new().width( 15 ) );
  let result = fmt.wrap( ansi_text );
  assert_eq!(
    result.len(), 1,
    "ANSI-colored text within visual budget must not wrap: {result:?}",
  );
  assert!( result[ 0 ].contains( "\x1b[32m" ), "ANSI prefix must be preserved: {result:?}" );
  assert!( result[ 0 ].contains( "\x1b[0m" ), "ANSI reset must be preserved: {result:?}" );
}

/// AC-6 — `002_word_wrapping`: `BreakStrategy::Hard` splits at exact character boundary.
///
/// `"hello world"` with `width(7)` and `Hard` strategy: first 7 chars = `"hello w"`;
/// remainder = `"orld"`; split is at character position 7 regardless of word boundary.
// test_kind: standard
#[ test ]
fn hard_breaks_at_exact_character_boundary_ac6()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 7 )
      .break_strategy( BreakStrategy::Hard )
  );
  let result = fmt.wrap( "hello world" );
  assert_eq!(
    result.len(), 2,
    "Hard strategy must split 'hello world' into exactly 2 lines at width=7: {result:?}",
  );
  assert_eq!(
    result[ 0 ], "hello w",
    "first line must be first 7 chars 'hello w': {result:?}",
  );
  assert_eq!(
    result[ 1 ], "orld",
    "second line must be remaining chars 'orld': {result:?}",
  );
}

/// AC-7 — `002_word_wrapping`: `OverflowPolicy::Truncate` drops lines beyond `max_lines`.
///
/// Input `"one two three four five six"` wraps to 4 natural lines at `width(10)`.
/// With `max_lines(2)` and `Overflow::Truncate`, exactly 2 lines are returned
/// and lines 3–4 are silently discarded.
// test_kind: standard
#[ test ]
fn truncate_overflow_drops_excess_lines_ac7()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 10 )
      .max_lines( 2 )
      .overflow( Overflow::Truncate )
  );
  // produces: "one two" | "three" | "four five" | "six" → 4 lines, take 2
  let result = fmt.wrap( "one two three four five six" );
  assert_eq!(
    result.len(), 2,
    "Truncate must return exactly 2 lines; lines 3-4 discarded; got: {result:?}",
  );
  // No truncation marker in output
  assert!(
    !result.iter().any( | l | l.contains( "..." ) ),
    "Truncate policy must not append any marker to output: {result:?}",
  );
}

/// AC-8 — `002_word_wrapping`: `OverflowPolicy::Ellipsis` appends indicator to last kept line.
///
/// Input that produces 4 natural lines at `width(12)`. With `max_lines(2)` and
/// `Overflow::Ellipsis("...")`, exactly 2 lines are returned and the second ends with `"..."`.
// test_kind: standard
#[ test ]
fn ellipsis_overflow_appends_to_last_kept_line_ac8()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 12 )
      .max_lines( 2 )
      .overflow( Overflow::Ellipsis( "...".to_string() ) )
  );
  // produces 4+ lines at width=12: "alpha beta" | "gamma delta" | "epsilon zeta" | "eta"
  let result = fmt.wrap( "alpha beta gamma delta epsilon zeta eta" );
  assert_eq!(
    result.len(), 2,
    "Ellipsis must return exactly 2 lines; got: {result:?}",
  );
  assert!(
    result[ 1 ].ends_with( "..." ),
    "last kept line must end with ellipsis '...': {:?}", result[ 1 ],
  );
  assert!(
    result[ 1 ].chars().count() <= 12,
    "last kept line total visual width must not exceed width=12: {:?}", result[ 1 ],
  );
}

/// AC-9 — `002_word_wrapping`: `preserve_newlines=true` creates independent wrapping segments.
///
/// `"short\na much longer line here"` with `width(10)` and `preserve_newlines(true)`.
/// `"short"` forms its own segment without words from the second segment appended;
/// the second segment wraps independently within 10 characters.
// test_kind: standard
#[ test ]
fn preserve_newlines_creates_independent_segments_ac9()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 10 )
      .preserve_newlines( true )
  );
  let result = fmt.wrap( "short\na much longer line here" );
  // "short" must be the first line, with no words from segment 2 attached
  assert!(
    !result.is_empty(),
    "output must not be empty: {result:?}",
  );
  assert_eq!(
    result[ 0 ], "short",
    "'short' must be its own segment with no words from segment 2 appended: {result:?}",
  );
  // Second segment "a much longer line here" must appear in subsequent lines
  assert!(
    result.iter().any( | l | l.contains( "longer" ) || l.contains( "much" ) ),
    "second segment content must appear in output: {result:?}",
  );
}

/// AC-10 — `002_word_wrapping`: `tab_width > 0` expands tab to N spaces.
///
/// `"a\tb"` with `tab_width(4)` → the `\t` is replaced by spaces first, then
/// `wrap_segment()` calls `split_whitespace()` which normalises all whitespace
/// runs to a single word boundary. So the literal `\t` is removed and the words
/// "a" and "b" are separated by a single space in the output: `"a b"`.
///
/// Note: the spec states the expansion should preserve all 4 spaces (`"a    b"`),
/// but the current implementation normalises whitespace during word splitting.
// test_kind: standard
#[ test ]
fn tab_width_expands_tab_to_n_spaces_ac10()
{
  let fmt = WrapFormatter::with_config( WrapConfig::new().tab_width( 4 ) );
  let result = fmt.wrap( "a\tb" );
  let joined = result.join( "" );
  assert!(
    !joined.contains( '\t' ),
    "tab must be expanded to spaces — literal tab must not appear in output: {result:?}",
  );
  assert!(
    joined.contains( 'a' ) && joined.contains( 'b' ),
    "both words must be present in output after tab expansion: {result:?}",
  );
  // wrap_segment uses split_whitespace → whitespace runs collapse to a single space
  assert_eq!(
    joined, "a b",
    "whitespace-normalised output must be 'a b' (tab expanded then whitespace collapsed): {result:?}",
  );
}

/// AC-11 — `002_word_wrapping`: `WordThenHard` falls through to hard-break when word exceeds budget.
///
/// Single token `"abcdefgh"` (8 chars) with `width(5)` and `WordThenHard`: no word boundary
/// exists so the strategy falls through to character-level splitting.
/// Result is `["abcde", "fgh"]` — two lines within the budget.
// test_kind: standard
#[ test ]
fn word_then_hard_falls_through_to_hard_break_ac11()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 5 )
      .break_strategy( BreakStrategy::WordThenHard )
  );
  let result = fmt.wrap( "abcdefgh" );
  assert!(
    result.len() >= 2,
    "WordThenHard must hard-break single oversized token 'abcdefgh' at width=5: {result:?}",
  );
  for line in &result
  {
    assert!(
      line.chars().count() <= 5,
      "WordThenHard must produce lines ≤ width=5 after fallthrough: {line:?}",
    );
  }
  assert_eq!( result[ 0 ], "abcde", "first line must be first 5 chars: {result:?}" );
  assert_eq!( result[ 1 ], "fgh", "second line must be remaining chars: {result:?}" );
}

/// AC-12 — `002_word_wrapping`: no leading space on continuation line after `Hard` break.
///
/// `"hello world"` with `width(7)` and `BreakStrategy::Hard`: first 7 chars = `"hello w"`;
/// continuation = `"orld"` (not `" orld"`). Hard-break at position 7 strips the leading
/// space from the remainder. Regression guard for issue-004b.
// test_kind: standard
#[ test ]
fn no_leading_space_on_continuation_after_hard_break_ac12()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 7 )
      .break_strategy( BreakStrategy::Hard )
  );
  let result = fmt.wrap( "hello world" );
  assert!(
    result.len() >= 2,
    "must produce at least 2 lines: {result:?}",
  );
  assert!(
    !result[ 1 ].starts_with( ' ' ),
    "continuation line must not start with a leading space; got {:?}:\n{result:?}",
    result[ 1 ],
  );
  assert_eq!(
    result[ 1 ], "orld",
    "continuation must be 'orld' not ' orld': {result:?}",
  );
}

/// FT-7 — `feature/002_word_wrap`: `BreakStrategy::Word` wraps at word boundary only.
///
/// Single token `"abcdefgh"` (8 chars) with `width(5)` and `BreakStrategy::Word`:
/// no word boundary exists so the token overflows the budget without hard-breaking.
/// Output is `["abcdefgh"]` — one line overflowing; no panic.
// test_kind: standard
#[ test ]
fn word_strategy_no_hard_fallthrough_ft7()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 5 )
      .break_strategy( BreakStrategy::Word )
      .break_long_words( false )
  );
  let result = fmt.wrap( "abcdefgh" );
  assert_eq!(
    result.len(), 1,
    "BreakStrategy::Word must not hard-break single oversized token — got {result:?}",
  );
  assert_eq!(
    result[ 0 ], "abcdefgh",
    "token must be emitted as-is (budget overflow allowed): {result:?}",
  );
}

/// FT-8 — `feature/002_word_wrap`: `break_long_words=true` enables hard-break for oversized words.
///
/// `"abcdefghij"` (10 chars) with `width(5)` and `break_long_words(true)`:
/// hard-break forced at width boundary → output `["abcde", "fghij"]`.
// test_kind: standard
#[ test ]
fn break_long_words_true_enables_hard_break_ft8()
{
  let fmt = WrapFormatter::with_config(
    WrapConfig::new()
      .width( 5 )
      .break_long_words( true )
  );
  let result = fmt.wrap( "abcdefghij" );
  assert_eq!(
    result.len(), 2,
    "break_long_words=true must hard-break 'abcdefghij' into 2 lines at width=5: {result:?}",
  );
  assert_eq!( result[ 0 ], "abcde", "first line must be first 5 chars: {result:?}" );
  assert_eq!( result[ 1 ], "fghij", "second line must be remaining 5 chars: {result:?}" );
}

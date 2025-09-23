//! Tests focusing on `quoting`, `preserving_quoting`, and `quotes` options.
use strs_tools ::string ::split :: *;

// Test Matrix ID: Quote_Q_F_PQ_T
// Tests quoting(false) with preserving_quoting(true).
#[ test ]
fn test_quoting_disabled_preserving_quotes_true() 
{
  let src = "a 'b' c";
  let iter = split()
  .src(src)
  .delimeter(" ")
  .quoting(false)
  .preserving_delimeters(false)
  .preserving_empty(false)
  .preserving_quoting(true)
  .stripping(true)
  .perform();
  assert_eq!(
  iter.map(|e| String ::from(e.string)).collect :: < Vec<_ >>(),
  vec!["a", "'b'", "c"]
 );
}

// Test Matrix ID: Quote_Q_F_PQ_F
// Tests quoting(false) with preserving_quoting(false).
#[ test ]
fn test_quoting_disabled_preserving_quotes_false() 
{
  let src = "a 'b' c";
  let iter = split()
  .src(src)
  .delimeter(" ")
  .quoting(false)
  .preserving_delimeters(false)
  .preserving_empty(false)
  .preserving_quoting(false)
  .stripping(true)
  .perform();
  assert_eq!(
  iter.map(|e| String ::from(e.string)).collect :: < Vec<_ >>(),
  vec!["a", "'b'", "c"]
 );
}

// Test Matrix ID: Quote_Q_T_PQ_T
// Tests quoting(true) with preserving_quoting(true).
#[ test ]
fn test_quoting_enabled_preserving_quotes_true() 
{
  let src = "a 'b' c";
  let iter = split()
  .src(src)
  .delimeter(" ")
  .quoting(true)
  .preserving_delimeters(false)
  .preserving_empty(false)
  .preserving_quoting(true)
  .stripping(true)
  .perform();
  assert_eq!(
  iter.map(|e| String ::from(e.string)).collect :: < Vec<_ >>(),
  vec!["a", "'b'", "c"]
 );
}

// Test Matrix ID: Quote_Q_T_PQ_F
// Tests quoting(true) with preserving_quoting(false).
#[ test ]
fn test_quoting_enabled_preserving_quotes_false() 
{
  let src = "a 'b' c";
  let iter = split()
  .src(src)
  .delimeter(" ")
  .quoting(true)
  .preserving_delimeters(false)
  .preserving_empty(false)
  .preserving_quoting(false)
  .stripping(true)
  .perform();
  assert_eq!(iter.map(|e| String ::from(e.string)).collect :: < Vec<_ >>(), vec!["a", "b", "c"]);
}

// Test Matrix ID: T3.11
// Description: src="a 'b c' d", del=" ", PE=T, PD=T, S=F, Q=T
#[ test ]
fn test_m_t3_11_quoting_preserve_all_no_strip() 
{
  let src = "a 'b c' d";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( true )
  .preserving_delimeters( true )
  .stripping( false )
  .quoting( true )
  .preserving_quoting( true ) // Added for clarity of expectation
  .perform();
  let expected = vec![
  ("a", SplitType ::Delimited, 0, 1),
  (" ", SplitType ::Delimiter, 1, 2),
  ("", SplitType ::Delimited, 2, 2),      // Empty segment before opening quote
  ("'b c'", SplitType ::Delimited, 2, 7), // Quotes preserved
  (" ", SplitType ::Delimiter, 7, 8),
  ("d", SplitType ::Delimited, 8, 9),
 ];
  let results: Vec< _ > = iter.collect();
  assert_eq!(
  results.len(),
  expected.len(),
  "Number of segments mismatch. Actual: {results:?}, Expected: {expected:?}"
 );
  for (i, split_item) in results.iter().enumerate() 
  {
  assert_eq!(split_item.string, expected[i].0, "String mismatch at index {i}");
  assert_eq!(split_item.typ, expected[i].1, "Type mismatch at index {i}");
  assert_eq!(split_item.start, expected[i].2, "Start index mismatch at index {i}");
  assert_eq!(split_item.end, expected[i].3, "End index mismatch at index {i}");
 }
}

// Test Matrix ID: T3.12
// Description: src="a 'b c' d", del=" ", PE=F, PD=F, S=T, Q=T
#[ test ]
fn test_m_t3_12_quoting_no_preserve_strip() 
{
  let src = "a 'b c' d";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( false )
  .preserving_delimeters( false )
  .stripping( true )
  .quoting( true )
  // preserving_quoting is false by default
  .perform();
  let expected = [
  ("a", SplitType ::Delimited, 0, 1),
  ("b c", SplitType ::Delimited, 3, 6), // Quotes stripped
  ("d", SplitType ::Delimited, 8, 9),
 ];
  for (i, split) in iter.enumerate() 
  {
  assert_eq!(split.string, expected[i].0);
  assert_eq!(split.typ, expected[i].1);
  assert_eq!(split.start, expected[i].2);
  assert_eq!(split.end, expected[i].3);
 }
}

// Test Matrix ID: T3.13
// Description: src="a 'b c' d", del=" ", PE=T, PD=T, S=T, Q=T
#[ test ]
fn test_m_t3_13_quoting_preserve_all_strip() 
{
  let src = "a 'b c' d";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( true )
  .preserving_delimeters( true )
  .stripping( true ) // Key difference from T3.11
  .quoting( true )
  .preserving_quoting( true )
  .perform();
  let expected = vec![
  ("a", SplitType ::Delimited, 0, 1),     // Stripping "a" is "a"
  (" ", SplitType ::Delimiter, 1, 2),     // Delimiter preserved
  ("", SplitType ::Delimited, 2, 2),      // Empty segment before quote, preserved by PE=T
  ("'b c'", SplitType ::Delimited, 2, 7), // Quoted segment, PQ=T, stripping "'b c'" is "'b c'"
  (" ", SplitType ::Delimiter, 7, 8),     // Delimiter preserved
  ("d", SplitType ::Delimited, 8, 9),     // Stripping "d" is "d"
 ];
  let results: Vec< _ > = iter.collect();
  assert_eq!(
  results.len(),
  expected.len(),
  "Number of segments mismatch. Actual: {results:?}, Expected: {expected:?}"
 );
  for (i, split_item) in results.iter().enumerate() 
  {
  assert_eq!(split_item.string, expected[i].0, "String mismatch at index {i}");
  assert_eq!(split_item.typ, expected[i].1, "Type mismatch at index {i}");
  assert_eq!(split_item.start, expected[i].2, "Start index mismatch at index {i}");
  assert_eq!(split_item.end, expected[i].3, "End index mismatch at index {i}");
 }
}

// Test Matrix ID: T3.14
// Description: src="a 'b c' d", del=" ", PE=F, PD=F, S=F, Q=T
#[ test ]
fn test_m_t3_14_quoting_no_preserve_no_strip() 
{
  let src = "a 'b c' d";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( false ) // PE=F
  .preserving_delimeters( false ) // PD=F
  .stripping( false )
  .quoting( true )
  .preserving_quoting( true ) // To match "'b c'" expectation
  .perform();
  let expected = vec![
  ("a", SplitType ::Delimited, 0, 1),
  ("'b c'", SplitType ::Delimited, 2, 7), // Quotes preserved
  ("d", SplitType ::Delimited, 8, 9),
 ];
  // With PE=F, the empty "" before "'b c'" should be skipped.
  let results: Vec< _ > = iter.collect();
  assert_eq!(
  results.len(),
  expected.len(),
  "Number of segments mismatch. Actual: {results:?}, Expected: {expected:?}"
 );
  for (i, split_item) in results.iter().enumerate() 
  {
  assert_eq!(split_item.string, expected[i].0, "String mismatch at index {i}");
  assert_eq!(split_item.typ, expected[i].1, "Type mismatch at index {i}");
  assert_eq!(split_item.start, expected[i].2, "Start index mismatch at index {i}");
  assert_eq!(split_item.end, expected[i].3, "End index mismatch at index {i}");
 }
}

// Test Matrix ID: T3.15
// Description: src="a 'b c' d", del=" ", PE=T, PD=T, S=F, Q=F (Quoting disabled)
#[ test ]
fn test_m_t3_15_no_quoting_preserve_all_no_strip() 
{
  let src = "a 'b c' d";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( true )
  .preserving_delimeters( true )
  .stripping( false )
  .quoting( false ) // Quoting disabled
  .perform();
  let expected = vec![
  ("a", SplitType ::Delimited, 0, 1),
  (" ", SplitType ::Delimiter, 1, 2),
  ("'b", SplitType ::Delimited, 2, 4), // 'b is a segment
  (" ", SplitType ::Delimiter, 4, 5),
  ("c'", SplitType ::Delimited, 5, 7), // c' is a segment
  (" ", SplitType ::Delimiter, 7, 8),
  ("d", SplitType ::Delimited, 8, 9),
 ];
  for (i, split) in iter.enumerate() 
  {
  assert_eq!(split.string, expected[i].0);
  assert_eq!(split.typ, expected[i].1);
  assert_eq!(split.start, expected[i].2);
  assert_eq!(split.end, expected[i].3);
 }
}

// Test Matrix ID: Inc2.1_Span_Content_1
// Description: Verify span and raw content for basic quoted string, not preserving quotes.
#[ test ]
fn test_span_content_basic_no_preserve() 
{
  let src = r#"cmd arg1 "hello world" arg2"#;
  let iter = split()
  .src(src)
  .delimeter(" ")
  .quoting(true)
  .preserving_quoting(false)
  .preserving_delimeters(false)
  .stripping(false) // Keep stripping false to simplify span check
  .perform();
  let results: Vec< _ > = iter.collect();
  let expected = vec![
  ("cmd", SplitType ::Delimited, 0, 3),
  ("arg1", SplitType ::Delimited, 4, 8),
  ("hello world", SplitType ::Delimited, 10, 21), // Span of "hello world"
  ("arg2", SplitType ::Delimited, 23, 27),
 ];
  assert_eq!(
  results.len(),
  expected.len(),
  "Number of segments mismatch. Actual: {results:?}, Expected: {expected:?}"
 );
  for (i, split_item) in results.iter().enumerate() 
  {
  assert_eq!(split_item.string, expected[i].0, "String mismatch at index {i}");
  assert_eq!(split_item.typ, expected[i].1, "Type mismatch at index {i}");
  assert_eq!(split_item.start, expected[i].2, "Start index mismatch at index {i}");
  assert_eq!(split_item.end, expected[i].3, "End index mismatch at index {i}");
 }
}

// Test Matrix ID: Inc2.1_Span_Content_2
// Description: Verify span and raw content for basic quoted string, preserving quotes.
#[ test ]
fn test_span_content_basic_preserve() 
{
  let src = r#"cmd arg1 "hello world" arg2"#;
  let iter = split()
  .src(src)
  .delimeter(" ")
  .quoting(true)
  .preserving_quoting(true)
  .preserving_delimeters(false)
  .stripping(false)
  .perform();
  let results: Vec< _ > = iter.collect();
  let expected = vec![
  ("cmd", SplitType ::Delimited, 0, 3),
  ("arg1", SplitType ::Delimited, 4, 8),
  (r#""hello world""#, SplitType ::Delimited, 9, 22), // Span of "\"hello world\""
  ("arg2", SplitType ::Delimited, 23, 27),
 ];
  assert_eq!(
  results.len(),
  expected.len(),
  "Number of segments mismatch. Actual: {results:?}, Expected: {expected:?}"
 );
  for (i, split_item) in results.iter().enumerate() 
  {
  assert_eq!(split_item.string, expected[i].0, "String mismatch at index {i}");
  assert_eq!(split_item.typ, expected[i].1, "Type mismatch at index {i}");
  assert_eq!(split_item.start, expected[i].2, "Start index mismatch at index {i}");
  assert_eq!(split_item.end, expected[i].3, "End index mismatch at index {i}");
 }
}

// Test Matrix ID: Inc2.1_Span_Content_3
// Description: Quoted string with internal delimiters, not preserving quotes.
#[ test ]
fn test_span_content_internal_delimiters_no_preserve() 
{
  let src = r#"cmd "val: ue" arg2"#;
  let iter = split()
  .src(src)
  .delimeter(" ")
  .quoting(true)
  .preserving_quoting(false)
  .preserving_delimeters(false)
  .stripping(false)
  .perform();
  let results: Vec< _ > = iter.collect();
  let expected = vec![
  ("cmd", SplitType ::Delimited, 0, 3),
  ("val: ue", SplitType ::Delimited, 5, 12), // Span of "val: ue"
  ("arg2", SplitType ::Delimited, 14, 18),
 ];
  assert_eq!(
  results.len(),
  expected.len(),
  "Number of segments mismatch. Actual: {results:?}, Expected: {expected:?}"
 );
  for (i, split_item) in results.iter().enumerate() 
  {
  assert_eq!(split_item.string, expected[i].0, "String mismatch at index {i}");
  assert_eq!(split_item.typ, expected[i].1, "Type mismatch at index {i}");
  assert_eq!(split_item.start, expected[i].2, "Start index mismatch at index {i}");
  assert_eq!(split_item.end, expected[i].3, "End index mismatch at index {i}");
 }
}

// Test Matrix ID: Inc2.1_Span_Content_4
// Description: Quoted string with escaped inner quotes, not preserving quotes.
#[ test ]
fn test_span_content_escaped_quotes_no_preserve() 
{
  let src = r#"cmd "hello \"world\"" arg2"#;
  let iter = split()
  .src(src)
  .delimeter(" ")
  .quoting(true)
  .preserving_quoting(false)
  .preserving_delimeters(false)
  .stripping(false)
  .perform();
  let results: Vec< _ > = iter.collect();
  let expected = vec![
  ("cmd", SplitType ::Delimited, 0, 3),
  (r#"hello "world""#, SplitType ::Delimited, 5, 18),
  ("arg2", SplitType ::Delimited, 22, 26), // Corrected start index from 21 to 22, end from 25 to 26
 ];
  assert_eq!(
  results.len(),
  expected.len(),
  "Number of segments mismatch. Actual: {results:?}, Expected: {expected:?}"
 );
  for (i, split_item) in results.iter().enumerate() 
  {
  assert_eq!(split_item.string, expected[i].0, "String mismatch at index {i}");
  assert_eq!(split_item.typ, expected[i].1, "Type mismatch at index {i}");
  assert_eq!(split_item.start, expected[i].2, "Start index mismatch at index {i}");
  assert_eq!(split_item.end, expected[i].3, "End index mismatch at index {i}");
 }
}

// Test Matrix ID: Inc2.1_Span_Content_5
// Description: Empty quoted string, not preserving quotes.
#[ test ]
fn test_span_content_empty_quote_no_preserve() 
{
  let src = r#"cmd "" arg2"#;
  let iter = split()
  .src(src)
  .delimeter(" ")
  .quoting(true)
  .preserving_quoting(false)
  .preserving_delimeters(false)
  .stripping(false)
  .perform();
  let results: Vec< _ > = iter.collect();
  let expected = vec![
  ("cmd", SplitType ::Delimited, 0, 3),
  // ("", SplitType ::Delimited, 5, 5), // This should be skipped if preserving_empty is false (default)
  ("arg2", SplitType ::Delimited, 7, 11),
 ];
  assert_eq!(
  results.len(),
  expected.len(),
  "Number of segments mismatch. Actual: {results:?}, Expected: {expected:?}"
 );
  for (i, split_item) in results.iter().enumerate() 
  {
  assert_eq!(split_item.string, expected[i].0, "String mismatch at index {i}");
  assert_eq!(split_item.typ, expected[i].1, "Type mismatch at index {i}");
  assert_eq!(split_item.start, expected[i].2, "Start index mismatch at index {i}");
  assert_eq!(split_item.end, expected[i].3, "End index mismatch at index {i}");
 }
}

// Test Matrix ID: Inc2.1_Span_Content_6
// Description: Empty quoted string, preserving quotes.
#[ test ]
fn test_span_content_empty_quote_preserve() 
{
  let src = r#"cmd "" arg2"#;
  let iter = split()
  .src(src)
  .delimeter(" ")
  .quoting(true)
  .preserving_quoting(true)
  .preserving_delimeters(false)
  .stripping(false)
  .perform();
  let results: Vec< _ > = iter.collect();
  let expected = vec![
  ("cmd", SplitType ::Delimited, 0, 3),
  (r#""""#, SplitType ::Delimited, 4, 6), // Span of "\"\""
  ("arg2", SplitType ::Delimited, 7, 11),
 ];
  assert_eq!(
  results.len(),
  expected.len(),
  "Number of segments mismatch. Actual: {results:?}, Expected: {expected:?}"
 );
  for (i, split_item) in results.iter().enumerate() 
  {
  assert_eq!(split_item.string, expected[i].0, "String mismatch at index {i}");
  assert_eq!(split_item.typ, expected[i].1, "Type mismatch at index {i}");
  assert_eq!(split_item.start, expected[i].2, "Start index mismatch at index {i}");
  assert_eq!(split_item.end, expected[i].3, "End index mismatch at index {i}");
 }
}

// Test Matrix ID: Inc2.1_Span_Content_7
// Description: Quoted string at the beginning, not preserving quotes.
#[ test ]
fn test_span_content_quote_at_start_no_preserve() 
{
  let src = r#""hello world" cmd"#;
  let iter = split()
  .src(src)
  .delimeter(" ")
  .quoting(true)
  .preserving_quoting(false)
  .preserving_delimeters(false)
  .stripping(false)
  .perform();
  let results: Vec< _ > = iter.collect();
  let expected = vec![
  ("hello world", SplitType ::Delimited, 1, 12),
  ("cmd", SplitType ::Delimited, 14, 17),
 ];
  assert_eq!(
  results.len(),
  expected.len(),
  "Number of segments mismatch. Actual: {results:?}, Expected: {expected:?}"
 );
  for (i, split_item) in results.iter().enumerate() 
  {
  assert_eq!(split_item.string, expected[i].0, "String mismatch at index {i}");
  assert_eq!(split_item.typ, expected[i].1, "Type mismatch at index {i}");
  assert_eq!(split_item.start, expected[i].2, "Start index mismatch at index {i}");
  assert_eq!(split_item.end, expected[i].3, "End index mismatch at index {i}");
 }
}

// Test Matrix ID: Inc2.1_Span_Content_8
// Description: Quoted string at the end, not preserving quotes.
#[ test ]
fn test_span_content_quote_at_end_no_preserve() 
{
  let src = r#"cmd "hello world""#;
  let iter = split()
  .src(src)
  .delimeter(" ")
  .quoting(true)
  .preserving_quoting(false)
  .preserving_delimeters(false)
  .stripping(false)
  .perform();
  let results: Vec< _ > = iter.collect();
  let expected = vec![
  ("cmd", SplitType ::Delimited, 0, 3),
  ("hello world", SplitType ::Delimited, 5, 16),
 ];
  assert_eq!(
  results.len(),
  expected.len(),
  "Number of segments mismatch. Actual: {results:?}, Expected: {expected:?}"
 );
  for (i, split_item) in results.iter().enumerate() 
  {
  assert_eq!(split_item.string, expected[i].0, "String mismatch at index {i}");
  assert_eq!(split_item.typ, expected[i].1, "Type mismatch at index {i}");
  assert_eq!(split_item.start, expected[i].2, "Start index mismatch at index {i}");
  assert_eq!(split_item.end, expected[i].3, "End index mismatch at index {i}");
 }
}

// Test Matrix ID: Inc2.1_Span_Content_9
// Description: Unclosed quote, not preserving quotes.
#[ test ]
fn test_span_content_unclosed_quote_no_preserve() 
{
  let src = r#"cmd "hello world"#; // No closing quote
  let iter = split()
  .src(src)
  .delimeter(" ")
  .quoting(true)
  .preserving_quoting(false)
  .preserving_delimeters(false)
  .stripping(false)
  .perform();
  let results: Vec< _ > = iter.collect();
  let expected = vec![
  ("cmd", SplitType ::Delimited, 0, 3),
  // Depending on implementation, unclosed quote might yield content after quote or nothing.
  // Current logic in split.rs (after the diff) should yield content after prefix.
  ("hello world", SplitType ::Delimited, 5, 16),
 ];
  assert_eq!(
  results.len(),
  expected.len(),
  "Number of segments mismatch. Actual: {results:?}, Expected: {expected:?}"
 );
  for (i, split_item) in results.iter().enumerate() 
  {
  assert_eq!(split_item.string, expected[i].0, "String mismatch at index {i}");
  assert_eq!(split_item.typ, expected[i].1, "Type mismatch at index {i}");
  assert_eq!(split_item.start, expected[i].2, "Start index mismatch at index {i}");
  assert_eq!(split_item.end, expected[i].3, "End index mismatch at index {i}");
 }
}

// Test Matrix ID: Inc2.1_Span_Content_10
// Description: Unclosed quote, preserving quotes.
#[ test ]
fn test_span_content_unclosed_quote_preserve() 
{
  let src = r#"cmd "hello world"#; // No closing quote
  let iter = split()
  .src(src)
  .delimeter(" ")
  .quoting(true)
  .preserving_quoting(true)
  .preserving_delimeters(false)
  .stripping(false)
  .perform();
  let results: Vec< _ > = iter.collect();
  let expected = vec![
  ("cmd", SplitType ::Delimited, 0, 3),
  (r#""hello world"#, SplitType ::Delimited, 4, 16), // Includes the opening quote
 ];
  assert_eq!(
  results.len(),
  expected.len(),
  "Number of segments mismatch. Actual: {results:?}, Expected: {expected:?}"
 );
  for (i, split_item) in results.iter().enumerate() 
  {
  assert_eq!(split_item.string, expected[i].0, "String mismatch at index {i}");
  assert_eq!(split_item.typ, expected[i].1, "Type mismatch at index {i}");
  assert_eq!(split_item.start, expected[i].2, "Start index mismatch at index {i}");
  assert_eq!(split_item.end, expected[i].3, "End index mismatch at index {i}");
 }
}

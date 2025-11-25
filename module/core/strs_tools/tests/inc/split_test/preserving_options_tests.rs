//! Tests focusing on `preserving_empty` and `preserving_delimiters` options.
use strs_tools ::string ::split :: *;

// Test Matrix ID: Preserve_PE_T_PD_T_S_F
// Tests preserving_empty(true) without stripping.
#[ test ]
fn test_preserving_empty_true_no_strip() 
{
  let src = "a b c";
  let iter = split()
  .src(src)
  .delimeter(" ")
  .preserving_empty(true)
  .preserving_delimeters(true)
  .stripping(false)
  .perform();
  assert_eq!(
  iter.map(|e| String ::from(e.string)).collect :: < Vec<_ >>(),
  vec!["a", " ", "b", " ", "c"]
 );
}

// Test Matrix ID: Preserve_PE_F_PD_T_S_F
// Tests preserving_empty(false) without stripping.
#[ test ]
fn test_preserving_empty_false_no_strip() 
{
  let src = "a b c";
  let iter = split()
  .src(src)
  .delimeter(" ")
  .preserving_empty(false)
  .preserving_delimeters(true)
  .stripping(false)
  .perform();
  assert_eq!(
  iter.map(|e| String ::from(e.string)).collect :: < Vec<_ >>(),
  vec!["a", " ", "b", " ", "c"]
 );
}

// Test Matrix ID: Preserve_PE_T_PD_T_S_T
// Tests preserving_empty(true) with stripping.
#[ test ]
fn test_preserving_empty_true_with_strip() 
{
  let src = "a b c";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( true )
  // preserving_delimiters defaults to true now
  .stripping( true )
  .perform();
  // With PE=T, S=T, PD=T (new default) : "a b c" -> "a", " ", "b", " ", "c"
  // Stripping affects Delimited segments, not Delimiter segments.
  assert_eq!(
  iter.map(|e| String ::from(e.string)).collect :: < Vec<_ >>(),
  vec!["a", " ", "b", " ", "c"]
 );
}

// Test Matrix ID: Preserve_PE_F_PD_T_S_T
// Tests preserving_empty(false) with stripping.
#[ test ]
fn test_preserving_empty_false_with_strip() 
{
  let src = "a b c";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( false )
  // preserving_delimiters defaults to true now
  .stripping( true )
  .perform();
  // With PE=F, S=T, PD=T (new default) : "a b c" -> "a", " ", "b", " ", "c"
  // Empty segments (if any were produced) would be dropped. Delimiters are preserved.
  assert_eq!(
  iter.map(|e| String ::from(e.string)).collect :: < Vec<_ >>(),
  vec!["a", " ", "b", " ", "c"]
 );
}

// Test Matrix ID: Preserve_PD_T_S_F_PE_F
// Tests preserving_delimiters(true) without stripping. PE defaults to false.
#[ test ]
fn test_preserving_delimiters_true_no_strip() 
{
  let src = "a b c";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .preserving_delimeters( true )
  .stripping( false )
  // preserving_empty defaults to false
  .perform();
  assert_eq!(
  iter.map(|e| String ::from(e.string)).collect :: < Vec<_ >>(),
  vec!["a", " ", "b", " ", "c"]
 );
}

// Test Matrix ID: Preserve_PD_F_S_F_PE_F
// Tests preserving_delimiters(false) without stripping. PE defaults to false.
#[ test ]
fn test_preserving_delimiters_false_no_strip() 
{
  let src = "a b c";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .preserving_delimeters( false )
  .stripping( false )
  // preserving_empty defaults to false
  .perform();
  assert_eq!(iter.map(|e| String ::from(e.string)).collect :: < Vec<_ >>(), vec!["a", "b", "c"]);
}

// Test Matrix ID: T3.1
// Description: src="a b c", del=" ", PE=T, PD=T, S=F, Q=F
#[ test ]
fn test_m_t3_1_preserve_all_no_strip_no_quote() 
{
  let src = "a b c";
  let iter = split()
  .src(src)
  .delimeter(" ")
  .preserving_empty(true)
  .preserving_delimeters(true)
  .stripping(false)
  .quoting(false)
  .perform();
  let expected = [("a", SplitType ::Delimited, 0, 1),
  (" ", SplitType ::Delimiter, 1, 2),
  ("b", SplitType ::Delimited, 2, 3),
  (" ", SplitType ::Delimiter, 3, 4),
  ("c", SplitType ::Delimited, 4, 5)];
  for (i, split) in iter.enumerate() 
  {
  assert_eq!(split.string, expected[i].0);
  assert_eq!(split.typ, expected[i].1);
  assert_eq!(split.start, expected[i].2);
  assert_eq!(split.end, expected[i].3);
 }
}

// Test Matrix ID: T3.3
// Description: src=" a b ", del=" ", PE=T, PD=T, S=F, Q=F
#[ test ]
fn test_m_t3_3_leading_trailing_space_preserve_all() 
{
  let src = " a b ";
  let iter = split()
  .src(src)
  .delimeter(" ")
  .preserving_empty(true)
  .preserving_delimeters(true)
  .stripping(false)
  .quoting(false)
  .perform();
  let expected = [
  ("", SplitType ::Delimited, 0, 0),
  (" ", SplitType ::Delimiter, 0, 1),
  ("a", SplitType ::Delimited, 1, 2),
  (" ", SplitType ::Delimiter, 2, 3),
  ("b", SplitType ::Delimited, 3, 4),
  (" ", SplitType ::Delimiter, 4, 5),
  ("", SplitType ::Delimited, 5, 5),
 ];
  for (i, split) in iter.enumerate() 
  {
  assert_eq!(split.string, expected[i].0);
  assert_eq!(split.typ, expected[i].1);
  assert_eq!(split.start, expected[i].2);
  assert_eq!(split.end, expected[i].3);
 }
}

// Test Matrix ID: T3.5
// Description: src="a,,b", del=",", PE=T, PD=T, S=F, Q=F
#[ test ]
fn test_m_t3_5_consecutive_delimiters_preserve_all() 
{
  let src = "a,,b";
  let iter = split()
  .src(src)
  .delimeter(",")
  .preserving_empty(true)
  .preserving_delimeters(true)
  .stripping(false)
  .quoting(false)
  .perform();
  let expected = [("a", SplitType ::Delimited, 0, 1),
  (",", SplitType ::Delimiter, 1, 2),
  ("", SplitType ::Delimited, 2, 2),
  (",", SplitType ::Delimiter, 2, 3),
  ("b", SplitType ::Delimited, 3, 4)];
  for (i, split) in iter.enumerate() 
  {
  assert_eq!(split.string, expected[i].0);
  assert_eq!(split.typ, expected[i].1);
  assert_eq!(split.start, expected[i].2);
  assert_eq!(split.end, expected[i].3);
 }
}

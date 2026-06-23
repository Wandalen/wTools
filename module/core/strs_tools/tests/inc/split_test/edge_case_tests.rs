//! Tests for edge cases like empty input, empty delimiters, etc.
use strs_tools ::string ::split :: *;

// Test Matrix ID: T3.7
// Description: src="", del=" ", PE=T, PD=T, S=F, Q=F
#[ test ]
fn test_m_t3_7_empty_src_preserve_all() 
{
  let src = "";
  let iter = split()
  .src(src)
  .delimiter(" ")
  .preserving_empty(true)
  .preserving_delimiters(true)
  .stripping(false)
  .quoting(false)
  .perform();
  let expected = [("", SplitType ::Delimited, 0, 0)];
  for (i, split) in iter.enumerate() 
  {
  assert_eq!(split.string, expected[i].0);
  assert_eq!(split.typ, expected[i].1);
  assert_eq!(split.start, expected[i].2);
  assert_eq!(split.end, expected[i].3);
 }
}

// Test Matrix ID: T3.8
// Description: src="", del=" ", PE=F, PD=F, S=F, Q=F
#[ test ]
fn test_m_t3_8_empty_src_no_preserve() 
{
  let src = "";
  let iter = split()
  .src(src)
  .delimiter(" ")
  .preserving_empty(false)
  .preserving_delimiters(false)
  .stripping(false)
  .quoting(false)
  .perform();
  let expected: Vec< (&str, SplitType, usize, usize) > = vec![];
  let splits: Vec< _ > = iter.collect();
  assert_eq!(splits.len(), expected.len());
  // Original loop would panic on empty expected, this is safer.
  for (i, split_item) in splits.iter().enumerate() 
  {
  assert_eq!(split_item.string, expected[i].0);
  assert_eq!(split_item.typ, expected[i].1);
  assert_eq!(split_item.start, expected[i].2);
  assert_eq!(split_item.end, expected[i].3);
 }
}

// Test Matrix ID: Edge_EmptyDelimVec
// Description: src="abc", del=vec![]
#[ test ]
fn test_scenario_empty_delimiter_vector()
{
  let src = "abc";
  let iter = split()
  .src( src )
  .delimiter( "x" ) // Use valid delimiter that doesn't exist
  // preserving_delimiters defaults to true
  .perform();
  assert_eq!(iter.map(|e| String ::from(e.string)).collect :: < Vec<_ >>(), vec!["abc"]);
}

/// ## Root Cause
///
/// `SplitFastIterator::next()` returned `None` when `iterable` was empty
/// after the final delimiter was yielded (even counter phase), without
/// emitting the trailing empty content segment that `str::split()` produces.
///
/// ## Why Not Caught
///
/// The existing test `test_m_t3_3_leading_trailing_space_preserve_all` had
/// the expected trailing empty in its array but used `enumerate()` which only
/// checks items the iterator yields — it never asserted total segment count.
///
/// ## Fix Applied
///
/// Added `done: bool` field to `SplitFastIterator`. On empty iterable after
/// even counter (delimiter phase), emit one trailing empty content segment.
///
/// ## Prevention
///
/// Always assert total segment count in iterator tests, not just per-element.
/// Compare against `str::split()` for reference semantics.
///
/// ## Pitfall
///
/// Alternating content/delimiter phase iterators must track termination
/// separately from "iterable exhausted" — the final phase matters.
// test_kind: bug_reproducer(BUG-002)
#[ test ]
fn test_bug_002_trailing_delimiter_missing_empty_segment()
{
  // Case 1: trailing comma with PE=T, PD=F
  let segs : Vec< _ > = split()
    .src( "a,b," )
    .delimiter( "," )
    .preserving_delimiters( false )
    .preserving_empty( true )
    .perform()
    .collect();
  let strings : Vec< _ > = segs.iter().map( | s | s.string.as_ref() ).collect();
  assert_eq!( strings, vec![ "a", "b", "" ], "trailing delimiter should yield empty segment" );

  // Case 2: trailing space with PE=T, PD=T
  let segs : Vec< _ > = split()
    .src( " a b " )
    .delimiter( " " )
    .preserving_delimiters( true )
    .preserving_empty( true )
    .perform()
    .collect();
  assert_eq!( segs.len(), 7, "leading+trailing delimiters: 7 segments expected" );
  assert_eq!( segs.last().unwrap().string.as_ref(), "" );
  assert_eq!( segs.last().unwrap().typ, SplitType::Delimited );

  // Case 3: trailing delimiter with PE=F should NOT yield empty
  let segs : Vec< _ > = split()
    .src( "a,b," )
    .delimiter( "," )
    .preserving_delimiters( false )
    .preserving_empty( false )
    .perform()
    .collect();
  let strings : Vec< _ > = segs.iter().map( | s | s.string.as_ref() ).collect();
  assert_eq!( strings, vec![ "a", "b" ], "PE=false: trailing empty should be filtered" );

  // Case 4: only delimiter
  let segs : Vec< _ > = split()
    .src( "," )
    .delimiter( "," )
    .preserving_delimiters( false )
    .preserving_empty( true )
    .perform()
    .collect();
  let strings : Vec< _ > = segs.iter().map( | s | s.string.as_ref() ).collect();
  assert_eq!( strings, vec![ "", "" ], "single delimiter: two empty segments" );
}

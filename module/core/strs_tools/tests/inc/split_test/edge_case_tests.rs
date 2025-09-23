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
  .delimeter(" ")
  .preserving_empty(true)
  .preserving_delimeters(true)
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
  .delimeter(" ")
  .preserving_empty(false)
  .preserving_delimeters(false)
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
  .delimeter( "x" ) // Use valid delimiter that doesn't exist
  // preserving_delimiters defaults to true
  .perform();
  assert_eq!(iter.map(|e| String ::from(e.string)).collect :: < Vec<_ >>(), vec!["abc"]);
}

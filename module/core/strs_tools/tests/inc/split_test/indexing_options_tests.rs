//! Tests focusing on `nth`, `first`, and `last` indexing options.
use strs_tools::string::split::*;

// Test Matrix ID: T3.9
// Description: src="abc", del="b", PE=T, PD=T, S=F, Q=F, Idx=0 (first)
#[test]
fn test_m_t3_9_mod_index_first()
{
  let src = "abc";
  let mut iter = split()
  .src( src )
  .delimeter( "b" )
  .preserving_empty( true )
  .preserving_delimeters( true )
  .stripping( false )
  .quoting( false )
  .perform();

  let result = iter.next(); // Call next() on the iterator

  let expected_split = ("a", SplitType::Delimeted, 0, 1);
  assert!(result.is_some());
  let split_item = result.unwrap();
  assert_eq!(split_item.string, expected_split.0);
  assert_eq!(split_item.typ, expected_split.1);
  assert_eq!(split_item.start, expected_split.2);
  assert_eq!(split_item.end, expected_split.3);
}

// Test Matrix ID: T3.10
// Description: src="abc", del="b", PE=F, PD=F, S=F, Q=F, Idx=-1 (last)
#[test]
fn test_m_t3_10_mod_index_last()
{
  let src = "abc";
  let iter = split() // Changed from `let mut iter`
  .src( src )
  .delimeter( "b" )
  .preserving_empty( false )
  .preserving_delimeters( false )
  .stripping( false )
  .quoting( false )
  .perform();

  let result = iter.last(); // Call last() on the iterator

  let expected_split = ("c", SplitType::Delimeted, 2, 3);
  assert!(result.is_some());
  let split_item = result.unwrap();
  assert_eq!(split_item.string, expected_split.0);
  assert_eq!(split_item.typ, expected_split.1);
  assert_eq!(split_item.start, expected_split.2);
  assert_eq!(split_item.end, expected_split.3);
}

// Test Matrix ID: Index_Nth_Positive_Valid
// Description: src="a,b,c,d", del=",", Idx=1 (second element)
#[test]
fn test_scenario_index_positive_1()
{
  let src = "a,b,c,d";
  let mut iter = split()
  .src( src )
  .delimeter( "," )
  .preserving_empty( false )
  .preserving_delimeters( false )
  .perform();

  let result = iter.nth( 1 ); // Call nth(1) on the iterator

  let expected_split = ("b", SplitType::Delimeted, 2, 3);
  assert!(result.is_some());
  let split_item = result.unwrap();
  assert_eq!(split_item.string, expected_split.0);
  assert_eq!(split_item.typ, expected_split.1);
  assert_eq!(split_item.start, expected_split.2);
  assert_eq!(split_item.end, expected_split.3);
}

// Test Matrix ID: Index_Nth_Negative_Valid
// Description: src="a,b,c,d", del=",", Idx=-2 (second to last element)
// Note: Standard iterators' nth() does not support negative indexing.
// This test will need to collect and then index from the end, or use `iter.rev().nth(1)` for second to last.
// For simplicity and directness, collecting and indexing is clearer if `perform_tuple` is not used.
#[test]
fn test_scenario_index_negative_2()
{
  let src = "a,b,c,d";
  let splits: Vec<_> = split()
  .src( src )
  .delimeter( "," )
  .preserving_empty( false )
  .preserving_delimeters( false )
  .perform()
  .collect();

  assert!(splits.len() >= 2); // Ensure there are enough elements
  let result = splits.get(splits.len() - 2).cloned(); // Get second to last

  let expected_split = ("c", SplitType::Delimeted, 4, 5);
  assert!(result.is_some());
  let split_item = result.unwrap();
  assert_eq!(split_item.string, expected_split.0);
  assert_eq!(split_item.typ, expected_split.1);
  assert_eq!(split_item.start, expected_split.2);
  assert_eq!(split_item.end, expected_split.3);
}

// Test Matrix ID: Index_Nth_Positive_OutOfBounds
// Description: src="a,b", del=",", Idx=5
#[test]
fn test_scenario_index_out_of_bounds_positive()
{
  let src = "a,b";
  let mut iter = split()
  .src( src )
  .delimeter( "," )
  // preserving_delimeters defaults to true
  .perform();
  let result = iter.nth( 5 );
  assert!(result.is_none());
}

// Test Matrix ID: Index_Nth_Negative_OutOfBounds
// Description: src="a,b", del=",", Idx=-5
#[test]
fn test_scenario_index_out_of_bounds_negative()
{
  let src = "a,b";
  let splits: Vec<_> = split()
  .src( src )
  .delimeter( "," )
  // preserving_delimeters defaults to true
  .perform()
  .collect();
  let result = if 5 > splits.len() { None } else { splits.get(splits.len() - 5).cloned() };
  assert!(result.is_none());
}

// Test Matrix ID: Index_Nth_WithPreserving
// Description: src="a,,b", del=",", PE=T, PD=T, Idx=1 (second element, which is a delimiter)
#[test]
fn test_scenario_index_preserving_delimiters_and_empty()
{
  let src = "a,,b";
  let mut iter = split()
  .src( src )
  .delimeter( "," )
  .preserving_empty( true )
  .preserving_delimeters( true )
  .perform();

  let result = iter.nth( 1 ); // Get the second element (index 1)

  let expected_split = (",", SplitType::Delimiter, 1, 2);
  assert!(result.is_some());
  let split_item = result.unwrap();
  assert_eq!(split_item.string, expected_split.0);
  assert_eq!(split_item.typ, expected_split.1);
  assert_eq!(split_item.start, expected_split.2);
  assert_eq!(split_item.end, expected_split.3);
}
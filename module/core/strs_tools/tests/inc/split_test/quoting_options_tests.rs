//! Tests focusing on `quoting`, `preserving_quoting`, and `quotes` options.
use strs_tools::string::split::*;

// Test Matrix ID: Quote_Q_F_PQ_T
// Tests quoting(false) with preserving_quoting(true).
#[test]
fn test_quoting_disabled_preserving_quotes_true()
{
  let src = "a 'b' c";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .quoting( false )
  .preserving_delimeters( false )
  .preserving_empty( false )
  .preserving_quoting( true )
  .stripping( true )
  .perform();
  assert_eq!( iter.map( | e | String::from( e.string ) ).collect::< Vec< _ > >(), vec![ "a", "'b'", "c" ] );
}

// Test Matrix ID: Quote_Q_F_PQ_F
// Tests quoting(false) with preserving_quoting(false).
#[test]
fn test_quoting_disabled_preserving_quotes_false()
{
  let src = "a 'b' c";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .quoting( false )
  .preserving_delimeters( false )
  .preserving_empty( false )
  .preserving_quoting( false )
  .stripping( true )
  .perform();
  assert_eq!( iter.map( | e | String::from( e.string ) ).collect::< Vec< _ > >(), vec![ "a", "'b'", "c" ] );
}

// Test Matrix ID: Quote_Q_T_PQ_T
// Tests quoting(true) with preserving_quoting(true).
#[test]
fn test_quoting_enabled_preserving_quotes_true()
{
  let src = "a 'b' c";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_delimeters( false )
  .preserving_empty( false )
  .preserving_quoting( true )
  .stripping( true )
  .perform();
  assert_eq!( iter.map( | e | String::from( e.string ) ).collect::< Vec< _ > >(), vec![ "a", "'b'", "c" ] );
}

// Test Matrix ID: Quote_Q_T_PQ_F
// Tests quoting(true) with preserving_quoting(false).
#[test]
fn test_quoting_enabled_preserving_quotes_false()
{
  let src = "a 'b' c";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .quoting( true )
  .preserving_delimeters( false )
  .preserving_empty( false )
  .preserving_quoting( false )
  .stripping( true )
  .perform();
  assert_eq!( iter.map( | e | String::from( e.string ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );
}

// Test Matrix ID: T3.11
// Description: src="a 'b c' d", del=" ", PE=T, PD=T, S=F, Q=T
#[test]
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
    ("a", SplitType::Delimeted, 0, 1),
    (" ", SplitType::Delimeter, 1, 2),
    ("", SplitType::Delimeted, 2, 2),         // Empty segment before opening quote
    ("'b c'", SplitType::Delimeted, 2, 7), // Quotes preserved
    (" ", SplitType::Delimeter, 7, 8),
    ("d", SplitType::Delimeted, 8, 9),
  ];
  let results: Vec<_> = iter.collect();
  assert_eq!(results.len(), expected.len(), "Number of segments mismatch. Actual: {:?}, Expected: {:?}", results, expected);
  for (i, split_item) in results.iter().enumerate() {
    assert_eq!(split_item.string, expected[i].0, "String mismatch at index {}", i);
    assert_eq!(split_item.typ, expected[i].1, "Type mismatch at index {}", i);
    assert_eq!(split_item.start, expected[i].2, "Start index mismatch at index {}", i);
    assert_eq!(split_item.end, expected[i].3, "End index mismatch at index {}", i);
  }
}

// Test Matrix ID: T3.12
// Description: src="a 'b c' d", del=" ", PE=F, PD=F, S=T, Q=T
#[test]
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
  let expected = vec![
    ("a", SplitType::Delimeted, 0, 1),
    ("b c", SplitType::Delimeted, 3, 6), // Quotes stripped
    ("d", SplitType::Delimeted, 8, 9),
  ];
  for (i, split) in iter.enumerate() {
    assert_eq!(split.string, expected[i].0);
    assert_eq!(split.typ, expected[i].1);
    assert_eq!(split.start, expected[i].2);
    assert_eq!(split.end, expected[i].3);
  }
}

// Test Matrix ID: T3.13
// Description: src="a 'b c' d", del=" ", PE=T, PD=T, S=T, Q=T
#[test]
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
    ("a", SplitType::Delimeted, 0, 1),     // Stripping "a" is "a"
    (" ", SplitType::Delimeter, 1, 2),     // Delimiter preserved
    ("", SplitType::Delimeted, 2, 2),      // Empty segment before quote, preserved by PE=T
    ("'b c'", SplitType::Delimeted, 2, 7), // Quoted segment, PQ=T, stripping "'b c'" is "'b c'"
    (" ", SplitType::Delimeter, 7, 8),     // Delimiter preserved
    ("d", SplitType::Delimeted, 8, 9),     // Stripping "d" is "d"
  ];
  let results: Vec<_> = iter.collect();
  assert_eq!(results.len(), expected.len(), "Number of segments mismatch. Actual: {:?}, Expected: {:?}", results, expected);
  for (i, split_item) in results.iter().enumerate() {
    assert_eq!(split_item.string, expected[i].0, "String mismatch at index {}", i);
    assert_eq!(split_item.typ, expected[i].1, "Type mismatch at index {}", i);
    assert_eq!(split_item.start, expected[i].2, "Start index mismatch at index {}", i);
    assert_eq!(split_item.end, expected[i].3, "End index mismatch at index {}", i);
  }
}

// Test Matrix ID: T3.14
// Description: src="a 'b c' d", del=" ", PE=F, PD=F, S=F, Q=T
#[test]
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
    ("a", SplitType::Delimeted, 0, 1),
    ("'b c'", SplitType::Delimeted, 2, 7), // Quotes preserved
    ("d", SplitType::Delimeted, 8, 9),
  ];
  // With PE=F, the empty "" before "'b c'" should be skipped.
  let results: Vec<_> = iter.collect();
  assert_eq!(results.len(), expected.len(), "Number of segments mismatch. Actual: {:?}, Expected: {:?}", results, expected);
   for (i, split_item) in results.iter().enumerate() {
    assert_eq!(split_item.string, expected[i].0, "String mismatch at index {}", i);
    assert_eq!(split_item.typ, expected[i].1, "Type mismatch at index {}", i);
    assert_eq!(split_item.start, expected[i].2, "Start index mismatch at index {}", i);
    assert_eq!(split_item.end, expected[i].3, "End index mismatch at index {}", i);
  }
}

// Test Matrix ID: T3.15
// Description: src="a 'b c' d", del=" ", PE=T, PD=T, S=F, Q=F (Quoting disabled)
#[test]
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
    ("a", SplitType::Delimeted, 0, 1),
    (" ", SplitType::Delimeter, 1, 2),
    ("'b", SplitType::Delimeted, 2, 4), // 'b is a segment
    (" ", SplitType::Delimeter, 4, 5),
    ("c'", SplitType::Delimeted, 5, 7), // c' is a segment
    (" ", SplitType::Delimeter, 7, 8),
    ("d", SplitType::Delimeted, 8, 9),
  ];
  for (i, split) in iter.enumerate() {
    assert_eq!(split.string, expected[i].0);
    assert_eq!(split.typ, expected[i].1);
    assert_eq!(split.start, expected[i].2);
    assert_eq!(split.end, expected[i].3);
  }
}
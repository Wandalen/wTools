//! Tests for interactions between multiple options (e.g., quoting + stripping, preserving + indexing).
use strs_tools::string::split::*;

// Test Matrix ID: T3.13
// Description: src="a 'b c' d", del=" ", PE=T, PD=T, S=T, Q=T
#[test]
fn test_m_t3_13_quoting_preserve_all_strip() // Renamed from test_split_indices_t3_13
{
  let src = "a 'b c' d";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( true )
  .preserving_delimeters( true )
  .stripping( true ) // S=T
  .quoting( true )
  .preserving_quoting( true ) // Explicitly preserve quotes
  .perform();
  let expected = vec![
    ("a", SplitType::Delimeted, 0, 1),
    (" ", SplitType::Delimiter, 1, 2),
    ("", SplitType::Delimeted, 2, 2),      // Empty segment before quote
    ("'b c'", SplitType::Delimeted, 2, 7), // Quotes preserved, stripping does not affect non-whitespace quotes
    (" ", SplitType::Delimiter, 7, 8),
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
fn test_m_t3_12_quoting_no_preserve_strip() // Renamed from test_split_indices_t3_12
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

// Test Matrix ID: Combo_PE_T_PD_T_S_F
// Description: src="a b c", del=" ", PE=T, S=F, PD=T
#[test]
fn test_combo_preserve_empty_true_preserve_delimiters_true_no_strip()
{
  let src = "a b c";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( true )
  .preserving_delimeters( true )
  .stripping( false )
  .perform();
  assert_eq!( iter.map( | e | String::from( e.string ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );
}

// Test Matrix ID: Combo_PE_F_PD_T_S_F
// Description: src="a b c", del=" ", PE=F, S=F, PD=T
#[test]
fn test_combo_preserve_empty_false_preserve_delimiters_true_no_strip()
{
  let src = "a b c";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( false )
  .preserving_delimeters( true )
  .stripping( false )
  .perform();
  assert_eq!( iter.map( | e | String::from( e.string ) ).collect::< Vec< _ > >(), vec![ "a", " ", "b", " ", "c" ] );
}

// Test Matrix ID: Combo_PE_T_PD_F_S_T
// Description: src="a b c", del=" ", PE=T, S=T, PD=F
#[test]
fn test_combo_preserve_empty_true_strip_no_delimiters()
{
  let src = "a b c";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( true )
  .preserving_delimeters( false ) // Explicitly false
  .stripping( true )
  .perform();
  assert_eq!( iter.map( | e | String::from( e.string ) ).collect::< Vec< _ > >(), vec![ "a", "b", "c" ] );
}
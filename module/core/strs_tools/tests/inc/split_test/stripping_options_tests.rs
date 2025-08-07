//! Tests focusing on the `stripping` option.
use strs_tools::string::split::*;

// Test Matrix ID: Strip_S_T_PE_T_DefaultDelim
// Tests stripping(true) with default delimiter behavior (space).
// With PE=true, PD=T (new default), S=true: "a b c" -> "a", " ", "b", " ", "c"
#[test]
fn test_stripping_true_default_delimiter() {
  let src = "a b c";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .stripping( true )
  .preserving_empty( true ) // Explicitly set, though default PE is false.
  // preserving_delimeters defaults to true
  .perform();
  assert_eq!(
    iter.map(|e| String::from(e.string)).collect::<Vec<_>>(),
    vec!["a", " ", "b", " ", "c"]
  );
}

// Test Matrix ID: Strip_S_F_PD_T_DefaultDelim
// Tests stripping(false) with default delimiter behavior (space).
#[test]
fn test_stripping_false_default_delimiter() {
  let src = "a b c";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .stripping( false )
  .preserving_delimeters( true ) // Explicitly set, matches new default
  .perform();
  assert_eq!(
    iter.map(|e| String::from(e.string)).collect::<Vec<_>>(),
    vec!["a", " ", "b", " ", "c"]
  );
}

// Test Matrix ID: Strip_S_T_PD_T_CustomDelimB
// Tests stripping(true) with a custom delimiter 'b'.
#[test]
fn test_stripping_true_custom_delimiter_b() {
  let src = "a b c";
  let iter = split()
  .src( src )
  .delimeter( "b" )
  .stripping( true )
  .preserving_delimeters( true ) // Explicitly set, matches new default
  .perform();
  assert_eq!(iter.map(|e| String::from(e.string)).collect::<Vec<_>>(), vec!["a", "b", "c"]);
}

// Test Matrix ID: Strip_S_T_PD_F_CustomDelimB
// Tests stripping(true) with a custom delimiter 'b' and preserving_delimiters(false).
#[test]
fn test_stripping_true_custom_delimiter_b_no_preserve_delimiters() {
  let src = "a b c";
  let iter = split()
    .src(src)
    .delimeter("b")
    .preserving_delimeters(false)
    .stripping(true)
    .perform();
  assert_eq!(iter.map(|e| String::from(e.string)).collect::<Vec<_>>(), vec!["a", "c"]);
}

// Test Matrix ID: T3.2
// Description: src="a b c", del=" ", PE=F, PD=F, S=F, Q=F
// Note: This test has stripping(false) but is relevant to basic non-stripping behavior.
#[test]
fn test_m_t3_2_no_preserve_no_strip_no_quote() {
  let src = "a b c";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( false )
  .preserving_delimeters( false )
  .stripping( false ) // Key for this test, though it's in stripping_options_tests for grouping by original file
  .quoting( false )
  .perform();
  let expected = [("a", SplitType::Delimeted, 0, 1),
    ("b", SplitType::Delimeted, 2, 3),
    ("c", SplitType::Delimeted, 4, 5)];
  for (i, split) in iter.enumerate() {
    assert_eq!(split.string, expected[i].0);
    assert_eq!(split.typ, expected[i].1);
    assert_eq!(split.start, expected[i].2);
    assert_eq!(split.end, expected[i].3);
  }
}

// Test Matrix ID: T3.4
// Description: src=" a b ", del=" ", PE=F, PD=F, S=F, Q=F
// Note: This test has stripping(false).
#[test]
fn test_m_t3_4_leading_trailing_space_no_preserve_no_strip() {
  let src = " a b ";
  let iter = split()
  .src( src )
  .delimeter( " " )
  .preserving_empty( false )
  .preserving_delimeters( false )
  .stripping( false ) // Key for this test
  .quoting( false )
  .perform();
  let expected = [("a", SplitType::Delimeted, 1, 2), ("b", SplitType::Delimeted, 3, 4)];
  for (i, split) in iter.enumerate() {
    assert_eq!(split.string, expected[i].0);
    assert_eq!(split.typ, expected[i].1);
    assert_eq!(split.start, expected[i].2);
    assert_eq!(split.end, expected[i].3);
  }
}

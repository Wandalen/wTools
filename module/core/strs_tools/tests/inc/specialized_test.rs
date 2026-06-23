#[ allow( unused_imports ) ]
use super::*;
use the_module::string::specialized::*;

#[ test ]
fn test_single_char_split_basic() {
  let input = "apple,banana,cherry";
  let results: Vec<_> = SingleCharSplitIterator::new( input, ',', false )
    .collect();

  assert_eq!( results.len(), 3 );
  assert_eq!( results[0].as_str(), "apple" );
  assert_eq!( results[1].as_str(), "banana" );
  assert_eq!( results[2].as_str(), "cherry" );
}

#[ test ]
fn test_single_char_split_with_empty_segments() {
  let input = "a,,b,c";
  let results: Vec<_> = SingleCharSplitIterator::new( input, ',', false )
    .collect();

  assert_eq!( results.len(), 4 );
  assert_eq!( results[0].as_str(), "a" );
  assert_eq!( results[1].as_str(), "" );
  assert_eq!( results[2].as_str(), "b" );
  assert_eq!( results[3].as_str(), "c" );
}

#[ test ]
fn test_single_char_split_preserve_delimiter() {
  let input = "a,b,c";
  let results: Vec<_> = SingleCharSplitIterator::new( input, ',', true )
    .collect();

  assert_eq!( results.len(), 5 ); // a, ,, b, ,, c
  assert_eq!( results[0].as_str(), "a" );
  assert_eq!( results[1].as_str(), "," );
  assert_eq!( results[2].as_str(), "b" );
  assert_eq!( results[3].as_str(), "," );
  assert_eq!( results[4].as_str(), "c" );
}

#[ test ]
fn test_algorithm_selection_single_char() {
  assert_eq!( AlgorithmSelector::select_split_algorithm( &[","] ), SplitAlgorithm::SingleChar );
  assert_eq!( AlgorithmSelector::select_split_algorithm( &[" "] ), SplitAlgorithm::SingleChar );
  assert_eq!( AlgorithmSelector::select_split_algorithm( &["\t"] ), SplitAlgorithm::SingleChar ); // SingleChar takes precedence
}

#[ test ]
fn test_algorithm_selection_boyer_moore() {
  assert_eq!( AlgorithmSelector::select_split_algorithm( &["::"] ), SplitAlgorithm::BoyerMoore );
  assert_eq!( AlgorithmSelector::select_split_algorithm( &["->"] ), SplitAlgorithm::BoyerMoore );
}

#[ test ]
fn test_algorithm_selection_csv() {
  assert_eq!( AlgorithmSelector::select_split_algorithm( &[","] ), SplitAlgorithm::SingleChar ); // SingleChar wins over CSV for single chars
  assert_eq!( AlgorithmSelector::select_split_algorithm( &["\t"] ), SplitAlgorithm::SingleChar ); // SingleChar wins over CSV
  assert_eq!( AlgorithmSelector::select_split_algorithm( &[";"] ), SplitAlgorithm::SingleChar ); // SingleChar wins over CSV
}

#[ test ]
fn test_smart_split_integration() {
  let input = "field1,field2,field3,field4";
  let results: Vec<_> = smart_split( input, &[","] ).collect();

  assert_eq!( results.len(), 4 );
  assert_eq!( results[0].as_str(), "field1" );
  assert_eq!( results[1].as_str(), "field2" );
  assert_eq!( results[2].as_str(), "field3" );
  assert_eq!( results[3].as_str(), "field4" );
}

#[ test ]
fn test_split_result_conversions() {
  let borrowed = SplitResult::Borrowed( "test" );
  let owned = SplitResult::Owned( "test".to_string() );

  assert_eq!( borrowed.as_str(), "test" );
  assert_eq!( owned.as_str(), "test" );
  assert_eq!( borrowed.as_ref(), "test" );
  assert_eq!( owned.as_ref(), "test" );
}

#[ test ]
#[ should_panic( expected = "SingleChar optimization requires ASCII delimiter" ) ]
fn test_single_char_non_ascii_panic() {
  SingleCharSplitIterator::new( "test", '™', false );
}

#[ test ]
fn test_boyer_moore_split_basic() {
  let input = "field1::field2::field3::field4";
  let results: Vec<_> = BoyerMooreSplitIterator::new( input, "::" )
    .collect();

  assert_eq!( results.len(), 4 );
  assert_eq!( results[0].as_str(), "field1" );
  assert_eq!( results[1].as_str(), "field2" );
  assert_eq!( results[2].as_str(), "field3" );
  assert_eq!( results[3].as_str(), "field4" );
}

#[ test ]
fn test_boyer_moore_split_with_empty_segments() {
  let input = "a::::b::c";
  let results: Vec<_> = BoyerMooreSplitIterator::new( input, "::" )
    .collect();

  // Expected: "a", "", "b", "c" (4 segments)
  // Input positions: a at 0, :: at 1-2, :: at 3-4, b at 5, :: at 6-7, c at 8
  assert_eq!( results.len(), 4 );
  assert_eq!( results[0].as_str(), "a" );
  assert_eq!( results[1].as_str(), "" );
  assert_eq!( results[2].as_str(), "b" );
  assert_eq!( results[3].as_str(), "c" );
}

#[ test ]
fn test_boyer_moore_no_pattern() {
  let input = "no delimiters here";
  let results: Vec<_> = BoyerMooreSplitIterator::new( input, "::" )
    .collect();

  assert_eq!( results.len(), 1 );
  assert_eq!( results[0].as_str(), "no delimiters here" );
}

#[ test ]
fn test_boyer_moore_different_patterns() {
  let input = "a->b->c->d";
  let results: Vec<_> = BoyerMooreSplitIterator::new( input, "->" )
    .collect();

  assert_eq!( results.len(), 4 );
  assert_eq!( results[0].as_str(), "a" );
  assert_eq!( results[1].as_str(), "b" );
  assert_eq!( results[2].as_str(), "c" );
  assert_eq!( results[3].as_str(), "d" );
}

#[ test ]
#[ should_panic( expected = "Boyer-Moore requires non-empty pattern" ) ]
fn test_boyer_moore_empty_pattern_panic() {
  BoyerMooreSplitIterator::new( "test", "" );
}

#[ test ]
#[ should_panic( expected = "Boyer-Moore optimization requires pattern length >= 2" ) ]
fn test_boyer_moore_single_char_pattern_panic() {
  BoyerMooreSplitIterator::new( "test", "a" );
}

#[ test ]
#[ should_panic( expected = "Boyer-Moore optimization works best with pattern length <= 8" ) ]
fn test_boyer_moore_long_pattern_panic() {
  BoyerMooreSplitIterator::new( "test", "verylongpattern" );
}

#[ test ]
fn test_boyer_moore_vs_smart_split_integration() {
  let input = "namespace::class::method::args";

  // Smart split should automatically select Boyer-Moore for "::" pattern
  let smart_results: Vec<_> = smart_split( input, &["::"] ).collect();

  // Direct Boyer-Moore usage
  let bm_results: Vec<_> = BoyerMooreSplitIterator::new( input, "::" ).collect();

  assert_eq!( smart_results.len(), bm_results.len() );
  for ( smart, bm ) in smart_results.iter().zip( bm_results.iter() ) {
    assert_eq!( smart.as_str(), bm.as_str() );
  }
}

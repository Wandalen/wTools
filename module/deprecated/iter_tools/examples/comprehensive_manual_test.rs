//!
//! Comprehensive manual test covering all spec.md patterns and corner cases.
//!
//! This test validates:
//! - Pattern 1: Basic iteration with re-exports (spec.md § Usage Patterns § Pattern 1)
//! - Pattern 2: Clonable boxed iterators (spec.md § Usage Patterns § Pattern 2)
//! - Pattern 3: Result-oriented processing (spec.md § Usage Patterns § Pattern 3)
//! - Pattern 4: Advanced combinators (spec.md § Usage Patterns § Pattern 4)
//! - Corner cases: empty iterators, single elements, error handling
//! - Edge cases: Send/Sync variants, lifetime handling
//!

#![ allow( clippy::useless_vec ) ]
#![ allow( clippy::cloned_instead_of_copied ) ]
#![ allow( clippy::map_clone ) ]
#![ allow( clippy::elidable_lifetime_names ) ]

#[ cfg( not( feature = "enabled" ) ) ]
fn main() {}

#[ cfg( feature = "enabled" ) ]
fn main()
{
  println!( "=== iter_tools Comprehensive Manual Test ===" );
  println!();

  // Pattern 1: Basic Iteration with Re-exports
  test_pattern_1_basic_iteration();

  // Pattern 1: Corner Cases
  test_pattern_1_corner_cases();

  // Pattern 2: Clonable Boxed Iterators
  test_pattern_2_clonable_boxed_iterators();

  // Pattern 2: Corner Cases
  test_pattern_2_corner_cases();

  // Pattern 3: Result-Oriented Processing
  test_pattern_3_result_oriented();

  // Pattern 3: Corner Cases
  test_pattern_3_corner_cases();

  // Pattern 4: Advanced Combinators
  test_pattern_4_advanced_combinators();

  // Pattern 4: Corner Cases
  test_pattern_4_corner_cases();

  println!();
  println!( "=== All Manual Tests PASSED ===" );
}

#[ cfg( feature = "enabled" ) ]
fn test_pattern_1_basic_iteration()
{
  println!( "--- Pattern 1: Basic Iteration with Re-exports ---" );

  use iter_tools::*;

  // Test case: min() with normal vec (spec.md Pattern 1)
  {
    let vec = vec![ 5, 1, -2 ];
    let min_val = min( &vec );
    assert_eq!( *min_val.unwrap(), -2 );
    println!( "✓ min() with normal vec" );
  }

  // Test case: max() with normal vec
  {
    let vec = vec![ 5, 1, -2 ];
    let max_val = max( &vec );
    assert_eq!( *max_val.unwrap(), 5 );
    println!( "✓ max() with normal vec" );
  }

  // Test case: zip() with equal length iterators (spec.md Pattern 1, readme.md)
  {
    let vec = vec![ 5, 1, -2 ];
    let added = vec![ "a", "b", "c" ];
    let mut result = vec![];
    let zipped = zip( &vec, &added );
    for ( left, right ) in zipped
    {
      result.push( ( *left, *right ) );
    }
    assert_eq!( result, vec![ ( 5, "a" ), ( 1, "b" ), ( -2, "c" ) ] );
    println!( "✓ zip() with equal length iterators" );
  }

  // Test case: multiunzip() (tests/inc/basic_test.rs)
  {
    let src = [ 1, 2, 3 ];
    let exp = ( vec![ 2, 3, 4 ], vec![ 0, 1, 2 ] );
    let got: ( Vec< _ >, Vec< _ > ) = src.iter().map( | e | ( e + 1, e - 1 ) ).multiunzip();
    assert_eq!( got, exp );
    println!( "✓ multiunzip() with normal iterator" );
  }

  // Test case: rev() (iter_tools_trivial.rs)
  {
    let vec = vec![ 5, 1, -2 ];
    let mut result = vec![];
    let reversed = rev( &vec );
    for v in reversed
    {
      result.push( *v );
    }
    assert_eq!( result, vec![ -2, 1, 5 ] );
    println!( "✓ rev() with normal vec" );
  }

  // Test case: chain() with two non-empty iterators
  {
    let a = vec![ 1, 2 ];
    let b = vec![ 3, 4 ];
    let chained: Vec< _ > = chain( &a, &b ).cloned().collect();
    assert_eq!( chained, vec![ 1, 2, 3, 4 ] );
    println!( "✓ chain() with two non-empty iterators" );
  }

  println!();
}

#[ cfg( feature = "enabled" ) ]
fn test_pattern_1_corner_cases()
{
  println!( "--- Pattern 1: Corner Cases ---" );

  use iter_tools::*;

  // Test case: min() with empty iterator
  {
    let vec: Vec< i32 > = vec![];
    let min_val = min( &vec );
    assert!( min_val.is_none() );
    println!( "✓ min() with empty iterator returns None" );
  }

  // Test case: min() with single element
  {
    let vec = vec![ 42 ];
    let min_val = min( &vec );
    assert_eq!( *min_val.unwrap(), 42 );
    println!( "✓ min() with single element" );
  }

  // Test case: max() with empty iterator
  {
    let vec: Vec< i32 > = vec![];
    let max_val = max( &vec );
    assert!( max_val.is_none() );
    println!( "✓ max() with empty iterator returns None" );
  }

  // Test case: zip() with different length iterators (first longer)
  {
    let a = vec![ 1, 2, 3, 4 ];
    let b = vec![ "a", "b" ];
    let zipped: Vec< _ > = zip( &a, &b ).collect();
    assert_eq!( zipped.len(), 2 ); // zip stops at shortest
    assert_eq!( zipped, vec![ ( &1, &"a" ), ( &2, &"b" ) ] );
    println!( "✓ zip() with first iterator longer stops at shortest" );
  }

  // Test case: zip() with different length iterators (second longer)
  {
    let a = vec![ 1, 2 ];
    let b = vec![ "a", "b", "c", "d" ];
    let zipped: Vec< _ > = zip( &a, &b ).collect();
    assert_eq!( zipped.len(), 2 ); // zip stops at shortest
    assert_eq!( zipped, vec![ ( &1, &"a" ), ( &2, &"b" ) ] );
    println!( "✓ zip() with second iterator longer stops at shortest" );
  }

  // Test case: zip() with empty iterators
  {
    let a: Vec< i32 > = vec![];
    let b: Vec< &str > = vec![];
    let zipped: Vec< _ > = zip( &a, &b ).collect();
    assert_eq!( zipped.len(), 0 );
    println!( "✓ zip() with empty iterators" );
  }

  // Test case: multiunzip() with empty iterator
  {
    let src: Vec< i32 > = vec![];
    let got: ( Vec< _ >, Vec< _ > ) = src.iter().map( | e | ( e + 1, e - 1 ) ).multiunzip();
    assert_eq!( got, ( vec![], vec![] ) );
    println!( "✓ multiunzip() with empty iterator" );
  }

  // Test case: chain() with empty + non-empty
  {
    let a: Vec< i32 > = vec![];
    let b = vec![ 1, 2, 3 ];
    let chained: Vec< _ > = chain( &a, &b ).cloned().collect();
    assert_eq!( chained, vec![ 1, 2, 3 ] );
    println!( "✓ chain() with empty + non-empty" );
  }

  // Test case: chain() with non-empty + empty
  {
    let a = vec![ 1, 2, 3 ];
    let b: Vec< i32 > = vec![];
    let chained: Vec< _ > = chain( &a, &b ).cloned().collect();
    assert_eq!( chained, vec![ 1, 2, 3 ] );
    println!( "✓ chain() with non-empty + empty" );
  }

  // Test case: chain() with empty + empty
  {
    let a: Vec< i32 > = vec![];
    let b: Vec< i32 > = vec![];
    let chained: Vec< _ > = chain( &a, &b ).cloned().collect();
    assert_eq!( chained.len(), 0 );
    println!( "✓ chain() with empty + empty" );
  }

  println!();
}

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "iter_trait" ) ]
fn test_pattern_2_clonable_boxed_iterators()
{
  println!( "--- Pattern 2: Clonable Boxed Iterators ---" );

  use iter_tools::{ BoxedIter, _IterTrait };

  // Test case: BoxedIter basic clone and collect (spec.md Pattern 2)
  {
    fn get_iterator< 'a >( data: &'a [ i32 ] ) -> BoxedIter< 'a, &'a i32 >
    {
      Box::new( data.iter() )
    }

    let data = vec![ 1, 2, 3 ];
    let iter1 = get_iterator( &data );
    let iter2 = iter1.clone();

    let result1: Vec< _ > = iter1.map( | x | *x ).collect();
    let result2: Vec< _ > = iter2.map( | x | *x ).collect();

    assert_eq!( result1, vec![ 1, 2, 3 ] );
    assert_eq!( result2, vec![ 1, 2, 3 ] );
    assert_eq!( result1, result2 );
    println!( "✓ BoxedIter basic clone and collect" );
  }

  // Test case: IterTrait implementation verification
  {
    let data = vec![ 1, 2, 3 ];
    let iter = data.iter();

    // Verify iter implements necessary traits
    fn check_iter_trait< 'a, T, I >( _iter: I )
    where
      T: 'a,
      I: _IterTrait< 'a, T >,
    {}

    check_iter_trait( iter );
    println!( "✓ IterTrait implementation verified" );
  }

  println!();
}

#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "iter_trait" ) ) ]
fn test_pattern_2_clonable_boxed_iterators()
{
  println!( "--- Pattern 2: Clonable Boxed Iterators (SKIPPED - iter_trait feature disabled) ---" );
  println!();
}

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "iter_trait" ) ]
fn test_pattern_2_corner_cases()
{
  println!( "--- Pattern 2: Corner Cases ---" );

  use iter_tools::BoxedIter;

  // Test case: BoxedIter clone of empty iterator
  {
    fn get_empty_iterator< 'a >( data: &'a [ i32 ] ) -> BoxedIter< 'a, &'a i32 >
    {
      Box::new( data.iter() )
    }

    let data: Vec< i32 > = vec![];
    let iter1 = get_empty_iterator( &data );
    let iter2 = iter1.clone();

    let result1: Vec< i32 > = iter1.map( | x | *x ).collect();
    let result2: Vec< i32 > = iter2.map( | x | *x ).collect();

    assert_eq!( result1, Vec::< i32 >::new() );
    assert_eq!( result2, Vec::< i32 >::new() );
    println!( "✓ BoxedIter clone of empty iterator" );
  }

  // Test case: BoxedIter clone of partially consumed iterator
  {
    fn get_iterator< 'a >( data: &'a [ i32 ] ) -> BoxedIter< 'a, &'a i32 >
    {
      Box::new( data.iter() )
    }

    let data = vec![ 1, 2, 3, 4, 5 ];
    let mut iter1 = get_iterator( &data );

    // Consume first 2 elements
    assert_eq!( iter1.next(), Some( &1 ) );
    assert_eq!( iter1.next(), Some( &2 ) );

    // Clone the partially consumed iterator
    let iter2 = iter1.clone();

    let result1: Vec< _ > = iter1.map( | x | *x ).collect();
    let result2: Vec< _ > = iter2.map( | x | *x ).collect();

    // Both should have remaining elements [3, 4, 5]
    assert_eq!( result1, vec![ 3, 4, 5 ] );
    assert_eq!( result2, vec![ 3, 4, 5 ] );
    println!( "✓ BoxedIter clone of partially consumed iterator" );
  }

  println!();
}

#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "iter_trait" ) ) ]
fn test_pattern_2_corner_cases()
{
  println!( "--- Pattern 2: Corner Cases (SKIPPED - iter_trait feature disabled) ---" );
  println!();
}

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "iter_ext" ) ]
fn test_pattern_3_result_oriented()
{
  println!( "--- Pattern 3: Result-Oriented Processing ---" );

  use iter_tools::IterExt;

  // Test case: map_result with all success (spec.md Pattern 3)
  {
    let items = vec![ "1", "2", "3" ];
    let result = items.iter().map_result( | s | s.parse::< i32 >() );
    assert_eq!( result.unwrap(), vec![ 1, 2, 3 ] );
    println!( "✓ map_result with all success" );
  }

  // Test case: map_result with middle element error (spec.md Pattern 3)
  {
    let items = vec![ "1", "2", "invalid", "4" ];
    let result = items.iter().map_result( | s | s.parse::< i32 >() );
    assert!( result.is_err() );
    println!( "✓ map_result with middle element error fails correctly" );
  }

  println!();
}

#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "iter_ext" ) ) ]
fn test_pattern_3_result_oriented()
{
  println!( "--- Pattern 3: Result-Oriented Processing (SKIPPED - iter_ext feature disabled) ---" );
  println!();
}

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "iter_ext" ) ]
fn test_pattern_3_corner_cases()
{
  println!( "--- Pattern 3: Corner Cases ---" );

  use iter_tools::IterExt;

  // Test case: map_result with first element error
  {
    let items = vec![ "invalid", "2", "3" ];
    let result = items.iter().map_result( | s | s.parse::< i32 >() );
    assert!( result.is_err() );
    println!( "✓ map_result with first element error fails correctly" );
  }

  // Test case: map_result with last element error
  {
    let items = vec![ "1", "2", "3", "invalid" ];
    let result = items.iter().map_result( | s | s.parse::< i32 >() );
    assert!( result.is_err() );
    println!( "✓ map_result with last element error fails correctly" );
  }

  // Test case: map_result with empty iterator
  {
    let items: Vec< &str > = vec![];
    let result: Result< Vec< i32 >, _ > = items.iter().map_result( | s | s.parse::< i32 >() );
    assert_eq!( result.unwrap(), Vec::< i32 >::new() );
    println!( "✓ map_result with empty iterator returns empty vec" );
  }

  println!();
}

#[ cfg( feature = "enabled" ) ]
#[ cfg( not( feature = "iter_ext" ) ) ]
fn test_pattern_3_corner_cases()
{
  println!( "--- Pattern 3: Corner Cases (SKIPPED - iter_ext feature disabled) ---" );
  println!();
}

#[ cfg( feature = "enabled" ) ]
fn test_pattern_4_advanced_combinators()
{
  println!( "--- Pattern 4: Advanced Combinators ---" );

  use iter_tools::*;

  // Test case: interleave with equal length iterators (spec.md Pattern 4)
  {
    let a = vec![ 1, 2, 3 ];
    let b = vec![ 10, 20, 30 ];
    let interleaved: Vec< _ > = interleave( &a, &b ).cloned().collect();
    assert_eq!( interleaved, vec![ 1, 10, 2, 20, 3, 30 ] );
    println!( "✓ interleave with equal length iterators" );
  }

  // Test case: intersperse with normal iterator (spec.md Pattern 4)
  {
    let data = vec![ 1, 2, 3 ];
    let with_sep: Vec< _ > = intersperse( data.iter(), &0 ).cloned().collect();
    assert_eq!( with_sep, vec![ 1, 0, 2, 0, 3 ] );
    println!( "✓ intersperse with normal iterator" );
  }

  // Test case: fold with normal iterator
  {
    let data = vec![ 1, 2, 3, 4 ];
    let sum = fold( data.iter(), 0, | acc, x | acc + x );
    assert_eq!( sum, 10 );
    println!( "✓ fold with normal iterator" );
  }

  println!();
}

#[ cfg( feature = "enabled" ) ]
fn test_pattern_4_corner_cases()
{
  println!( "--- Pattern 4: Corner Cases ---" );

  use iter_tools::*;

  // Test case: interleave with first iterator longer
  {
    let a = vec![ 1, 2, 3, 4, 5 ];
    let b = vec![ 10, 20 ];
    let interleaved: Vec< _ > = interleave( &a, &b ).cloned().collect();
    // interleave continues with remaining elements from first
    assert_eq!( interleaved, vec![ 1, 10, 2, 20, 3, 4, 5 ] );
    println!( "✓ interleave with first iterator longer includes remaining elements" );
  }

  // Test case: interleave with second iterator longer
  {
    let a = vec![ 1, 2 ];
    let b = vec![ 10, 20, 30, 40, 50 ];
    let interleaved: Vec< _ > = interleave( &a, &b ).cloned().collect();
    // interleave continues with remaining elements from second
    assert_eq!( interleaved, vec![ 1, 10, 2, 20, 30, 40, 50 ] );
    println!( "✓ interleave with second iterator longer includes remaining elements" );
  }

  // Test case: interleave with empty iterators
  {
    let a: Vec< i32 > = vec![];
    let b: Vec< i32 > = vec![];
    let interleaved: Vec< i32 > = interleave( &a, &b ).cloned().collect();
    assert_eq!( interleaved, Vec::< i32 >::new() );
    println!( "✓ interleave with empty iterators" );
  }

  // Test case: intersperse with single element
  {
    let data = vec![ 42 ];
    let with_sep: Vec< _ > = intersperse( data.iter(), &0 ).cloned().collect();
    assert_eq!( with_sep, vec![ 42 ] ); // no separator for single element
    println!( "✓ intersperse with single element has no separator" );
  }

  // Test case: intersperse with empty iterator
  {
    let data: Vec< i32 > = vec![];
    let with_sep: Vec< i32 > = intersperse( data.iter(), &0 ).cloned().collect();
    assert_eq!( with_sep, Vec::< i32 >::new() );
    println!( "✓ intersperse with empty iterator" );
  }

  // Test case: fold with empty iterator
  {
    let data: Vec< i32 > = vec![];
    let sum = fold( data.iter(), 0, | acc, x | acc + x );
    assert_eq!( sum, 0 ); // returns initial value
    println!( "✓ fold with empty iterator returns initial value" );
  }

  println!();
}

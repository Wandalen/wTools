//!
//! Corner case tests for `interval_adapter`.
//!
//! Tests boundary conditions, edge cases, and exceptional scenarios including:
//! - Empty intervals (left == right)
//! - Single-element intervals
//! - Negative value intervals
//! - Mixed sign intervals
//! - Boundary value intervals (MIN/MAX)
//! - Zero-length iterations
//!

#![ cfg_attr( feature = "no_std", no_std ) ]
#![ allow( clippy ::reversed_empty_ranges ) ] // Intentionally testing empty/reverse intervals

#[ allow( unused_imports ) ]
use interval_adapter as the_module;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;
use test_tools :: a_id;

tests_impls!
{
  //
  // Empty Intervals: left == right
  //

  fn empty_interval_half_open()
  {
    use the_module :: { IterableInterval, NonIterableInterval, Bound };

    // Half-open interval 5..5 should be empty (yields no elements)
    let src = 5..5;
    a_id!( src.closed_left(), 5 );
    a_id!( src.closed_right(), 4 ); // Excluded(5) → 4
    a_id!( src.closed_len(), 0 ); // 4 - 5 + 1 = 0

    // Iteration should yield zero elements
    let mut count = 0;
    for _ in 5..5
    {
      count += 1;
    }
    a_id!( count, 0 );
  }

  fn empty_interval_closed()
  {
    use the_module :: { IterableInterval, NonIterableInterval };

    // Closed interval with impossible bounds: 5..=4 (right < left)
    // This creates reverse interval which spec says is NOT validated
    let src = 5..=4;
    a_id!( src.closed_left(), 5 );
    a_id!( src.closed_right(), 4 );
    // Note: closed_len will underflow/wrap for reverse intervals
  }

  //
  // Single Element Intervals
  //

  fn single_element_interval_closed()
  {
    use the_module :: { IterableInterval, NonIterableInterval };

    // Single element: 5..=5
    let src = 5..=5;
    a_id!( src.closed_left(), 5 );
    a_id!( src.closed_right(), 5 );
    a_id!( src.closed_len(), 1 );

    // Should yield exactly one element
    let mut count = 0;
    let mut last = 0;
    for i in 5..=5
    {
      count += 1;
      last = i;
    }
    a_id!( count, 1 );
    a_id!( last, 5 );
  }

  fn single_element_interval_half_open()
  {
    use the_module :: { IterableInterval, NonIterableInterval };

    // Single element: 5..6 (half-open)
    let src = 5..6;
    a_id!( src.closed_left(), 5 );
    a_id!( src.closed_right(), 5 ); // Excluded(6) → 5
    a_id!( src.closed_len(), 1 );

    // Should yield exactly one element
    let mut count = 0;
    let mut last = 0;
    for i in 5..6
    {
      count += 1;
      last = i;
    }
    a_id!( count, 1 );
    a_id!( last, 5 );
  }

  fn single_element_at_zero()
  {
    use the_module :: { IterableInterval, NonIterableInterval };

    // Edge case: single element at zero
    let src = 0..=0;
    a_id!( src.closed_left(), 0 );
    a_id!( src.closed_right(), 0 );
    a_id!( src.closed_len(), 1 );

    let mut count = 0;
    for i in 0..=0
    {
      count += 1;
      a_id!( i, 0 );
    }
    a_id!( count, 1 );
  }

  //
  // Negative Intervals
  //

  fn negative_interval_closed()
  {
    use the_module :: { IterableInterval, NonIterableInterval };

    // Negative range: -5..=-1
    let src = -5..=-1;
    a_id!( src.closed_left(), -5 );
    a_id!( src.closed_right(), -1 );
    a_id!( src.closed_len(), 5 );

    // Verify iteration produces correct elements
    let mut count = 0;
    let expected = [ -5, -4, -3, -2, -1 ];
    for i in -5..=-1
    {
      a_id!( i, expected[ count ] );
      count += 1;
    }
    a_id!( count, 5 );
  }

  fn negative_interval_half_open()
  {
    use the_module :: { IterableInterval, NonIterableInterval };

    // Negative half-open: -5..-1
    let src = -5..-1;
    a_id!( src.closed_left(), -5 );
    a_id!( src.closed_right(), -2 ); // Excluded(-1) → -2
    a_id!( src.closed_len(), 4 );

    // Verify iteration
    let mut count = 0;
    let expected = [ -5, -4, -3, -2 ];
    for i in -5..-1
    {
      a_id!( i, expected[ count ] );
      count += 1;
    }
    a_id!( count, 4 );
  }

  fn mixed_sign_interval()
  {
    use the_module :: { IterableInterval, NonIterableInterval };

    // Mixed signs: -2..=3
    let src = -2..=3;
    a_id!( src.closed_left(), -2 );
    a_id!( src.closed_right(), 3 );
    a_id!( src.closed_len(), 6 );

    // Verify iteration crosses zero correctly
    let mut count = 0;
    let expected = [ -2, -1, 0, 1, 2, 3 ];
    for i in -2..=3
    {
      a_id!( i, expected[ count ] );
      count += 1;
    }
    a_id!( count, 6 );
  }

  fn crossing_zero_half_open()
  {
    use the_module :: { IterableInterval, NonIterableInterval };

    // Crossing zero: -3..2
    let src = -3..2;
    a_id!( src.closed_left(), -3 );
    a_id!( src.closed_right(), 1 ); // Excluded(2) → 1
    a_id!( src.closed_len(), 5 );

    // Verify zero crossing iteration
    let mut count = 0;
    let expected = [ -3, -2, -1, 0, 1 ];
    for i in -3..2
    {
      a_id!( i, expected[ count ] );
      count += 1;
    }
    a_id!( count, 5 );
  }

  //
  // Unbounded Interval Conversions
  //

  fn unbounded_left_closed_conversion()
  {
    use the_module :: { NonIterableInterval, Bound };

    // ..=5 (unbounded left, included right)
    let src = ..=5;
    a_id!( src.left(), Bound ::< isize >::Unbounded );
    a_id!( src.right(), Bound ::Included( 5 ) );
    a_id!( NonIterableInterval ::< isize >::closed_left( &src ), 0 ); // Unbounded left → 0
    a_id!( NonIterableInterval ::< isize >::closed_right( &src ), 5 );
  }

  fn unbounded_right_closed_conversion()
  {
    use the_module :: { NonIterableInterval, Bound };

    // 5.. (unbounded right)
    let src = 5..;
    a_id!( src.left(), Bound ::Included( 5 ) );
    a_id!( src.right(), Bound ::< isize >::Unbounded );
    a_id!( NonIterableInterval ::< isize >::closed_left( &src ), 5 );
    a_id!( NonIterableInterval ::< isize >::closed_right( &src ), isize ::MAX ); // Unbounded right → isize::MAX
  }

  fn fully_unbounded_conversion()
  {
    use the_module :: { NonIterableInterval, Bound };

    // .. (fully unbounded)
    let src = ..;
    a_id!( src.left(), Bound ::< isize >::Unbounded );
    a_id!( src.right(), Bound ::< isize >::Unbounded );
    a_id!( NonIterableInterval ::< isize >::closed_left( &src ), 0 ); // Unbounded left → 0
    a_id!( NonIterableInterval ::< isize >::closed_right( &src ), isize ::MAX ); // Unbounded right → isize::MAX
  }

  //
  // Array and Tuple Conversions with Edge Cases
  //

  fn array_conversion_empty()
  {
    use the_module :: { IntoInterval, NonIterableInterval };

    // Array with same values creates empty half-open interval conceptually
    let src = [ 5, 5 ];
    let interval = src.into_interval();
    a_id!( interval.closed_left(), 5 );
    a_id!( interval.closed_right(), 5 );
    a_id!( interval.closed_len(), 1 ); // Arrays are treated as closed intervals
  }

  fn tuple_conversion_negative()
  {
    use the_module :: { IntoInterval, NonIterableInterval };

    // Tuple with negative values
    let src = ( -10, -5 );
    let interval = src.into_interval();
    a_id!( interval.closed_left(), -10 );
    a_id!( interval.closed_right(), -5 );
    a_id!( interval.closed_len(), 6 );
  }

  fn bound_tuple_excluded_both()
  {
    use the_module :: { IntoInterval, NonIterableInterval, Bound };

    // Tuple with both bounds excluded
    let src = ( Bound ::Excluded( 0 ), Bound ::Excluded( 5 ) );
    let interval = src.into_interval();
    a_id!( interval.closed_left(), 1 ); // Excluded(0) → 1
    a_id!( interval.closed_right(), 4 ); // Excluded(5) → 4
    a_id!( interval.closed_len(), 4 );
  }

  fn bound_tuple_mixed_bounds()
  {
    use the_module :: { IntoInterval, NonIterableInterval, Bound };

    // Mixed: left excluded, right included
    let src = ( Bound ::Excluded( 0 ), Bound ::Included( 5 ) );
    let interval = src.into_interval();
    a_id!( interval.closed_left(), 1 );
    a_id!( interval.closed_right(), 5 );
    a_id!( interval.closed_len(), 5 );
  }
}

tests_index!
{
  empty_interval_half_open,
  empty_interval_closed,
  single_element_interval_closed,
  single_element_interval_half_open,
  single_element_at_zero,
  negative_interval_closed,
  negative_interval_half_open,
  mixed_sign_interval,
  crossing_zero_half_open,
  unbounded_left_closed_conversion,
  unbounded_right_closed_conversion,
  fully_unbounded_conversion,
  array_conversion_empty,
  tuple_conversion_negative,
  bound_tuple_excluded_both,
  bound_tuple_mixed_bounds,
}

//! Trivial example demonstrating `data_type` facade crate usage.
//!
//! Shows basic usage of:
//! - Interval types (via `interval_adapter`)
//! - Collection macros (via `collection_tools`)
//! - Either type (via `either` crate)

// Collection macros must be imported from collection_tools crate directly
// They can't be re-exported through modules due to macro system limitations
#[ cfg( feature = "dt_collection" ) ]
use collection_tools::{ hmap, hset, bmap, bset };

#[ cfg( feature = "enabled" ) ]
fn main()
{
  // Interval types demonstration (dt_interval feature)
  #[ cfg( feature = "dt_interval" ) ]
  {
    use data_type::{ Interval, Bound };

    println!( "=== Interval Types ===" );

    // Basic interval from range
    let interval = Interval::from( 1..10 );
    println!( "Interval from 1..10: {interval:?}" );

    // Interval with explicit bounds
    let bounded = Interval::new( Bound::Included( 0 ), Bound::Excluded( 100 ) );
    println!( "Bounded interval [0, 100): {bounded:?}" );

    println!();
  }

  // Collection macros demonstration (dt_collection feature)
  #[ cfg( feature = "dt_collection" ) ]
  {
    println!( "=== Collection Macros ===" );

    // HashMap macro
    let hash_map = hmap! { "alice" => 30, "bob" => 25, "carol" => 35 };
    println!( "HashMap: {hash_map:?}" );

    // HashSet macro
    let hash_set = hset! { 1, 2, 3, 2, 1 };
    println!( "HashSet: {hash_set:?}" );

    // BTreeMap macro
    let btree_map = bmap! { 1 => "one", 2 => "two", 3 => "three" };
    println!( "BTreeMap: {btree_map:?}" );

    // BTreeSet macro
    let btree_set = bset! { 3, 1, 4, 1, 5 };
    println!( "BTreeSet: {btree_set:?}" );

    println!();
  }

  // Either type demonstration (dt_either feature)
  #[ cfg( feature = "dt_either" ) ]
  {
    use data_type::Either;

    println!( "=== Either Type ===" );

    // Either::Left
    let left_value : Either< i32, &str > = Either::Left( 42 );
    println!( "Left value: {left_value:?}" );

    // Either::Right
    let right_value : Either< i32, &str > = Either::Right( "hello" );
    println!( "Right value: {right_value:?}" );

    // flip() method
    let flipped = Either::< i32, i32 >::Left( 13 ).flip();
    println!( "Flipped Left(13): {flipped:?}" );

    // Pattern matching
    match left_value
    {
      Either::Left( n ) => println!( "Matched Left: {n}" ),
      Either::Right( s ) => println!( "Matched Right: {s}" ),
    }

    println!();
  }

  println!( "All features demonstrated successfully!" );
}

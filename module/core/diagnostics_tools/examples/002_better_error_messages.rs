//! # Example 002: Better Error Messages  
//!
//! This example shows the power of enhanced error messages and diff output.
//! You'll see why diagnostics_tools is superior for debugging complex data.
//!
//! ## What you'll learn:
//! - Value comparison with `a_id!` and `a_not_id!`
//! - Beautiful diff output for mismatched data
//! - How to debug complex structures effectively
//!
//! ## Run this example:
//! ```bash
//! cargo run --example 002_better_error_messages
//! ```

use diagnostics_tools::*;
use std::collections::HashMap;

#[ derive( Debug, PartialEq ) ]
struct User
{
  name : String,
  age : u32,
  email : String,
  active : bool,
}

fn main()
{
  println!( "🔍 Demonstrating enhanced error messages and diffs" );
  println!( "This example shows successful comparisons. To see error diffs," );
  println!( "uncomment the examples in the demonstrate_failures() function.\n" );

  // ✅ Basic value comparisons
  println!( "1. Basic value comparisons:" );
  
  let expected_count = 5;
  let actual_count = 5;
  
  // Instead of assert_eq!(a, b), use a_id!(a, b) 
  a_id!( actual_count, expected_count );
  println!( "   ✓ Counts match: {}", actual_count );
  
  // Instead of assert_ne!(a, b), use a_not_id!(a, b)
  a_not_id!( actual_count, 0 );
  println!( "   ✓ Count is not zero" );

  // ✅ String comparisons
  println!( "\n2. String comparisons:" );
  
  let greeting = "Hello, World!";
  let expected = "Hello, World!";
  
  a_id!( greeting, expected );
  println!( "   ✓ Greeting matches expected value" );

  // ✅ Vector comparisons  
  println!( "\n3. Vector comparisons:" );
  
  let fruits = vec![ "apple", "banana", "cherry" ];
  let expected_fruits = vec![ "apple", "banana", "cherry" ];
  
  a_id!( fruits, expected_fruits );
  println!( "   ✓ Fruit lists are identical" );

  // ✅ Struct comparisons
  println!( "\n4. Struct comparisons:" );
  
  let user = User
  {
    name : "Alice".to_string(),
    age : 30,
    email : "alice@example.com".to_string(), 
    active : true,
  };
  
  let expected_user = User
  {
    name : "Alice".to_string(),
    age : 30,
    email : "alice@example.com".to_string(),
    active : true, 
  };
  
  a_id!( user, expected_user );
  println!( "   ✓ User structs are identical" );

  // ✅ HashMap comparisons
  println!( "\n5. HashMap comparisons:" );
  
  let mut scores = HashMap::new();
  scores.insert( "Alice", 95 );
  scores.insert( "Bob", 87 );
  
  let mut expected_scores = HashMap::new();
  expected_scores.insert( "Alice", 95 );
  expected_scores.insert( "Bob", 87 );
  
  a_id!( scores, expected_scores );
  println!( "   ✓ Score maps are identical" );

  println!( "\n🎉 All comparisons passed!" );
  
  // Show what failure looks like (but commented out so example succeeds)
  demonstrate_failures();
  
  println!( "\n💡 Key advantages of diagnostics_tools:" );
  println!( "   • Colored diff output shows exactly what differs" );
  println!( "   • Works with any type that implements Debug + PartialEq" );
  println!( "   • Structured formatting makes complex data easy to read" );
  println!( "   • Same performance as standard assertions" );
  println!( "\n➡️  Next: Run example 003 to learn about compile-time checks!" );
}

fn demonstrate_failures()
{
  println!( "\n6. What error messages look like:" );
  println!( "   (Uncomment code in demonstrate_failures() to see actual diffs)" );
  
  // Uncomment these to see beautiful error diffs:
  
  // Different vectors:
  // let actual = vec![ 1, 2, 3 ];
  // let expected = vec![ 1, 2, 4 ];  
  // a_id!( actual, expected );
  
  // Different structs:
  // let user1 = User { name: "Alice".to_string(), age: 30, email: "alice@example.com".to_string(), active: true };
  // let user2 = User { name: "Alice".to_string(), age: 31, email: "alice@example.com".to_string(), active: true };
  // a_id!( user1, user2 );
  
  // Different strings:
  // let actual = "Hello, World!";
  // let expected = "Hello, Universe!";
  // a_id!( actual, expected );
  
  println!( "   💡 Uncomment examples above to see colorful diff output!" );
}
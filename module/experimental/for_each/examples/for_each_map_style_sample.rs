//! Demonstrates map-style invocation with prefix and postfix token injection.
//!
//! This example shows the powerful map-style syntax using `@Prefix`, `@Postfix`,
//! and `@Each` keywords to inject tokens before and after each element.
//!
//! The macro automatically unwraps braces and composes the final expression for
//! each element, enabling complex token tree transformations.

use for_each::for_each;

fn main()
{
  // Apply dbg! with prefix and postfix tokens to each element
  for_each!
  {
    dbg where
    @Prefix { "prefix".to_string() + }
    @Postfix { + "postfix" }
    @Each "a" "b" "c"
  };

  // The above macro invocation generates the equivalent of:
  // dbg!( "prefix".to_string() + "a" + "postfix" );
  // dbg!( "prefix".to_string() + "b" + "postfix" );
  // dbg!( "prefix".to_string() + "c" + "postfix" );
}
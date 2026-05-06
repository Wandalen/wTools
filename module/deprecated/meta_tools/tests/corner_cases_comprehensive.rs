//! Comprehensive corner case testing for `meta_tools` macros.
//! Tests edge cases, macro combinations, and unusual but valid usage patterns.
//!
//! ## Known Limitations (Not Tested)
//!
//! - **Complex expressions**: `for_each!` doesnt support expressions with operators or method calls
//!   (e.g., `1 + 1`, `"hello".len()`) because macro parser stops at operator/method tokens
//! - **Empty `for_each`**: Requires at least one element
//! - **Nested macros**: Macro nesting is not supported

use meta_tools as the_module;
#[ allow( unused_imports ) ]
use the_module :: *;

#[ test ]
fn corner_case_single_element()
{
  // Corner Case: for_each with single element
  for_each!( stringify, "single" );
}

#[ test ]
fn corner_case_unicode_content()
{
  // Corner Case: for_each with unicode content
  for_each!( stringify, "こんにちは", "🦀", "Здравствуй" );
}

#[ test ]
fn corner_case_mixed_literals()
{
  // Corner Case: for_each with numeric and boolean literals
  for_each!( stringify, 42, true, 3.14, 'x' );
}

#[ test ]
#[ allow( unused_variables ) ]
fn corner_case_variable_references()
{
  // Corner Case: for_each with variable references
  let x = "var_x";
  let y = "var_y";
  for_each!( stringify, x, y );
}

#[ test ]
#[ allow( unused_variables ) ]
fn corner_case_macro_hygiene_shadowing()
{
  // Corner Case: Macro hygiene - variable shadowing
  {
    let value = "outer";
    for_each!( stringify, value );
    {
      let value = "inner";
      for_each!( stringify, value );
    }
    for_each!( stringify, value );
  }
}

#[ test ]
fn corner_case_large_element_count()
{
  // Corner Case: for_each with many elements (20 items stress test)
  for_each!(
    stringify,
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    11, 12, 13, 14, 15, 16, 17, 18, 19, 20
  );
}

#[ test ]
fn corner_case_macro_invocation_variations()
{
  // Corner Case: Different macro invocation styles
  for_each!( stringify, "test1" ); // Minimal spacing
  for_each!( stringify,  "test2"  ); // Extra spaces
  for_each!(
    stringify,
    "test3"
  ); // Newlines
}

#[ cfg( feature = "meta_idents_concat" ) ]
#[ test ]
fn corner_case_paste_basic()
{
  // Corner Case: meta_idents_concat basic usage
  use ::paste ::paste;

  paste! {
    let [<test_ value>] = 42;
    assert_eq!( test_value, 42 );
  }
}

#[ cfg( all( feature = "meta_for_each", feature = "meta_idents_concat" ) ) ]
#[ test ]
fn corner_case_multiple_macros_same_scope()
{
  // Corner Case: Using multiple different macros in same scope
  use ::paste ::paste;

  for_each!( stringify, "a", "b" );

  paste! {
    let [<combined_ name>] = "value";
    assert_eq!( combined_name, "value" );
  }

  for_each!( stringify, "c", "d" );
}

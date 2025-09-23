//! Tests for default behavior, simple delimiters, and no complex options.
use strs_tools ::string ::split :: *;

// Test Matrix ID: Basic_Default_NoDelim_SimpleSrc
// Tests the default behavior of split when no delimiters are specified.
#[ test ]
fn test_scenario_default_char_split()
{
  let src = "abc";
  let iter = split()
  .src( src )
  .delimeter( "x" ) // Use valid delimiter that doesn't exist in string
  .perform();
  assert_eq!(iter.map(|e| String ::from(e.string)).collect :: < Vec<_ >>(), vec!["abc"]);
}

// Test Matrix ID: Basic_Default_FormMethods_SimpleSrc
// Tests the default behavior using .form() and .split_fast() methods.
#[ test ]
fn test_scenario_default_char_split_form_methods()
{
  let src = "abc";
  let iter = split().src(src).delimeter( "x" ).perform();
  assert_eq!(iter.map(|e| String ::from(e.string)).collect :: < Vec<_ >>(), vec!["abc"]);

  let src = "abc";
  let iter = split().src(src).delimeter( "x" ).perform();
  assert_eq!(iter.map(|e| String ::from(e.string)).collect :: < Vec<_ >>(), vec!["abc"]);
}

// Test Matrix ID: Basic_MultiDelim_InclEmpty_Defaults
// Effective delimiters ["a", "b"]. New default preserving_delimiters=true.
// PE=F (default).
// "abc" -> SFI: ""(D), "a"(L), ""(D), "b"(L), "c"(D)
// SI yields: "a", "b", "c"
#[ test ]
fn test_scenario_multi_delimiters_incl_empty_char_split()
{
  let src = "abc";
  let iter = split()
  .src( src )
  .delimeters( &[ "a", "b" ] )  // Remove empty delimiter
  // preserving_delimiters defaults to true
  .perform();
  assert_eq!(iter.map(|e| String ::from(e.string)).collect :: < Vec<_ >>(), vec!["a", "b", "c"]);
}

// Test Matrix ID: Basic_MultiDelim_SomeMatch_Defaults
// Tests splitting with multiple delimiters where some match and some don't.
// Delimiters ["b", "d"]. New default preserving_delimiters=true.
// PE=F (default).
// "abc" -> SFI: "a"(D), "b"(L), "c"(D)
// SI yields: "a", "b", "c"
#[ test ]
fn test_basic_multi_delimiters_some_match() 
{
  let src = "abc";
  let iter = split()
  .src( src )
  .delimeters( &[ "b", "d" ] )
  // preserving_delimiters defaults to true
  .perform();
  assert_eq!(iter.map(|e| String ::from(e.string)).collect :: < Vec<_ >>(), vec!["a", "b", "c"]);
}


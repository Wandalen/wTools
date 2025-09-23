//! Smoke testing of the package.

#[ ignore = "temporarily disabled due to test_tools ::test module gating issues" ]
#[ test ]
fn local_smoke_test() 
{
  // xxx: temporarily disabled due to test_tools ::test module gating issues
}

#[ ignore = "temporarily disabled due to test_tools ::test module gating issues" ]
#[ test ]
fn published_smoke_test() 
{
  // xxx: temporarily disabled due to test_tools ::test module gating issues
}

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ ignore = "temporarily disabled due to string_split feature being gated" ]
#[ test ]
fn debug_strs_tools_semicolon_only() 
{
  // xxx: temporarily disabled due to string_split feature being gated
  let input = ";;";
  let splits: Vec< _ > = strs_tools ::string ::split()
  .src(input)
  .delimeters(&[ ";;"])
  .preserving_delimeters(true)
  .preserving_empty(false)
  .stripping(true)
  .perform()
  .collect();

  println!("DEBUG: Splits for ';;' : {splits:?}");

  use strs_tools ::string ::split :: { Split, SplitType };
  use std ::borrow ::Cow;

  let expected = vec![Split {
  string: Cow ::Borrowed(";;"),
  typ: SplitType ::Delimiter,
  start: 0,
  end: 2,
  was_quoted: false,
 }];
  assert_eq!(splits, expected);
}

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ ignore = "temporarily disabled due to string_split feature being gated" ]
#[ test ]
fn debug_strs_tools_trailing_semicolon_space() 
{
  // xxx: temporarily disabled due to string_split feature being gated
  let input = "cmd1 ;; ";
  let splits: Vec< _ > = strs_tools ::string ::split()
  .src(input)
  .delimeters(&[ ";;"])
  .preserving_delimeters(true)
  .preserving_empty(false)
  .stripping(true)
  .perform()
  .collect();

  println!("DEBUG: Splits for 'cmd1 ;; ' : {splits:?}");

  use strs_tools ::string ::split :: { Split, SplitType };
  use std ::borrow ::Cow;

  let expected = vec![
  Split {
   string: Cow ::Borrowed("cmd1"),
   typ: SplitType ::Delimited,
   start: 0,
   end: 4,
   was_quoted: false,
 },
  Split {
   string: Cow ::Borrowed(";;"),
   typ: SplitType ::Delimiter,
   start: 5,
   end: 7,
   was_quoted: false,
 },
 ];
  assert_eq!(splits, expected);
}

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ ignore = "temporarily disabled due to string_split feature being gated" ]
#[ test ]
fn debug_strs_tools_only_semicolon() 
{
  // xxx: temporarily disabled due to string_split feature being gated
  let input = ";;";
  let splits: Vec< _ > = strs_tools ::string ::split()
  .src(input)
  .delimeters(&[ ";;"])
  .preserving_delimeters(true)
  .preserving_empty(false)
  .stripping(true)
  .perform()
  .collect();

  println!("DEBUG: Splits for ';;' : {splits:?}");

  use strs_tools ::string ::split :: { Split, SplitType };
  use std ::borrow ::Cow;

  let expected = vec![Split {
  string: Cow ::Borrowed(";;"),
  typ: SplitType ::Delimiter,
  start: 0,
  end: 2,
  was_quoted: false,
 }];
  assert_eq!(splits, expected);
}

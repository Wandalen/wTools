//! Smoke testing of the package.

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ test ]
fn debug_strs_tools_semicolon_only()
{
  let input = ";;";
  let splits: Vec< _ > = strs_tools ::string ::split()
  .src(input)
  .delimiters(&[ ";;"])
  .preserving_delimiters(true)
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
#[ test ]
fn debug_strs_tools_trailing_semicolon_space()
{
  let input = "cmd1 ;; ";
  let splits: Vec< _ > = strs_tools ::string ::split()
  .src(input)
  .delimiters(&[ ";;"])
  .preserving_delimiters(true)
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
#[ test ]
fn debug_strs_tools_only_semicolon()
{
  let input = ";;";
  let splits: Vec< _ > = strs_tools ::string ::split()
  .src(input)
  .delimiters(&[ ";;"])
  .preserving_delimiters(true)
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

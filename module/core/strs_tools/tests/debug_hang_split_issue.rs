//! For debugging split issues that cause hangs.
// This file is for debugging purposes only and will be removed after the issue is resolved.

#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
#[ test ]
fn debug_hang_split_issue() 
{
  use strs_tools ::string ::split;

  let input = r#""value with \\"quotes\\" and \\\\slash\\\\""#; // The problematic quoted string
  let splitter = split()
  .src(input)
  .delimeter(" :: ")
  .quoting(true)
  .perform();

  println!("Input: {input:?}");
  for item in splitter 
  {
  println!("Split item: {item:?}");
 }
}

//! Minimal reproducible example for strs_tools unescaping bug.

use strs_tools::string::split::Split;

fn main()
{
  let input = r#"cmd key::"value with \"quotes\" and \\slash\\""#;
  let splits_iter = strs_tools::split()
      .src( input )
      .delimeter( vec![ " ", "::" ] )
      .preserving_delimeters( true )
      .quoting( true )
      .form()
      .split(); // Use the full iterator

  let splits: Vec< Split<'_> > = splits_iter.collect();
  println!( "{:#?}", splits );
}
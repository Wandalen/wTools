
use super::*;

//

#[ cfg( not( feature = "no_std" ) ) ]
#[ test ]
fn basic()
{
  use the_module::string::indentation;

  /* test.case( "basic" ) */
  {
    let src = "a\nbc";
    let exp = "---a\n---bc";
    let got = indentation( "---", src, "" );
    a_id!( got, exp );
  }

  /* test.case( "empty string" ) */
  {
    let src = "";
    let exp = "";
    let got = indentation( "---", src, "" );
    a_id!( got, exp );
  }

  /* test.case( "two strings" ) */
  {
    let src = "a\nb";
    let exp = "---a+++\n---b+++";
    let got = indentation( "---", src, "+++" );
    a_id!( got, exp );
  }

  /* test.case( "last empty" ) */
  {
    let src = "a\n";
    let exp = "---a+++\n---+++";
    let got = indentation( "---", src, "+++" );
    // println!( "got : '{}'", got );
    a_id!( got, exp );
  }

  /* test.case( "first empty" ) */
  {
    let src = "\nb";
    let exp = "---+++\n---b+++";
    let got = indentation( "---", src, "+++" );
    // println!( "got : '{}'", got );
    a_id!( got, exp );
  }

  /* test.case( "two empty string" ) */
  {
    let src = "\n";
    let exp = "---+++\n---+++";
    let got = indentation( "---", src, "+++" );
    // println!( "got : '{}'", got );
    a_id!( got, exp );
  }

}

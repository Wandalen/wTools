#[ allow( unused_imports ) ]
use super::*;

// xxx : qqq : make example from this test and add also into readme

#[ test ]
fn exmaple()
{

  use core::fmt;
  use format_tools::
  {
    WithDebug,
    WithDisplay,
    to_string_with_fallback,
  };

  struct Both;

  impl fmt::Debug for Both
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "This is debug" )
    }
  }

  impl fmt::Display for Both
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "This is display" )
    }
  }

  struct OnlyDebug;

  impl fmt::Debug for OnlyDebug
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "This is debug" )
    }
  }

  let src = Both;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, src );
  let exp = "This is display".to_string();
  assert_eq!( got, exp );

  let src = OnlyDebug;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, src );
  let exp = "This is debug".to_string();
  assert_eq!( got, exp );

}

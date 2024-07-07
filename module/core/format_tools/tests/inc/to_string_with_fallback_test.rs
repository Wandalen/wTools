#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  ToStringWithFallback,
  // ToStringWithFallbackParams,
  WithDebug,
  WithDisplay,
  // the_module::to_string_with_fallback::Ref,
  to_string_with_fallback,
};

use std::
{
  // fmt,
  // collections::HashMap,
  borrow::Cow,
};

//

#[ test ]
fn to_string_with_fallback_basic()
{

  // - the_module::to_string_with_fallback::Ref should implement copy

  fn f1( _src : the_module::to_string_with_fallback::Ref::< '_, Struct1, WithDisplay, WithDebug > )
  where
    for< 'a > the_module::to_string_with_fallback::Ref::< 'a, Struct1, WithDisplay, WithDebug > : Copy + Clone,
  {}

  struct Struct1;
  let src = Struct1;
  let ref1 = the_module::to_string_with_fallback::Ref::< '_, _, WithDisplay, WithDebug >::from( &src );
  let ref2 = ref1;
  f1( ref1 );
  f1( ref2 );

  // -

  let src = 13i32;
  let got = the_module::to_string_with_fallback::Ref::< '_, _, WithDisplay, WithDebug >::from( &src ).to_string_with_fallback();
  let exp = "13".to_string();
  a_id!( got, exp );

  let src = "abc".to_string();
  let got = the_module::to_string_with_fallback::Ref::< '_, _, WithDisplay, WithDebug >::from( &src ).to_string_with_fallback();
  let exp = "abc".to_string();
  a_id!( got, exp );

  // -

}

//

#[ test ]
fn to_string_with_fallback_variants()
{

  // - only display

  struct OnlyDisplay;
  impl fmt::Display for OnlyDisplay
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      write!( f, "This is display" )
    }
  }

  let src = OnlyDisplay;
  let got = the_module::to_string_with_fallback::Ref::< '_, _, WithDisplay, WithDebug >::from( &src ).to_string_with_fallback();
  let exp = "This is display".to_string();
  a_id!( got, exp );

  // - only debug

  struct OnlyDebug;

  impl fmt::Debug for OnlyDebug
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      write!( f, "This is debug" )
    }
  }

  let src = OnlyDebug;
  let _ref1 = the_module::to_string_with_fallback::Ref::< '_, _, WithDisplay, WithDebug >::from( &src );

  let src = OnlyDebug;
  let got = the_module::to_string_with_fallback::Ref::< '_, _, WithDisplay, WithDebug >::from( &src ).to_string_with_fallback();
  let exp = "This is debug".to_string();
  a_id!( got, exp );

  let src = OnlyDebug;
  let got = the_module::to_string_with_fallback::Ref::< '_, _, WithDebug, WithDisplay >::from( &src ).to_string_with_fallback();
  let exp = "This is debug".to_string();
  a_id!( got, exp );

  // - both debug and display

  struct Both;

  impl fmt::Debug for Both
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      write!( f, "This is debug" )
    }
  }

  impl fmt::Display for Both
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      write!( f, "This is display" )
    }
  }

  let src = Both;
  let got = the_module::to_string_with_fallback::Ref::< '_, _, WithDisplay, WithDebug >::from( &src ).to_string_with_fallback();
  let exp = "This is display".to_string();
  a_id!( got, exp );

  let src = Both;
  let got = the_module::to_string_with_fallback::Ref::< '_, _, WithDebug, WithDisplay >::from( &src ).to_string_with_fallback();
  let exp = "This is debug".to_string();
  a_id!( got, exp );

  // -

}

//

#[ test ]
fn to_string_with_fallback_macro()
{

  // - only debug

  struct OnlyDebug;

  impl fmt::Debug for OnlyDebug
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      write!( f, "This is debug" )
    }
  }

  let src = OnlyDebug;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  let exp = "This is debug".to_string();
  a_id!( got, exp );

  let src = OnlyDebug;
  let got = to_string_with_fallback!( WithDebug, WithDisplay, &src );
  let exp = "This is debug".to_string();
  a_id!( got, exp );

  // - both debug and display

  struct Both;

  impl fmt::Debug for Both
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      write!( f, "This is debug" )
    }
  }

  impl fmt::Display for Both
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      write!( f, "This is display" )
    }
  }

  let src = Both;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  let exp = "This is display".to_string();
  a_id!( got, exp );

  let src = Both;
  let got = to_string_with_fallback!( WithDebug, WithDisplay, &src );
  let exp = "This is debug".to_string();
  a_id!( got, exp );

}

//

#[ test ]
fn display_is_not_implemented()
{

  let src = vec![ 1, 2, 3 ];
  let got = the_module
  ::to_string_with_fallback
  ::Ref
  ::< '_, _, WithDisplay, WithDebug >
  ::from( &src )
  .to_string_with_fallback();
  let exp : Cow< '_, String > = Cow::Owned( "[1, 2, 3]".to_string() );
  a_id!( got, exp );

  let src = vec![ 1, 2, 3 ];
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  let exp : Cow< '_, String > = Cow::Owned( "[1, 2, 3]".to_string() );
  a_id!( got, exp );

}

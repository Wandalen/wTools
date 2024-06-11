#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  // ToStringWith,
  _ToStringWithFallback,
  ToStringWithFallbackParams,
  WithDebug,
  WithDisplay,
  ToStringWithFallbackRef,
  to_string_with_fallback,
};

//

#[ test ]
fn to_string_with_fallback_basic()
{

  // - ToStringWithFallbackRef should implement copy

  fn f1( _src : ToStringWithFallbackRef::< '_, Struct1, ToStringWithFallbackParams< WithDisplay, WithDebug > > )
  where
    for< 'a > ToStringWithFallbackRef::< 'a, Struct1, ToStringWithFallbackParams< WithDisplay, WithDebug > > : Copy + Clone,
  {}

  struct Struct1;
  let src = Struct1;
  let ref1 = ToStringWithFallbackRef::< '_, _, ToStringWithFallbackParams< WithDisplay, WithDebug > >::from( &src );
  let ref2 = ref1;
  f1( ref1 );
  f1( ref2 );

  // -

  let src = 13i32;
  let got = ( &ToStringWithFallbackRef::< '_, _, ToStringWithFallbackParams< WithDisplay, WithDebug > >::from( &src ) ).to_string_with_fallback();
  let exp = "13".to_string();
  a_id!( got, exp );

  let src = "abc".to_string();
  let got = ( &ToStringWithFallbackRef::< '_, _, ToStringWithFallbackParams< WithDisplay, WithDebug > >::from( &src ) ).to_string_with_fallback();
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
  let got = ( &ToStringWithFallbackRef::< '_, _, ToStringWithFallbackParams< WithDisplay, WithDebug > >::from( &src ) ).to_string_with_fallback();
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
  let _ref1 = ToStringWithFallbackRef::< '_, _, ToStringWithFallbackParams< WithDisplay, WithDebug > >::from( &src );
  let got = ( &ToStringWithFallbackRef::< '_, _, ToStringWithFallbackParams< WithDisplay, WithDebug > >::from( &src ) ).to_string_with_fallback();
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
  let got = ( &ToStringWithFallbackRef::< '_, _, ToStringWithFallbackParams< WithDisplay, WithDebug > >::from( &src ) ).to_string_with_fallback();
  let exp = "This is display".to_string();
  a_id!( got, exp );

  let src = Both;
  let got = ( &ToStringWithFallbackRef::< '_, _, ToStringWithFallbackParams< WithDebug, WithDisplay > >::from( &src ) ).to_string_with_fallback();
  let exp = "This is debug".to_string();
  a_id!( got, exp );

  // -

}

//

#[ test ]
fn to_string_with_fallback_macro()
{

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
  // let got = ( &ToStringWithFallbackRef::< '_, _, ToStringWithFallbackParams< WithDisplay, WithDebug > >::from( &src ) ).to_string_with_fallback();
  let got = to_string_with_fallback!( WithDisplay, WithDebug, src );
  let exp = "This is display".to_string();
  a_id!( got, exp );

  let src = Both;
  // let got = ( &ToStringWithFallbackRef::< '_, _, ToStringWithFallbackParams< WithDisplay, WithDebug > >::from( &src ) ).to_string_with_fallback();
  let got = to_string_with_fallback!( WithDebug, WithDisplay, src );
  let exp = "This is debug".to_string();
  a_id!( got, exp );

}

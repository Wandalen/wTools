#[ allow( unused_imports ) ]
use super::*;

use assistant::
{
  // Fields,
  // IteratorTrait,
  // MaybeAs,
  ToStringWith,
  ToStringWithFallback,
  WithDebug,
  WithDisplay,
  Ref,
};

// use std::
// {
//   // fmt,
//   // collections::HashMap,
//   // borrow::Cow,
// };

//

#[ test ]
fn to_string_with_test()
{

  // -

  let src = 13i32;
  let got = ToStringWith::< WithDebug >::to_string_with( &src );
  let exp = "13".to_string();
  a_id!( got, exp );

  let src = "abc".to_string();
  let got = ToStringWith::< WithDebug >::to_string_with( &src );
  let exp = "\"abc\"".to_string();
  a_id!( got, exp );

  // -

  let src = 13i32;
  let got = ToStringWith::< WithDisplay >::to_string_with( &src );
  let exp = "13".to_string();
  a_id!( got, exp );

  let src = "abc".to_string();
  let got = ToStringWith::< WithDisplay >::to_string_with( &src );
  let exp = "abc".to_string();
  a_id!( got, exp );

  // -

}

//

macro_rules! to_string_with_fallback
{
  () => { < < Self as GraphNodesNominalInterface >::NodeHandle as HasId >::Id };
}

#[ test ]
fn to_string_with_fallback_test()
{

  // -

//   let src = 13i32;
//   let got = ToStringWithFallback::< WithDisplay, WithDebug >::to_string_with_fallback( &Ref::from( &src ) );
//   // let got = ( &Ref::from( &src ) ).to_string_with_fallback();
//   let exp = "13".to_string();
//   a_id!( got, exp );
//
//   let src = "abc".to_string();
//   let got = ToStringWithFallback::< WithDisplay, WithDebug >::to_string_with_fallback( &Ref::from( &src ) );
//   let exp = "abc".to_string();
//   a_id!( got, exp );

  // - only display

//   struct OnlyDisplay;
//   impl fmt::Display for OnlyDisplay
//   {
//     fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
//     {
//       write!( f, "This is OnlyDisplay" )
//     }
//   }
//
//   let src = OnlyDisplay;
//   let got = ToStringWithFallback::< WithDisplay, WithDebug >::to_string_with_fallback( &( &src, ) );
//   let exp = "This is OnlyDisplay".to_string();
//   a_id!( got, exp );

  // - only debug

//   #[ derive( Clone, Copy ) ]
//   struct OnlyDebug;
//
//   impl fmt::Debug for OnlyDebug
//   {
//     fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
//     {
//       write!( f, "This is debug" )
//     }
//   }
//
//   // impl fmt::Display for OnlyDebug
//   // {
//   //   fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
//   //   {
//   //     write!( f, "This is display" )
//   //   }
//   // }
//
//   let src = OnlyDebug;
//   let got = ToStringWithFallback::< WithDisplay, WithDebug >::to_string_with_fallback( &Ref::from( &src ) );
//   let exp = "This is debug".to_string();
//   a_id!( got, exp );

  // -


}

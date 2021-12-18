
#[derive(  Debug, PartialEq  )]
pub struct Struct1< 'a >
{
  pub string_slice_1 : &'a str,
}

impl< 'a > Struct1< 'a >
{
  #[inline]
  pub fn former() -> Struct1Former< 'a >
  {
    Struct1Former
    {
      string_slice_1 : ::core::option::Option::None,
    }
  }
}

pub struct Struct1Former< 'a >
{
  string_slice_1 : ::core::option::Option< &'a str >,
}

impl< 'a > Struct1Former< 'a >
{
  #[inline]
  pub fn form( mut self ) -> Struct1< 'a >
  {
    let string_slice_1 = if self.string_slice_1.is_some()
    {
      self.string_slice_1.take().unwrap()
    }
    else
    {
      let val : &'a str = ::core::default::Default::default();
      val
    };
    Struct1 { string_slice_1 }
  }

  #[inline]
  pub fn string_slice_1< Src >( mut self, src : Src ) -> Self
  where
    Src : ::core::convert::Into< &'a str >,
  {
    debug_assert!( self.string_slice_1.is_none() );
    self.string_slice_1 = ::core::option::Option::Some( src.into() );
    self
  }
}

//

include!( "./string_slice_only_test.rs" );

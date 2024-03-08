
// -

// trait i32Get
// {
//   fn get( &self ) -> &i32;
// }
//
// impl i32Get for i32
// {
//   #[ inline( always ) ]
//   fn get( &self ) -> &i32
//   {
//     &self
//   }
// }
//
// // -
//
// trait StringGet
// {
//   fn get( &self ) -> &String;
// }
//
// impl StringGet for String
// {
//   #[ inline( always ) ]
//   fn get( &self ) -> &String
//   {
//     &self
//   }
// }
//
// // -
//
// trait f32Get
// {
//   fn get( &self ) -> &f32;
// }
//
// impl f32Get for f32
// {
//   #[ inline( always ) ]
//   fn get( &self ) -> &f32
//   {
//     &self
//   }
// }

///
/// Set value trait.
///

trait SetValue< T, IntoT >
where
  IntoT : Into< T >,
{
  fn set( &mut self, value : IntoT );
}

///
/// Options1
///

#[ derive( Debug, Default ) ]
struct Options1
{
  field1 : i32,
  field2 : String,
}

impl Into< i32 > for &Options1
{
  fn into( self ) -> i32
  {
    self.field1
  }
}

impl Into< String > for &Options1
{
  fn into( self ) -> String
  {
    self.field2.clone()
  }
}

impl< IntoT > SetValue< i32, IntoT > for Options1
where
  IntoT : Into< i32 >,
{
  #[ inline( always ) ]
  fn set( &mut self, value : IntoT )
  {
    self.field1 = value.into().clone();
  }
}

impl< IntoT > SetValue< String, IntoT > for Options1
where
  IntoT : Into< String >,
{
  #[ inline( always ) ]
  fn set( &mut self, value : IntoT )
  {
    self.field2 = value.into().clone();
  }
}

///
/// Options2
///

#[ derive( Debug, Default ) ]
struct Options2
{
  field1 : i32,
  field2 : String,
  field3 : f32,
}

impl< IntoT > SetValue< i32, IntoT > for Options2
where
  IntoT : Into< i32 >,
{
  #[ inline( always ) ]
  fn set( &mut self, value : IntoT )
  {
    self.field1 = value.into().clone();
  }
}

impl< IntoT > SetValue< String, IntoT > for Options2
where
  IntoT : Into< String >,
{
  #[ inline( always ) ]
  fn set( &mut self, value : IntoT )
  {
    self.field2 = value.into().clone();
  }
}

///
/// Set value, alternative implementation.
///

trait SetValue2
{
  // fn set_with_type< T >( &mut self, value : T )
  // where
  //   Self : SetValue< T >;
  fn set_with_type< T, IntoT >( &mut self, value : IntoT )
  where
    IntoT : Into< T >,
    Self : SetValue< T, IntoT >;
}

// impl SetValue2 for Options2
// {
//
//   // #[ inline( always ) ]
//   // fn set_with_type< T >( &mut self, value : T )
//   // where
//   //   Self : SetValue< T >,
//   // {
//   //   self.set( value );
//   //   // self.set( Into::< T >::into( value ) );
//   // }
//
//   #[ inline( always ) ]
//   fn set_with_type< T, IntoT >( &mut self, value : IntoT )
//   where
//     IntoT : Into< T >,
//     Self : SetValue< T, IntoT >,
//   {
//     self.set( value );
//     // self.set( Into::< T >::into( value ) );
//   }
//
// }

#[ test ]
fn main()
{

  let mut o1 = Options1::default();
  o1.set( 42 ); // Sets field1
  o1.set( "Hello, world!" ); // Sets field2
  println!( "field1: {}, field2: {}", o1.field1, o1.field2 );

  let mut o2 = Options2::default();
  // o2.set( Into::< i32 >::into( &o1 ) );
  // o2.set( Into::< String >::into( &o1 ) );
  // o2.set_with_type::< i32, _ >( &o1 );
  // o2.set_with_type::< String, _ >( &o1 );

}

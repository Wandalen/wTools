
///
/// Set value trait.
///

pub trait SetValue< T, IntoT >
where
  IntoT : Into< T >,
{
  fn set( &mut self, value : IntoT );
}

///
/// Options1
///

#[ derive( Debug, Default, PartialEq ) ]
pub struct Options1
{
  field1 : i32,
  field2 : String,
  field3 : f32,
}

impl From< &Options1 > for i32
{
  #[ inline( always ) ]
  fn from( src : &Options1 ) -> Self
  {
    src.field1.clone()
  }
}

impl From< &Options1 > for String
{
  #[ inline( always ) ]
  fn from( src : &Options1 ) -> Self
  {
    src.field2.clone()
  }
}

impl From< &Options1 > for f32
{
  #[ inline( always ) ]
  fn from( src : &Options1 ) -> Self
  {
    src.field3.clone()
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

impl< IntoT > SetValue< f32, IntoT > for Options1
where
  IntoT : Into< f32 >,
{
  #[ inline( always ) ]
  fn set( &mut self, value : IntoT )
  {
    self.field3 = value.into().clone();
  }
}

///
/// Options2
///

#[ derive( Debug, Default, PartialEq ) ]
pub struct Options2
{
  field1 : i32,
  field2 : String,
}

impl From< &Options2 > for i32
{
  #[ inline( always ) ]
  fn from( src : &Options2 ) -> Self
  {
    src.field1.clone()
  }
}

impl From< &Options2 > for String
{
  #[ inline( always ) ]
  fn from( src : &Options2 ) -> Self
  {
    src.field2.clone()
  }
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
/// Options2SetAll.
///

pub trait Options2SetAll< IntoT >
where
  IntoT : Into< i32 >,
  IntoT : Into< String >,
  IntoT : Clone,
{
  fn set_all( &mut self, value : IntoT );
}

impl< T, IntoT > Options2SetAll< IntoT > for T
where
  T : SetValue< i32, IntoT >,
  T : SetValue< String, IntoT >,
  IntoT : Into< i32 >,
  IntoT : Into< String >,
  IntoT : Clone,
{
  #[ inline( always ) ]
  fn set_all( &mut self, value : IntoT )
  {
    SetValue::< i32, _ >::set( self, value.clone() );
    SetValue::< String, _ >::set( self, value.clone() );
  }
}

// impl Into< Options2 > for &T
// where
//
// {
//   #[ inline( always ) ]
//   fn into( self ) -> String
//   {
//     self.field2.clone()
//   }
// }

///
/// Set with type.
///

pub trait SetWithType
{
  fn set_with_type< T, IntoT >( &mut self, value : IntoT )
  where
    IntoT : Into< T >,
    Self : SetValue< T, IntoT >;
}

impl SetWithType for Options2
{

  #[ inline( always ) ]
  fn set_with_type< T, IntoT >( &mut self, value : IntoT )
  where
    IntoT : Into< T >,
    Self : SetValue< T, IntoT >,
  {
    SetValue::< T, IntoT >::set( self, value );
    // self.set( value );
    // self.set( Into::< T >::into( value ) );
  }

}

#[ test ]
fn main()
{

  let mut o1 = Options1::default();
  o1.set( 42 );
  o1.set( "Hello, world!" );
  o1.set( 13.01 );
  println!( "field1: {}, field2: {}", o1.field1, o1.field2 );
  let exp = Options1 { field1 : 42, field2 : "Hello, world!".to_string(), field3 : 13.01 };
  assert_eq!( o1, exp );

  // set( Into::< i32 >::into( &o1 ) )

  let mut o1 = Options1::default();
  o1.set( 42 );
  o1.set( "Hello, world!" );
  o1.set( 13.01 );
  let mut o2 = Options2::default();
  o2.set( Into::< i32 >::into( &o1 ) );
  o2.set( Into::< String >::into( &o1 ) );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );

  // set_with_type

  let mut o1 = Options1::default();
  o1.set( 42 );
  o1.set( "Hello, world!" );
  o1.set( 13.01 );
  let mut o2 = Options2::default();
  o2.set_with_type::< i32, _ >( &o1 );
  o2.set_with_type::< String, _ >( &o1 );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );

  // o2.set_all( &o1 )

  let mut o1 = Options1::default();
  o1.set( 42 );
  o1.set( "Hello, world!" );
  o1.set( 13.01 );
  let mut o2 = Options2::default();
  o2.set_all( &o1 );
  let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
  assert_eq!( o2, exp );

  // o1.set_all( &o2 )

  let mut o2 = Options2::default();
  o2.set( 42 );
  o2.set( "Hello, world!" );
  let mut o1 = Options1::default();
  o1.set_all( &o2 );
  let exp = Options1 { field1 : 42, field2 : "Hello, world!".to_string(), field3 : 0.0 };
  assert_eq!( o1, exp );

//   // o2 : Options2 = o1.into()
//
//   let mut o1 = Options1::default();
//   o1.set( 42 );
//   o1.set( "Hello, world!" );
//   o1.set( 13.01 );
//   let o2 : Options2 = o1.into();
//   let exp = Options2 { field1 : 42, field2 : "Hello, world!".to_string() };
//   assert_eq!( o2, exp );

}


#[ allow( unused_imports ) ]
use super::*;
use the_module::prelude::*;

//

#[ clone_dyn ]
trait Trait1< T1 : ::core::fmt::Debug, T2 >
where
  T2 : ::core::fmt::Debug,
  Self : ::core::fmt::Debug,
{
  fn dbg( &self ) -> String
  {
    format!( "{:?}", self )
  }
}

//

#[ derive( Debug, Clone ) ]
struct Struct1< T1, T2 >
where
  T1 : ::core::fmt::Debug,
  T2 : ::core::fmt::Debug,
{
  a : T1,
  b : T2,
}

impl Trait1< i32, char > for Struct1< i32, char >
{
  fn dbg( &self ) -> String
  {
    format!( "{self:?}( {:?} {:?} )", self.a, self.b )
  }
}

//

impl Trait1< i32, char > for i32
{
  fn dbg( &self ) -> String
  {
    format!( "{:?}", self )
  }
}

impl Trait1< i32, char > for i64
{
  fn dbg( &self ) -> String
  {
    format!( "{:?}", self )
  }
}

impl Trait1< i32, char > for String
{
  fn dbg( &self ) -> String
  {
    format!( "{:?}", self )
  }
}

impl< T > Trait1< i32, char > for &[ T ]
where
  T : the_module::CloneDyn,
  Self : ::core::fmt::Debug,
{
  fn dbg( &self ) -> String
  {
    format!( "{:?}", self )
  }
}

impl Trait1< i32, char > for &str
{
  fn dbg( &self ) -> String
  {
    format!( "{:?}", self )
  }
}


#[ test ]
fn basic()
{

  //

  let e_i32 : Box< dyn Trait1< i32, char > > = Box::new( 13 );
  let e_i64 : Box< dyn Trait1< i32, char > > = Box::new( 14 );
  let e_string : Box< dyn Trait1< i32, char > > = Box::new( "abc".to_string() );
  let e_str_slice : Box< dyn Trait1< i32, char > > = Box::new( "abcd" );
  let e_slice : Box< dyn Trait1< i32, char > > = Box::new( &[ 1i32, 2i32 ] as &[ i32 ] );

  //

  let vec : Vec< Box< dyn Trait1< i32, char > > > = vec![ e_i32.clone(), e_i64.clone(), e_string.clone(), e_str_slice.clone(), e_slice.clone() ];
  let vec = vec.iter().map( | e | e.dbg() ).collect::< Vec< _ > >();
  let vec2 = vec!
  [
    "13".to_string(),
    "14".to_string(),
    "\"abc\"".to_string(),
    "\"abcd\"".to_string(),
    "[1, 2]".to_string(),
  ];
  a_id!( vec, vec2 );

  //

  let vec : Vec< Box< dyn Trait1< i32, char > > > = vec![ e_i32.clone(), e_i64.clone(), e_string.clone(), e_str_slice.clone(), e_slice.clone() ];
  let vec2 = the_module::clone( &vec );
  let vec = vec.iter().map( | e | e.dbg() ).collect::< Vec< _ > >();
  let vec2 = vec2.iter().map( | e | e.dbg() ).collect::< Vec< _ > >();
  a_id!( vec, vec2 );

  //

  let vec : Vec< Box< dyn Trait1< i32, char > > > = vec![ e_i32.clone(), e_i64.clone(), e_string.clone(), e_str_slice.clone(), e_slice.clone() ];
  let vec2 = vec.clone();
  let vec = vec.iter().map( | e | e.dbg() ).collect::< Vec< _ > >();
  let vec2 = vec2.iter().map( | e | e.dbg() ).collect::< Vec< _ > >();
  a_id!( vec, vec2 );

  //

}

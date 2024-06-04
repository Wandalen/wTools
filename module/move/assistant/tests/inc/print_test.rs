#[ allow( unused_imports ) ]
use super::*;

use assistant::
{
  Fields,
  IteratorTrait,
};

use std::
{
  collections::HashMap,
};

/// Struct representing a test object with various fields.
pub struct TestObject
{
  pub id : String,
  pub created_at : i64,
  pub file_ids : Vec< String >,
  pub tools : Option< Vec< HashMap< String, String > > >,
}

impl Fields< &'static str, String > for TestObject
{
  fn fields( &self ) -> impl IteratorTrait< Item = ( &'static str, String ) >
  {
    let mut vec : Vec< ( &'static str, String ) > = Vec::new();

    vec.push( ( "id", self.id.clone() ) );
    vec.push( ( "created_at", self.created_at.to_string() ) );
    vec.push( ( "file_ids", format!( "{:?}", self.file_ids ) ) );

    if let Some( tools ) = &self.tools
    {
      vec.push( ( "tools", format!( "{:?}", tools ) ) );
    }

    vec.into_iter()
  }
}

// impl< 'a > Fields< usize, &'a TestObject > for Vec< TestObject >
// {
//   fn fields( &'a self ) -> impl IteratorTrait< Item = ( usize, &'a TestObject ) >
//   {
//     self.iter().enumerate()
//   }
// }

#[ test ]
fn basic()
{


}
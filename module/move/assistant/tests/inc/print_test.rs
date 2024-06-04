#[ allow( unused_imports ) ]
use super::*;

use assistant::
{
  Fields,
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
  fn fields( &self ) -> impl Iterator< Item = ( &'static str, String ) > + ExactSizeIterator + Clone
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

#[ test ]
fn basic()
{


}
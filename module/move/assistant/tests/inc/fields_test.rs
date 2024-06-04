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
  fn fields( &self ) -> impl Iterator< Item = ( &'static str, String ) > + Clone
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
  let test_object = TestObject
  {
    id : "12345".to_string(),
    created_at : 1627845583,
    file_ids : vec![ "file1".to_string(), "file2".to_string() ],
    tools : Some
    (
      vec!
      [{
        let mut map = HashMap::new();
        map.insert( "tool1".to_string(), "value1".to_string() );
        map.insert( "tool2".to_string(), "value2".to_string() );
        map
      }]
    ),
  };

  let fields: Vec<( &'static str, String )> = test_object.fields().collect();

  assert_eq!( fields.len(), 4 );
  assert_eq!( fields[ 0 ], ( "id", "12345".to_string() ) );
  assert_eq!( fields[ 1 ], ( "created_at", "1627845583".to_string() ) );
  assert_eq!( fields[ 2 ], ( "file_ids", "[\"file1\", \"file2\"]".to_string() ) );
  assert_eq!( fields[ 3 ].0, "tools" );
}
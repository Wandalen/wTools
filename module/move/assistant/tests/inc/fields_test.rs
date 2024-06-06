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
  borrow::Cow,
};

/// Struct representing a test object with various fields.
#[ derive( Clone, Debug ) ]
pub struct TestObject
{
  pub id : String,
  pub created_at : i64,
  pub file_ids : Vec< String >,
  pub tools : Option< Vec< HashMap< String, String > > >,
}

impl< 'a > Fields< 'a, &'static str, String > for TestObject
{
  fn fields( &'a self ) -> impl IteratorTrait< Item = ( &'static str, Option< Cow< 'a, String > > ) >
  {
    let mut vec : Vec< ( &'static str, Option< Cow< 'a, String > > ) > = Vec::new();

    vec.push( ( "id", Some( Cow::Borrowed( &self.id ) ) ) ) );
    vec.push( ( "created_at", Some( Cow::Owned( self.created_at.to_string() ) ) ) );
    vec.push( ( "file_ids", Some( Cow::Owned( format!( "{:?}", self.file_ids ) ) ) ) );

    if let Some( tools ) = &self.tools
    {
      vec.push( ( "tools", Some( Cow::Owned( format!( "{:?}", tools ) ) ) ) );
    }
    else
    {
      vec.push( None )
    }

    vec.into_iter()
  }
}

impl< 'a > Fields< 'a, usize, TestObject > for Vec< TestObject >
{
  fn fields( &'a self ) -> impl IteratorTrait< Item = ( usize, Cow< 'a, TestObject > ) >
  {
    self.iter().enumerate().map( | ( key, val ) | ( key, Some( Cow::Borrowed( val ) ) ) )
  }
}

//

fn is_borrowed< 'a, T : Clone >( src : &Cow< 'a, T > ) -> bool
{
  match src
  {
    Cow::Borrowed( _ ) => true,
    Cow::Owned( _ ) => false,
  }
}

//

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

  let fields: Vec< _ > = test_object.fields().collect();

  assert_eq!( fields.len(), 4 );
  assert!( is_borrowed( &fields[ 0 ].1 ) );
  assert!( !is_borrowed( &fields[ 1 ].1 ) );
  assert!( !is_borrowed( &fields[ 2 ].1 ) );
  assert!( !is_borrowed( &fields[ 3 ].1 ) );
  assert_eq!( fields[ 0 ], ( "id", Cow::Borrowed( &"12345".to_string() ) ) );
  assert_eq!( fields[ 1 ], ( "created_at", Cow::Owned( "1627845583".to_string() ) ) );
  assert_eq!( fields[ 2 ], ( "file_ids", Cow::Owned( "[\"file1\", \"file2\"]".to_string() ) ) );
  assert_eq!( fields[ 3 ].0, "tools" );

}

//

#[ test ]
fn test_vec_fields()
{
  let test_objects = vec!
  [
    TestObject
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
    },
    TestObject
    {
      id : "67890".to_string(),
      created_at : 13,
      file_ids : vec![ "file3".to_string(), "file4".to_string() ],
      tools : None,
    }
  ];

  let fields: Vec< _ > = test_objects.fields().collect();
  assert_eq!( fields.len(), 2 );
  assert_eq!( fields[ 0 ].0, 0 );
  assert_eq!( fields[ 1 ].0, 1 );
}

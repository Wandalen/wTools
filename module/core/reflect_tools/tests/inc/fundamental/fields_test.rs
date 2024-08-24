#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  Fields,
  IteratorTrait,
  OptionalCow,
  // ToStringWith,
  // WithDebug,
};

// xxx2 : check

use std::
{
  // fmt,
  // collections::HashMap,
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

impl Fields< &'static str, OptionalCow< '_, String, () > >
for TestObject
{
  type Key< 'k > = &'static str;
  type Val< 'v > = OptionalCow< 'v, String, () >;

  fn fields( &self ) -> impl IteratorTrait< Item = ( &'static str, OptionalCow< '_, String, () > ) >
  {
    let mut dst : Vec< ( &'static str, OptionalCow< '_, String, () > ) > = Vec::new();

    dst.push( ( "id", Some( Cow::Borrowed( &self.id ) ).into() ) );
    dst.push( ( "created_at", Some( Cow::Owned( self.created_at.to_string() ) ).into() ) );
    dst.push( ( "file_ids", Some( Cow::Owned( format!( "{:?}", self.file_ids ) ) ).into() ) );

    if let Some( tools ) = &self.tools
    {
      dst.push( ( "tools", Some( Cow::Owned( format!( "{:?}", tools ) ) ).into() ) );
    }
    else
    {
      dst.push( ( "tools", None.into() ) );
    }

    dst.into_iter()
  }
}

//

// #[ allow( dead_code ) ]
// fn is_borrowed< 'a, T : Clone >( src : &Option< Cow< 'a, T > > ) -> bool
// {
//   if src.is_none()
//   {
//     return false;
//   }
//   match src.as_ref().unwrap()
//   {
//     Cow::Borrowed( _ ) => true,
//     Cow::Owned( _ ) => false,
//   }
// }

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

  let fields : Vec< ( &str, OptionalCow< '_, String, () > ) > = test_object.fields().collect();

  assert_eq!( fields.len(), 4 );
  assert!( fields[ 0 ].1.is_borrowed() );
  assert!( !fields[ 1 ].1.is_borrowed() );
  assert!( !fields[ 2 ].1.is_borrowed() );
  assert!( !fields[ 3 ].1.is_borrowed() );
  assert_eq!( fields[ 0 ], ( "id", Some( Cow::Borrowed( &"12345".to_string() ) ).into() ) );
  assert_eq!( fields[ 1 ], ( "created_at", Some( Cow::Owned( "1627845583".to_string() ) ).into() ) );
  assert_eq!( fields[ 2 ], ( "file_ids", Some( Cow::Owned( "[\"file1\", \"file2\"]".to_string() ) ).into() ) );
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
    },
  ];

  let fields : Vec< _ > = Fields::< usize, Option< _ > >::fields( &test_objects ).collect();
  assert_eq!( fields.len(), 2 );
  assert_eq!( fields[ 0 ].0, 0 );
  assert_eq!( fields[ 1 ].0, 1 );

}

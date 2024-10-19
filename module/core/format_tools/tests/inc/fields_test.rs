#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  Fields,
  IteratorTrait,
  OptionalCow,
  WithRef,
};

use std::
{
  // fmt,
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

impl Fields< &'_ str, Option< Cow< '_, str > > >
for TestObject
{
  type Key< 'k > = &'k str;
  type Val< 'v > = Option< Cow< 'v, str > >;

  fn fields( &self ) -> impl IteratorTrait< Item = ( &'_ str, Option< Cow< '_, str > > ) >
  {
    use format_tools::ref_or_display_or_debug::field;
    let mut dst : Vec< ( &'_ str, Option< Cow< '_, str > > ) > = Vec::new();

    dst.push( field!( &self.id ) );
    dst.push( field!( &self.created_at ) );
    dst.push( field!( &self.file_ids ) );

    if let Some( tools ) = &self.tools
    {
      dst.push( field!( tools ) );
    }
    else
    {
      dst.push( ( "tools", Option::None ) );
    }

    dst.into_iter()
  }

}

pub fn is_borrowed( cow : &Option< Cow< '_, str > > ) -> bool
{
  matches!( cow, Some( Cow::Borrowed( _ ) ) )
}

//

#[ test ]
fn basic_with_ref_display_debug()
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

  let fields : Vec< ( &str, Option< Cow< '_, str > > ) > =
  Fields::< &'static str, Option< Cow< '_, str > > >::fields( &test_object ).collect();

  // let fields : Vec< ( &str, Option< Cow< '_, str > > ) > = test_object.fields().collect();

  assert_eq!( fields.len(), 4 );
  assert!( is_borrowed( &fields[ 0 ].1 ) );
  assert!( !is_borrowed( &fields[ 1 ].1 ) );
  assert!( !is_borrowed( &fields[ 2 ].1 ) );
  assert!( !is_borrowed( &fields[ 3 ].1 ) );
  assert_eq!( fields[ 0 ], ( "id", Some( Cow::Borrowed( "12345" ) ).into() ) );
  assert_eq!( fields[ 0 ], ( "id", Some( Cow::Owned( "12345".to_string() ) ).into() ) );
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

  // let fields : Vec< _ > = test_objects.fields().collect();
  let fields : Vec< _ > = Fields::< usize, Option< _ > >::fields( &test_objects ).collect();
  assert_eq!( fields.len(), 2 );
  assert_eq!( fields[ 0 ].0, 0 );
  assert_eq!( fields[ 1 ].0, 1 );

}

#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  Fields,
  IteratorTrait,
  MaybeAs,
  ToStringWith,
  WithDebug,
  WithDisplay,
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

impl< 'a, How > Fields< 'a, &'static str, MaybeAs< 'a, str, How > >
for TestObject
where
  // MaybeAs< 'a, str, How > : Clone, // xxx
  How : Clone + Copy + 'static,
  String : ToStringWith< 'a, How >,
  i64 : ToStringWith< 'a, How >,
  Vec< String > : ToStringWith< 'a, How >,
  Vec< HashMap< String, String > > : ToStringWith< 'a, How >,
{
  fn fields( &'a self ) -> impl IteratorTrait< Item = ( &'static str, MaybeAs< 'a, str, How > ) >
  {
    let mut dst : Vec< ( &'static str, MaybeAs< 'a, str, How > ) > = Vec::new();

    fn from< 'a, V, How >( src : &'a V ) -> MaybeAs< 'a, str, How >
    where
      How : Clone + Copy + 'static,
      V : ToStringWith< 'a, How > + 'a,
    {
      MaybeAs::< 'a, str, How >::from
      (
        < V as ToStringWith< 'a, How > >::to_string_with( src )
      )
    }

    fn add< 'a, V, How >
    (
      dst : &mut Vec< ( &'static str, MaybeAs< 'a, str, How > ) >,
      key : &'static str,
      src : &'a V
    )
    where
      How : Clone + Copy + 'static,
      V : ToStringWith< 'a, How > + 'a,
    {
      let val = from( src );
      dst.push( ( key, val ) );
    }

    dst.push( ( "id", from( &self.id ) ) );
    // add( &mut dst, "id", &self.id );
    // dst.push( ( "id", MaybeAs::< 'a, String, How >::from( &self.id ) ) );
    add( &mut dst, "created_at", &self.created_at );
    add( &mut dst, "file_ids", &self.file_ids );

    if let Some( tools ) = &self.tools
    {
      add( &mut dst, "tools", tools );
    }
    else
    {
      dst.push( ( "tools", MaybeAs::none() ) );
    }

    dst.into_iter()
  }
}

//

#[ allow( dead_code ) ]
fn is_borrowed< 'a, T : Clone >( src : &Option< Cow< 'a, T > > ) -> bool
{
  if src.is_none()
  {
    return false;
  }
  match src.as_ref().unwrap()
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

  let fields : Vec< ( &str, MaybeAs< '_, str, WithDebug > ) > = test_object.fields().collect();

  assert_eq!( fields.len(), 4 );
  // assert!( is_borrowed( &fields[ 0 ].1 ) );
  // assert!( !is_borrowed( &fields[ 1 ].1 ) );
  // assert!( !is_borrowed( &fields[ 2 ].1 ) );
  // assert!( !is_borrowed( &fields[ 3 ].1 ) );
  // xxx
  assert_eq!( fields[ 0 ], ( "id", Some( Cow::Borrowed( &"\"12345\"" ) ).into() ) );
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

  let fields : Vec< _ > = test_objects.fields().collect();
  assert_eq!( fields.len(), 2 );
  assert_eq!( fields[ 0 ].0, 0 );
  assert_eq!( fields[ 1 ].0, 1 );

}

// xxx : fix the test
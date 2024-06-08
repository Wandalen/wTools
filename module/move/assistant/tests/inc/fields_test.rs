#[ allow( unused_imports ) ]
use super::*;

use assistant::
{
  Fields,
  IteratorTrait,
  MaybeAs,
};

use std::
{
  fmt,
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

// impl< 'a > Fields< 'a, &'static str, Option< Cow< 'a, String > > >
// for TestObject
// {
//   fn fields( &'a self ) -> impl IteratorTrait< Item = ( &'static str, Option< Cow< 'a, String > > ) >
//   {
//     let mut vec : Vec< ( &'static str, Option< Cow< 'a, String > > ) > = Vec::new();
//
//     vec.push( ( "id", Some( Cow::Borrowed( &self.id ) ) ) );
//     vec.push( ( "created_at", Some( Cow::Owned( self.created_at.to_string() ) ) ) );
//     vec.push( ( "file_ids", Some( Cow::Owned( format!( "{:?}", self.file_ids ) ) ) ) );
//
//     if let Some( tools ) = &self.tools
//     {
//       vec.push( ( "tools", Some( Cow::Owned( format!( "{:?}", tools ) ) ) ) );
//     }
//     else
//     {
//       vec.push( ( "tools", None ) );
//     }
//
//     vec.into_iter()
//   }
// }

// =

#[ derive( Debug, Default, Clone, Copy ) ]
pub struct WithDebug;

#[ derive( Debug, Default, Clone, Copy ) ]
pub struct WithDisplay;

#[ derive( Debug, Default, Clone, Copy ) ]
pub struct WithWell;

pub trait ToStringWith< 'a, How >
{
  // fn to_string_with( &'a self ) -> MaybeAs< 'a, String, StringFromDebug >;
  fn to_string_with( &'a self ) -> String;
}

impl< 'a, T > ToStringWith< 'a, WithDebug > for T
where
  T : fmt::Debug,
{
  // fn to_string_with( &'a self ) -> MaybeAs< 'a, String, StringFromDebug >
  fn to_string_with( &'a self ) -> String
  {
    format!( "{:?}", self )
    // MaybeAs::from( format!( "{:?}", self ) )
  }
}

// impl< 'a > Fields< 'a, &'static str, MaybeAs< 'a, String, StringFromDebug > >
impl< 'a, V > Fields< 'a, &'static str, MaybeAs< 'a, V, WithDebug > >
for TestObject
where
  V : ToStringWith< 'a, WithDebug > + Clone + 'a,
  // MaybeAs< 'a, V, WithDebug > : From< String >,
  String : Into< MaybeAs< 'a, V, WithDebug > >,
{
  // fn fields( &'a self ) -> impl IteratorTrait< Item = ( &'static str, MaybeAs< 'a, String, StringFromDebug > ) >
  fn fields( &'a self ) -> impl IteratorTrait< Item = ( &'static str, MaybeAs< 'a, V, WithDebug > ) >
  {
    // let mut vec : Vec< ( &'static str, MaybeAs< 'a, String, StringFromDebug > ) > = Vec::new();
    let mut vec : Vec< ( &'static str, MaybeAs< 'a, V, WithDebug > ) > = Vec::new();

    // vec.push( ( "id", MaybeAs::< 'a, V, WithDebug >::from( self.id.to_string_with() ) ) );
    vec.push( ( "id", self.id.to_string_with().into() ) );
    vec.push( ( "created_at", self.created_at.to_string_with().into() ) );
    vec.push( ( "file_ids", self.file_ids.to_string_with().into() ) );

    if let Some( tools ) = &self.tools
    {
      vec.push( ( "tools", self.tools.to_string_with().into() ) );
    }
    else
    {
      vec.push( ( "tools", MaybeAs::none() ) );
    }

    vec.into_iter()
  }
}

//

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

  let fields: Vec< _ > = test_object.fields().collect();

  assert_eq!( fields.len(), 4 );
  // assert!( is_borrowed( &fields[ 0 ].1 ) );
  // assert!( !is_borrowed( &fields[ 1 ].1 ) );
  // assert!( !is_borrowed( &fields[ 2 ].1 ) );
  // assert!( !is_borrowed( &fields[ 3 ].1 ) );
  // xxx
  // assert_eq!( fields[ 0 ], ( "id", Some( Cow::Borrowed( &"12345".to_string() ) ).into() ) );
  // assert_eq!( fields[ 1 ], ( "created_at", Some( Cow::Owned( "1627845583".to_string() ) ).into() ) );
  // assert_eq!( fields[ 2 ], ( "file_ids", Some( Cow::Owned( "[\"file1\", \"file2\"]".to_string() ) ).into() ) );
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

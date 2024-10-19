//! A strucutre for diagnostic and demonstration purpose.

// use super::*;

// use crate::
// {
//   Fields,
//   IteratorTrait,
//   TableWithFields,
//   WithRef,
//   OptionalCow,
// };

use std::
{
  collections::HashMap,
  hash::Hasher,
  hash::Hash,
  cmp::Ordering,
  // borrow::Cow,
};

/// Struct representing a test object with various fields.
#[ derive( Clone, Debug, PartialEq, Eq ) ]
pub struct TestObjectWithoutImpl
{
  pub id : String,
  pub created_at : i64,
  pub file_ids : Vec< String >,
  pub tools : Option< Vec< HashMap< String, String > > >,
}

// TableWithFields is not implemented for TestObjectWithoutImpl intentionally

// impl TableWithFields for TestObjectWithoutImpl {}
//
// impl Fields< &'_ str, Option< Cow< '_, str > > >
// for TestObjectWithoutImpl
// {
//   type Key< 'k > = &'k str;
//   type Val< 'v > = Option< Cow< 'v, str > >;
//
//   fn fields( &self ) -> impl IteratorTrait< Item = ( &'_ str, Option< Cow< '_, str > > ) >
//   {
//     use format_tools::ref_or_display_or_debug_multiline::field;
//     // use format_tools::ref_or_display_or_debug::field;
//     let mut dst : Vec< ( &'_ str, Option< Cow< '_, str > > ) > = Vec::new();
//
//     dst.push( field!( &self.id ) );
//     dst.push( field!( &self.created_at ) );
//     dst.push( field!( &self.file_ids ) );
//
//     if let Some( tools ) = &self.tools
//     {
//       dst.push( field!( tools ) );
//     }
//     else
//     {
//       dst.push( ( "tools", Option::None ) );
//     }
//
//     dst.into_iter()
//   }
//
// }

impl Hash for TestObjectWithoutImpl
{

  fn hash< H: Hasher >( &self, state: &mut H )
  {
    self.id.hash( state );
    self.created_at.hash( state );
    self.file_ids.hash( state );

    if let Some( tools ) = &self.tools
    {
      for tool in tools
      {
        for ( key, value ) in tool
        {
          key.hash( state );
          value.hash( state );
        }
      }
    }
    else
    {
      state.write_u8( 0 );
    }
  }

}

impl PartialOrd for TestObjectWithoutImpl
{

  fn partial_cmp( &self, other: &Self ) -> Option< Ordering >
  {
    Some( self.cmp( other ) )
  }

}

impl Ord for TestObjectWithoutImpl
{

  fn cmp( &self, other: &Self ) -> Ordering
  {
    self.id
    .cmp( &other.id )
    .then_with( | | self.created_at.cmp( &other.created_at ) )
    .then_with( | | self.file_ids.cmp( &other.file_ids ) )
  }

}

/// Generate a dynamic array of test objects.
pub fn test_objects_gen() -> Vec< TestObjectWithoutImpl >
{

  vec!
  [
    TestObjectWithoutImpl
    {
      id : "1".to_string(),
      created_at : 1627845583,
      file_ids : vec![ "file1".to_string(), "file2".to_string() ],
      tools : None
    },
    TestObjectWithoutImpl
    {
      id : "2".to_string(),
      created_at : 13,
      file_ids : vec![ "file3".to_string(), "file4\nmore details".to_string() ],
      tools : Some
      (
        vec!
        [
          {
            let mut map = HashMap::new();
            map.insert( "tool1".to_string(), "value1".to_string() );
            map
          },
          {
            let mut map = HashMap::new();
            map.insert( "tool2".to_string(), "value2".to_string() );
            map
          }
        ]
      ),
    },
  ]

}

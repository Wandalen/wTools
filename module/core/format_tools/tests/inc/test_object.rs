#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  Fields,
  IteratorTrait,
  TableWithFields,
  WithRef,
  OptionalCow,
};

use std::
{
  collections::HashMap,
  // borrow::Cow,
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

impl TableWithFields for TestObject {}

impl Fields< &'_ str, OptionalCow< '_, str, WithRef > >
for TestObject
{
  type Key< 'k > = &'k str;
  type Val< 'v > = OptionalCow< 'v, str, WithRef >;

  fn fields( &self ) -> impl IteratorTrait< Item = ( &'_ str, OptionalCow< '_, str, WithRef > ) >
  {
    use format_tools::ref_or_display_or_debug_multiline::field;
    // use format_tools::ref_or_display_or_debug::field;
    let mut dst : Vec< ( &'_ str, OptionalCow< '_, str, WithRef > ) > = Vec::new();

    dst.push( field!( &self.id ) );
    dst.push( field!( &self.created_at ) );
    dst.push( field!( &self.file_ids ) );

    if let Some( tools ) = &self.tools
    {
      dst.push( field!( tools ) );
    }
    else
    {
      dst.push( ( "tools", OptionalCow::none() ) );
    }

    dst.into_iter()
  }

}

//

pub fn test_objects_gen() -> Vec< TestObject >
{

  vec!
  [
    TestObject
    {
      id : "1".to_string(),
      created_at : 1627845583,
      file_ids : vec![ "file1".to_string(), "file2".to_string() ],
      tools : None
    },
    TestObject
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

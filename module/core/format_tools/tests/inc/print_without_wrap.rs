#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  // print,
  Fields,
  IteratorTrait,
  AsTable,
  IntoAsTable,
  Cells,
  TableSize,
  TableRows,
  TableHeader,
  Context,
  WithRef,
  MaybeAs,
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

impl Fields< &'_ str, MaybeAs< '_, str, WithRef > >
for TestObject
{
  type Key< 'k > = &'k str;
  type Val< 'v > = MaybeAs< 'v, str, WithRef >;

  fn fields( &self ) -> impl IteratorTrait< Item = ( &'_ str, MaybeAs< '_, str, WithRef > ) >
  {
    use format_tools::ref_or_display_or_debug_multiline::field;
    // use format_tools::ref_or_display_or_debug::field;
    let mut dst : Vec< ( &'_ str, MaybeAs< '_, str, WithRef > ) > = Vec::new();

    dst.push( field!( &self.id ) );
    dst.push( field!( &self.created_at ) );
    dst.push( field!( &self.file_ids ) );

    if let Some( tools ) = &self.tools
    {
      dst.push( field!( tools ) );
    }
    else
    {
      dst.push( ( "tools", MaybeAs::none() ) );
    }

    dst.into_iter()
  }

}

// xxx : finish or remove

// let as_table : AsTable< '_, Vec< TestObject >, usize, TestObject, str, WithRef > = AsTable::new( &test_objects );

// impl IntoAsTable
// for Vec< TestObject >
// {
//
//   type Table = Self;
//   type RowKey = usize;
//   type Row = TestObject;
//   type CellKey = str;
//   type CellRepr = WithRef;
//
//   fn as_table( &self ) -> AsTable< '_, Self::Table, Self::RowKey, Self::Row, Self::CellKey, Self::CellRepr >
//   {
//     *self
//   }
//
// }

//

fn test_objects_gen() -> Vec< TestObject >
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

//

#[ test ]
fn without_wrap()
{
  let test_objects = test_objects_gen();

  let mut output = String::new();
  let formatter = the_module::Styles::default();

  let as_table = AsTable::new( &test_objects );
  // let mut context : Context< '_, _ > = Context::new( &mut output, formatter );
  let mut context = Context::new( &mut output, formatter );
  let result = the_module::TableFormatter::fmt( &as_table, &mut context );
  assert!( result.is_ok() );

  let exp = r#"│ id │ created_at │          file_ids          │           tools            │
│ 1  │ 1627845583 │        [                   │                            │
│    │            │            "file1",        │                            │
│    │            │            "file2",        │                            │
│    │            │        ]                   │                            │
│ 2  │     13     │ [                          │ [                          │
│    │            │     "file3",               │     {                      │
│    │            │     "file4\nmore details", │         "tool1": "value1", │
│    │            │ ]                          │     },                     │
│    │            │                            │     {                      │
│    │            │                            │         "tool2": "value2", │
│    │            │                            │     },                     │
│    │            │                            │ ]                          │"#;

  a_id!( output.as_str(), exp );

}

//

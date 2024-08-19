#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  // print,
  Fields,
  IteratorTrait,
  AsTable,
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
fn table_to_string()
// where
  // for< 'a > AsTable< 'a, Vec< TestObject >, usize, TestObject, &'static str, String, &'static str > : TableFormatter< 'a >,
{
  use the_module::TableToString;
  let test_objects = test_objects_gen();

  let cells = Cells::< str, WithRef >::cells( &test_objects[ 0 ] );
  assert_eq!( cells.len(), 4 );
  let cells = Cells::< str, WithRef >::cells( &test_objects[ 1 ] );
  assert_eq!( cells.len(), 4 );
  drop( cells );

  let as_table : AsTable< '_, Vec< TestObject >, usize, TestObject, str, WithRef > = AsTable::new( &test_objects );
  let mcells = TableSize::mcells( &as_table );
  assert_eq!( mcells, [ 4, 2 ] );
  let rows = TableRows::rows( &as_table );
  assert_eq!( rows.len(), 2 );
  dbg!( rows.collect::< Vec< _ > >() );
  let header = TableHeader::header( &as_table );
  assert!( header.is_some() );
  let header = header.unwrap();
  assert_eq!( header.len(), 4 );
  assert_eq!( header.clone().collect::< Vec< _ > >(), vec!
  [
    ( "id", "id" ),
    ( "created_at", "created_at" ),
    ( "file_ids", "file_ids" ),
    ( "tools", "tools" ),
  ]);
  dbg!( header.collect::< Vec< _ > >() );

  let mut output = String::new();
  let mut context = Context::new( &mut output, Default::default() );
  // let mut context : Context< '_, print::All > = Context::new( &mut output, Default::default() );
  let got = the_module::TableFormatter::fmt( &as_table, &mut context );
  assert!( got.is_ok() );
  println!( "{}", &output );

  // with explicit arguments

  let as_table : AsTable< '_, Vec< TestObject >, usize, TestObject, str, WithRef > = AsTable::new( &test_objects );
  let table_string = as_table.table_to_string();
  println!( "\ntable_string\n{table_string}" );
  assert!( table_string.contains( "id" ) );
  assert!( table_string.contains( "created_at" ) );
  assert!( table_string.contains( "file_ids" ) );
  assert!( table_string.contains( "tools" ) );

  // without explicit arguments

  println!( "" );
  let as_table = AsTable::new( &test_objects );
  let table_string = as_table.table_to_string();
  assert!( table_string.contains( "id" ) );
  assert!( table_string.contains( "created_at" ) );
  assert!( table_string.contains( "file_ids" ) );
  assert!( table_string.contains( "tools" ) );
  println!( "\ntable_string\n{table_string}" );

}

//

#[ test ]
fn custom_formatter()
{
  // use the_module::TableToString;
  let test_objects = test_objects_gen();

  let mut output = String::new();
  let mut formatter = the_module::Styles::default();

  formatter.cell_prefix = "( ".into();
  formatter.cell_postfix = " )".into();
  formatter.cell_separator = "|".into();

  formatter.row_prefix = ">".into();
  formatter.row_postfix = "<".into();
  formatter.row_separator = "\n".into();

  let as_table = AsTable::new( &test_objects );
  // let mut context : Context< '_, print::All > = Context::new( &mut output, formatter );
  let mut context = Context::new( &mut output, formatter );
  let result = the_module::TableFormatter::fmt( &as_table, &mut context );
  assert!( result.is_ok() );

  println!( "\noutput\n{output}" );
  assert!( output.contains( "id" ) );
  assert!( output.contains( "created_at" ) );
  assert!( output.contains( "file_ids" ) );
  assert!( output.contains( "tools" ) );

  let exp = r#">( id )|( created_at )|(          file_ids          )|(           tools            )<
>( 1  )|( 1627845583 )|(        [                   )|(                            )<
>(    )|(            )|(            "file1",        )|(                            )<
>(    )|(            )|(            "file2",        )|(                            )<
>(    )|(            )|(        ]                   )|(                            )<
>( 2  )|(     13     )|( [                          )|( [                          )<
>(    )|(            )|(     "file3",               )|(     {                      )<
>(    )|(            )|(     "file4\nmore details", )|(         "tool1": "value1", )<
>(    )|(            )|( ]                          )|(     },                     )<
>(    )|(            )|(                            )|(     {                      )<
>(    )|(            )|(                            )|(         "tool2": "value2", )<
>(    )|(            )|(                            )|(     },                     )<
>(    )|(            )|(                            )|( ]                          )<"#;

  a_id!( output.as_str(), exp );


}

//

#[ test ]
fn filter_col()
{
  let test_objects = test_objects_gen();

  let mut output = String::new();
  let mut formatter = the_module::Styles::default();

  // formatter.filter_col = print::No;
  // xxx : implement

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



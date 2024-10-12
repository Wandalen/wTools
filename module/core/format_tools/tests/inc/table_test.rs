#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  AsTable,
  Cells,
  TableRows,
  TableHeader,
  WithRef,
};

//

#[ test ]
fn basic()
// where
  // for< 'a > AsTable< 'a, Vec< test_object::TestObject >, usize, test_object::TestObject, &'static str, String, &'static str > : TableFormatter< 'a >,
{
  let test_objects = test_object::test_objects_gen();

  let cells = Cells::< str, WithRef >::cells( &test_objects[ 0 ] );
  assert_eq!( cells.len(), 4 );
  let cells = Cells::< str, WithRef >::cells( &test_objects[ 1 ] );
  assert_eq!( cells.len(), 4 );
  drop( cells );

  let as_table : AsTable< '_, Vec< test_object::TestObject >, usize, test_object::TestObject, str, WithRef > = AsTable::new( &test_objects );
  // let mcells = TableSize::mcells( &as_table );
  // assert_eq!( mcells, [ 4, 3 ] );
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

}

//

#[ test ]
fn dlist_basic()
{
  use test_object::TestObject;

  let data : collection_tools::Vec< TestObject > = dlist!
  {
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
  };

  use the_module::TableFormatter;
  let _as_table : AsTable< '_, Vec< TestObject >, &str, TestObject, str, WithRef > = AsTable::new( &data );
  let as_table = AsTable::new( &data );

  let rows = TableRows::rows( &as_table );
  assert_eq!( rows.len(), 2 );

  let mut output = String::new();
  let mut context = the_module::print::Context::new( &mut output, Default::default() );
  let _got = the_module::TableFormatter::fmt( &as_table, &mut context );
  let got = as_table.table_to_string();
  assert!( got.contains( "│ id │ created_at │          file_ids          │           tools            │" ) );
  assert!( got.contains( "│     13     │ [                          │ [                          │" ) );
  assert!( got.contains( "│ 1627845583 │        [                   │                            │" ) );

  // let got = AsTable::new( &data ).table_to_string();
  // assert!( got.contains( "│ id │ created_at │          file_ids          │           tools            │" ) );
  // assert!( got.contains( "│     13     │ [                          │ [                          │" ) );
  // assert!( got.contains( "│ 1627845583 │        [                   │                            │" ) );

}

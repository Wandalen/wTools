#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  AsTable,
  TableRows,
  WithRef,
  print,
};

use std::
{
  collections::HashMap,
  // borrow::Cow,
};

use test_object::TestObject;

//

#[ test ]
fn dlist_basic()
{

  let data : collection_tools::Dlist< TestObject > = dlist!
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
  let mut context = print::Context::new( &mut output, Default::default() );
  let _got = the_module::TableFormatter::fmt( &as_table, &mut context );
  let got = as_table.table_to_string();
  assert!( got.contains( "│ id │ created_at │          file_ids          │           tools            │" ) );
  assert!( got.contains( "│     13     │ [                          │ [                          │" ) );
  assert!( got.contains( "│ 1627845583 │        [                   │                            │" ) );

}

//

#[ test ]
fn hmap_basic()
{

  let data : collection_tools::HashMap< &str, TestObject > = hmap!
  {
    "a" => TestObject
    {
      id : "1".to_string(),
      created_at : 1627845583,
      file_ids : vec![ "file1".to_string(), "file2".to_string() ],
      tools : None
    },
    "b" => TestObject
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
  let _as_table : AsTable< '_, HashMap< &str, TestObject >, &str, TestObject, str, WithRef > = AsTable::new( &data );
  let as_table = AsTable::new( &data );

  let rows = TableRows::rows( &as_table );
  assert_eq!( rows.len(), 2 );

  let mut output = String::new();
  let mut context = print::Context::new( &mut output, Default::default() );
  let _got = the_module::TableFormatter::fmt( &as_table, &mut context );
  let got = as_table.table_to_string();
  assert!( got.contains( "│ id │ created_at │          file_ids          │           tools            │" ) );
  assert!( got.contains( "│     13     │ [                          │ [                          │" ) );
  assert!( got.contains( "│ 1627845583 │        [                   │                            │" ) );

}

//

#[ test ]
fn bmap_basic()
{

  let data : Bmap< &str, TestObject > = bmap!
  {
    "a" => TestObject
    {
      id : "1".to_string(),
      created_at : 1627845583,
      file_ids : vec![ "file1".to_string(), "file2".to_string() ],
      tools : None
    },
    "b" => TestObject
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
  let _as_table : AsTable< '_, Bmap< &str, TestObject >, &str, TestObject, str, WithRef > = AsTable::new( &data );
  let as_table = AsTable::new( &data );

  let rows = TableRows::rows( &as_table );
  assert_eq!( rows.len(), 2 );

  let mut output = String::new();
  let mut context = print::Context::new( &mut output, Default::default() );
  let _got = the_module::TableFormatter::fmt( &as_table, &mut context );
  let got = as_table.table_to_string();
  assert!( got.contains( "│ id │ created_at │          file_ids          │           tools            │" ) );
  assert!( got.contains( "│     13     │ [                          │ [                          │" ) );
  assert!( got.contains( "│ 1627845583 │        [                   │                            │" ) );

}

// qqq : xxx : implement for other containers

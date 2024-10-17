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

use std::
{
  borrow::Cow,
};

//

#[ test ]
fn basic()
// where
  // for< 'a > AsTable< 'a, Vec< test_object::TestObject >, usize, test_object::TestObject, &'static str, String, &'static str > : TableFormatter< 'a >,
{
  let test_objects = test_object::test_objects_gen();

  let cells = Cells::< str>::cells( &test_objects[ 0 ] );
  assert_eq!( cells.len(), 4 );
  let cells = Cells::< str>::cells( &test_objects[ 1 ] );
  assert_eq!( cells.len(), 4 );
  drop( cells );

  let as_table : AsTable< '_, Vec< test_object::TestObject >, usize, test_object::TestObject, str> = AsTable::new( &test_objects );
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
fn iterator_over_optional_cow()
{
  // use test_object::TestObject2 as TestObject2;
  use the_module::
  {
    Fields,
    IteratorTrait,
    TableWithFields,
    WithRef,
    OptionalCow,
  };

  /// Struct representing a test object with various fields.
  #[ derive( Clone, Debug, PartialEq, Eq ) ]
  pub struct TestObject2
  {
    pub id : String,
    pub created_at : i64,
    pub file_ids : Vec< String >,
    pub tools : Option< Vec< HashMap< String, String > > >,
  }

  impl TableWithFields for TestObject2 {}

  impl Fields< &'_ str, Option< Cow< '_, str > > >
  for TestObject2
  {
    type Key< 'k > = &'k str;
    type Val< 'v > = Option< Cow< 'v, str > >;

    fn fields( &self ) -> impl IteratorTrait< Item = ( &'_ str, Option< Cow< '_, str > > ) >
    {
      use format_tools::ref_or_display_or_debug_multiline::field;
      // use format_tools::ref_or_display_or_debug::field;
      let mut dst : Vec< ( &'_ str, Option< Cow< '_, str > > ) > = Vec::new();

      // trace_macros!( true );
      dst.push( field!( &self.id ) );
      // trace_macros!( false );

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

  let data : collection_tools::Vec< TestObject2 > = dlist!
  {
    TestObject2
    {
      id : "1".to_string(),
      created_at : 1627845583,
      file_ids : vec![ "file1".to_string(), "file2".to_string() ],
      tools : None
    },
    TestObject2
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
  let _as_table : AsTable< '_, Vec< TestObject2 >, &str, TestObject2, str> = AsTable::new( &data );
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

  let got = AsTable::new( &data ).table_to_string();
  assert!( got.contains( "│ id │ created_at │          file_ids          │           tools            │" ) );
  assert!( got.contains( "│     13     │ [                          │ [                          │" ) );
  assert!( got.contains( "│ 1627845583 │        [                   │                            │" ) );

}

//

#[ test ]
fn iterator_over_strings()
{

  fn to_owned< 'a, T1 >( src : ( T1, Option< Cow< 'a, str > > ) ) -> ( T1, String )
  {
    let val = match src.1
    {
      Some( c ) => c.into_owned(),
      None => String::default(),
    };
    ( src.0, val )
  }

  // fn into< 'a, T1, T2 : Copy >( src : ( T1, OptionalCow< 'a, str, T2 > ) ) -> ( T1, Option< Cow< 'a, str > > )
  // {
  //   ( src.0, src.1.into() )
  // }

  // use test_object::TestObject as TestObject3;
  use the_module::
  {
    Fields,
    IteratorTrait,
    TableWithFields,
    WithRef,
    OptionalCow,
  };

  use std::borrow::Cow;

  /// Struct representing a test object with various fields.
  #[ derive( Clone, Debug, PartialEq, Eq ) ]
  pub struct TestObject3
  {
    pub id : String,
    pub created_at : i64,
    pub file_ids : Vec< String >,
    pub tools : Option< Vec< HashMap< String, String > > >,
  }

  impl TableWithFields for TestObject3 {}

  impl Fields< &'_ str, String >
  for TestObject3
  {
    type Key< 'k > = &'k str;
    type Val< 'v > = String;

    fn fields( &self ) -> impl IteratorTrait< Item = ( &'_ str, String ) >
    {
      use format_tools::ref_or_display_or_debug_multiline::field;
      // use format_tools::ref_or_display_or_debug::field;
      let mut dst : Vec< ( &'_ str, String ) > = Vec::new();

      dst.push( to_owned( field!( &self.id ) ) );

      dst.push( to_owned( field!( &self.created_at ) ) );
      dst.push( to_owned( field!( &self.file_ids ) ) );

      if let Some( tools ) = &self.tools
      {
        dst.push( to_owned( field!( tools ) ) );
      }
      else
      {
        dst.push( ( "tools", String::default() ) );
      }

      dst.into_iter()
    }

  }

  let _data : collection_tools::Vec< TestObject3 > = dlist!
  {
    TestObject3
    {
      id : "1".to_string(),
      created_at : 1627845583,
      file_ids : vec![ "file1".to_string(), "file2".to_string() ],
      tools : None
    },
    TestObject3
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

  // no variability in what Fields iterate over by design!

  // use the_module::TableFormatter;
  // let _as_table : AsTable< '_, Vec< TestObject3 >, &str, TestObject3, str> = AsTable::new( &data );
  // let as_table = AsTable::new( &data );

//   let rows = TableRows::rows( &as_table );
//   assert_eq!( rows.len(), 2 );
//
//   let mut output = String::new();
//   let mut context = the_module::print::Context::new( &mut output, Default::default() );
//   let _got = the_module::TableFormatter::fmt( &as_table, &mut context );
//   let got = as_table.table_to_string();
//   assert!( got.contains( "│ id │ created_at │          file_ids          │           tools            │" ) );
//   assert!( got.contains( "│     13     │ [                          │ [                          │" ) );
//   assert!( got.contains( "│ 1627845583 │        [                   │                            │" ) );

//   let got = AsTable::new( &data ).table_to_string();
//   assert!( got.contains( "│ id │ created_at │          file_ids          │           tools            │" ) );
//   assert!( got.contains( "│     13     │ [                          │ [                          │" ) );
//   assert!( got.contains( "│ 1627845583 │        [                   │                            │" ) );

}

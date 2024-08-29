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

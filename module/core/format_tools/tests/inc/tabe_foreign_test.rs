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
fn iterator_over_optional_cow()
{
  use the_module::TestObjectWithoutImpl as TestObjectWithoutImpl;
  use the_module::
  {
    Fields,
    IteratorTrait,
    TableWithFields,
    WithRef,
    OptionalCow,
  };

//   // xxx : that should fail
//   impl TableWithFields for TestObjectWithoutImpl {}
//
//   impl Fields< &'_ str, Option< Cow< '_, str > > >
//   for TestObjectWithoutImpl
//   {
//     type Key< 'k > = &'k str;
//     type Val< 'v > = Option< Cow< 'v, str > >;
//
//     fn fields( &self ) -> impl IteratorTrait< Item = ( &'_ str, Option< Cow< '_, str > > ) >
//     {
//       use format_tools::ref_or_display_or_debug_multiline::field;
//       // use format_tools::ref_or_display_or_debug::field;
//       let mut dst : Vec< ( &'_ str, Option< Cow< '_, str > > ) > = Vec::new();
//
//       dst.push( field!( &self.id ) );
//       dst.push( field!( &self.created_at ) );
//       dst.push( field!( &self.file_ids ) );
//
//       if let Some( tools ) = &self.tools
//       {
//         dst.push( field!( tools ) );
//       }
//       else
//       {
//         dst.push( ( "tools", Option::None ) );
//       }
//
//       dst.into_iter()
//     }
//
//   }

  let data : collection_tools::Vec< TestObjectWithoutImpl > = the_module::test_objects_gen();

//   use the_module::TableFormatter;
//   let _as_table : AsTable< '_, Vec< TestObjectWithoutImpl >, &str, TestObjectWithoutImpl, str> = AsTable::new( &data );
//   let as_table = AsTable::new( &data );
//
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
//
//   let got = AsTable::new( &data ).table_to_string();
//   assert!( got.contains( "│ id │ created_at │          file_ids          │           tools            │" ) );
//   assert!( got.contains( "│     13     │ [                          │ [                          │" ) );
//   assert!( got.contains( "│ 1627845583 │        [                   │                            │" ) );

}

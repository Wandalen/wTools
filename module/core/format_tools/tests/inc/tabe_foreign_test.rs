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
fn iterator_over_objects_without_impl()
{
  use the_module::TestObjectWithoutImpl as TestObjectWithoutImpl;
  use the_module::
  {
    Fields,
    IteratorTrait,
    TableWithFields,
    WithRef,
    OptionalCow,
    output_format,
  };

  // xxx : Clone should not be required
  #[ derive( Debug, Clone ) ]
  pub struct TestObjecWrap( TestObjectWithoutImpl );

  impl TableWithFields for TestObjecWrap {}
  impl Fields< &'_ str, Option< Cow< '_, str > > >
  for TestObjecWrap
  {
    type Key< 'k > = &'k str;
    type Val< 'v > = Option< Cow< 'v, str > >;

    fn fields( &self ) -> impl IteratorTrait< Item = ( &'_ str, Option< Cow< '_, str > > ) >
    {
      use format_tools::ref_or_display_or_debug_multiline::field;
      let mut dst = Vec::new();

      dst.push( field!( &self.0.id ) );
      dst.push( field!( &self.0.created_at ) );
      dst.push( field!( &self.0.file_ids ) );

      if let Some( tools ) = &self.0.tools
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

  let data : collection_tools::Vec< TestObjecWrap > = the_module::test_objects_gen()
  .into_iter()
  .map( | e | TestObjecWrap( e ) )
  .collect()
  ;

  use the_module::TableFormatter;
  let _as_table : AsTable< '_, Vec< TestObjecWrap >, &str, TestObjecWrap, str > = AsTable::new( &data );
  let as_table = AsTable::new( &data );

  let rows = TableRows::rows( &as_table );
  assert_eq!( rows.len(), 2 );

  let mut output = String::new();
  let mut context = the_module::print::Context::new( &mut output, Default::default() );
  let _result = the_module::TableFormatter::fmt( &as_table, &mut context );

  // = output as table

  let got = as_table.table_to_string();
  assert!( got.contains( "│ id │ created_at │          file_ids          │           tools            │" ) );
  assert!( got.contains( "│     13     │ [                          │ [                          │" ) );
  assert!( got.contains( "│ 1627845583 │        [                   │                            │" ) );

  let got = AsTable::new( &data ).table_to_string();
  assert!( got.contains( "│ id │ created_at │          file_ids          │           tools            │" ) );
  assert!( got.contains( "│     13     │ [                          │ [                          │" ) );
  assert!( got.contains( "│ 1627845583 │        [                   │                            │" ) );

  let got = AsTable::new( &data ).table_to_string_with_format( &output_format::Table::default() );
  println!( "{}", &got );
  assert!( got.contains( "│ id │ created_at │          file_ids          │           tools            │" ) );
  assert!( got.contains( "│     13     │ [                          │ [                          │" ) );
  assert!( got.contains( "│ 1627845583 │        [                   │                            │" ) );

  // = output as records

  // let format = output_format::Records::default();
  let mut output = String::new();
  let format = the_module::output_format::Records::default();
  let printer = the_module::print::Printer::with_format( &format );
  let mut context = the_module::print::Context::new( &mut output, printer );
  let got = the_module::TableFormatter::fmt( &as_table, &mut context );
  assert!( got.is_ok() );

  let got = AsTable::new( &data ).table_to_string_with_format( &output_format::Records::default() );
  println!( "{}", &got );
  assert!( got.contains( "│ id         │ 1            │" ) );
  assert!( got.contains( "│ created_at │ 1627845583   │" ) );
  assert!( got.contains( "│ id         │ 2                          │" ) );
  assert!( got.contains( "│ created_at │ 13                         │" ) );

  // = output as keys

  let got = AsTable::new( &data ).table_to_string_with_format( &output_format::Keys::default() );
  println!( "{}", &got );
  assert!( got.contains( "- id" ) );
  assert!( got.contains( "- created_at" ) );
  assert!( got.contains( "- file_ids" ) );
  assert!( got.contains( "- tools" ) );
  assert!( got.contains( "4 fields" ) );

  // assert!( false );

}

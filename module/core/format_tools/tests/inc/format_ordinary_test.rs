#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  AsTable,
  WithRef,
  filter,
  print,
  output_format,
};

use std::
{
  // collections::HashMap,
  borrow::Cow,
};

//

#[ test ]
fn basic()
{
  let test_objects = test_object::test_objects_gen();

  let _as_table : AsTable< '_, Vec< test_object::TestObject >, usize, test_object::TestObject, str, WithRef > = AsTable::new( &test_objects );
  let as_table = AsTable::new( &test_objects );

  let mut output = String::new();
  let mut context = print::Context::new( &mut output, Default::default() );
  // let mut context : Context< '_, print::All > = Context::new( &mut output, Default::default() );
  let got = the_module::TableFormatter::fmt( &as_table, &mut context );
  assert!( got.is_ok() );
  println!( "{}", &output );

  // Example of output formatting as table.
  //
  //  sid | sname | gap
  // -----+-------+-----
  //    3 | Alice |   5
  //    6 | Joe   |   1
  //   10 | Boris |   5
  // (3 rows)

  let exp = r#"│ id │ created_at │          file_ids          │           tools            │
─────────────────────────────────────────────────────────────────────────────
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

#[ test ]
fn table_to_string()
{
  use the_module::TableFormatter;
  let test_objects = test_object::test_objects_gen();

  // with explicit arguments

  let as_table : AsTable< '_, Vec< test_object::TestObject >, usize, test_object::TestObject, str, WithRef > = AsTable::new( &test_objects );
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
fn custom_format()
{
  // use the_module::TableFormatter;
  let test_objects = test_object::test_objects_gen();

  let mut format = output_format::Ordinary::default();
  format.cell_prefix = "( ".into();
  format.cell_postfix = " )".into();
  format.cell_separator = "|".into();
  format.row_prefix = ">".into();
  format.row_postfix = "<".into();
  format.row_separator = "\n".into();

  let printer = print::Printer::with_format( &format );
  let as_table = AsTable::new( &test_objects );
  let mut output = String::new();
  let mut context = print::Context::new( &mut output, printer );
  let result = the_module::TableFormatter::fmt( &as_table, &mut context );
  assert!( result.is_ok() );

  println!( "\noutput\n{output}" );
  assert!( output.contains( "id" ) );
  assert!( output.contains( "created_at" ) );
  assert!( output.contains( "file_ids" ) );
  assert!( output.contains( "tools" ) );

  let exp = r#">( id )|( created_at )|(          file_ids          )|(           tools            )<
─────────────────────────────────────────────────────────────────────────────────────
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

  // using table_to_string_with_format

  use the_module::TableFormatter;

  let mut format = output_format::Ordinary::default();
  format.cell_prefix = "( ".into();
  format.cell_postfix = " )".into();
  format.cell_separator = "|".into();
  format.row_prefix = ">".into();
  format.row_postfix = "<".into();
  format.row_separator = "\n".into();

  // let as_table = AsTable::new( &test_objects );
  let got = AsTable::new( &test_objects ).table_to_string_with_format( &format );
  let exp = r#">( id )|( created_at )|(          file_ids          )|(           tools            )<
─────────────────────────────────────────────────────────────────────────────────────
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
  a_id!( got, exp );

}



#[ test ]
fn filter_col_none()
{
  let test_objects = test_object::test_objects_gen();

  let mut format = output_format::Ordinary::default();
  format.cell_prefix = "( ".into();
  format.cell_postfix = " )".into();
  format.cell_separator = "|".into();
  format.row_prefix = ">".into();
  format.row_postfix = "<".into();
  format.row_separator = "\n".into();

  let mut printer = print::Printer::with_format( &format );
  printer.filter_col = &filter::None;

  let as_table = AsTable::new( &test_objects );
  let mut output = String::new();
  let mut context = print::Context::new( &mut output, printer );
  let result = the_module::TableFormatter::fmt( &as_table, &mut context );
  assert!( result.is_ok() );

  println!( "\noutput\n{output}" );

  let exp = r#"><
──
><
><"#;

  a_id!( output.as_str(), exp );

}

//

#[ test ]
fn filter_col_callback()
{
  let test_objects = test_object::test_objects_gen();

  let mut format = output_format::Ordinary::default();
  format.cell_prefix = "( ".into();
  format.cell_postfix = " )".into();
  format.cell_separator = "|".into();
  format.row_prefix = ">".into();
  format.row_postfix = "<".into();
  format.row_separator = "\n".into();

  let mut printer = print::Printer::with_format( &format );
  printer.filter_col = &| title : &str |
  {
    title != "tools"
  };

  let as_table = AsTable::new( &test_objects );
  let mut output = String::new();
  let mut context = print::Context::new( &mut output, printer );
  let result = the_module::TableFormatter::fmt( &as_table, &mut context );
  assert!( result.is_ok() );

  println!( "\noutput\n{output}" );

  let exp = r#">( id )|( created_at )|(          file_ids          )<
──────────────────────────────────────────────────────
>( 1  )|( 1627845583 )|(        [                   )<
>(    )|(            )|(            "file1",        )<
>(    )|(            )|(            "file2",        )<
>(    )|(            )|(        ]                   )<
>( 2  )|(     13     )|( [                          )<
>(    )|(            )|(     "file3",               )<
>(    )|(            )|(     "file4\nmore details", )<
>(    )|(            )|( ]                          )<"#;

  a_id!( output.as_str(), exp );

}

//

#[ test ]
fn filter_row_none()
{
  let test_objects = test_object::test_objects_gen();

  let mut format = output_format::Ordinary::default();
  format.cell_prefix = "( ".into();
  format.cell_postfix = " )".into();
  format.cell_separator = "|".into();
  format.row_prefix = ">".into();
  format.row_postfix = "<".into();
  format.row_separator = "\n".into();

  let mut printer = print::Printer::with_format( &format );
  printer.filter_row = &filter::None;

  let as_table = AsTable::new( &test_objects );
  let mut output = String::new();
  let mut context = print::Context::new( &mut output, printer );
  let result = the_module::TableFormatter::fmt( &as_table, &mut context );
  assert!( result.is_ok() );

  println!( "\noutput\n{output}" );

  let exp = r#""#;

  a_id!( output.as_str(), exp );

}

//

#[ test ]
fn filter_row_callback()
{
  let test_objects = test_object::test_objects_gen();

  let mut format = output_format::Ordinary::default();
  format.cell_prefix = "( ".into();
  format.cell_postfix = " )".into();
  format.cell_separator = "|".into();
  format.row_prefix = ">".into();
  format.row_postfix = "<".into();
  format.row_separator = "\n".into();

  let mut printer = print::Printer::with_format( &format );
  printer.filter_row = &| _typ, irow, _row : &[ ( Cow< '_, str >, [ usize ; 2 ] ) ] |
  {
    irow != 1
  };

  let as_table = AsTable::new( &test_objects );
  let mut output = String::new();
  let mut context = print::Context::new( &mut output, printer );
  let result = the_module::TableFormatter::fmt( &as_table, &mut context );
  assert!( result.is_ok() );

  println!( "\noutput\n{output}" );

  let exp = r#">( id )|( created_at )|(          file_ids          )|(           tools            )<
─────────────────────────────────────────────────────────────────────────────────────
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

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

  let _as_table : AsTable< '_, Vec< test_object::TestObject >, usize, test_object::TestObject, str> = AsTable::new( &test_objects );
  let as_table = AsTable::new( &test_objects );

  let mut output = String::new();
  let format = output_format::Records::default();
  let printer = print::Printer::with_format( &format );
  let mut context = print::Context::new( &mut output, printer );
  let got = the_module::TableFormatter::fmt( &as_table, &mut context );
  assert!( got.is_ok() );
  println!( "{}", &output );

  let exp = r#" = 1
│ id         │ 1            │
│ created_at │ 1627845583   │
│ file_ids   │ [            │
│            │     "file1", │
│            │     "file2", │
│            │ ]            │
│ tools      │              │
 = 2
│ id         │ 2                          │
│ created_at │ 13                         │
│ file_ids   │ [                          │
│            │     "file3",               │
│            │     "file4\nmore details", │
│            │ ]                          │
│ tools      │ [                          │
│            │     {                      │
│            │         "tool1": "value1", │
│            │     },                     │
│            │     {                      │
│            │         "tool2": "value2", │
│            │     },                     │
│            │ ]                          │"#;
  a_id!( output.as_str(), exp );

}

//

#[ test ]
fn custom_format()
{
  // use the_module::TableFormatter;
  let test_objects = test_object::test_objects_gen();

  let mut format = output_format::Records::default();
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

  let exp = r#" = 1
>( id         )|( 1            )<
>( created_at )|( 1627845583   )<
>( file_ids   )|( [            )<
>(            )|(     "file1", )<
>(            )|(     "file2", )<
>(            )|( ]            )<
>( tools      )|(              )<
 = 2
>( id         )|( 2                          )<
>( created_at )|( 13                         )<
>( file_ids   )|( [                          )<
>(            )|(     "file3",               )<
>(            )|(     "file4\nmore details", )<
>(            )|( ]                          )<
>( tools      )|( [                          )<
>(            )|(     {                      )<
>(            )|(         "tool1": "value1", )<
>(            )|(     },                     )<
>(            )|(     {                      )<
>(            )|(         "tool2": "value2", )<
>(            )|(     },                     )<
>(            )|( ]                          )<"#;
  a_id!( output.as_str(), exp );

  // using table_to_string_with_format

  use the_module::TableFormatter;

  let mut format = output_format::Records::default();
  format.cell_prefix = "( ".into();
  format.cell_postfix = " )".into();
  format.cell_separator = "|".into();
  format.row_prefix = ">".into();
  format.row_postfix = "<".into();
  format.row_separator = "\n".into();

  // let as_table = AsTable::new( &test_objects );
  let got = AsTable::new( &test_objects ).table_to_string_with_format( &format );
  let exp = r#" = 1
>( id         )|( 1            )<
>( created_at )|( 1627845583   )<
>( file_ids   )|( [            )<
>(            )|(     "file1", )<
>(            )|(     "file2", )<
>(            )|( ]            )<
>( tools      )|(              )<
 = 2
>( id         )|( 2                          )<
>( created_at )|( 13                         )<
>( file_ids   )|( [                          )<
>(            )|(     "file3",               )<
>(            )|(     "file4\nmore details", )<
>(            )|( ]                          )<
>( tools      )|( [                          )<
>(            )|(     {                      )<
>(            )|(         "tool1": "value1", )<
>(            )|(     },                     )<
>(            )|(     {                      )<
>(            )|(         "tool2": "value2", )<
>(            )|(     },                     )<
>(            )|( ]                          )<"#;
  a_id!( got, exp );

}

//

#[ test ]
fn filter_col_none()
{
  let test_objects = test_object::test_objects_gen();

  let mut format = output_format::Records::default();
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

  let exp = r#" = 1

 = 2
"#;

  a_id!( output.as_str(), exp );

}

//

#[ test ]
fn filter_col_callback()
{
  let test_objects = test_object::test_objects_gen();

  let mut format = output_format::Records::default();
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

  let exp = r#" = 1
>( id         )|( 1            )<
>( created_at )|( 1627845583   )<
>( file_ids   )|( [            )<
>(            )|(     "file1", )<
>(            )|(     "file2", )<
>(            )|( ]            )<
 = 2
>( id         )|( 2                          )<
>( created_at )|( 13                         )<
>( file_ids   )|( [                          )<
>(            )|(     "file3",               )<
>(            )|(     "file4\nmore details", )<
>(            )|( ]                          )<"#;

  a_id!( output.as_str(), exp );

}

//

#[ test ]
fn filter_row_none()
{
  let test_objects = test_object::test_objects_gen();

  let mut format = output_format::Records::default();
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

  let mut format = output_format::Records::default();
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

  let exp = r#" = 2
>( id         )|( 2                          )<
>( created_at )|( 13                         )<
>( file_ids   )|( [                          )<
>(            )|(     "file3",               )<
>(            )|(     "file4\nmore details", )<
>(            )|( ]                          )<
>( tools      )|( [                          )<
>(            )|(     {                      )<
>(            )|(         "tool1": "value1", )<
>(            )|(     },                     )<
>(            )|(     {                      )<
>(            )|(         "tool2": "value2", )<
>(            )|(     },                     )<
>(            )|( ]                          )<"#;

  a_id!( output.as_str(), exp );

}

//

// xxx : enable

#[ test ]
fn test_width_limiting()
{
  use the_module::string;

  for width in min_width()..max_width()
  {
    println!("width: {}", width);

    let test_objects = test_object::test_objects_gen();
    let as_table = AsTable::new( &test_objects );

    let mut format = output_format::Records::default();
    format.max_width = width;

    let mut output = String::new();
    let printer = print::Printer::with_format( &format );
    let mut context = print::Context::new( &mut output, printer );

    let got = the_module::TableFormatter::fmt( &as_table, &mut context );

    assert!( got.is_ok() );
    
    for line in string::lines( &output )
    {
      if line.starts_with(" = ") 
      {
        continue;
      }

      if line.chars().count() > width 
      {
        println!("{}", output);
      }

      assert!( line.chars().count() <= width );
    }
  }
}

#[ test ]
fn test_error_on_unsatisfiable_limit()
{
  // 0 is a special value that signifies no limit.
  for width in 1..( min_width() )
  {
    println!( "width: {}", width );

    let test_objects = test_object::test_objects_gen();
    let as_table = AsTable::new( &test_objects );

    let mut format = output_format::Records::default();
    format.max_width = width;

    let mut output = String::new();
    let printer = print::Printer::with_format( &format );
    let mut context = print::Context::new( &mut output, printer );

    let got = the_module::TableFormatter::fmt( &as_table, &mut context );

    assert!( got.is_err() );
  }
}

#[ test ]
fn test_table_not_grows()
{
  use the_module::string;

  let expected_width = max_width();
  
  // The upper bound was chosen arbitrarily.
  for width in ( expected_width + 1 )..500
  {
    println!( "width: {}", width );

    let test_objects = test_object::test_objects_gen();
    let as_table = AsTable::new( &test_objects );

    let mut format = output_format::Records::default();
    format.max_width = width;

    let mut output = String::new();
    let printer = print::Printer::with_format( &format );
    let mut context = print::Context::new( &mut output, printer );

    let got = the_module::TableFormatter::fmt( &as_table, &mut context );

    assert!( got.is_ok() );
    println!("{}", output);

    for line in string::lines( &output )
    {
      if line.starts_with(" = ") 
      {
        continue;
      }

      assert!( line.chars().count() <= expected_width );
    }
  }
}

/// Utility function for calculating minimum table width with `test_objects_gen()` with
/// the default table style.
fn min_width() -> usize
{
  let format = output_format::Records::default();
  format.min_width()
}

/// Utility function for calculating default table width with `test_objects_gen()` with
/// the default table style with table width limit equals to 0.
fn max_width() -> usize
{
  use the_module::string;

  let test_objects = test_object::test_objects_gen();
  let as_table = AsTable::new( &test_objects );

  let format = output_format::Records::default();

  let mut output = String::new();
  let printer = print::Printer::with_format( &format );
  let mut context = print::Context::new( &mut output, printer );

  let got = the_module::TableFormatter::fmt( &as_table, &mut context );
  assert!( got.is_ok() );

  string::lines( &output ).map( |s| s.chars().count() ).max().unwrap_or(0)
}
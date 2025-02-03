#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn path_with_no_glob_patterns()
{
  assert_eq!( the_module::path::is_glob( "file.txt" ), false );
}

#[ test ]
fn path_with_unescaped_glob_star()
{
  assert_eq!( the_module::path::is_glob( "*.txt" ), true );
}

#[ test ]
fn path_with_escaped_glob_star()
{
  assert_eq!( the_module::path::is_glob( "\\*.txt" ), false );
}

#[ test ]
fn path_with_unescaped_brackets()
{
  assert_eq!( the_module::path::is_glob( "file[0-9].txt" ), true );
}

#[ test ]
fn path_with_escaped_brackets()
{
  assert_eq!( the_module::path::is_glob( "file\\[0-9].txt" ), false );
}

#[ test ]
fn path_with_unescaped_question_mark()
{
  assert_eq!( the_module::path::is_glob( "file?.txt" ), true );
}

#[ test ]
fn path_with_escaped_question_mark()
{
  assert_eq!( the_module::path::is_glob( "file\\?.txt" ), false );
}

#[ test ]
fn path_with_unescaped_braces()
{
  assert_eq!( the_module::path::is_glob( "file{a,b}.txt" ), true );
}

#[ test ]
fn path_with_escaped_braces()
{
  assert_eq!( the_module::path::is_glob( "file\\{a,b}.txt" ), false );
}

#[ test ]
fn path_with_mixed_escaped_and_unescaped_glob_characters()
{
  assert_eq!( the_module::path::is_glob( "file\\*.txt" ), false );
  assert_eq!( the_module::path::is_glob( "file[0-9]\\*.txt" ), true );
}

#[ test ]
fn path_with_nested_brackets()
{
  assert_eq!( the_module::path::is_glob( "file[[0-9]].txt" ), true );
}

#[ test ]
fn path_with_nested_escaped_brackets()
{
  assert_eq!( the_module::path::is_glob( "file\\[\\[0-9\\]\\].txt" ), false );
}

#[ test ]
fn path_with_escaped_backslash_before_glob_characters()
{
  assert_eq!( the_module::path::is_glob( "file\\*.txt" ), false );
}

#[ test ]
fn path_with_escaped_double_backslashes_before_glob_characters()
{
  assert_eq!( the_module::path::is_glob( "file\\\\*.txt" ), true );
}

#[ test ]
fn path_with_complex_mix_of_escaped_and_unescaped_glob_characters()
{
  assert_eq!( the_module::path::is_glob( "file\\[0-9]*?.txt" ), true );
}

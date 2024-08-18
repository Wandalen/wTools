#[ allow( unused_imports ) ]
use super::*;

#[ cfg( feature = "enabled" ) ]
#[ path = "." ]
mod fundamental
{
  #[ allow( unused_imports ) ]
  use super::*;

  mod fields_test;
  mod md_math_test;
  mod print_test;
  mod print_without_wrap;
  mod string_test;
  mod to_string_test;
  mod to_string_with_fallback_test;

}

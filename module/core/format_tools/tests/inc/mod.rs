#[ allow( unused_imports ) ]
use super::*;

#[ cfg( feature = "enabled" ) ]
#[ path = "." ]
mod fundamental
{
  #[ allow( unused_imports ) ]
  use super::*;

  mod test_object;

  mod table_test;
  mod format_ordinary_test;
  mod format_records_test;

  mod collection_test;
  mod fields_test;
  mod md_math_test;
  mod string_test;
  mod to_string_test;
  mod to_string_with_fallback_test;

}

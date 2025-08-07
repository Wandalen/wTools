use super::*;

#[ cfg( feature = "enabled" ) ]
#[ path = "." ]
mod fundamental
{
  use super::*;

  mod test_object;

  mod table_test;
  mod tabe_foreign_test;

  mod format_table_test;
  mod format_records_test;
  // mod format_keys_test; // qqq : xxx : implement

  mod collection_test;
  mod fields_test;
  mod md_math_test;
  mod string_test;
  mod to_string_test;
  mod to_string_with_fallback_test;

}

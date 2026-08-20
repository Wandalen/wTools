//! Simple test to verify debug assertion functions are available

#[ test ]
#[ cfg( feature = "standalone_build" ) ] // the four debug_assert_* macros are defined only in standalone mode
fn test_debug_assertion_functions_available()
{
  // Test that debug assertion functions can be called
  test_tools ::debug_assert_identical!(42, 42);
  test_tools ::debug_assert_id!(42, 42);
  test_tools ::debug_assert_not_identical!(42, 43);
  test_tools ::debug_assert_ni!(42, 43);
}
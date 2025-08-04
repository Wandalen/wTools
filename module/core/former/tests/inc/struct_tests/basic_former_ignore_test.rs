//! Basic test to verify the Former derive works with new #[former_ignore] attribute

#[allow(unused_imports)]
use ::former::prelude::*;
use ::former::Former;

#[test]
fn basic_former_ignore_test()
{
  /// Test struct with former_ignore attribute (not using standalone constructors)
  #[derive(Debug, PartialEq, Former)]
  pub struct BasicConfig
  {
    name: String,             // Regular field
    #[former_ignore]          // This field should be ignored for some purpose
    internal_flag: bool,
  }

  // Test basic Former functionality
  let config = BasicConfig::former()
    .name("test".to_string())
    .internal_flag(true)
    .form();

  assert_eq!(config.name, "test");
  assert_eq!(config.internal_flag, true);
}
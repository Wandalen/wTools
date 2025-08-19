#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Simple test for #[`former_ignore`] attribute - minimal test to verify basic functionality

#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

#[ test ]
fn simple_former_ignore_test()
{
  /// Test struct with standalone constructors and `former_ignore` attribute
  #[ derive( Debug, PartialEq, Former ) ]
  #[ standalone_constructors ]
  pub struct SimpleConfig
  {
    name: String,             // Constructor arg (not ignored)
    #[ former_ignore ]          // This field is NOT a constructor arg
    value: Option<i32>,
  }

  // Since value is marked with #[ former_ignore ], the standalone constructor
  // should return a Former that allows setting the ignored field
  let config_former = simple_config("test".to_string());
  
  // Set the ignored field and form
  let config = config_former
    .value(42)
    .form();

  assert_eq!(config.name, "test");
  assert_eq!(config.value, Some(42));
}

#[ test ]
fn simple_no_ignore_test()
{
  /// Test struct with NO ignored fields - should return Self directly
  #[ derive( Debug, PartialEq, Former ) ]
  #[ standalone_constructors ]
  pub struct DirectConfig
  {
    name: String,  // Constructor arg (not ignored)
    value: i32,    // Constructor arg (not ignored)
  }

  // NO fields ignored, so direct_config() should return Self directly
  let config = direct_config("test".to_string(), 42);
  assert_eq!(config.name, "test");
  assert_eq!(config.value, 42);
}
#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Test specifically for #[`former_ignore`] behavior in standalone constructors

#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

#[ test ]
fn standalone_constructor_no_ignore_returns_self()
{
  /// Test struct with NO ignored fields - constructor should return Self directly
  #[ derive( Debug, PartialEq, Former ) ]
  #[ standalone_constructors ]
  pub struct DirectStruct
  {
    name: String,  // Constructor arg (not ignored)
    value: i32,    // Constructor arg (not ignored)
  }

  // NO fields ignored, so direct_struct() should return Self directly
  let instance = direct_struct("test".to_string(), 42);
  
  // Verify we got Self directly (no need to call .form())
  assert_eq!(instance.name, "test");
  assert_eq!(instance.value, 42);
}

#[ test ]
fn standalone_constructor_with_ignore_returns_former()
{
  /// Test struct with some ignored fields - constructor should return Former
  #[ derive( Debug, PartialEq, Former ) ]
  #[ standalone_constructors ]
  pub struct PartialStruct
  {
    name: String,             // Constructor arg (not ignored)
    #[ former_ignore ]          // This field is NOT a constructor arg
    value: Option<i32>,
  }

  // Since value is marked with #[ former_ignore ], the standalone constructor
  // should take only name as argument and return a Former
  let config_former = partial_struct("test".to_string());
  
  // Set the ignored field and form
  let config = config_former
    .value(42)
    .form();

  assert_eq!(config.name, "test");
  assert_eq!(config.value, Some(42));
  
  // Test without setting the ignored field
  let config2_former = partial_struct("test2".to_string());
  let config2 = config2_former.form();
  
  assert_eq!(config2.name, "test2");
  assert_eq!(config2.value, None);
}
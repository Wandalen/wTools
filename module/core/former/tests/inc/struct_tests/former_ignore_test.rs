//! Test for new #[`former_ignore`] attribute functionality
//! 
//! This test verifies that the new #[`former_ignore`] attribute works correctly with
//! standalone constructors, implementing the inverted logic from the old #[`arg_for_constructor`].

#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

/// Test struct with standalone constructors and `former_ignore` attribute
#[ derive( Debug, PartialEq, Former ) ]
#[ standalone_constructors ]
pub struct ServerConfig
{
  host: String,           // Constructor arg (not ignored)
  port: u16,              // Constructor arg (not ignored)
  #[ former_ignore ]        // This field is NOT a constructor arg
  timeout: Option<u32>,
}

#[ test ]
fn former_ignore_standalone_constructor_test()
{
  // Since timeout is marked with #[ former_ignore ], the standalone constructor
  // should return a Former that allows setting the ignored field
  let config_former = server_config("localhost".to_string(), 8080u16);
  
  // Set the ignored field and form
  let config = config_former
    .timeout(5000u32)
    .form();

  assert_eq!(config.host, "localhost");
  assert_eq!(config.port, 8080u16);
  assert_eq!(config.timeout, Some(5000u32));
}

#[ test ]
fn former_ignore_no_ignored_fields_test()
{
  /// Test struct with NO ignored fields - should return Self directly
  #[ derive( Debug, PartialEq, Former ) ]
  #[ standalone_constructors ]
  pub struct Point
  {
    x: i32,  // Constructor arg (not ignored)
    y: i32,  // Constructor arg (not ignored)
  }

  // NO fields ignored, so point() should return Self directly
  let p = point(10, 20);
  assert_eq!(p.x, 10);
  assert_eq!(p.y, 20);
}
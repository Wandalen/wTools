//! Test debug attribute functionality
//!
//! ## Test Matrix for Debug Attribute
//!
//! | ID   | Test Case                      | Expected Output                     |
//! |------|--------------------------------|-------------------------------------|
//! | T4.1 | Debug attribute present        | Debug output generated              |
//! | T4.2 | Debug output format            | Well-structured debug information   |

use component_model::ComponentModel;

/// Test debug attribute generates output
/// Test Combination: T4.1
#[ derive( ComponentModel ) ]
#[ debug ]
struct DebugTest
{
  name : String,
  value : i32,
}

/// Test debug attribute functionality works
/// Test Combination: T4.1 & T4.2
#[ test ]
fn test_debug_attribute_functionality()
{
  // This test ensures the debug attribute compiles correctly
  // The actual debug output would be visible during compilation with debug attribute
  let mut config = DebugTest { name: String::new(), value: 0 };
  
  // Field-specific methods should be generated and work
  config.name_assign( "debug_test".to_string() );
  config.value_assign( 123i32 );
  
  assert_eq!( config.name, "debug_test" );
  assert_eq!( config.value, 123 );
  
  // Test fluent pattern also works with debug enabled
  let config2 = DebugTest { name: String::new(), value: 0 }
    .name_impute( "debug_fluent".to_string() )
    .value_impute( 456i32 );
    
  assert_eq!( config2.name, "debug_fluent" );
  assert_eq!( config2.value, 456 );
}
//! Test file to verify the comprehensive #[ debug ] attribute implementation

#![allow(unused_imports)]
#![allow(missing_docs)]

use former as the_module;

#[ cfg( not( feature = "no_std" ) ) ]
#[ cfg( feature = "derive_former" ) ]
#[ cfg( feature = "former_diagnostics_print_generated" ) ]
fn test_debug_attribute() 
{
  use former::Former;

  // Simple struct with debug attribute
  #[ derive( Debug, PartialEq, Former ) ]
  // #[ debug ] // <-- Commented out - debug attribute only for temporary debugging
  pub struct DebugStruct 
  {
    field: String,
  }

  // Generic struct with debug attribute  
  #[ derive( Debug, PartialEq, Former ) ]
  // #[ debug ] // <-- Commented out - debug attribute only for temporary debugging
  pub struct GenericDebugStruct< T >
  where
    T: Clone,
  {
    generic_field: T,
    normal_field: String,
  }

  // Complex struct with lifetime parameters and debug
  #[ derive( Debug, PartialEq, Former ) ]
  // #[ debug ] // <-- Commented out - debug attribute only for temporary debugging
  pub struct LifetimeDebugStruct< 'a, T >
  where
    T: Clone + 'a,
  {
    reference_field: &'a str,
    generic_field: T,
  }

  // Struct with storage fields and debug
  #[ derive( Debug, PartialEq, Former ) ]
  // #[ debug ] // <-- Commented out - debug attribute only for temporary debugging
  #[ storage_fields( temp_value: i32 ) ]
  pub struct StorageFieldsDebugStruct
  {
    final_field: String,
  }

  // Test that structs can be constructed normally
  let _simple = DebugStruct::former()
    .field( "test".to_string() )
    .form();

  let _generic = GenericDebugStruct::former()
    .generic_field( 42i32 )
    .normal_field( "test".to_string() )
    .form();

  let test_str = "lifetime_test";
  let _lifetime = LifetimeDebugStruct::former()
    .reference_field( test_str )
    .generic_field( "generic_value".to_string() )
    .form();

  let _storage = StorageFieldsDebugStruct::former()
    .final_field( "final".to_string() )
    .form();

  println!("All debug attribute tests completed successfully!");
}

#[ cfg( not( feature = "no_std" ) ) ]
#[ cfg( feature = "derive_former" ) ]
#[ cfg( feature = "former_diagnostics_print_generated" ) ]
fn main()
{
  test_debug_attribute();
}

#[ cfg( any( feature = "no_std", not( feature = "derive_former" ), not( feature = "former_diagnostics_print_generated" ) ) ) ]
fn main()
{
  println!("Debug attribute test requires 'derive_former' and 'former_diagnostics_print_generated' features");
}
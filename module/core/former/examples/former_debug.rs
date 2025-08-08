
//! Comprehensive demonstration of the `#[ debug ]` attribute for Former derive macro.
//!
//! The `#[ debug ]` attribute provides detailed debug information about:
//! - Input analysis (generics, lifetimes, fields)
//! - Code generation process
//! - Generated code structure  
//! - Any transformations or validations performed
//!
//! To see the debug output, run with the diagnostic feature:
//! ```bash
//! cargo run --example former_debug --features former_diagnostics_print_generated
//! ```

#[cfg(any(not(feature = "derive_former"), not(feature = "enabled")))]
fn main() {
  println!("This example requires the 'derive_former' and 'enabled' features");
}

#[cfg(all(feature = "derive_former", feature = "enabled"))]
fn main() {
  use former::Former;

  println!("=== Former Debug Attribute Comprehensive Example ===");
  println!();

  // Example 1: Simple struct with debug - shows basic input analysis
  #[ derive( Debug, PartialEq, Former ) ]
  // #[ debug ] // <-- Commented out - debug attribute only for temporary debugging
  pub struct UserProfile {
    age: i32,
    username: String,
    bio_optional: Option<String>,
  }

  // Example 2: Generic struct with debug - shows generic parameter analysis
  #[ derive( Debug, PartialEq, Former ) ]
  // #[ debug ] // <-- Commented out - debug attribute only for temporary debugging
  pub struct GenericContainer<T, U>
  where
    T: Clone + core::fmt::Debug,
    U: Default,
  {
    primary: T,
    secondary: U,
    metadata: String,
  }

  // Example 3: Lifetime parameters with debug - shows lifetime handling
  #[ derive( Debug, PartialEq, Former ) ]
  // #[ debug ] // <-- Commented out - debug attribute only for temporary debugging
  pub struct LifetimeStruct<'a> {
    name: &'a str,
    data: String,
  }

  // Example 4: Struct with storage fields and debug
  #[ derive( Debug, PartialEq, Former ) ]
  // #[ debug ] // <-- Commented out - debug attribute only for temporary debugging
  #[ storage_fields( temp_id: u64, processing_state: bool ) ]
  pub struct StorageStruct {
    id: u64,
    name: String,
    tags: Vec<String>,
  }

  println!("Building examples to demonstrate debug attribute functionality...");
  println!();

  // Build example 1: Simple struct
  let profile = UserProfile::former()
    .age(30)
    .username("JohnDoe".to_string())
    .bio_optional("Software Developer".to_string())
    .form();

  println!("1. Simple UserProfile: {profile:?}");

  // Build example 2: Generic struct
  let generic: GenericContainer<String, i32> = GenericContainer::former()
    .primary("test".to_string())
    .secondary(42i32)
    .metadata("example metadata".to_string())
    .form();

  println!("2. Generic Container: {generic:?}");

  // Build example 3: Lifetime struct
  let name = "lifetime_example";
  let lifetime_struct = LifetimeStruct::former()
    .name(name)
    .data("owned data".to_string())
    .form();

  println!("3. Lifetime Struct: {lifetime_struct:?}");

  // Build example 4: Storage struct
  let storage_struct = StorageStruct::former()
    .id(12345u64)
    .name("storage_example".to_string())
    .tags(vec!["storage".to_string(), "debug".to_string()])
    .form();

  println!("4. Storage Struct: {storage_struct:?}");

  println!();
  println!("=== Debug Information ===");

  #[ cfg( feature = "former_diagnostics_print_generated" ) ]
  {
    println!("Debug output should have been displayed above showing:");
    println!("  • Input Analysis: Field types, generic parameters, constraints");
    println!("  • Generic Classification: How generics are categorized and handled");
    println!("  • Components Analysis: What ecosystem components will be generated");
    println!("  • Generated Code: The complete Former pattern implementation");
    println!();
    println!("This comprehensive debug information helps developers:");
    println!("  • Understand macro processing decisions");
    println!("  • Debug complex generic scenarios");
    println!("  • Verify correct trait bound propagation");
    println!("  • Troubleshoot lifetime parameter issues");
  }

  #[cfg(not(feature = "former_diagnostics_print_generated"))]
  {
    println!("To see comprehensive debug information, run with:");
    println!("cargo run --example former_debug --features former_diagnostics_print_generated");
    println!();
    println!("The debug output will show detailed information about:");
    println!("  • Input analysis (generics, lifetimes, fields)");
    println!("  • Code generation process and decisions");
    println!("  • Generated code structure and components");
    println!("  • Transformations and validations performed");
  }
}

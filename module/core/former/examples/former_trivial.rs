//! # Builder Pattern Implementation with Former
//!
//! This module demonstrates the use of the `Former` trait to apply the builder pattern for Rust structs.
//! The `Former` trait simplifies the instantiation of structs by enabling a fluent, method-chaining approach
//! to set fields before finalizing the instance with `.form()`. It is particularly useful for structs with optional fields
//! or when a clear and concise way to instantiate complex data structures is needed.
//!
//! ## How Former Works
//!
//! - **Trait Derivation** : By deriving `Former` on a struct, you automatically generate builder methods for each field.
//! - **Fluent Interface** : Each field's builder method allows for setting the value of that field and returns a mutable reference to the builder,
//!   enabling method chaining.
//! - **Optional Fields** : Optional fields can be easily handled without needing to explicitly set them to `None`.
//! - **Finalization** : The `.form()` method finalizes the building process and returns the constructed struct instance.
//!
//! This approach abstracts away the need for manually implementing a builder for each struct, making code more readable and maintainable.
//!

#[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
fn main() {}

#[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
fn main()
{
  use former::Former;

  // Use attribute debug to print expanded code.
  #[ derive( Debug, PartialEq, Former ) ]
  // Uncomment to see what derive expand into
  // #[ debug ]
  pub struct UserProfile
  {
    age : i32,
    username : String,
    bio_optional : Option< String >, // Fields could be optional
  }

  let profile = UserProfile::former()
  .age( 30 )
  .username( "JohnDoe".to_string() )
  .bio_optional( "Software Developer".to_string() ) // Optionally provide a bio
  .form();

  dbg!( &profile );
  // Expected output:
  // &profile = UserProfile {
  //   age: 30,
  //   username: "JohnDoe",
  //   bio_optional: Some("Software Developer"),
  // }

}

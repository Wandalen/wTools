//#![ feature( proc_macro_totokens ) ] // Enable unstable proc_macro_totokens feature
#![doc(html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png")]
#![doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
)]
#![doc(html_root_url = "https://docs.rs/former_derive_meta/latest/former_derive_meta/")]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[allow(unused_imports)]
use macro_tools::prelude::*;

#[cfg(feature = "derive_former")]
mod derive_former;

/// Derive macro for generating a `Former` struct, applying a Builder Pattern to the annotated struct.
///
/// This macro simplifies the construction of complex objects by automatically generating a builder (former) for
/// the specified struct. It supports extensive customization through attributes that control defaults, setter generation,
/// and field customization, allowing for flexible and fluent object construction.
///
/// # Struct Attributes
///
/// - `debug`: Enables debug mode which can be used to print or log the internal state of the builder for debugging purposes.
/// - `perform`: Specifies a custom method to be invoked automatically at the end of the build process.
/// - `storage_fields`: Specifies fields that should be treated as part of the storage for the former.
/// - `mutator`: Defines a custom mutator class or function to manipulate the data just before the object is finalized.
/// - `standalone_constructors`: Generates top-level constructor functions (e.g., `my_struct()`, `my_variant()`). Return type depends on `arg_for_constructor` (see Option 2 logic in Readme/advanced.md).
///
/// # Field Attributes
///
/// - `former`: General attribute to specify various options like defaults or inclusion in the former.
/// - `scalar`: Indicates that the field is a scalar value, enabling direct assignment without the need for a sub-former. Affects the *associated method* constructor for enum variants.
/// - `collection`: Marks the field as a collection that can use specific former methods to manage its contents.
/// - `subform`: Specifies that the field should utilize a nested former, facilitating the construction of complex nested structures.
/// - `arg_for_constructor`: Marks a field as a required argument for the standalone constructor. Affects constructor signature and return type (see Option 2 logic in Readme/advanced.md).
///
/// # Usage Example
///
/// Below is a typical usage example where the macro is applied to a struct:
///
/// ```rust, ignore
///
/// # #[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
/// # fn main()
/// # {
///   use former::Former;
///
///   // Use attribute debug to print expanded code.
///   #[ derive( Debug, PartialEq, Former ) ]
///   // Uncomment to see what derive expand into
///   // #[ debug ]
///   pub struct UserProfile
///   {
///     age : i32,
///     username : String,
///     bio_optional : Option< String >, // Fields could be optional
///   }
///
///   let profile = UserProfile::former()
///   .age( 30 )
///   .username( "JohnDoe".to_string() )
///   .bio_optional( "Software Developer".to_string() ) // Optionally provide a bio
///   .form();
///
///   dbg!( &profile );
///   // Expected output:
///   // &profile = UserProfile {
///   //   age: 30,
///   //   username: "JohnDoe",
///   //   bio_optional: Some("Software Developer"),
///   // }
///
/// # }
///
/// ```
///
/// This pattern enables fluent and customizable construction of `UserProfile` instances, allowing for easy setting and modification of its fields.
#[cfg(feature = "enabled")]
#[cfg(feature = "derive_former")]
#[
  proc_macro_derive
  (
    Former,
    attributes // This list defines attributes the derive macro processes
    (
      debug, perform, storage_fields, mutator, // struct attributes
      former, scalar, subform_scalar, subform_collection, subform_entry, // field attributes
      // <<< Added the new attributes here >>>
      standalone_constructors, // Add struct-level attribute
      arg_for_constructor      // Add field-level attribute
    )
  )
]
pub fn former(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let result = derive_former::former(input);
  match result {
    Ok(stream) => stream.into(),
    Err(err) => err.to_compile_error().into(),
  }
}

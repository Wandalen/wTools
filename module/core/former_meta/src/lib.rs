//! # Former Meta - Procedural Macro Implementation
//!
//! This crate provides the procedural macro implementation for the Former derive macro.
//! It handles the complex code generation required to implement the builder pattern with
//! advanced features like subforms, collections, and custom validation.
//!
//! ## Architecture Overview
//!
//! The Former meta crate is organized around several key components:
//!
//! ### Core Processing Pipeline
//! 1. **Input Parsing**: Parse derive input and extract struct/enum information
//! 2. **Attribute Processing**: Parse and validate all Former-specific attributes
//! 3. **Type Analysis**: Analyze generic parameters, lifetimes, and field types
//! 4. **Code Generation**: Generate the complete Former ecosystem
//! 5. **Output Assembly**: Combine generated code into final token stream
//!
//! ### Key Modules
//! - [`derive_former`]: Main entry point and orchestration logic
//! - Field attribute processing and validation
//! - Struct attribute parsing and management  
//! - Generic parameter handling for complex scenarios
//! - Code generation for structs and enums
//!
//! ## Supported Constructs
//!
//! ### Struct Support
//! - **Simple Structs**: Basic field-based structures
//! - **Generic Structs**: Complex generic parameters with constraints
//! - **Lifetime Parameters**: Full lifetime parameter support
//! - **Tuple Structs**: Positional field structures
//!
//! ### Enum Support  
//! - **Unit Variants**: Simple enum variants without data
//! - **Tuple Variants**: Variants with positional fields
//! - **Struct Variants**: Variants with named fields
//! - **Mixed Enums**: Enums combining different variant types
//!
//! ## Advanced Features
//!
//! ### Collection Integration
//! - Automatic detection and handling of standard collections
//! - Custom collection support through trait implementations
//! - Specialized builders for Vec, `HashMap`, `HashSet`, etc.
//!
//! ### Subform Support
//! - Nested structure building with full type safety
//! - Automatic trait bound propagation
//! - Context preservation across subform boundaries
//!
//! ### Validation and Mutation
//! - Pre-formation validation through custom mutators
//! - Storage field manipulation before final formation
//! - Custom end handlers for specialized formation logic
//!
//! ## Error Handling and Diagnostics
//!
//! The macro provides comprehensive error reporting:
//! - Clear error messages for attribute misuse
//! - Helpful suggestions for common mistakes
//! - Debug output capabilities for troubleshooting
//! - Integration with Rust's diagnostic system
//!
//! ## Performance Considerations
//!
//! - **Compile-time Generation**: All code generated at compile time
//! - **Minimal Runtime Overhead**: Generated code is highly optimized
//! - **Memory Efficient**: Strategic use of references and zero-cost abstractions
//! - **Lazy Evaluation**: Complex analysis only when needed

//#![ feature( proc_macro_totokens ) ] // Enable unstable proc_macro_totokens feature
#![doc(html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png")]
#![doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
)]
#![doc(html_root_url = "https://docs.rs/former_derive_meta/latest/former_derive_meta/")]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]

#[ allow( unused_imports ) ]
use macro_tools::{Result, diag};

#[ cfg( feature = "derive_former" ) ]
mod derive_former;

/// Derive macro for generating a `Former` struct, applying a Builder Pattern to the annotated struct.
///
/// This macro simplifies the construction of complex objects by automatically generating a builder (former) for
/// the specified struct. It supports extensive customization through attributes that control defaults, setter generation,
/// and field customization, allowing for flexible and fluent object construction.
///
/// # Core Capabilities and Limitations
///
/// ## ✅ Supported Scenarios
/// - **Complex Lifetime Parameters**: Handles `<'a, T>` patterns, multiple lifetimes, and where clauses
/// - **Generic Constraints**: Works with `where T: Hash + Eq`, complex trait bounds
/// - **Nested Structures**: Subform support for complex hierarchical data
/// - **Collection Types**: `HashMap`, Vec, `HashSet` with proper trait bound handling
/// - **Optional Fields**: Automatic `Option< T >` handling with sensible defaults
/// - **Custom Mutators**: Pre-formation data manipulation and validation
///
/// ## ⚠️ Common Pitfalls and Solutions
///
/// ### 1. Commented-Out Derive Attributes (90% of issues)
/// ```rust,ignore
/// // ❌ WRONG: Derive commented out - will appear as "complex" issue
/// // #[ derive( Debug, PartialEq, Former ) ]
/// #[ derive( Debug, PartialEq ) ]
/// pub struct MyStruct { ... }
///
/// // ✅ CORRECT: Uncomment derive attribute
/// #[ derive( Debug, PartialEq, Former ) ]
/// pub struct MyStruct { ... }
/// ```
///
/// ### 2. Feature Gate Requirements for Collections
/// ```rust,ignore
/// // ✅ REQUIRED: Collection tests need proper feature gates
/// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
/// mod test_with_collections;
/// ```
///
/// ### 3. Hash+Eq Trait Bounds for `HashMap` Keys
/// ```rust,ignore
/// // ❌ WRONG: Using non-Hash type as HashMap key
/// pub struct Definition; // No Hash+Eq implementation
/// pub struct MyStruct {
///   map: HashMap<Definition, String>, // Will fail
/// }
///
/// // ✅ CORRECT: Implement required traits or use different key type
/// #[ derive( Hash, Eq, PartialEq ) ]
/// pub struct Definition; // Now implements Hash+Eq
/// ```
///
/// ### 4. Lifetime Parameter Complexity
/// ```rust,ignore
/// // ✅ WORKS: Complex lifetime scenarios are supported
/// #[ derive( Former ) ]
/// pub struct Child<'child, T>
/// where
///   T: 'child + ?Sized,
/// {
///   name: String,
///   data: &'child T,
/// }
/// ```
///
/// ## 📋 Diagnostic Workflow
/// When encountering issues:
/// 1. **Check for commented derives** (resolves 90% of issues)
/// 2. **Verify feature gate configuration** (for collection tests)
/// 3. **Assess trait bound requirements** (Hash+Eq for `HashMap` keys)
/// 4. **Test incremental complexity** (start simple, add complexity gradually)
/// 5. **Enable debug output** (use `#[ debug ]` to see generated code)
/// 6. **Check lifetime parameters** (ensure proper lifetime annotations)
///
/// ### Common Error Patterns and Solutions
///
/// #### E0277: Trait bound not satisfied
/// ```text
/// error[E0277]: the trait bound `MyType: Hash` is not satisfied
/// ```
/// **Solution**: Implement required traits for `HashMap` keys:
/// ```rust,ignore
/// #[ derive( Hash, Eq, PartialEq ) ]
/// struct MyType { /* fields */ }
/// ```
///
/// #### E0106: Missing lifetime specifier
/// ```text
/// error[E0106]: missing lifetime specifier
/// ```
/// **Solution**: Add proper lifetime parameters:
/// ```rust,ignore
/// #[ derive( Former ) ]
/// struct MyStruct<'a> {
///     reference: &'a str,
/// }
/// ```
///
/// #### Commented Derive Issues
/// ```rust,ignore
/// // ❌ WRONG: This will appear as a "complex" compilation error
/// // #[ derive( Debug, PartialEq, Former ) ]
/// #[ derive( Debug, PartialEq ) ]
/// struct MyStruct { field: String }
///
/// // ✅ CORRECT: Uncomment the derive attribute
/// #[ derive( Debug, PartialEq, Former ) ]
/// struct MyStruct { field: String }
/// ```
///
/// #### Collection Feature Gate Issues
/// ```rust,ignore
/// // ✅ REQUIRED: Add feature gates for collection tests
/// #[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
/// mod collection_tests {
///     // HashMap/Vec tests here
/// }
/// ```
///
/// # Struct Attributes
///
/// - `debug`: Enables debug mode which can be used to print or log the internal state of the builder for debugging purposes.
/// - `perform`: Specifies a custom method to be invoked automatically at the end of the build process.
/// - `storage_fields`: Specifies fields that should be treated as part of the storage for the former.
/// - `mutator`: Defines a custom mutator class or function to manipulate the data just before the object is finalized.
/// - `standalone_constructors`: Generates top-level constructor functions (e.g., `my_struct()`, `my_variant()`). Return type depends on `former_ignore` (see Option 2 logic in Readme/advanced.md).
///
/// # Field Attributes
///
/// - `former`: General attribute to specify various options like defaults or inclusion in the former.
/// - `scalar`: Indicates that the field is a scalar value, enabling direct assignment without the need for a sub-former. Affects the *associated method* constructor for enum variants.
/// - `collection`: Marks the field as a collection that can use specific former methods to manage its contents.
/// - `subform`: Specifies that the field should utilize a nested former, facilitating the construction of complex nested structures.
/// - `former_ignore`: Excludes a field from being an argument for the standalone constructor. Affects constructor signature and return type (see Option 2 logic in Readme/advanced.md).
///
/// # Usage Examples
///
/// ## Basic Structure Building
///
/// ```rust,ignore
/// use former::Former;
///
/// #[ derive( Debug, PartialEq, Former ) ]
/// pub struct UserProfile {
///     age: i32,
///     username: String,
///     bio_optional: Option< String >,
/// }
///
/// let profile = UserProfile::former()
///     .age(30)
///     .username("JohnDoe".to_string())
///     .bio_optional("Software Developer".to_string())
///     .form();
/// ```
///
/// ## Collection Handling
///
/// ```rust,ignore
/// use former::Former;
/// use std::collections::HashMap;
///
/// #[ derive( Debug, Former ) ]
/// pub struct Config {
///     #[ collection ]
///     settings: HashMap<String, String>,
///     #[ collection ]
///     tags: Vec< String >,
/// }
///
/// let config = Config::former()
///     .settings().insert("debug", "true").end()
///     .tags().push("production").push("web").end()
///     .form();
/// ```
///
/// ## Complex Generic Scenarios
///
/// ```rust,ignore
/// use former::Former;
///
/// #[ derive( Debug, Former ) ]
/// pub struct Container<'a, T>
/// where
///     T: Clone + 'a,
/// {
///     data: &'a T,
///     metadata: Option< String >,
/// }
///
/// let value = "hello".to_string();
/// let container = Container::former()
///     .data(&value)
///     .metadata("example".to_string())
///     .form();
/// ```
///
/// ## Custom Validation with Mutators
///
/// ```rust,ignore
/// use former::Former;
///
/// #[ derive( Debug, Former ) ]
/// #[ mutator( custom ) ]
/// pub struct ValidatedStruct {
///     min_value: i32,
///     max_value: i32,
/// }
///
/// // Custom mutator implementation
/// impl FormerMutator for ValidatedStructDefinitionTypes {
///     fn form_mutation(storage: &mut Self::Storage, _context: &mut Option< Self::Context >) {
///         if let (Some(min), Some(max)) = (&storage.min_value, &storage.max_value) {
///             if min > max {
///                 std::mem::swap(&mut storage.min_value, &mut storage.max_value);
///             }
///         }
///     }
/// }
/// ```
///
/// ## Debugging Generated Code
///
/// The Former derive macro provides comprehensive debugging capabilities through the `#[ debug ]` attribute,
/// following the design principle that "Proc Macros: Must Implement a 'debug' Attribute".
///
/// ### Debug Attribute Usage
///
/// ```rust,ignore
/// use former::Former;
///
/// // Standalone debug attribute
/// #[ derive( Debug, PartialEq, Former ) ]
/// #[ debug ]  // <-- Enables comprehensive debug output
/// pub struct Person {
///     name: String,
///     age: u32,
///     email: Option< String >,
/// }
///
/// // Within #[ former( ... ) ] container
/// #[ derive( Debug, PartialEq, Former ) ]
/// #[ former( debug, standalone_constructors ) ]  // <-- Debug with other attributes
/// pub struct Config {
///     host: String,
///     port: u16,
/// }
/// ```
///
/// ### Comprehensive Debug Information
///
/// When `#[ debug ]` is present and the `former_diagnostics_print_generated` feature is enabled,
/// the macro provides detailed information in four phases:
///
/// #### Phase 1: Input Analysis
/// - **Target Type Information**: Name, kind (struct/enum), visibility
/// - **Generic Parameters Analysis**: Lifetimes, type parameters, const parameters, where clauses
/// - **Field/Variant Analysis**: Field names, types, visibility for structs; variant information for enums
/// - **Attribute Configuration**: All parsed Former attributes, storage fields, mutator settings
///
/// #### Phase 2: Generic Classification
/// - **Classification Results**: How generics are categorized (lifetime-only, type-only, mixed, empty)
/// - **Generated Generic Components**: `impl_generics`, `ty_generics`, `where_clause` breakdown
/// - **Strategy Explanation**: Why certain generation strategies were chosen
///
/// #### Phase 3: Generated Components Analysis
/// - **Core Components**: `FormerStorage`, `FormerDefinition`, `FormerDefinitionTypes`, Former struct
/// - **Trait Implementations**: `EntityToStorage`, `EntityToFormer`, `EntityToDefinition`, etc.
/// - **Formation Process**: Step-by-step formation workflow explanation
/// - **Customizations**: How attributes affect the generated code structure
///
/// #### Phase 4: Complete Generated Code
/// - **Final `TokenStream`**: The complete code that will be compiled
/// - **Integration Points**: How generated code integrates with existing types
///
/// ### Enabling Debug Output
///
/// ```bash
/// # See debug information during compilation
/// cargo build --features former_diagnostics_print_generated
///
/// # For examples
/// cargo run --example former_debug --features former_diagnostics_print_generated
///
/// # For tests with debug output
/// cargo test --features former_diagnostics_print_generated
/// ```
///
/// ### Debug Use Cases
///
/// The debug attribute is particularly useful for:
///
/// 1. **Understanding Macro Behavior**: See exactly how the macro processes your struct/enum definition
/// 2. **Debugging Complex Scenarios**: Troubleshoot generic parameters, lifetime issues, trait bound problems
/// 3. **Learning Former Pattern**: Understand the complete ecosystem generated for your types
/// 4. **Verifying Configuration**: Confirm that attributes are parsed correctly and generate expected code
/// 5. **Performance Analysis**: Understand the complexity of generated code for optimization
///
/// ### Integration with Development Workflow
///
/// The debug system integrates seamlessly with existing development tools:
/// - **Zero Runtime Cost**: Debug analysis only runs during compilation
/// - **Conditional Compilation**: Debug code only included with feature flag
/// - **IDE Integration**: Debug output appears in compiler output and can be captured by IDEs
/// - **CI/CD Friendly**: Can be enabled in build pipelines for automated analysis
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_former" ) ]
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
      former_ignore,           // Add field-level attribute
      arg_for_constructor      // Add field-level attribute for constructor inclusion
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

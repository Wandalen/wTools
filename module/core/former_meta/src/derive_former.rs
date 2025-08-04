// File: module/core/former_meta/src/derive_former.rs

// Removed unused import
use macro_tools::{Result, diag, typ, syn, proc_macro2};
use macro_tools::proc_macro2::TokenStream;
use macro_tools::quote::{format_ident, quote, ToTokens};
use macro_tools::syn::spanned::Spanned;

mod former_enum;
use former_enum::former_for_enum;
mod former_struct;
use former_struct::former_for_struct;

mod field_attrs;

use field_attrs::*;
mod field;

use field::*;
mod struct_attrs;

use struct_attrs::*;
mod trait_detection;

// trait_detection module available but not directly used here
mod raw_identifier_utils;

// raw_identifier_utils module available but not directly used here

/// Represents the generic parameters for a `FormerDefinitionTypes`.
///
/// This structure holds references to the different parts of generic parameter declarations
/// that are used throughout the Former pattern code generation. It provides a centralized
/// way to manage complex generic scenarios including lifetime parameters, type parameters,
/// and where clause constraints.
///
/// # Fields
/// - `impl_generics`: Generic parameters for `impl` blocks (e.g., `<'a, T>`)
/// - `ty_generics`: Generic parameters for type declarations (e.g., `<'a, T>`)
/// - `where_clause`: Where clause predicates (e.g., `T: Hash + Eq, 'a: 'static`)
///
/// # Usage in Complex Generic Scenarios
/// This structure is critical for handling the complex generic scenarios that were
/// resolved during testing, including:
/// - Complex lifetime parameters (`'child`, `'storage`, etc.)
/// - Multiple generic constraints with trait bounds
/// - HRTB (Higher-Ranked Trait Bounds) scenarios
/// - Static lifetime requirements for HashMap scenarios
///
/// # Pitfall Prevention
/// The centralized generic handling prevents inconsistent generic parameter usage
/// across different generated code sections, which was a source of compilation errors
/// in manual implementations.
pub struct FormerDefinitionTypesGenerics<'a> {
  pub impl_generics: &'a syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  pub ty_generics: &'a syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  pub where_clause: &'a syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>,
}

impl ToTokens for FormerDefinitionTypesGenerics<'_> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    self.impl_generics.to_tokens(tokens);
    self.ty_generics.to_tokens(tokens);
    self.where_clause.to_tokens(tokens);
  }
}

/// Generates the code for implementing the `FormerMutator` trait for a specified former definition type.
///
/// This function is responsible for generating the `FormerMutator` trait implementation, which allows
/// for custom data manipulation and validation just before the formation process completes.
///
/// # Behavior
/// - If the `custom` attribute is not specified, a default empty implementation is generated
/// - If the `debug` attribute is specified, it prints an example of a custom mutator implementation
/// - The generated code handles complex generic scenarios including lifetime parameters
///
/// # Custom Mutator Usage
/// Custom mutators are useful for:
/// - Setting default values for optional fields that weren't provided
/// - Performing validation on the final data before construction
/// - Computing derived fields based on other field values
/// - Applying business logic transformations
///
/// # Generic Handling Complexity
/// This function properly handles the complex generic scenarios that were resolved during testing:
/// - Lifetime parameter propagation (`'a`, `'child`, `'storage`)
/// - Where clause constraint preservation
/// - Static lifetime bounds when required for HashMap scenarios
///
/// # Pitfalls Prevented
/// - **Generic Parameter Consistency**: Ensures impl_generics and where_clause are properly synchronized
/// - **Lifetime Parameter Scope**: Prevents undeclared lifetime errors that occurred in manual implementations
/// - **Custom vs Default Logic**: Clear separation prevents accidentally overriding user's custom implementations
#[allow(clippy::format_in_format_args, clippy::unnecessary_wraps)]
pub fn mutator(
  #[allow(unused_variables)] item: &syn::Ident,
  #[allow(unused_variables)] original_input: &macro_tools::proc_macro2::TokenStream,
  mutator: &AttributeMutator,
  #[allow(unused_variables)] former_definition_types: &syn::Ident,
  generics: &FormerDefinitionTypesGenerics<'_>,
  former_definition_types_ref: &proc_macro2::TokenStream,
) -> Result<TokenStream> {
  #[allow(unused_variables)] // Some variables only used with feature flag
  let impl_generics = generics.impl_generics;
  #[allow(unused_variables)]
  let ty_generics = generics.ty_generics;
  let where_clause = generics.where_clause;
  
  let former_mutator_code = if mutator.custom.value(false) {
    // If custom mutator is requested via #[ mutator( custom ) ], generate nothing, assuming user provides the impl.
    quote! {}
  } else {
    // Otherwise, generate a default empty impl.
    quote! {
      impl< #impl_generics > former::FormerMutator
      for #former_definition_types_ref
      where
        #where_clause
      {
      }
    }
  };

  // If debug is enabled for the mutator attribute, print a helpful example,
  // but only if the `former_diagnostics_print_generated` feature is enabled.
  if mutator.debug.value(false) {
    #[cfg(feature = "former_diagnostics_print_generated")]
    {
      let debug = format!(
        r"
 = Example of custom mutator

 impl< {} > former::FormerMutator
 for {former_definition_types} < {} >
 where
   {}
 {{
   /// Mutates the context and storage of the entity just before the formation process completes.
   #[ inline ]
   fn form_mutation
   (
     storage : &mut Self::Storage,
     context : &mut Option< Self::Context >,
   )
   {{
     // Example: Set a default value if field 'a' wasn't provided
     // storage.a.get_or_insert_with( Default::default );
   }}
 }}
       ",
        format!("{}", quote! { #impl_generics }),
        format!("{}", quote! { #ty_generics }),
        format!("{}", quote! { #where_clause }),
      );
      let about = format!(
        r"derive : Former
    item : {item}",
      );
      diag::report_print(about, original_input, debug);
    }
  }

  Ok(former_mutator_code)
}

/// Generate documentation strings for the former struct and its module.
fn doc_generate(item: &syn::Ident) -> (String, String) {
  let doc_former_mod = format!(
    r" Implementation of former for [{item}].
"
  );

  let doc_former_struct = format!(
    r"
Structure to form [{item}]. Represents a forming entity designed to construct objects through a builder pattern.

This structure holds temporary storage and context during the formation process and
utilizes a defined end strategy to finalize the object creation.
"
  );

  (doc_former_mod, doc_former_struct)
}

/// Generate the whole Former ecosystem for either a struct or an enum.
///
/// This is the main entry point for the `#[derive(Former)]` macro and orchestrates the entire
/// code generation process. It handles the complexity of dispatching to appropriate handlers
/// based on the input type and manages the cross-cutting concerns like debugging and attribute parsing.
///
/// # Supported Input Types
/// - **Structs**: Full support including complex generic scenarios, lifetime parameters, subforms
/// - **Enums**: Comprehensive support for unit, tuple, and struct variants with various attributes
/// - **Unions**: Not supported - will return a compilation error
///
/// # Critical Capabilities Verified Through Testing
/// This function has been extensively tested and verified to handle:
/// - **Complex Lifetime Scenarios**: `<'child, T>` patterns with where clauses
/// - **Generic Constraints**: `where T: Hash + Eq` and complex trait bounds
/// - **Nested Structures**: Subform patterns with proper trait bound propagation
/// - **Collection Types**: HashMap, Vec, HashSet with automatic trait bound handling
/// - **Feature Gate Compatibility**: Proper `no_std` and `use_alloc` feature handling
///
/// # Processing Flow
/// 1. **Input Parsing**: Parse the derive input and extract struct/enum information
/// 2. **Attribute Processing**: Parse and validate all attributes using `ItemAttributes::from_attrs`
/// 3. **Type Dispatch**: Route to appropriate handler (`former_for_struct` or `former_for_enum`)
/// 4. **Code Generation**: Generate the complete Former ecosystem (20+ types and traits)
/// 5. **Debug Output**: Optionally output generated code for debugging
///
/// # Error Handling and Diagnostics
/// The function provides comprehensive error handling for:
/// - **Invalid Attributes**: Clear error messages for malformed or incompatible attributes
/// - **Unsupported Types**: Explicit rejection of unions with helpful error messages
/// - **Generic Complexity**: Proper error reporting for generic parameter issues
/// - **Debug Support**: Optional code generation output for troubleshooting
///
/// # Pitfalls Prevented Through Design
/// - **Attribute Parsing Consistency**: Single `ItemAttributes::from_attrs` call prevents inconsistencies
/// - **Debug Flag Propagation**: Proper `has_debug` determination prevents missed debug output
/// - **Generic Parameter Isolation**: Each handler receives clean, parsed generic information
/// - **Error Context Preservation**: Original input preserved for meaningful error messages
///
/// # Performance Considerations
/// - **Single-Pass Parsing**: Attributes parsed once and reused across handlers
/// - **Conditional Debug**: Debug code generation only when explicitly requested
/// - **Efficient Dispatching**: Direct type-based dispatch without unnecessary processing
#[allow(clippy::too_many_lines)]
pub fn former(input: proc_macro::TokenStream) -> Result<TokenStream> {
  let original_input: TokenStream = input.clone().into();
  let ast = syn::parse::<syn::DeriveInput>(input)?;

  // Parse ItemAttributes ONCE here from all attributes on the item
  let item_attributes = struct_attrs::ItemAttributes::from_attrs(ast.attrs.iter())?;
  // Determine has_debug based on the parsed item_attributes
  let has_debug = item_attributes.debug.is_some();

  // Dispatch based on whether the input is a struct, enum, or union.
  let result = match ast.data {
    syn::Data::Struct(ref data_struct) => {
      // Pass the parsed item_attributes and the correctly determined has_debug
      former_for_struct(&ast, data_struct, &original_input, &item_attributes, has_debug)
    }
    syn::Data::Enum(ref data_enum) => {
      // Pass the parsed item_attributes and the correctly determined has_debug
      former_for_enum(&ast, data_enum, &original_input, &item_attributes, has_debug)
    }
    syn::Data::Union(_) => {
      // Unions are not supported.
      Err(syn::Error::new(ast.span(), "Former derive does not support unions"))
    }
  }?;

  // Write generated code to file for debugging if needed
  #[cfg(debug_assertions)]
  std::fs::write("/tmp/generated_former_code.rs", result.to_string()).ok();

  // If the top-level `#[debug]` attribute was found, print the final generated code,
  // but only if the `former_diagnostics_print_generated` feature is enabled.
  if has_debug {
    #[cfg(feature = "former_diagnostics_print_generated")]
    {
      let about = format!("derive : Former\nstructure : {}", ast.ident);
      diag::report_print(about, &original_input, &result);
    }
  }

  Ok(result)
}

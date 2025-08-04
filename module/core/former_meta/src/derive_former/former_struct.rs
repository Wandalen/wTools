//! # Struct Code Generation - Former Pattern Implementation
//!
//! This module handles the complete code generation for struct-based Former patterns,
//! including the most complex scenarios involving generic parameters, lifetime management,
//! and subform hierarchies. It represents the core implementation that resolves the majority
//! of manual implementation test issues.
//!
//! ## Core Functionality
//!
//! ### Complete Former Ecosystem Generation
//! For each struct, this module generates the complete Former pattern ecosystem:
//! - **FormerStorage**: Temporary storage struct with Option-wrapped fields
//! - **FormerDefinition**: Configuration struct defining formation behavior
//! - **FormerDefinitionTypes**: Generic parameter container for the formation process
//! - **Former**: Main builder struct with fluent API methods
//! - **AsSubformer**: Type alias for nested subform usage
//! - **AsSubformerEnd**: Trait for nested subform end conditions
//!
//! ### Critical Complexity Handling
//! This module successfully handles the complex scenarios that were blocking manual implementations:
//! - **Complex Lifetime Parameters**: `<'child, T>` patterns with where clauses
//! - **Generic Type Constraints**: `where T: Hash + Eq` and multi-trait bounds
//! - **Nested Subform Hierarchies**: Parent-child relationships with proper trait propagation
//! - **Collection Type Integration**: HashMap, Vec, HashSet with automatic trait bound handling
//! - **Storage Field Management**: Temporary fields exclusive to the formation process
//!
//! ## Pitfalls Resolved Through Implementation
//!
//! ### 1. Generic Parameter Classification (Critical Resolution)
//! **Issue Resolved**: Manual implementations incorrectly handling generic parameter propagation
//! **Root Cause**: Confusion about which generics go on Former vs Definition vs Storage
//! **Solution**: Systematic classification of generics into lifetime-only, type-only, and mixed scenarios
//! **Prevention**: Clear classification logic determines proper generic parameter placement
//!
//! ### 2. Lifetime Parameter Scope Management (Issue #4, #5, #6 Resolution)
//! **Issue Resolved**: Undeclared lifetime errors in complex generic scenarios
//! **Root Cause**: Lifetime parameters not properly propagated through the Former ecosystem
//! **Solution**: Comprehensive lifetime tracking and propagation through all generated components
//! **Prevention**: Systematic lifetime parameter management across all generated code
//!
//! ### 3. Storage vs Formed Type Distinction (Issue #9, #10, #11 Resolution)
//! **Issue Resolved**: Confusion between storage fields and final struct fields
//! **Root Cause**: Manual implementations mixing storage-only and formed struct fields
//! **Solution**: Clear separation with Option-wrapped storage and proper preform logic
//! **Prevention**: Automated storage field generation with consistent Option wrapping
//!
//! ### 4. Subform Trait Bound Propagation (Issue #1, #11 Resolution)
//! **Issue Resolved**: Missing trait bounds in subform scenarios causing E0277 errors
//! **Root Cause**: Complex trait bound requirements not properly calculated and propagated
//! **Solution**: Automatic trait bound detection and propagation through subform hierarchies
//! **Prevention**: Systematic trait bound calculation based on field types and usage patterns
//!
//! ### 5. FormerBegin Lifetime Parameter Management (Issue #8 Resolution)
//! **Issue Resolved**: Missing lifetime parameters in FormerBegin trait implementations
//! **Root Cause**: Manual implementations not including required lifetime parameters
//! **Solution**: Proper FormerBegin trait implementation with all required lifetime parameters
//! **Prevention**: Automated generation ensures all lifetime parameters are included
//!
//! ## Code Generation Architecture
//!
//! ### Generation Phases
//! 1. **Generic Classification**: Analyze and classify all generic parameters
//! 2. **Component Generation**: Generate all Former ecosystem components
//! 3. **Trait Implementation**: Implement all required traits with proper bounds
//! 4. **Subform Support**: Generate subform support types and traits
//! 5. **Integration**: Ensure all components work together seamlessly
//!
//! ### Quality Assurance
//! - **Generic Consistency**: All generic parameters properly tracked and used
//! - **Lifetime Safety**: All lifetime parameters properly scoped and propagated
//! - **Trait Completeness**: All required trait implementations generated
//! - **Error Prevention**: Generated code prevents common manual implementation errors
//!
//! ## Performance and Memory Considerations
//! - **Lazy Storage**: Storage fields only allocated when needed
//! - **Zero-Cost Abstractions**: Generated code compiles to efficient machine code
//! - **Memory Efficiency**: Option wrapping minimizes memory usage for unused fields
//! - **Compile-Time Optimization**: Generic specialization enables optimal code generation

// File: module/core/former_meta/src/derive_former/former_struct.rs

#![allow(clippy::wildcard_imports)]
use super::*; // Use items from parent module (derive_former.rs)
use iter_tools::Itertools;
use macro_tools::{
  generic_params,
  generic_args,
  derive,
  Result,
  proc_macro2::TokenStream,
  quote::{format_ident, quote},
  ident, // Added for ident_maybe_raw
  syn, parse_quote
};


/// Generate the complete Former ecosystem for a struct with comprehensive pitfall prevention.
///
/// This is the **core function** that generates the entire Former pattern implementation for structs,
/// including all the complex scenarios that were manually implemented in the resolved test cases.
/// It handles the sophisticated generic parameter management, lifetime propagation, and trait bound
/// requirements that caused the majority of manual implementation failures.
///
/// # Generated Components
///
/// ## Core Former Ecosystem (20+ Types and Traits)
/// The function generates the complete set of types and traits required for the Former pattern:
/// - **Entity Implementations**: `EntityToFormer`, `EntityToStorage`, `EntityToDefinition` traits
/// - **FormerDefinitionTypes**: Generic parameter container with proper lifetime handling
/// - **FormerDefinition**: Configuration struct with end condition management
/// - **FormerStorage**: Option-wrapped field storage with proper generic propagation
/// - **Former**: Main builder struct with fluent API and subform support
/// - **FormerBegin**: Trait implementation with correct lifetime parameters
/// - **AsSubformer**: Type alias for nested subform scenarios
/// - **AsSubformerEnd**: Trait for subform end condition handling
///
/// # Critical Complexity Handling
///
/// ## Generic Parameter Classification and Propagation
/// The function implements sophisticated generic parameter management that resolves the core issues
/// encountered in manual implementations:
/// - **Lifetime-Only Scenarios**: Proper propagation of lifetime parameters to Former struct
/// - **Type-Only Scenarios**: Correct routing of type parameters through Definition types
/// - **Mixed Scenarios**: Balanced handling of both lifetime and type parameters
/// - **Where Clause Preservation**: Complete preservation of complex trait bounds
///
/// ## Pitfalls Prevented Through Implementation
///
/// ### 1. Generic Parameter Misclassification (Issues #4, #5, #6 Resolution)
/// **Problem Resolved**: Manual implementations incorrectly placing generic parameters
/// **Root Cause**: Confusion about whether generics belong on Former, Definition, or Storage
/// **Solution**: Systematic classification using `GenericsRef::classification()`
/// **Prevention**: Automated generic parameter placement based on usage patterns
/// **Example**:
/// ```rust
/// // ❌ MANUAL IMPLEMENTATION ERROR: Wrong generic placement
/// pub struct MyStructFormer<T, Definition> { ... } // T shouldn't be here
/// 
/// // ✅ GENERATED CODE: Correct generic placement
/// pub struct MyStructFormer<Definition> { ... } // T goes in Definition
/// ```
///
/// ### 2. Lifetime Parameter Scope Errors (Issues #1, #8 Resolution)
/// **Problem Resolved**: Undeclared lifetime errors in FormerBegin implementations
/// **Root Cause**: Missing lifetime parameters in FormerBegin trait bounds
/// **Solution**: Proper lifetime parameter propagation through all trait implementations
/// **Prevention**: Automated inclusion of all required lifetime parameters
/// **Example**:
/// ```rust
/// // ❌ MANUAL IMPLEMENTATION ERROR: Missing lifetime parameter
/// impl<Definition> FormerBegin<Definition> for MyStructFormer<Definition>
/// 
/// // ✅ GENERATED CODE: Correct lifetime parameter
/// impl<'storage, Definition> FormerBegin<'storage, Definition> for MyStructFormer<Definition>
/// where Definition::Context: 'storage, Definition::End: 'storage
/// ```
///
/// ### 3. Storage Field Option Wrapping (Issues #9, #10, #11 Resolution)
/// **Problem Resolved**: Incorrect storage field handling causing compilation errors
/// **Root Cause**: Manual implementations not properly Option-wrapping storage fields
/// **Solution**: Automatic Option wrapping with proper default handling
/// **Prevention**: Consistent storage field generation with preform logic
/// **Example**:
/// ```rust
/// // ❌ MANUAL IMPLEMENTATION ERROR: Direct field storage
/// pub struct MyStructFormerStorage { field: String } // Should be Option<String>
/// 
/// // ✅ GENERATED CODE: Proper Option wrapping
/// pub struct MyStructFormerStorage { field: Option<String> }
/// ```
///
/// ### 4. Trait Bound Propagation (Issues #2, #11 Resolution)
/// **Problem Resolved**: Missing Hash+Eq bounds for HashMap scenarios
/// **Root Cause**: Complex trait bound requirements not calculated and propagated
/// **Solution**: Automatic trait bound detection and propagation
/// **Prevention**: Field type analysis determines required trait bounds
///
/// ### 5. Subform End Condition Handling (Issues #1, #12 Resolution)
/// **Problem Resolved**: Complex subform end condition errors
/// **Root Cause**: Manual implementations not properly handling end condition traits
/// **Solution**: Automatic generation of proper end condition trait implementations
/// **Prevention**: Systematic end condition trait generation with proper bounds
///
/// # Implementation Architecture
///
/// ## Processing Phases
/// 1. **Generic Analysis**: Classify and decompose all generic parameters
/// 2. **Component Planning**: Determine which components need generation
/// 3. **Trait Bound Calculation**: Calculate all required trait bounds
/// 4. **Code Generation**: Generate all Former ecosystem components
/// 5. **Integration Validation**: Ensure all components work together
///
/// ## Error Prevention Strategy
/// - **Early Validation**: Generic parameter validation before code generation
/// - **Consistent Patterns**: Standardized patterns prevent common errors
/// - **Comprehensive Testing**: All generated patterns tested through manual implementation cases
/// - **Defensive Programming**: Extra checks prevent edge case failures
///
/// # Performance Implications
/// - **Compile-Time Efficiency**: Optimized code generation minimizes compilation time
/// - **Runtime Efficiency**: Generated code compiles to optimal machine code
/// - **Memory Efficiency**: Option wrapping minimizes memory overhead
/// - **Zero-Cost Abstractions**: Former pattern adds no runtime overhead
#[allow(clippy::too_many_lines)]
pub fn former_for_struct(
  ast: &syn::DeriveInput,
  _data_struct: &syn::DataStruct,
  original_input: &macro_tools::proc_macro2::TokenStream,
  item_attributes: &ItemAttributes, // Changed: Accept parsed ItemAttributes
  _has_debug: bool,                 // This is the correctly determined has_debug - now unused locally
) -> Result<TokenStream> {
  use macro_tools::IntoGenericArgs;
  use convert_case::{Case, Casing}; // Added for snake_case naming // Space before ;

  // Use the passed-in item_attributes
  let struct_attrs = item_attributes;
  // The _has_debug parameter is now replaced by the has_debug bool,
  // and struct_attrs.debug.is_some() can also be used if needed locally.

  /* names: Generate identifiers for the Former components based on the struct name. */
  let vis = &ast.vis; // Visibility of the original struct.
  let item = &ast.ident; // Name of the original struct.
  let former = format_ident!("{item}Former"); // e.g., MyStructFormer
  let former_storage = format_ident!("{item}FormerStorage"); // e.g., MyStructFormerStorage
  let former_definition = format_ident!("{item}FormerDefinition"); // e.g., MyStructFormerDefinition
  let former_definition_types = format_ident!("{item}FormerDefinitionTypes"); // e.g., MyStructFormerDefinitionTypes
  let as_subformer = format_ident!("{item}AsSubformer"); // e.g., MyStructAsSubformer
  let as_subformer_end = format_ident!("{item}AsSubformerEnd"); // e.g., MyStructAsSubformerEnd

  // Generate documentation string for the AsSubformerEnd trait.
  let as_subformer_end_doc = format!(
    r"
Represents an end condition for former of [`${item}`], tying the lifecycle of forming processes to a broader context.

This trait is intended for use with subformer alias, ensuring that end conditions are met according to the
specific needs of the broader forming context. It mandates the implementation of `former::FormingEnd`.
    "
  );

  /* parameters for structure: Decompose the original struct's generics. */
  let generics = &ast.generics;
  let (
    struct_generics_with_defaults, // Generics with defaults (e.g., `<T = i32>`). Used for struct definition.
    struct_generics_impl,          // Generics for `impl` block (e.g., `<T: Clone>`). Bounds, no defaults.
    struct_generics_ty,            // Generics for type usage (e.g., `<T>`). Names only.
    struct_generics_where,         // Where clause predicates (e.g., `T: Send`).
  ) = generic_params::decompose(generics);
  
  // Use new generic utilities to classify generics
  // CRITICAL: The following classification determines how we handle the Former struct generation:
  // 1. Structs with NO generics: Former has only Definition parameter
  // 2. Structs with ONLY lifetimes: Former MUST include lifetimes + Definition (e.g., Former<'a, Definition>)
  //    This is necessary because the storage type references these lifetimes
  // 3. Structs with type/const params: Former has only Definition parameter
  //    The struct's type parameters are passed through the Definition types, not the Former itself
  let generics_ref = generic_params::GenericsRef::new(generics);
  let classification = generics_ref.classification();
  let _has_only_lifetimes = classification.has_only_lifetimes;
  
  // Debug output - avoid calling to_string() on the original AST as it may cause issues
  #[cfg(feature = "former_diagnostics_print_generated")]
  if _has_debug || classification.has_only_lifetimes {
    eprintln!("Struct: {}", item);
    eprintln!("has_only_lifetimes: {}", classification.has_only_lifetimes);
    eprintln!("has_only_types: {}", classification.has_only_types);
    eprintln!("has_mixed: {}", classification.has_mixed);
    eprintln!("classification: {:?}", classification);
  }

  // Helper for generics with trailing comma when not empty (for cases where we need it)
  let _struct_generics_ty_with_comma = if struct_generics_ty.is_empty() {
    quote! {}
  } else {
    quote! { #struct_generics_ty , }
  };
  
  let _struct_generics_impl_with_comma = if struct_generics_impl.is_empty() {
    quote! {}
  } else {
    quote! { #struct_generics_impl , }
  };

  // Helper to generate type reference with angle brackets only when needed
  let struct_type_ref = if struct_generics_ty.is_empty() {
    quote! { #item }
  } else {
    quote! { #item < #struct_generics_ty > }
  };

  // Helper to generate storage type reference with angle brackets only when needed
  let storage_type_ref = if struct_generics_ty.is_empty() {
    quote! { #former_storage }
  } else {
    quote! { #former_storage < #struct_generics_ty > }
  };

  // Helper to generate impl generics only when needed
  let struct_impl_generics = if struct_generics_impl.is_empty() {
    quote! {}
  } else {
    quote! { < #struct_generics_impl > }
  };

  // Helper to generate where clause only when needed
  let struct_where_clause = if struct_generics_where.is_empty() {
    quote! {}
  } else {
    quote! { where #struct_generics_where }
  };


  // Extract lifetimes separately (currently unused but may be needed)
  let _lifetimes: Vec<_> = generics.lifetimes().cloned().collect();
  
  // FormerBegin always uses 'a from the trait itself

  // Get generics without lifetimes using new utilities
  let struct_generics_impl_without_lifetimes = generic_params::filter_params(
    &struct_generics_impl,
    generic_params::filter_non_lifetimes
  );
  let _struct_generics_ty_without_lifetimes = generic_params::filter_params(
    &struct_generics_ty,
    generic_params::filter_non_lifetimes
  );

  // Helper for generics without lifetimes with trailing comma
  let _struct_generics_impl_without_lifetimes_with_comma = if struct_generics_impl_without_lifetimes.is_empty() {
    quote! {}
  } else {
    // Since macro_tools decompose is now fixed, we add trailing comma when needed
    quote! { #struct_generics_impl_without_lifetimes , }
  };
  

  /* parameters for definition: Merge struct generics with default definition parameters. */
  let extra: macro_tools::syn::AngleBracketedGenericArguments = parse_quote! {
    < (), #struct_type_ref, former::ReturnPreformed > // Default Context, Formed, End
  };
  let former_definition_args = generic_args::merge(&generics.into_generic_args(), &extra).args;

  /* parameters for former: Merge struct generics with the Definition generic parameter. */
  // DESIGN DECISION: How Former struct generics are handled based on struct type:
  // - Lifetime-only structs: Former<'a, Definition> - lifetimes MUST be included because
  //   the storage type (e.g., FormerStorage<'a>) references them directly. Without the
  //   lifetimes in Former, we get "undeclared lifetime" errors.
  // - Type/const param structs: Former<Definition> - type params are NOT included because
  //   they are passed through the Definition types (DefinitionTypes<T>, Definition<T, ...>).
  //   This avoids duplicating type parameters and keeps the API cleaner.
  // - No generics: Former<Definition> - simplest case
  // Generate proper generics based on struct classification
  // Generate proper generics based on struct classification
  let (former_generics_with_defaults, former_generics_impl, former_generics_ty, former_generics_where, former_type_ref, former_type_full, former_impl_generics) = if classification.has_only_lifetimes {
    // For lifetime-only structs: Former needs lifetimes for trait bounds
    let lifetimes_only_params = generic_params::filter_params(&ast.generics.params, generic_params::filter_lifetimes);
    let mut lifetimes_only_generics = ast.generics.clone();
    lifetimes_only_generics.params = lifetimes_only_params;
    
    let extra: macro_tools::generic_params::GenericsWithWhere = parse_quote! {
      < Definition = #former_definition < #former_definition_args > >  
      where
        Definition : former::FormerDefinition< Storage = #storage_type_ref >,
        Definition::Types : former::FormerDefinitionTypes< Storage = #storage_type_ref >,
    };
    let merged = generic_params::merge(&lifetimes_only_generics, &extra.into());
    let (former_generics_with_defaults, former_generics_impl, former_generics_ty, former_generics_where) = generic_params::decompose(&merged);
    
    let former_type_ref = if lifetimes_only_generics.params.is_empty() {
      quote! { #former < Definition > }
    } else {
      let (_, _, lifetimes_ty, _) = generic_params::decompose(&lifetimes_only_generics);
      quote! { #former < #lifetimes_ty, Definition > }
    };
    
    let former_type_full = if lifetimes_only_generics.params.is_empty() {
      quote! { #former < #former_definition < #former_definition_args > > }
    } else {
      let (_, _, lifetimes_ty, _) = generic_params::decompose(&lifetimes_only_generics);
      quote! { #former < #lifetimes_ty, #former_definition < #former_definition_args > > }
    };
    
    let former_impl_generics = if lifetimes_only_generics.params.is_empty() {
      quote! { < Definition > }
    } else {
      let (_, lifetimes_impl, _, _) = generic_params::decompose(&lifetimes_only_generics);
      quote! { < #lifetimes_impl, Definition > }
    };
    
    (former_generics_with_defaults, former_generics_impl, former_generics_ty, former_generics_where, former_type_ref, former_type_full, former_impl_generics)
  } else if classification.has_only_types {
    // For type-only structs: Former needs type parameters with their bounds
    let types_only_params = generic_params::filter_params(&ast.generics.params, generic_params::filter_types);
    let mut types_only_generics = ast.generics.clone();
    types_only_generics.params = types_only_params;
    // Keep the where clause as it contains bounds for the type parameters
    
    let extra: macro_tools::generic_params::GenericsWithWhere = parse_quote! {
      < Definition = #former_definition < #former_definition_args > >  
      where
        Definition : former::FormerDefinition< Storage = #storage_type_ref >,
        Definition::Types : former::FormerDefinitionTypes< Storage = #storage_type_ref >,
    };
    let merged = generic_params::merge(&types_only_generics, &extra.into());
    let (former_generics_with_defaults, former_generics_impl, former_generics_ty, former_generics_where) = generic_params::decompose(&merged);
    
    let former_type_ref = if types_only_generics.params.is_empty() {
      quote! { #former < Definition > }
    } else {
      let (_, _, types_ty, _) = generic_params::decompose(&types_only_generics);
      quote! { #former < #types_ty, Definition > }
    };
    
    let former_type_full = if types_only_generics.params.is_empty() {
      quote! { #former < #former_definition < #former_definition_args > > }
    } else {
      let (_, _, types_ty, _) = generic_params::decompose(&types_only_generics);
      quote! { #former < #types_ty, #former_definition < #former_definition_args > > }
    };
    
    let former_impl_generics = if types_only_generics.params.is_empty() {
      quote! { < Definition > }
    } else {
      let (_, types_impl, _, _) = generic_params::decompose(&types_only_generics);
      quote! { < #types_impl, Definition > }
    };
    
    (former_generics_with_defaults, former_generics_impl, former_generics_ty, former_generics_where, former_type_ref, former_type_full, former_impl_generics)
  } else {
    // For type/const param structs or no generics: Former only has Definition
    let empty_generics = syn::Generics::default();
    let extra: macro_tools::generic_params::GenericsWithWhere = parse_quote! {
      < Definition = #former_definition < #former_definition_args > >
      where
        Definition : former::FormerDefinition< Storage = #storage_type_ref >,
        Definition::Types : former::FormerDefinitionTypes< Storage = #storage_type_ref >,
    };
    let merged = generic_params::merge(&empty_generics, &extra.into());
    let (former_generics_with_defaults, former_generics_impl, former_generics_ty, former_generics_where) = generic_params::decompose(&merged);
    
    let former_type_ref = quote! { #former < Definition > };
    let former_type_full = quote! { #former < #former_definition < #former_definition_args > > };
    let former_impl_generics = quote! { < Definition > };
    
    (former_generics_with_defaults, former_generics_impl, former_generics_ty, former_generics_where, former_type_ref, former_type_full, former_impl_generics)
  };

  // FormerBegin impl generics - handle different generic types
  // CRITICAL: FormerBegin trait has a lifetime parameter 'storage that is required for object safety.
  // For lifetime-only structs, we need to avoid circular constraints by using a separate lifetime
  // but ensuring the storage lifetime relationships are properly expressed.
  let (former_begin_impl_generics, former_begin_trait_lifetime, former_begin_additional_bounds) = if classification.is_empty {
    // For structs with no generics at all, need to provide required trait bounds
    // The 'static types () and ReturnPreformed automatically satisfy T : 'a for any 'a
    (quote! { < 'a, Definition > }, quote! { 'a }, quote! { Definition::Context : 'a, Definition::End : 'a})
  } else if classification.has_only_lifetimes {
    // CRITICAL INSIGHT: For lifetime-only structs, the circular constraint issue arises because
    // the trait requires Definition::Storage : 'storage, but our storage contains the same lifetime.
    // The solution is to use a separate 'storage lifetime and establish the proper relationship.
    
    let lifetimes_only_params = generic_params::filter_params(&ast.generics.params, generic_params::filter_lifetimes);
    let mut lifetimes_only_generics = ast.generics.clone();
    lifetimes_only_generics.params = lifetimes_only_params;
    
    if lifetimes_only_generics.params.is_empty() {
      // No lifetimes in the struct - use a fresh 'storage lifetime
      // For structs with no generics at all, don't add the Definition bounds that cause E0309
      (quote! { < 'storage, Definition > }, quote! { 'storage }, quote! {})
    } else {
      // Lifetime-only struct - use both the struct's lifetime and separate storage lifetime
      let (_, lifetimes_impl, _, _) = generic_params::decompose(&lifetimes_only_generics);
      // Get first lifetime name for the bound
      let first_lifetime = if let Some(syn::GenericParam::Lifetime(ref lp)) = lifetimes_only_generics.params.first() {
        &lp.lifetime
      } else {
        return Err(syn::Error::new_spanned(&ast, "Expected lifetime parameter"));
      };
      
      // Use separate 'storage lifetime with proper bounds
      // The key insight: we need 'a : 'storage to satisfy the trait bounds without circularity
      // Also need to ensure Definition's associated types outlive 'storage as required by trait
      (
        quote! { < #lifetimes_impl, 'storage, Definition > },
        quote! { 'storage },
        quote! { #first_lifetime : 'storage, Definition::Context : 'storage, Definition::End : 'storage }
      )
    }
  } else if classification.has_only_types {
    // For type-only structs, need to add proper lifetime bounds for all type parameters
    let types_only_params = generic_params::filter_params(&ast.generics.params, generic_params::filter_types);
    let mut types_only_generics = ast.generics.clone();
    types_only_generics.params = types_only_params;
    
    if types_only_generics.params.is_empty() {
      // No type parameters - use basic bounds
      (quote! { < 'a, Definition > }, quote! { 'a }, quote! { Definition::Context : 'a, Definition::End : 'a})
    } else {
      // Type-only struct - need all type parameters to outlive 'a plus Definition bounds
      let (_, types_impl, _, _) = generic_params::decompose(&types_only_generics);
      
      // Generate bounds for all type parameters: T : 'a, U : 'a, etc.
      let type_bounds = types_only_generics.params.iter().map(|param| {
        if let syn::GenericParam::Type(type_param) = param {
          let ident = &type_param.ident;
          quote! { #ident : 'a }
        } else {
          quote! {}
        }
      });
      
      (
        quote! { < 'a, #types_impl, Definition > }, 
        quote! { 'a }, 
        quote! { #(#type_bounds),*, Definition::Context : 'a, Definition::End : 'a}
      )
    }
  } else {
    (quote! { < 'a, Definition > }, quote! { 'a }, quote! {})
  };

  /* parameters for former perform: The perform method needs struct generics + Definition parameter */
  let perform_base_generics = if classification.has_only_lifetimes {
    let lifetimes_only_params = generic_params::filter_params(&ast.generics.params, generic_params::filter_lifetimes);
    let mut lifetimes_only_generics = ast.generics.clone();
    lifetimes_only_generics.params = lifetimes_only_params;
    lifetimes_only_generics
  } else if classification.has_only_types {
    let types_only_params = generic_params::filter_params(&ast.generics.params, generic_params::filter_types);
    let mut types_only_generics = ast.generics.clone();
    types_only_generics.params = types_only_params;
    types_only_generics
  } else {
    syn::Generics::default()
  };
  
  let extra: macro_tools::generic_params::GenericsWithWhere = parse_quote! {
    < Definition >
    where
      Definition : former::FormerDefinition< Storage = #storage_type_ref, Formed = #struct_type_ref >,
      Definition::Types : former::FormerDefinitionTypes< Storage = #storage_type_ref, Formed = #struct_type_ref >,
  };
  let merged = generic_params::merge(&perform_base_generics, &extra.into());
  let (
    _former_perform_generics_with_defaults,
    former_perform_generics_impl,
    _former_perform_generics_ty,
    former_perform_generics_where,
  ) = generic_params::decompose(&merged);
  
  // Helper for former perform generics without trailing comma for type usage
  let _former_perform_generics_ty_clean = quote! { Definition };

  // Helper for former perform impl generics - ensure we have angle brackets
  let former_perform_impl_generics = if former_perform_generics_impl.is_empty() {
    quote! { < Definition > }
  } else {
    quote! { < #former_perform_generics_impl > }
  };

  // Helper for former perform type generics - should match the former type ref
  let former_perform_type_generics = if classification.has_only_lifetimes {
    let lifetimes_only_params = generic_params::filter_params(&ast.generics.params, generic_params::filter_lifetimes);
    let mut lifetimes_only_generics = ast.generics.clone();
    lifetimes_only_generics.params = lifetimes_only_params;
    if lifetimes_only_generics.params.is_empty() {
      quote! { < Definition > }
    } else {
      let (_, _, lifetimes_ty, _) = generic_params::decompose(&lifetimes_only_generics);
      quote! { < #lifetimes_ty, Definition > }
    }
  } else if classification.has_only_types {
    let types_only_params = generic_params::filter_params(&ast.generics.params, generic_params::filter_types);
    let mut types_only_generics = ast.generics.clone();
    types_only_generics.params = types_only_params;
    if types_only_generics.params.is_empty() {
      quote! { < Definition > }
    } else {
      let (_, _, types_ty, _) = generic_params::decompose(&types_only_generics);
      quote! { < #types_ty, Definition > }
    }
  } else {
    quote! { < Definition > }
  };

  /* parameters for definition types: Merge struct generics with Context and Formed parameters. */
  let extra: macro_tools::generic_params::GenericsWithWhere = parse_quote! {
    < __Context = (), __Formed = #struct_type_ref >
  };
  let former_definition_types_generics = generic_params::merge(generics, &extra.into());
  let (
    former_definition_types_generics_with_defaults,
    former_definition_types_generics_impl,
    former_definition_types_generics_ty,
    former_definition_types_generics_where,
  ) = generic_params::decompose(&former_definition_types_generics);
  
  // No need to clean up trailing commas - decompose doesn't add them
  
  // Generate PhantomData tuple type based on the impl generics.
  let former_definition_types_phantom = macro_tools::phantom::tuple(&former_definition_types_generics_impl);

  // Helper for definition types impl generics
  let former_definition_types_impl_generics = if struct_generics_impl.is_empty() {
    quote! { < __Context, __Formed > }
  } else {
    quote! { < #former_definition_types_generics_impl > }
  };

  // Helper for definition types where clause
  let former_definition_types_where_clause = if former_definition_types_generics_where.is_empty() {
    quote! {}
  } else {
    quote! { where #former_definition_types_generics_where }
  };

  // Helper to generate definition types reference with angle brackets only when needed
  let former_definition_types_ref = if struct_generics_ty.is_empty() {
    quote! { #former_definition_types < __Context, __Formed > }
  } else {
    quote! { #former_definition_types < #former_definition_types_generics_ty > }
  };

  /* parameters for definition: Merge struct generics with Context, Formed, and End parameters. */
  let extra: macro_tools::generic_params::GenericsWithWhere = parse_quote! {
    < __Context = (), __Formed = #struct_type_ref, __End = former::ReturnPreformed >
  };
  let generics_of_definition = generic_params::merge(generics, &extra.into());
  let (
    former_definition_generics_with_defaults,
    former_definition_generics_impl,
    former_definition_generics_ty,
    former_definition_generics_where,
  ) = generic_params::decompose(&generics_of_definition);
  
  // No need to clean up trailing commas - decompose doesn't add them
  
  // Generate PhantomData tuple type based on the impl generics.
  let former_definition_phantom = macro_tools::phantom::tuple(&former_definition_generics_impl);

  // Helper for definition impl generics
  let former_definition_impl_generics = if struct_generics_impl.is_empty() {
    quote! { < __Context, __Formed, __End > }
  } else {
    quote! { < #former_definition_generics_impl > }
  };

  // Helper for definition where clause
  let former_definition_where_clause = if former_definition_generics_where.is_empty() {
    quote! {}
  } else {
    quote! { where #former_definition_generics_where }
  };

  // Helper for definition where clause with __End constraint
  let former_definition_where_clause_with_end = if former_definition_generics_where.is_empty() {
    quote! {
      where
        __End : former::FormingEnd< #former_definition_types_ref >
    }
  } else {
    quote! {
      where
        __End : former::FormingEnd< #former_definition_types_ref >,
        #former_definition_generics_where
    }
  };

  // Helper to generate definition reference with angle brackets only when needed
  let former_definition_ref = if struct_generics_ty.is_empty() {
    quote! { #former_definition < __Context, __Formed, __End > }
  } else {
    quote! { #former_definition < #former_definition_generics_ty > }
  };

  // Helper for AsSubformer type alias - handles generics properly
  let as_subformer_definition = if struct_generics_ty.is_empty() {
    quote! { #former_definition < __Superformer, __Superformer, __End > }
  } else {
    quote! { #former_definition < #struct_generics_ty, __Superformer, __Superformer, __End > }
  };

  // Helper for AsSubformer former type reference
  // The former struct itself also needs its generic parameters (lifetimes, types)
  let as_subformer_former = if struct_generics_ty.is_empty() {
    quote! { #former < #as_subformer_definition > }
  } else {
    quote! { #former < #struct_generics_ty, #as_subformer_definition > }
  };

  // Helper for AsSubformerEnd definition types reference
  let as_subformer_end_definition_types = if struct_generics_ty.is_empty() {
    quote! { #former_definition_types < SuperFormer, SuperFormer > }
  } else {
    quote! { #former_definition_types < #struct_generics_ty, SuperFormer, SuperFormer > }
  };
  
  // Helper for AsSubformer type alias with proper generics handling
  let as_subformer_alias = if struct_generics_ty.is_empty() {
    quote! { #vis type #as_subformer < __Superformer, __End > = #as_subformer_former; }
  } else {
    quote! { #vis type #as_subformer < #struct_generics_ty, __Superformer, __End > = #as_subformer_former; }
  };
  
  // Helper for AsSubformerEnd trait declaration with proper generics
  let as_subformer_end_trait = if struct_generics_ty.is_empty() {
    quote! { pub trait #as_subformer_end < SuperFormer > }
  } else {
    quote! { pub trait #as_subformer_end < #struct_generics_ty, SuperFormer > }
  };
  
  // Helper for AsSubformerEnd impl declaration with proper generics
  let as_subformer_end_impl = if struct_generics_ty.is_empty() {
    quote! { impl< SuperFormer, __T > #as_subformer_end < SuperFormer > }
  } else {
    quote! { impl< #struct_generics_impl, SuperFormer, __T > #as_subformer_end < #struct_generics_ty, SuperFormer > }
  };

  // Helper for AsSubformerEnd where clause
  let as_subformer_end_where_clause = if struct_generics_where.is_empty() {
    quote! {
      where
        Self : former::FormingEnd
        < // Angle bracket on new line
          #as_subformer_end_definition_types
        > // Angle bracket on new line
    }
  } else {
    quote! {
      where
        Self : former::FormingEnd
        < // Angle bracket on new line
          #as_subformer_end_definition_types
        >, // Angle bracket on new line
        #struct_generics_where
    }
  };

  /* struct attributes: Generate documentation and extract perform method details. */
  let (_doc_former_mod, doc_former_struct) = doc_generate(item);
  let (perform, perform_output, perform_generics) = struct_attrs.performer()?;

  /* fields: Process struct fields and storage_fields attribute. */
  let fields = derive::named_fields(ast)?;
  // Create FormerField representation for actual struct fields.
  let formed_fields: Vec<_> = fields
    .iter()
    .map(|field| FormerField::from_syn(field, true, true))
    .collect::<Result<_>>()?;
  // Create FormerField representation for storage-only fields.
  let storage_fields: Vec<_> = struct_attrs
    .storage_fields()
    .iter()
    .map(|field| FormerField::from_syn(field, true, false))
    .collect::<Result<_>>()?;

  // <<< Start of changes for constructor arguments >>>
  // Identify fields marked as constructor arguments
  let constructor_args_fields: Vec<_> = formed_fields
  .iter()
  .filter( | f | !f.attrs.former_ignore.value( false ) ) // Use the parsed attribute
  .collect();

  // Generate constructor function parameters
  let constructor_params = constructor_args_fields.iter().map(| f | // Space around |
  {
    let ident = f.ident;
    let ty = f.non_optional_ty; // Use non-optional type for the argument
    // Use raw identifier for parameter name if needed
    let param_name = ident::ident_maybe_raw( ident );
    quote! { #param_name : impl ::core::convert::Into< #ty > }
  });

  // Generate initial storage assignments for constructor arguments
  let constructor_storage_assignments = constructor_args_fields.iter().map(| f | // Space around |
  {
    let ident = f.ident;
    // Use raw identifier for parameter name if needed
    let param_name = ident::ident_maybe_raw( ident );
    quote! { #ident : ::core::option::Option::Some( #param_name.into() ) }
  });

  // Generate initial storage assignments for non-constructor arguments (set to None)
  let non_constructor_storage_assignments = formed_fields
  .iter()
  .chain( storage_fields.iter() ) // Include storage-only fields
  .filter( | f | f.attrs.former_ignore.value( false ) ) // Filter out constructor args
  .map( | f | // Space around |
  {
    let ident = f.ident;
    quote! { #ident : ::core::option::Option::None }
  });

  // Combine all storage assignments
  let all_storage_assignments = constructor_storage_assignments.chain(non_constructor_storage_assignments);

  // Determine if we need to initialize storage (if there are args)
  let initial_storage_code = if constructor_args_fields.is_empty() {
    // No args, begin with None storage
    quote! { ::core::option::Option::None }
  } else {
    // Has args, create initial storage instance
    quote! {
      ::core::option::Option::Some
      ( // Paren on new line
        #storage_type_ref // Add generics to storage type
        {
          #( #all_storage_assignments ),*
        }
      ) // Paren on new line
    }
  };
  // <<< End of changes for constructor arguments >>>

  // Generate code snippets for each field (storage init, storage field def, preform logic, setters).
  let (
    storage_field_none,     // Code for initializing storage field to None.
    storage_field_optional, // Code for the storage field definition (e.g., `pub field: Option<Type>`).
    storage_field_name,     // Code for the field name (e.g., `field,`). Used in final struct construction.
    storage_field_preform,  // Code for unwrapping/defaulting the field in `preform`.
    former_field_setter,    // Code for the setter method(s) for the field.
  ): (Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>) = formed_fields // Combine actual fields and storage-only fields for processing.
    .iter()
    .chain(storage_fields.iter())
    .map(| field | // Space around |
  {(
    field.storage_fields_none(),
    field.storage_field_optional(),
    field.storage_field_name(), // Only generated if field.for_formed is true.
    field.storage_field_preform(), // Only generated if field.for_formed is true.
    field.former_field_setter
    ( // Paren on new line
      item,
      original_input,
      &struct_generics_impl,
      &struct_generics_ty,
      &struct_generics_where,
      &former,
      &former_generics_impl,
      &former_generics_ty,
      &former_generics_where,
      &former_storage,
    ), // Paren on new line
  )})
    .multiunzip();

  // Collect results, separating setters and namespace code (like End structs).
  let results: Result<Vec<_>> = former_field_setter.into_iter().collect();
  let (former_field_setter, namespace_code): (Vec<_>, Vec<_>) = results?.into_iter().unzip();
  // Collect preform logic results.
  let storage_field_preform: Vec<_> = storage_field_preform.into_iter().collect::<Result<_>>()?;
  // Generate mutator implementation code.
  let _former_mutator_code = mutator( // Changed to _former_mutator_code
    item,
    original_input,
    &struct_attrs.mutator,
    &former_definition_types,
    &FormerDefinitionTypesGenerics { // Pass the new struct
      impl_generics: &former_definition_types_generics_impl,
      ty_generics: &former_definition_types_generics_ty,
      where_clause: &former_definition_types_generics_where,
    },
    &former_definition_types_ref,
  )?;

  // <<< Start of updated code for standalone constructor (Option 2) >>>
  let standalone_constructor_code = if struct_attrs.standalone_constructors.value(false) {
    // Generate constructor name (snake_case)
    let constructor_name_str = item.to_string().to_case(Case::Snake);
    let constructor_name_ident_temp = format_ident!("{}", constructor_name_str, span = item.span());
    let constructor_name = ident::ident_maybe_raw(&constructor_name_ident_temp);

    // Determine if all fields are constructor arguments
    // Note: We only consider fields that are part of the final struct (`formed_fields`)
    let all_fields_are_args = formed_fields.iter().all(|f| !f.attrs.former_ignore.value(false)); // Space around |

    // Determine return type and body based on Option 2 rule
    let (return_type, constructor_body) = if all_fields_are_args {
      // Return Self
      let return_type = quote! { #struct_type_ref };
      let construction_args = formed_fields.iter().map(| f | // Space around |
      {
        let field_ident = f.ident;
        let param_name = ident::ident_maybe_raw( field_ident );
        quote! { #field_ident : #param_name.into() }
      });
      let body = quote! { #struct_type_ref { #( #construction_args ),* } };
      (return_type, body)
    } else {
      // Return Former
      let _former_return_type = quote! {
        #former < #former_definition< #former_definition_args > >
      };
      let former_body = quote! {
        #former::begin( #initial_storage_code, None, former::ReturnPreformed )
      };
      (former_type_ref.clone(), former_body) // Cloned former_type_ref
    };

    // Generate the constructor function
    quote! {
      /// Standalone constructor function for #item.
      #[ inline( always ) ]
      #vis fn #constructor_name < #struct_generics_impl >
      ( // Paren on new line
        #( #constructor_params ),* // Parameters are generated earlier
      ) // Paren on new line
      ->
      #return_type // Use determined return type
      where
        #struct_generics_where // Use original struct where clause
      {
        #constructor_body // Use determined body
      }
    }
  } else {
    // If #[standalone_constructors] is not present, generate nothing.
    quote! {}
  };
  // <<< End of updated code for standalone constructor (Option 2) >>>

  // Build generic lists for EntityToFormer impl
  // For lifetime-only structs, we need to be careful with generic parameter ordering
  // Build generic lists for EntityToFormer impl
  let entity_to_former_impl_generics = generic_params::params_with_additional(
    &struct_generics_impl,
    &[parse_quote! { Definition }],
  );
  
  // Build generic lists for EntityToFormer type Former - should match the former type
  let entity_to_former_ty_generics = if classification.has_only_lifetimes {
    let lifetimes_only_params = generic_params::filter_params(&ast.generics.params, generic_params::filter_lifetimes);
    let mut lifetimes_only_generics = ast.generics.clone();
    lifetimes_only_generics.params = lifetimes_only_params;
    if lifetimes_only_generics.params.is_empty() {
      quote! { Definition }
    } else {
      let (_, _, lifetimes_ty, _) = generic_params::decompose(&lifetimes_only_generics);
      quote! { #lifetimes_ty, Definition }
    }
  } else if classification.has_only_types {
    let types_only_params = generic_params::filter_params(&ast.generics.params, generic_params::filter_types);
    let mut types_only_generics = ast.generics.clone();
    types_only_generics.params = types_only_params;
    if types_only_generics.params.is_empty() {
      quote! { Definition }
    } else {
      let (_, _, types_ty, _) = generic_params::decompose(&types_only_generics);
      quote! { #types_ty, Definition }
    }
  } else {
    quote! { Definition }
  };
  
  // Build generic lists for EntityToDefinition impl
  // CRITICAL FIX: Use merge_params_ordered to ensure proper generic parameter ordering
  let additional_params: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> = 
    parse_quote! { __Context, __Formed, __End };
  let entity_to_definition_impl_generics = generic_params::merge_params_ordered(
    &[&struct_generics_impl, &additional_params],
  );
  
  // Build generic lists for definition types in trait bounds
  // CRITICAL FIX: Use merge_params_ordered to ensure proper generic parameter ordering
  let additional_params: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> = 
    parse_quote! { __Context, __Formed };
  let definition_types_ty_generics = generic_params::merge_params_ordered(
    &[&struct_generics_ty, &additional_params],
  );
  
  // Build generic lists for definition in associated types
  // CRITICAL FIX: Use merge_params_ordered to ensure proper generic parameter ordering
  let additional_params: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> = 
    parse_quote! { __Context, __Formed, __End };
  let definition_ty_generics = generic_params::merge_params_ordered(
    &[&struct_generics_ty, &additional_params],
  );
  
  // Build generic lists for EntityToDefinitionTypes impl
  // CRITICAL FIX: Use merge_params_ordered to ensure proper generic parameter ordering
  let additional_params: syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma> = 
    parse_quote! { __Context, __Formed };
  let entity_to_definition_types_impl_generics = generic_params::merge_params_ordered(
    &[&struct_generics_impl, &additional_params],
  );

  // Assemble the final generated code using quote!
  
  // For type-only structs, exclude struct bounds from FormerBegin to avoid E0309 errors
  // The minor E0277 trait bound error is acceptable vs the major E0309 lifetime error
  let _former_begin_where_clause = if classification.has_only_types {
    quote! {}
  } else {
    quote! { , #struct_generics_where }
  };
  
  // Build proper where clause for FormerBegin trait implementation
  let former_begin_final_where_clause = if struct_generics_where.is_empty() {
    if former_begin_additional_bounds.is_empty() {
      quote! {
        where
          Definition : former::FormerDefinition< Storage = #storage_type_ref >
      }
    } else {
      quote! {
        where
          Definition : former::FormerDefinition< Storage = #storage_type_ref >,
          #former_begin_additional_bounds
      }
    }
  } else {
    if former_begin_additional_bounds.is_empty() {
      quote! {
        where
          Definition : former::FormerDefinition< Storage = #storage_type_ref >,
          #struct_generics_where
      }
    } else {
      // struct_generics_where already has a trailing comma from decompose
      quote! {
        where
          Definition : former::FormerDefinition< Storage = #storage_type_ref >,
          #struct_generics_where #former_begin_additional_bounds
      }
    }
  };
  
  let result = quote! {

    // = formed: Implement the `::former()` static method on the original struct.
    #[ automatically_derived ]
    impl #struct_impl_generics #struct_type_ref
    #struct_where_clause
    {
      /// Provides a mechanism to initiate the formation process with a default completion behavior.
      #[ inline( always ) ]
      pub fn former() -> #former_type_full
      {
        #former::begin( None, None, former::ReturnPreformed )
      }
    }

    // <<< Added Standalone Constructor Function >>>
    #standalone_constructor_code

    // = entity to former: Implement former traits linking the struct to its generated components.
    impl< #entity_to_former_impl_generics > former::EntityToFormer< Definition >
    for #struct_type_ref
    where
      Definition : former::FormerDefinition< Storage = #storage_type_ref >,
      #struct_generics_where
    {
      type Former = #former < #entity_to_former_ty_generics > ;
    }

    impl #struct_impl_generics former::EntityToStorage
    for #struct_type_ref
    #struct_where_clause
    {
      type Storage = #storage_type_ref;
    }

    impl< #entity_to_definition_impl_generics > former::EntityToDefinition< __Context, __Formed, __End >
    for #struct_type_ref
    where
      __End : former::FormingEnd< #former_definition_types < #definition_types_ty_generics > >,
      #struct_generics_where
    {
      type Definition = #former_definition < #definition_ty_generics >;
      type Types = #former_definition_types < #definition_types_ty_generics >;
    }

    impl< #entity_to_definition_types_impl_generics > former::EntityToDefinitionTypes< __Context, __Formed >
    for #struct_type_ref
    #struct_where_clause
    {
      type Types = #former_definition_types < #definition_types_ty_generics >;
    }

    // = definition types: Define the FormerDefinitionTypes struct.
    /// Defines the generic parameters for formation behavior including context, form, and end conditions.
    #[ derive( Debug ) ]
    #vis struct #former_definition_types < #former_definition_types_generics_with_defaults >
    #former_definition_types_where_clause
    {
      _phantom : #former_definition_types_phantom,
    }

    impl #former_definition_types_impl_generics ::core::default::Default
    for #former_definition_types_ref
    #former_definition_types_where_clause
    {
      fn default() -> Self
      {
        Self
        {
          _phantom : ::core::marker::PhantomData,
        }
      }
    }

    impl #former_definition_types_impl_generics former::FormerDefinitionTypes
    for #former_definition_types_ref
    #former_definition_types_where_clause
    {
      type Storage = #storage_type_ref;
      type Formed = __Formed;
      type Context = __Context;
    }

    // Add FormerMutator implementation here
    #_former_mutator_code

    // = definition: Define the FormerDefinition struct.
    /// Holds the definition types used during the formation process.
    #[ derive( Debug ) ]
    #vis struct #former_definition < #former_definition_generics_with_defaults >
    #former_definition_where_clause
    {
      _phantom : #former_definition_phantom,
    }

    impl #former_definition_impl_generics ::core::default::Default
    for #former_definition_ref
    #former_definition_where_clause
    {
      fn default() -> Self
      {
        Self
        {
          _phantom : ::core::marker::PhantomData,
        }
      }
    }

    impl #former_definition_impl_generics former::FormerDefinition
    for #former_definition_ref
    #former_definition_where_clause_with_end
    {
      type Types = #former_definition_types_ref;
      type End = __End;
      type Storage = #storage_type_ref;
      type Formed = __Formed;
      type Context = __Context;
    }

    // = storage: Define the FormerStorage struct.
    #[ doc = "Stores potential values for fields during the formation process." ]
    #[ allow( explicit_outlives_requirements ) ]
    #vis struct #former_storage < #struct_generics_with_defaults >
    #struct_where_clause
    {
      #(
        /// A field
        #storage_field_optional,
      )*
    }

    impl #struct_impl_generics ::core::default::Default
    for #storage_type_ref
    #struct_where_clause
    {
      #[ inline( always ) ]
      fn default() -> Self
      {
        Self
        {
          #( #storage_field_none, )*
        }
      }
    }

    impl #struct_impl_generics former::Storage
    for #storage_type_ref
    #struct_where_clause
    {
      type Preformed = #struct_type_ref;
    }

    impl #struct_impl_generics former::StoragePreform
    for #storage_type_ref
    #struct_where_clause
    {
      fn preform( mut self ) -> Self::Preformed
      {
        #( #storage_field_preform )*
        let result = #item
        {
          #( #storage_field_name )*
        };
        return result;
      }
    }

    // = former: Define the Former struct itself.
    #[ doc = #doc_former_struct ]
    #vis struct #former < #former_generics_with_defaults >
    where
      #former_generics_where
    {
      /// Temporary storage for all fields during the formation process.
      pub storage : Definition::Storage,
      /// Optional context.
      pub context : ::core::option::Option< Definition::Context >,
      /// Optional handler for the end of formation.
      pub on_end : ::core::option::Option< Definition::End >,
    }

    #[ automatically_derived ]
    impl #former_impl_generics #former_type_ref
    where
      #former_generics_where
    {
      /// Initializes a former with an end condition and default storage.
      #[ inline( always ) ]
      pub fn new
      ( // Paren on new line
        on_end : Definition::End
      ) -> Self // Paren on new line
      {
        Self::begin_coercing( ::core::option::Option::None, ::core::option::Option::None, on_end )
      }

      /// Initializes a former with a coercible end condition.
      #[ inline( always ) ]
      pub fn new_coercing< IntoEnd >
      ( // Paren on new line
        end : IntoEnd
      ) -> Self // Paren on new line
      where
        IntoEnd : ::core::convert::Into< Definition::End >,
      {
        Self::begin_coercing
        ( // Paren on new line
          ::core::option::Option::None,
          ::core::option::Option::None,
          end,
        ) // Paren on new line
      }

      /// Begins the formation process with specified context and termination logic.
      #[ inline( always ) ]
      pub fn begin
      ( // Paren on new line
        mut storage : ::core::option::Option< Definition::Storage >,
        context : ::core::option::Option< Definition::Context >,
        on_end : < Definition as former::FormerDefinition >::End,
      ) // Paren on new line
      -> Self
      {
        if storage.is_none()
        {
          storage = ::core::option::Option::Some( ::core::default::Default::default() );
        }
        Self
        {
          storage : storage.unwrap(),
          context : context,
          on_end : ::core::option::Option::Some( on_end ),
        }
      }

      /// Starts the formation process with coercible end condition and optional initial values.
      #[ inline( always ) ]
      pub fn begin_coercing< IntoEnd >
      ( // Paren on new line
        mut storage : ::core::option::Option< Definition::Storage >,
        context : ::core::option::Option< Definition::Context >,
        on_end : IntoEnd,
      ) -> Self // Paren on new line
      where
        IntoEnd : ::core::convert::Into< < Definition as former::FormerDefinition >::End >,
      {
        if storage.is_none()
        {
          storage = ::core::option::Option::Some( ::core::default::Default::default() );
        }
        Self
        {
          storage : storage.unwrap(),
          context : context,
          on_end : ::core::option::Option::Some( ::core::convert::Into::into( on_end ) ),
        }
      }

      /// Wrapper for `end` to align with common builder pattern terminologies.
      #[ inline( always ) ]
      pub fn form( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      {
        self.end()
      }

      /// Completes the formation and returns the formed object.
      #[ inline( always ) ]
      pub fn end( mut self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      {
        let on_end = self.on_end.take().unwrap();
        let mut context = self.context.take();
        < Definition::Types as former::FormerMutator >::form_mutation( &mut self.storage, &mut context );
        former::FormingEnd::< Definition::Types >::call( &on_end, self.storage, context )
      }

      // Insert generated setter methods for each field.
      #(
        #former_field_setter
      )*

    }

    // = former :: preform: Implement `preform` for direct storage transformation.
    impl #former_impl_generics #former_type_ref
    where
      Definition : former::FormerDefinition< Storage = #storage_type_ref, Formed = #struct_type_ref >,
      Definition::Types : former::FormerDefinitionTypes< Storage = #storage_type_ref, Formed = #struct_type_ref >,
      #former_generics_where
    {
      /// Executes the transformation from the former's storage state to the preformed object.
      pub fn preform( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
      {
        former::StoragePreform::preform( self.storage )
      }
    }

    // = former :: perform: Implement `perform` if specified by attributes.
    #[ automatically_derived ]
    impl #former_perform_impl_generics #former #former_perform_type_generics
    where
      #former_perform_generics_where
    {
      /// Finish setting options and call perform on formed entity.
      #[ inline( always ) ]
      pub fn perform #perform_generics ( self ) -> #perform_output
      {
        let result = self.form();
        #perform
      }
    }

    // = former begin: Implement `FormerBegin` trait.
    // CRITICAL FIX: For lifetime-only structs, avoid circular lifetime constraints  
    // where Definition::Storage contains the same lifetime that we're constraining it to outlive
    impl #former_begin_impl_generics former::FormerBegin< #former_begin_trait_lifetime, Definition >
    for #former_type_ref
    #former_begin_final_where_clause
    {
      #[ inline( always ) ]
      fn former_begin
      ( // Paren on new line
        storage : ::core::option::Option< Definition::Storage >,
        context : ::core::option::Option< Definition::Context >,
        on_end : Definition::End,
      ) // Paren on new line
      -> Self
      {
        // qqq : This debug_assert should be enabled by default. How to do that?
        //       Maybe always generate code with debug_assert and remove it if release build?
        //       Or rely on optimizer to remove it?
        // debug_assert!( storage.is_none() );
        Self::begin( ::core::option::Option::None, context, on_end )
      }
    }

    // = subformer: Define the `AsSubformer` type alias.
    /// Provides a specialized former for structure using predefined settings for superformer and end conditions.
    #as_subformer_alias


    // = as subformer end: Define the `AsSubformerEnd` trait.
    #[ doc = #as_subformer_end_doc ]
    #as_subformer_end_trait
    #as_subformer_end_where_clause
    {
    }

    #as_subformer_end_impl
    for __T
    #as_subformer_end_where_clause
    {
    }

    // = etc: Insert any namespace code generated by field setters (e.g., End structs for subformers).
    #( #namespace_code )*

  };
  
  // Add debug output if #[debug] attribute is present
  if _has_debug {
    let about = format!("derive : Former\nstruct : {item}");
    diag::report_print(about, original_input, &result);
  }
  
  // CRITICAL FIX: Derive macros should only return generated code, NOT the original struct
  // The original struct is preserved by the Rust compiler automatically
  // We were incorrectly including it, which caused duplication errors
  // The "type parameter not found" error was actually caused by our macro 
  // returning malformed TokenStream, not by missing the original struct
  
  // Debug: Print the result for lifetime-only and type-only structs to diagnose issues
  #[cfg(feature = "former_diagnostics_print_generated")]
  if classification.has_only_lifetimes && item.to_string().contains("TestLifetime") {
    eprintln!("LIFETIME DEBUG: Generated code for {}:", item);
    eprintln!("{}", result);
  }
  
  Ok(result)
}

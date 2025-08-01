//! # Field Processing and Analysis - Former Pattern Field Handling
//!
//! This module provides comprehensive field processing capabilities for the Former derive macro,
//! including sophisticated type analysis, attribute handling, and code generation for field-specific
//! setters and storage management. It resolves many of the complex field-level issues encountered
//! in manual implementation testing.
//!
//! ## Core Functionality
//!
//! ### Field Analysis and Classification
//! - **Type Introspection**: Deep analysis of field types including generics and lifetimes
//! - **Container Detection**: Automatic detection of Vec, HashMap, HashSet, and other collections
//! - **Optional Type Handling**: Sophisticated handling of `Option<T>` wrapped fields
//! - **Attribute Integration**: Seamless integration with field-level attributes
//!
//! ### Code Generation Capabilities
//! - **Storage Field Generation**: Option-wrapped storage fields with proper defaults
//! - **Setter Method Generation**: Type-appropriate setter methods (scalar, subform, collection)
//! - **Preform Logic**: Proper conversion from storage to formed struct
//! - **Generic Propagation**: Maintaining generic parameters through all generated code
//!
//! ## Critical Pitfalls Resolved
//!
//! ### 1. Optional Type Detection and Handling
//! **Issue Resolved**: Confusion between `Option<T>` fields and non-optional fields in storage
//! **Root Cause**: Manual implementations not properly distinguishing optional vs required fields
//! **Solution**: Systematic optional type detection with proper storage generation
//! **Prevention**: Automated `is_optional` detection prevents manual implementation errors
//!
//! ### 2. Container Type Classification (Issues #3, #11 Resolution)
//! **Issue Resolved**: Collection types not properly detected for subform generation
//! **Root Cause**: Manual implementations missing collection-specific logic
//! **Solution**: Comprehensive container kind detection using `container_kind::of_optional`
//! **Prevention**: Automatic collection type classification enables proper setter generation
//!
//! ### 3. Generic Parameter Preservation (Issues #2, #4, #5, #6 Resolution)
//! **Issue Resolved**: Complex generic types losing generic parameter information
//! **Root Cause**: Field type analysis not preserving full generic information
//! **Solution**: Complete type preservation with `non_optional_ty` tracking
//! **Prevention**: Full generic parameter preservation through field processing pipeline
//!
//! ### 4. Storage vs Formed Field Distinction (Issues #9, #10, #11 Resolution)
//! **Issue Resolved**: Confusion about which fields belong in storage vs formed struct
//! **Root Cause**: Manual implementations mixing storage and formed field logic
//! **Solution**: Clear `for_storage` and `for_formed` flags with separate processing paths
//! **Prevention**: Explicit field categorization prevents mixing storage and formed logic
//!
//! ## Field Processing Architecture
//!
//! ### Analysis Phase
//! 1. **Attribute Parsing**: Parse and validate all field-level attributes
//! 2. **Type Analysis**: Deep introspection of field type including generics
//! 3. **Container Detection**: Identify collection types and their characteristics
//! 4. **Optional Detection**: Determine if field is Option-wrapped
//! 5. **Classification**: Categorize field for appropriate code generation
//!
//! ### Generation Phase
//! 1. **Storage Generation**: Create Option-wrapped storage fields
//! 2. **Setter Generation**: Generate appropriate setter methods based on field type
//! 3. **Preform Logic**: Create conversion logic from storage to formed
//! 4. **Generic Handling**: Ensure generic parameters are properly propagated
//!
//! ## Quality Assurance Features
//! - **Type Safety**: All generated code maintains Rust's type safety guarantees
//! - **Generic Consistency**: Generic parameters consistently tracked and used
//! - **Lifetime Safety**: Lifetime parameters properly scoped and propagated
//! - **Attribute Validation**: Field attributes validated against field types

// File: module/core/former_meta/src/derive_former/field.rs

use super::*;
use macro_tools::container_kind;

/// Comprehensive field definition and analysis for Former pattern generation.
///
/// This structure encapsulates all the information needed to generate proper Former pattern
/// code for a single field, including complex type analysis, attribute handling, and
/// code generation support. It resolves many of the field-level complexities that caused
/// manual implementation failures.
///
/// # Core Field Information
///
/// ## Type Analysis
/// - **`ty`**: Complete field type as specified in the original struct
/// - **`non_optional_ty`**: Inner type for Option-wrapped fields, or same as `ty` for non-optional
/// - **`is_optional`**: Whether the field is wrapped in `Option<T>`
/// - **`of_type`**: Container classification (Vec, HashMap, HashSet, etc.)
///
/// ## Field Classification
/// - **`for_storage`**: Whether this field should appear in the FormerStorage struct
/// - **`for_formed`**: Whether this field should appear in the final formed struct
/// - **`attrs`**: Parsed field-level attributes affecting code generation
///
/// # Critical Design Decisions
///
/// ## Optional Type Handling Strategy
/// The structure distinguishes between fields that are naturally `Option<T>` in the original
/// struct versus fields that become `Option<T>` in the storage struct:
/// - **Natural Optional**: `field: Option<String>` → storage: `field: Option<Option<String>>`  
/// - **Storage Optional**: `field: String` → storage: `field: Option<String>`
///
/// ## Container Type Classification
/// Automatic detection of collection types enables appropriate setter generation:
/// - **Vec-like**: Generates collection subform setters
/// - **HashMap-like**: Generates entry subform setters with proper key type validation
/// - **Scalar**: Generates simple scalar setters
///
/// # Pitfalls Prevented Through Design
///
/// ## 1. Type Information Loss (Critical Prevention)
/// **Problem**: Complex generic types losing parameter information during processing
/// **Prevention**: Complete type preservation with separate `ty` and `non_optional_ty` tracking
/// **Example**: `HashMap<K, V>` information fully preserved for proper trait bound generation
///
/// ## 2. Optional Type Confusion (Prevention)
/// **Problem**: Confusion between naturally optional fields and storage-optional fields
/// **Prevention**: Clear `is_optional` flag with proper handling in storage generation
/// **Example**: `Option<String>` vs `String` handled correctly in storage generation
///
/// ## 3. Container Misclassification (Prevention)
/// **Problem**: Collection types not recognized, leading to inappropriate setter generation
/// **Prevention**: Comprehensive container type detection using `container_kind` analysis
/// **Example**: `Vec<T>` automatically detected for collection subform generation
///
/// # Usage in Code Generation
/// This structure is used throughout the Former pattern code generation to:
/// - Determine appropriate setter method types
/// - Generate proper storage field declarations
/// - Create correct preform conversion logic
/// - Maintain generic parameter consistency
#[allow(dead_code)]
pub struct FormerField<'a> {
  pub attrs: FieldAttributes,
  pub vis: &'a syn::Visibility,
  pub ident: &'a syn::Ident,
  pub colon_token: &'a Option<syn::token::Colon>,
  pub ty: &'a syn::Type,
  pub non_optional_ty: &'a syn::Type,
  pub is_optional: bool,
  pub of_type: container_kind::ContainerKind,
  pub for_storage: bool,
  pub for_formed: bool,
}

impl<'a> FormerField<'a> {
  /** methods

  `from_syn`

  `storage_fields_none`
  `storage_field_optional`
  `storage_field_preform`
  `storage_field_name`
  `former_field_setter`
  `scalar_setter`
  `subform_entry_setter`
  `subform_collection_setter`

  `scalar_setter_name`
  `subform_scalar_setter_name`,
  `subform_collection_setter_name`
  `subform_entry_setter_name`
  `scalar_setter_required`

  */
  /// Construct a comprehensive FormerField from a syn::Field with full type analysis and pitfall prevention.
  ///
  /// This is the **critical constructor** that performs deep analysis of a struct field and creates
  /// the complete FormerField representation needed for code generation. It handles all the complex
  /// type scenarios that caused manual implementation failures and ensures proper field categorization.
  ///
  /// # Processing Steps
  ///
  /// ## 1. Attribute Processing
  /// Parses and validates all field-level attributes using `FieldAttributes::from_attrs()`:
  /// - Configuration attributes (`#[former(default = ...)]`)
  /// - Setter type attributes (`#[scalar]`, `#[subform_collection]`, etc.)
  /// - Constructor argument markers (`#[arg_for_constructor]`)
  ///
  /// ## 2. Type Analysis and Classification
  /// Performs comprehensive type analysis to determine field characteristics:
  /// - **Optional Detection**: Uses `typ::is_optional()` to detect `Option<T>` wrapping
  /// - **Container Classification**: Uses `container_kind::of_optional()` for collection detection
  /// - **Generic Extraction**: Extracts inner type from `Option<T>` for further processing
  ///
  /// ## 3. Field Categorization
  /// Determines how the field should be used in code generation:
  /// - **Storage Fields**: Fields that appear in FormerStorage struct
  /// - **Formed Fields**: Fields that appear in the final formed struct
  /// - **Both**: Fields that appear in both (most common case)
  ///
  /// # Pitfalls Prevented
  ///
  /// ## 1. Optional Type Detection Errors (Critical Prevention)
  /// **Problem**: Manual implementations incorrectly handling `Option<T>` fields
  /// **Prevention**: Systematic optional detection with proper inner type extraction
  /// **Example**:
  /// ```rust
  /// // Field: Option<HashMap<K, V>>
  /// // ✅ Correctly detected: is_optional = true, non_optional_ty = HashMap<K, V>
  /// ```
  ///
  /// ## 2. Container Type Misclassification (Prevention)
  /// **Problem**: Collection fields not recognized, leading to wrong setter generation
  /// **Prevention**: Comprehensive container kind detection
  /// **Example**:
  /// ```rust
  /// // Field: Vec<Child>
  /// // ✅ Correctly classified: of_type = ContainerKind::Vector
  /// ```
  ///
  /// ## 3. Generic Parameter Loss (Prevention)
  /// **Problem**: Complex generic types losing parameter information during processing
  /// **Prevention**: Complete type preservation with `non_optional_ty` tracking
  /// **Example**:
  /// ```rust
  /// // Field: Option<HashMap<K, V>> where K: Hash + Eq
  /// // ✅ Full generic information preserved in non_optional_ty
  /// ```
  ///
  /// ## 4. Field Identifier Validation (Prevention)
  /// **Problem**: Tuple struct fields causing crashes due to missing identifiers
  /// **Prevention**: Explicit identifier validation with clear error messages
  /// **Example**:
  /// ```rust
  /// // ❌ Would cause error: struct TupleStruct(String);
  /// // ✅ Clear error message: "Expected that each field has key, but some does not"
  /// ```
  ///
  /// # Error Handling
  /// - **Missing Identifiers**: Clear error for tuple struct fields or anonymous fields
  /// **Generic Extraction Errors**: Proper error propagation from `typ::parameter_first()`
  /// - **Attribute Parsing Errors**: Full error context preservation from attribute parsing
  ///
  /// # Usage Context
  /// This method is called for every field in a struct during Former pattern generation:
  /// - Regular struct fields → `for_storage = true, for_formed = true`
  /// - Storage-only fields → `for_storage = true, for_formed = false`
  /// - Special processing fields → Custom flag combinations
  pub fn from_syn(field: &'a syn::Field, for_storage: bool, for_formed: bool) -> Result<Self> {
    let attrs = FieldAttributes::from_attrs(field.attrs.iter())?;
    let vis = &field.vis;
    let ident = field.ident.as_ref().ok_or_else(|| {
      syn_err!(
        field,
        "Expected that each field has key, but some does not:\n  {}",
        qt! { #field }
      )
    })?;
    let colon_token = &field.colon_token;
    let ty = &field.ty;
    let is_optional = typ::is_optional(ty);
    let of_type = container_kind::of_optional(ty).0;
    let non_optional_ty: &syn::Type = if is_optional { typ::parameter_first(ty)? } else { ty };
    let field2 = Self {
      attrs,
      vis,
      ident,
      colon_token,
      ty,
      non_optional_ty,
      is_optional,
      of_type,
      for_storage,
      for_formed,
    };
    Ok(field2)
  }

  /// Generate fields for initializer of a struct setting each field to `None`.
  ///
  /// This method creates the initialization code for storage fields in the Former pattern,
  /// setting all fields to `None` initially. This resolves the storage initialization
  /// pitfall that caused manual implementation failures.
  ///
  /// # Purpose and Usage
  /// Used for initializing FormerStorage, where all fields start as `None` and are
  /// populated through the builder pattern. This prevents the common manual implementation
  /// error of forgetting to initialize storage fields.
  ///
  /// # Pitfall Prevention
  /// **Issue Resolved**: Manual implementations forgetting to initialize storage fields
  /// **Root Cause**: Missing `None` initialization causing compile errors
  /// **Solution**: Systematic `None` initialization for all storage fields
  /// **Prevention**: Automated field initialization prevents initialization errors
  ///
  /// # Generated Code Example
  /// ```ignore
  /// int_1 : ::core::option::Option::None,
  /// string_1 : ::core::option::Option::None, 
  /// int_optional_1 : ::core::option::Option::None,
  /// ```
  #[inline(always)]
  pub fn storage_fields_none(&self) -> TokenStream {
    let ident = Some(self.ident.clone());
    let tokens = qt! { ::core::option::Option::None };
    let ty2: syn::Type = syn::parse2(tokens).unwrap();

    qt! {
      #ident : #ty2
    }
  }

  /// Generate Option-wrapped storage field declaration for Former pattern.
  ///
  /// This method creates storage field declarations with proper Option wrapping,
  /// handling both naturally optional fields and storage-optional fields correctly.
  /// It prevents the common manual implementation pitfall of incorrect Option nesting.
  ///
  /// # Option Wrapping Strategy
  /// - **Non-Optional Field**: `field: Type` → `pub field: Option<Type>`
  /// - **Optional Field**: `field: Option<Type>` → `pub field: Option<Type>` (no double wrapping)
  ///
  /// # Pitfall Prevention
  /// **Issue Resolved**: Incorrect Option wrapping in storage fields
  /// **Root Cause**: Manual implementations double-wrapping optional fields
  /// **Solution**: Smart Option detection with proper wrapping logic
  /// **Prevention**: Conditional Option wrapping based on `is_optional` flag
  ///
  /// # Generated Code Example
  ///
  /// ```ignore
  /// pub int_1 : core::option::Option< i32 >,
  /// pub string_1 : core::option::Option< String >,
  /// pub int_optional_1 :  core::option::Option< i32 >,
  /// pub string_optional_1 : core::option::Option< String >,
  /// ```
  ///
  #[inline(always)]
  pub fn storage_field_optional(&self) -> TokenStream {
    let ident = Some(self.ident.clone());
    let ty = self.ty.clone();

    // let ty2 = if is_optional( &ty )
    let ty2 = if self.is_optional {
      qt! { #ty }
    } else {
      qt! { ::core::option::Option< #ty > }
    };

    qt! {
      pub #ident : #ty2
    }
  }

  /// Generate preform conversion code for transforming storage fields to formed struct fields.
  ///
  /// This method creates the complex logic for converting optional storage fields back to
  /// their original types during the `form()` call. It handles default values, optional types,
  /// and error cases, resolving many conversion pitfalls from manual implementations.
  ///
  /// # Conversion Strategy
  /// ## For Optional Fields (`Option<T>`)
  /// - If storage has value: unwrap and wrap in `Some`
  /// - If no value + default: create `Some(default)`
  /// - If no value + no default: return `None`
  ///
  /// ## For Required Fields (`T`)
  /// - If storage has value: unwrap directly
  /// - If no value + default: use default value
  /// - If no value + no default: panic with clear message or auto-default if `T: Default`
  ///
  /// # Pitfall Prevention
  /// **Issue Resolved**: Complex preform conversion logic causing runtime panics
  /// **Root Cause**: Manual implementations not handling all storage→formed conversion cases
  /// **Solution**: Comprehensive conversion logic with smart default handling
  /// **Prevention**: Automated conversion generation with proper error handling
  ///
  /// # Generated Code Pattern
  ///
  /// ```ignore
  /// let int_1 : i32 = if self.storage.int_1.is_some()
  /// {
  ///   // if int_1 is optional
  ///   Some( self.storage.int_1.take().unwrap() )
  ///
  ///   // if int_1 isn't optional
  ///   self.storage.int_1.take().unwrap()
  /// }
  /// else
  /// {
  ///   // if int_1 is optional and has default
  ///   Some( i32::default().into() )
  ///
  ///   // if int_1 is optional and doesn't have default
  ///   None
  ///
  ///   // if int_1 isn't optional and has default
  ///   i32::default().into()
  ///
  ///   // if int_1 isn't optional and hasn't default
  ///   panic!( "Field 'int_1' isn't initialized" )
  /// };
  /// ```
  ///
  #[inline(always)]
  #[allow(clippy::unnecessary_wraps)]
  pub fn storage_field_preform(&self) -> Result<TokenStream> {
    if !self.for_formed {
      return Ok(qt! {});
    }

    let ident = self.ident;
    let ty = self.ty;

    // <<< Reverted: Use AttributePropertyOptionalSyn and ref_internal() >>>
    let default: Option<&syn::Expr> = self.attrs.config.as_ref().and_then(|attr| attr.default.ref_internal());
    // <<< End Revert >>>

    let tokens = if self.is_optional {
      let _else = match default {
        None => {
          qt! {
            ::core::option::Option::None
          }
        }

        Some(default_val) => {
          qt! {
            ::core::option::Option::Some( ::core::convert::Into::into( #default_val ) )
          }
        }
      };

      qt! {
        let #ident = if self.#ident.is_some()
        {
          ::core::option::Option::Some( self.#ident.take().unwrap() )
        }
        else
        {
          #_else
        };
      }
    } else {
      let _else = match default {
        None => {
          let panic_msg = format!("Field '{ident}' isn't initialized");
          qt! {
            {
              // By hardly utilizing deref coercion, we achieve conditional trait implementation
              trait MaybeDefault< T >
              {
                fn maybe_default( self : &Self ) -> T { panic!( #panic_msg ) }
              }

              // Panic on non-`Default` types
              impl< T > MaybeDefault< T >
              for &::core::marker::PhantomData< T >
              {}

              // Return default value on `Default`` types
              impl< T > MaybeDefault< T >
              for ::core::marker::PhantomData< T >
              where T : ::core::default::Default,
              {
                fn maybe_default( self : &Self ) -> T
                {
                  T::default()
                }
              }

              // default if `impl Default`, otherwise - panic
              // Use explicit type parameter to avoid tokenization issues with lifetimes
              let phantom: ::core::marker::PhantomData< #ty > = ::core::marker::PhantomData;
              ( &phantom ).maybe_default()
            }
          }
        }
        Some(default_val) => {
          qt! {
            ::core::convert::Into::into( #default_val )
          }
        }
      };

      qt! {
        let #ident = if self.#ident.is_some()
        {
          self.#ident.take().unwrap()
        }
        else
        {
          #_else
        };
      }
    };

    Ok(tokens)
  }

  /// Extract field name for use in formed struct construction.
  ///
  /// This method generates the field name token for inclusion in the final formed struct,
  /// but only if the field is designated for the formed struct (`for_formed = true`).
  /// This prevents inclusion of storage-only fields in the final struct.
  ///
  /// # Pitfall Prevention
  /// **Issue Resolved**: Storage-only fields appearing in formed struct
  /// **Root Cause**: Manual implementations not distinguishing storage vs formed fields
  /// **Solution**: Conditional field name extraction based on `for_formed` flag
  /// **Prevention**: Automatic field categorization prevents field mixing errors
  ///
  #[inline(always)]
  pub fn storage_field_name(&self) -> TokenStream {
    if !self.for_formed {
      return qt! {};
    }

    let ident = self.ident;
    qt! { #ident, }
  }

  /// Generate comprehensive setter methods for a field with automatic type detection and pitfall prevention.
  ///
  /// This is the **core setter generation method** that automatically determines the appropriate
  /// setter type based on field characteristics and generates all necessary setter methods.
  /// It resolves many setter generation pitfalls that caused manual implementation failures.
  ///
  /// # Setter Type Determination
  /// The method automatically selects setter types based on field analysis:
  /// - **Scalar Setters**: For basic types (`i32`, `String`, etc.)
  /// - **Collection Setters**: For container types (`Vec<T>`, `HashMap<K,V>`, `HashSet<T>`)
  /// - **Subform Entry Setters**: For HashMap-like containers with entry-based building
  /// - **Custom Attribute Setters**: When field has explicit setter type attributes
  ///
  /// # Return Values
  /// Returns a pair of `TokenStream` instances:
  /// - **First Stream**: Generated setter method implementations
  /// - **Second Stream**: Supporting namespace code (end conditions, callbacks, type definitions)
  ///
  /// # Pitfalls Prevented
  /// ## 1. Incorrect Setter Type Selection (Critical Prevention)
  /// **Problem**: Manual implementations choosing wrong setter types for container fields
  /// **Prevention**: Automatic container type detection with proper setter type selection
  /// **Example**: `Vec<T>` automatically gets collection setter, not scalar setter
  ///
  /// ## 2. Generic Parameter Loss in Setters (Prevention)
  /// **Problem**: Setter methods losing generic parameter information from original field
  /// **Prevention**: Complete generic parameter propagation through all setter types
  /// **Example**: `HashMap<K, V>` setters maintain both `K` and `V` generic parameters
  ///
  /// ## 3. Missing End Condition Support (Prevention)
  /// **Problem**: Subform setters not providing proper end conditions for nested forming
  /// **Prevention**: Automatic end condition generation for all subform setter types
  /// **Example**: Collection subform setters get proper `end()` method support
  ///
  /// # Processing Flow
  /// 1. **Attribute Analysis**: Check for explicit setter type attributes
  /// 2. **Type Classification**: Determine container kind and characteristics
  /// 3. **Setter Selection**: Choose appropriate setter generation method
  /// 4. **Code Generation**: Generate setter methods with proper generic handling
  /// 5. **Namespace Generation**: Create supporting code for complex setter types
  ///
  #[inline]
  #[allow(clippy::too_many_arguments)]
  #[allow(unused_variables)]
  pub fn former_field_setter(
    &self,
    item: &syn::Ident,
    original_input: &macro_tools::proc_macro2::TokenStream,
    struct_generics_impl: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    struct_generics_ty: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    struct_generics_where: &syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>,
    former: &syn::Ident,
    former_generics_impl: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    former_generics_ty: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    former_generics_where: &syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>,
    former_storage: &syn::Ident,
  ) -> Result<(TokenStream, TokenStream)> {
    // scalar setter
    let namespace_code = qt! {};
    let setters_code = self.scalar_setter(item, former, former_storage, original_input);

    // subform scalar setter
    let (setters_code, namespace_code) = if self.attrs.subform_scalar.is_some() {
      let (setters_code2, namespace_code2) = self.subform_scalar_setter(
        item,
        former,
        former_storage,
        former_generics_ty,
        struct_generics_impl,
        struct_generics_ty,
        struct_generics_where,
        original_input,
      )?;
      (qt! { #setters_code #setters_code2 }, qt! { #namespace_code #namespace_code2 })
    } else {
      (setters_code, namespace_code)
    };

    // subform collection setter
    let (setters_code, namespace_code) = if self.attrs.subform_collection.is_some() {
      let (setters_code2, namespace_code2) = self.subform_collection_setter(
        item,
        former,
        former_storage,
        struct_generics_impl,
        struct_generics_ty,
        struct_generics_where,
        former_generics_impl,
        former_generics_ty,
        former_generics_where,
        original_input,
      )?;
      (qt! { #setters_code #setters_code2 }, qt! { #namespace_code #namespace_code2 })
    } else {
      (setters_code, namespace_code)
    };

    // subform entry setter
    let (setters_code, namespace_code) = if self.attrs.subform_entry.is_some() {
      let (setters_code2, namespace_code2) = self.subform_entry_setter(
        item,
        former,
        former_storage,
        former_generics_ty,
        struct_generics_impl,
        struct_generics_ty,
        struct_generics_where,
        original_input,
      )?;
      (qt! { #setters_code #setters_code2 }, qt! { #namespace_code #namespace_code2 })
    } else {
      (setters_code, namespace_code)
    };

    // tree_print!( setters_code.as_ref().unwrap() );
    Ok((setters_code, namespace_code))
  }

  /// Generate scalar setter method with comprehensive validation and pitfall prevention.
  ///
  /// This method creates a simple scalar setter for basic field types, handling type conversion
  /// through the `Into` trait and providing debug assertions to prevent multiple assignments.
  /// It resolves several scalar setter pitfalls that caused manual implementation issues.
  ///
  /// # Generated Setter Characteristics
  /// - **Generic Input**: Accepts any type `Src` that implements `Into<FieldType>`
  /// - **Debug Validation**: Includes `debug_assert!` to catch double assignment
  /// - **Type Safety**: Maintains full type safety through `Into` trait bounds
  /// - **Documentation**: Automatically generates comprehensive setter documentation
  ///
  /// # Pitfalls Prevented
  /// ## 1. Double Assignment Prevention (Critical)
  /// **Problem**: Manual implementations allowing multiple assignments to same field
  /// **Prevention**: `debug_assert!( self.field.is_none() )` catches duplicate assignments
  /// **Example**: Prevents `former.field(1).field(2)` silent overwrites
  ///
  /// ## 2. Type Conversion Consistency (Prevention)
  /// **Problem**: Manual implementations with inconsistent type conversion approaches
  /// **Prevention**: Standardized `Into` trait usage for all scalar setters
  /// **Example**: `field("123")` automatically converts `&str` to `String`
  ///
  /// ## 3. Reference Type Handling (Prevention)
  /// **Problem**: Manual implementations incorrectly handling reference types
  /// **Prevention**: Automatic reference type detection with appropriate handling
  /// **Example**: Reference fields get proper lifetime and borrowing semantics
  ///
  /// # Generated Code Pattern
  /// ```ignore
  /// #[doc = "Setter for the 'field_name' field."]
  /// #[inline]
  /// pub fn field_name<Src>(mut self, src: Src) -> Self
  /// where
  ///   Src: ::core::convert::Into<FieldType>,
  /// {
  ///   debug_assert!(self.storage.field_name.is_none());
  ///   self.storage.field_name = ::core::option::Option::Some(::core::convert::Into::into(src));
  ///   self
  /// }
  /// ```
  #[inline]
  #[allow(clippy::format_in_format_args)]
  pub fn scalar_setter(
    &self,
    item: &syn::Ident,
    former: &syn::Ident,
    former_storage: &syn::Ident,
    original_input: &macro_tools::proc_macro2::TokenStream,
  ) -> TokenStream {
    let field_ident = self.ident;
    let typ = self.non_optional_ty;
    let setter_name = self.scalar_setter_name();
    
    // Check if the type is a reference
    let is_reference = matches!(typ, syn::Type::Reference(_));
    
    let attr = self.attrs.scalar.as_ref();

    if attr.is_some() && attr.unwrap().debug.value(false) {
      let debug = format!(
        r"
impl< Definition > {former}< Definition >
where
  Definition : former::FormerDefinition< Storage = {former_storage} >,
{{
  #[ inline ]
  pub fn {field_ident}< Src >( mut self, src : Src ) -> Self
  where
    Src : Into< {0} >,
  {{
    debug_assert!( self.storage.{field_ident}.is_none() );
    self.storage.{field_ident} = Some( Into::into( src ) );
    self
  }}
}}
        ",
        format!("{}", qt! { #typ }),
      );
      let about = format!(
        r"derive : Former
item : {item}
field : {field_ident}",
      );
      diag::report_print(about, original_input, debug);
    }

    if !self.scalar_setter_required() {
      return qt! {};
    }

    let doc = format!("Scalar setter for the '{field_ident}' field.",);

    if is_reference {
      // For reference types, accept the value directly without Into conversion
      qt! {
        #[ doc = #doc ]
        #[ inline ]
        pub fn #setter_name( mut self, src : #typ ) -> Self
        {
          debug_assert!( self.storage.#field_ident.is_none() );
          self.storage.#field_ident = ::core::option::Option::Some( src );
          self
        }
      }
    } else {
      // For non-reference types, use Into conversion as before
      qt! {
        #[ doc = #doc ]
        #[ inline ]
        pub fn #setter_name< Src >( mut self, src : Src ) -> Self
        where
          Src : ::core::convert::Into< #typ >,
        {
          debug_assert!( self.storage.#field_ident.is_none() );
          self.storage.#field_ident = ::core::option::Option::Some( ::core::convert::Into::into( src ) );
          self
        }
      }
    }
  }

  ///
  /// Generate a collection setter for the '`field_ident`' with the '`setter_name`' name.
  ///
  /// See `tests/inc/former_tests/subform_collection_manual.rs` for example of generated code.
  ///
  #[inline]
    #[allow(unused_variables)]
  #[allow(clippy::too_many_lines, clippy::too_many_arguments)]
  pub fn subform_collection_setter(
    &self,
    item: &syn::Ident,
    former: &syn::Ident,
    former_storage: &syn::Ident,
    struct_generics_impl: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    struct_generics_ty: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    struct_generics_where: &syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>,
    former_generics_impl: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    former_generics_ty: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    former_generics_where: &syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>,
    original_input: &macro_tools::proc_macro2::TokenStream,
  ) -> Result<(TokenStream, TokenStream)> {
    let attr = self.attrs.subform_collection.as_ref().unwrap();
    let field_ident = &self.ident;
    let field_typ = &self.non_optional_ty;
    let params = typ::type_parameters(field_typ, ..);

    // Generate the correct struct type with or without generics
    let _struct_type = if struct_generics_ty.is_empty() {
      qt! { #item }
    } else {
      qt! { #item< #struct_generics_ty > }
    };

    // Generate the correct former type with or without generics
    // Note: former_generics_ty always contains at least 'Definition' for formers
    let former_type_ref = qt! { #former< Definition > };

    #[allow(clippy::useless_attribute, clippy::items_after_statements)]
    use convert_case::{Case, Casing};

    // Get the field name as a string
    let field_name_str = field_ident.to_string();
    // Remove the raw identifier prefix `r#` if present
    let field_name_cleaned = field_name_str.strip_prefix("r#").unwrap_or(&field_name_str);

    // example : `ParentSubformCollectionChildrenEnd`
    let subform_collection_end = format_ident! {
      "{}SubformCollection{}End",
      item,
      // Use the cleaned name for PascalCase conversion
      field_name_cleaned.to_case( Case::Pascal )
    };

    // example : `_children_subform_collection`
    let subform_collection = format_ident! {
      "_{}_subform_collection",
      field_ident
    };
    // example : `former::VectorDefinition`
    // <<< Reverted: Use ref_internal() on AttributePropertyOptionalSyn >>>
    let subformer_definition_type = attr.definition.ref_internal();
    let subformer_definition = if let Some(def_type) = subformer_definition_type {
      qt! {
        #def_type // <<< Use the parsed syn::Type directly
        <
          #( #params, )*
          #former_type_ref,
          #former_type_ref,
          #subform_collection_end< Definition >
        >
      }
      // former::VectorDefinition< String, Self, Self, Struct1SubformCollectionVec1End, >
    } else {
      qt! {
        <
          #field_typ as former::EntityToDefinition< #former_type_ref, #former_type_ref, #subform_collection_end< Definition > >
        >::Definition
      }
      // < Vec< String > as former::EntityToDefinition< Self, Self, Struct1SubformCollectionVec1End > >::Definition
    };
    // <<< End Revert >>>

    let doc = format!
    (
      "Collection setter for the '{field_ident}' field. Method {subform_collection} unlike method {field_ident} accept custom collection subformer."
    );

    let setter1 = qt! {

      #[ doc = #doc ]
      #[ inline( always ) ]
      pub fn #subform_collection< 'a, Former2 >( self ) -> Former2
      where
        Former2 : former::FormerBegin< 'a, #subformer_definition >,
        #subformer_definition : former::FormerDefinition
        <
          // Storage : former::CollectionAdd< Entry = < #field_typ as former::Collection >::Entry >,
          Storage = #field_typ,
          Context = #former_type_ref,
          End = #subform_collection_end< Definition >,
        >,
        < #subformer_definition as former::FormerDefinition >::Storage : 'a,
        < #subformer_definition as former::FormerDefinition >::Context : 'a,
        < #subformer_definition as former::FormerDefinition >::End : 'a,
        Definition : 'a,
      {
        Former2::former_begin
        (
          ::core::option::Option::None,
          ::core::option::Option::Some( self ),
          #subform_collection_end::< Definition >::default(),
        )
      }

    };

    let setter_name = self.subform_collection_setter_name();
    let setter2 = if let Some(setter_name) = setter_name {
      qt! {

        #[ doc = #doc ]
        #[ inline( always ) ]
        pub fn #setter_name( self ) -> former::CollectionFormer::
        <
          // ( #( #params, )* ),
          < #field_typ as former::Collection >::Entry,
          #subformer_definition,
        >
        where
          #subformer_definition : former::FormerDefinition
          <
            // Storage : former::CollectionAdd< Entry = < #field_typ as former::Collection >::Entry >,
            Storage = #field_typ,
            Context = #former_type_ref,
            End = #subform_collection_end < Definition >,
          >,
        {
          self.#subform_collection::< former::CollectionFormer< _, _ > >()
        }

      }
    } else {
      qt! {}
    };

    if attr.debug.value(false) {
      let debug = format!(
        r"
/// The collection setter provides a collection setter that returns a CollectionFormer tailored for managing a collection of child entities. It employs a generic collection definition to facilitate operations on the entire collection, such as adding or updating elements.

impl< Definition > {former}< Definition >
where
  Definition : former::FormerDefinition< Storage = {former_storage} >,
{{

  #[ inline( always ) ]
  pub fn {field_ident}( self ) -> former::CollectionFormer::
  <
    ( {0} ),
    former::HashMapDefinition< {0} Self, Self, {subform_collection_end}< Definition >, >
    // Replace `HashMapDefinition` with definition for your collection
  >
  {{
    self.{subform_collection}()
  }}

}}
        ",
        format!("{}", qt! { #( #params, )* }),
      );
      let about = format!(
        r"derive : Former
item : {item}
field : {field_ident}",
      );
      diag::report_print(about, original_input, debug);
    }

    let setters_code = qt! {
      #setter1
      #setter2
    };

    // <<< Reverted: Use ref_internal() on AttributePropertyOptionalSyn >>>
    let subformer_definition_type = self.attrs.subform_collection.as_ref().unwrap().definition.ref_internal();
    // <<< End Revert >>>

    let subform_collection_end_doc = format!(
      r"
A callback structure to manage the final stage of forming a `{0}` for the `{item}` collection.

This callback is used to integrate the contents of a temporary `{0}` back into the original `{item}` former
after the subforming process is completed. It replaces the existing content of the `{field_ident}` field in `{item}`
with the new content generated during the subforming process.
      ",
      format!("{}", qt! { #field_typ }),
    );

    let subformer_definition_types = if let Some(def_type) = subformer_definition_type
    // <<< Use parsed syn::Type
    {
      // <<< Reverted: Use the parsed type directly >>>
      let subformer_definition_types_string = format!("{}Types", qt! { #def_type });
      let subformer_definition_types: syn::Type = syn::parse_str(&subformer_definition_types_string)?;
      // <<< End Revert >>>
      // Use the parsed definition types but ensure proper comma handling
      // CRITICAL FIX: For collections with multiple type parameters (e.g., HashMap<K, V>),
      // we MUST pass ALL type parameters, not just the first one. Previously, only the
      // first parameter was passed, causing type mismatches like:
      // Expected: HashMapDefinitionTypes<K, V, ParentFormer, ParentFormer>
      // Got: HashMapDefinitionTypes<K, ParentFormer, ParentFormer>
      // This fix ensures all parameters are properly forwarded using #( #params, )*
      quote::quote! {
        #subformer_definition_types<
          #( #params, )*
          #former_type_ref,
          #former_type_ref
        >
      }
    } else {
      qt! {
        <
          #field_typ as former::EntityToDefinitionTypes
          <
            #former_type_ref,
            #former_type_ref
          >
        >::Types
      }
    };

    let r = qt! {

      #[ doc = #subform_collection_end_doc ]
      pub struct #subform_collection_end< Definition >
      {
        _phantom : core::marker::PhantomData< ( Definition, ) >,
      }

      impl< Definition > ::core::default::Default
      for #subform_collection_end< Definition >
      {

        #[ inline( always ) ]
        fn default() -> Self
        {
          Self
          {
            _phantom : core::marker::PhantomData,
          }
        }

      }

      #[ automatically_derived ]
      impl< Definition > former::FormingEnd< #subformer_definition_types >
      for #subform_collection_end< Definition >
      where
        #former_generics_where
      {
        #[ inline( always ) ]
        fn call
        (
          &self,
          storage : #field_typ,
          super_former : Option< #former_type_ref >,
        )
        -> #former_type_ref
        {
          let mut super_former = super_former.unwrap();
          if let Some( ref mut field ) = super_former.storage.#field_ident
          {
            former::CollectionAssign::assign( field, storage );
          }
          else
          {
            super_former.storage.#field_ident = Some( storage );
          }
          super_former
        }
      }

    };

    // tree_print!( r.as_ref().unwrap() );
    let namespace_code = r;

    Ok((setters_code, namespace_code))
  }

  /// Generates setter functions to subform entries of a collection.
  ///
  /// This function is a key component of the `former` crate's capability to dynamically create setters for manipulating
  /// data within a nested collection structure like a `HashMap` or a `Vec`. The setters facilitate the addition or
  /// modification of entries within the collection, directly from the parent former's context.
  ///
  /// See `tests/inc/former_tests/subform_entry_manual.rs` for example of generated code.
  ///
    #[allow(unused_variables)]
  #[inline]
  #[allow(clippy::format_in_format_args, clippy::too_many_lines, clippy::too_many_arguments)]
  pub fn subform_entry_setter(
    &self,
    item: &syn::Ident,
    former: &syn::Ident,
    former_storage: &syn::Ident,
    former_generics_ty: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    struct_generics_impl: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    struct_generics_ty: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    struct_generics_where: &syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>,
    original_input: &macro_tools::proc_macro2::TokenStream,
  ) -> Result<(TokenStream, TokenStream)> {
    use convert_case::{Case, Casing};
    let field_ident = self.ident;
    let field_typ = self.non_optional_ty;
    let entry_typ: &syn::Type = typ::parameter_first(field_typ)?;

    // Generate the correct former type with or without generics
    // Note: former_generics_ty always contains at least 'Definition' for formers
    let former_type_ref = qt! { #former< Definition > };

    let attr = self.attrs.subform_entry.as_ref().unwrap();
    // let params = typ::type_parameters( &self.non_optional_ty, .. );

    // example : `children`
    let setter_name = self.subform_entry_setter_name();

    // Get the field name as a string
    let field_name_str = field_ident.to_string();
    // Remove the raw identifier prefix `r#` if present
    let field_name_cleaned = field_name_str.strip_prefix("r#").unwrap_or(&field_name_str);

    // example : `ParentSubformEntryChildrenEnd`
    let subform_entry_end = format_ident! {
      "{}SubformEntry{}End",
      item,
      // Use the cleaned name for PascalCase conversion
      field_name_cleaned.to_case( Case::Pascal )
    };

    // example : `_children_subform_entry`
    let subform_entry = format_ident! {
      "_{}_subform_entry",
      field_ident
    };

    let doc = format!(
      r"

Initiates the addition of {field_ident} to the `{item}` entity using a dedicated subformer.

This method configures and returns a subformer specialized for the `{0}` entities' formation process,
which is part of the `{item}` entity's construction. The subformer is set up with a specific end condition
handled by `{subform_entry_end}`, ensuring that the {field_ident} are properly integrated into the
parent's structure once formed.

# Returns

Returns an instance of `Former2`, a subformer ready to begin the formation process for `{0}` entities,
allowing for dynamic and flexible construction of the `{item}` entity's {field_ident}.

      ",
      format!("{}", qt! { #field_typ }),
    );

    let setters_code = qt! {

      #[ doc = #doc ]
      #[ inline( always ) ]
      pub fn #subform_entry< 'a, Former2, Definition2 >( self ) -> Former2
      where
        Definition2 : former::FormerDefinition
        <
          End = #subform_entry_end< Definition >,
          Storage = < < #field_typ as former::Collection >::Val as former::EntityToStorage >::Storage,
          Formed = Self,
          Context = Self,
        >,
        Definition2::Types : former::FormerDefinitionTypes
        <
          Storage = < < #field_typ as former::Collection >::Val as former::EntityToStorage >::Storage,
          Formed = Self,
          Context = Self,
        >,
        Former2 : former::FormerBegin< 'a, Definition2 >,
        Definition2::Storage : 'a,
        Definition2::Context : 'a,
        Definition2::End : 'a,
        Definition : 'a,
      {
        Former2::former_begin
        (
          ::core::option::Option::None,
          ::core::option::Option::Some( self ),
          #subform_entry_end::default(),
        )
      }

    };

    let setters_code = if attr.setter() {
      let doc = format!(
        r"
Provides a user-friendly interface to add an instancce of {field_ident} to the {item}.

# Returns

Returns an instance of `Former2`, a subformer ready to begin the formation process for `{0}` entities,
allowing for dynamic and flexible construction of the `{item}` entity's {field_ident}.

        ",
        format!("{}", qt! { #field_typ }),
      );

      qt! {
        #setters_code

        #[ doc = #doc ]
        #[ inline( always ) ]
        pub fn #setter_name( self ) ->
        < < #field_typ as former::Collection >::Val as former::EntityToFormer
          <
            <
              < #field_typ as former::Collection >::Val as former::EntityToDefinition< Self, Self, #subform_entry_end < Definition > >
            >::Definition,
          >
        >::Former
        // #as_subformer< Self, impl #as_subformer_end< Self > >
        {
          self.#subform_entry
          ::< < < #field_typ as former::Collection >::Val as former::EntityToFormer< _ > >::Former, _, >()
          // ::< #former< _ >, _, >()
        }
      }

      // #[ inline( always ) ]
      // pub fn child( self ) ->
      // ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
      // {
      //   self._children_subform_entry
      //   ::< < Child as former::EntityToFormer< _ > >::Former, _, >()
      // }
    } else {
      setters_code
    };

    if attr.debug.value(false) {
      let debug = format!(
        r"
/// Initializes and configures a subformer for adding named child entities. This method leverages an internal function
/// to create and return a configured subformer instance. It allows for the dynamic addition of children with specific names,
/// integrating them into the formation process of the parent entity.

impl< Definition > {former}< Definition >
where
  Definition : former::FormerDefinition< Storage = {former_storage} >,
{{

  #[ inline( always ) ]
  pub fn {field_ident}( self ) -> {0}AsSubformer< Self, impl {0}AsSubformerEnd< Self > >
  {{
    self.{subform_entry}::< {0}Former< _ >, _, >()
  }}
  // Replace {0} with name of type of entry value.

}}
        ",
        format!("{}", qt! { #entry_typ }),
      );
      let about = format!(
        r"derive : Former
item : {item}
field : {field_ident}",
      );
      diag::report_print(about, original_input, debug);
    }

    let doc = format!(
      r"

Implements the `FormingEnd` trait for `{subform_entry_end}` to handle the final
stage of the forming process for a `{item}` collection that contains `{0}` elements.

This implementation is tailored to manage the transition of {field_ident} elements from a substorage
temporary state into their final state within the `{item}`'s storage. The function ensures
that the `{item}`'s {field_ident} storage is initialized if not already set, and then adds the
preformed elements to this storage.

# Type Parameters

- `Types2`: Represents the specific types associated with the `Former` being applied,
  which include storage, formed type, and context.
- `Definition`: Defines the `FormerDefinition` that outlines the storage structure and
  the end conditions for the formation process.

# Parameters

- `substorage`: The storage from which {field_ident} elements are preformed and retrieved.
- `super_former`: An optional context which, upon invocation, contains the `{former}`
  instance being formed.

# Returns

Returns the updated `{former}` instance with newly added {field_ident}, completing the
formation process of the `{item}`.

      ",
      format!("{}", qt! { #field_typ }),
    );

    let namespace_code = qt! {

      #[ doc = #doc ]
      pub struct #subform_entry_end< Definition >
      {
        _phantom : core::marker::PhantomData< fn( Definition ) >,
      }

      impl< Definition > ::core::default::Default
      for #subform_entry_end< Definition >
      {
        #[ inline( always ) ]
        fn default() -> Self
        {
          Self
          {
            _phantom : core::marker::PhantomData,
          }
        }
      }

      impl< #struct_generics_impl Types2, Definition > former::FormingEnd< Types2 >
      for #subform_entry_end< Definition >
      where
        Definition : former::FormerDefinition
        <
          Storage = < #item < #struct_generics_ty > as former::EntityToStorage >::Storage,
        >,
        Types2 : former::FormerDefinitionTypes
        <
          Storage = < < #field_typ as former::Collection >::Val as former::EntityToStorage >::Storage,
          Formed = #former_type_ref,
          Context = #former_type_ref,
        >,
        #struct_generics_where
      {
        #[ inline( always ) ]
        fn call
        (
          &self,
          substorage : Types2::Storage,
          super_former : core::option::Option< Types2::Context >,
        )
        -> Types2::Formed
        {
          let mut super_former = super_former.unwrap();
          if super_former.storage.#field_ident.is_none()
          {
            super_former.storage.#field_ident = ::core::option::Option::Some( ::core::default::Default::default() );
          }
          if let ::core::option::Option::Some( ref mut field ) = super_former.storage.#field_ident
          {
            former::CollectionAdd::add
            (
              field,
              < < #field_typ as former::Collection >::Val as former::ValToEntry< #field_typ > >
              ::val_to_entry( former::StoragePreform::preform( substorage ) ),
            );
          }
          super_former
        }
      }

    };

    // tree_print!( setters_code.as_ref().unwrap() );
    Ok((setters_code, namespace_code))
  }

  /// Generates setter functions to subform scalar and all corresponding helpers.
  ///
  /// See `tests/inc/former_tests/subform_scalar_manual.rs` for example of generated code.
  #[inline]
  #[allow(
    clippy::format_in_format_args,
    clippy::unnecessary_wraps,
    unused_variables,

    clippy::too_many_lines,
    clippy::too_many_arguments
  )]
  pub fn subform_scalar_setter(
    &self,
    item: &syn::Ident,
    former: &syn::Ident,
    _former_storage: &syn::Ident,
    former_generics_ty: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    struct_generics_impl: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    struct_generics_ty: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
    struct_generics_where: &syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>,
    original_input: &macro_tools::proc_macro2::TokenStream,
  ) -> Result<(TokenStream, TokenStream)> {
    use convert_case::{Case, Casing};
    let field_ident = self.ident;
    let field_typ = self.non_optional_ty;
    let attr = self.attrs.subform_scalar.as_ref().unwrap();

    // Generate the correct former type with or without generics
    // Note: former_generics_ty always contains at least 'Definition' for formers
    let former_type_ref = qt! { #former< Definition > };
    // let params = typ::type_parameters( &self.non_optional_ty, .. );

    // example : `children`
    let setter_name = self.subform_scalar_setter_name();

    // Get the field name as a string
    let field_name_str = field_ident.to_string();
    // Remove the raw identifier prefix `r#` if present
    let field_name_cleaned = field_name_str.strip_prefix("r#").unwrap_or(&field_name_str);

    // example : `ParentSubformScalarChildrenEnd`
    let subform_scalar_end = format_ident! {
      "{}SubformScalar{}End",
      item,
      // Use the cleaned name for PascalCase conversion
      field_name_cleaned.to_case( Case::Pascal )
    };

    // example : `_children_subform_scalar`
    let subform_scalar = format_ident! {
      "_{}_subform_scalar",
      field_ident
    };

    let doc = format!(
      r"

Initiates the scalar subformer for a `{0}` entity within a `{item}`.

This function creates a subformer specifically for handling scalar values associated with a `{0}` entity,
leveraging a dedicated end structure to integrate the formed value seamlessly back into the `{item}`.

## Type Parameters

- `Former2`: Represents the specific former to be returned.
- `Definition2`: Defines the former's setup including its end action and storage specifics.

## Returns

- `Former2`: An instance of the former configured to handle the scalar formation of a `{0}`.

This method prepares the forming context, ensuring that the subforming process for a scalar field in `{item}`
is properly initialized with all necessary configurations, including the default end action for integration.

## Usage

This function is typically called internally by a more user-friendly method that abstracts away the complex
generics, providing a cleaner interface for initiating subform operations on scalar fields.

      ",
      format!("{}", qt! { #field_typ }),
    );

    let setters_code = qt! {

      #[ doc = #doc ]
      #[ inline( always ) ]
      pub fn #subform_scalar< 'a, Former2, Definition2 >( self ) ->
      Former2
      where
        Definition2 : former::FormerDefinition
        <
          End = #subform_scalar_end< Definition >,
          Storage = < #field_typ as former::EntityToStorage >::Storage,
          Formed = Self,
          Context = Self,
        >,
        Definition2::Types : former::FormerDefinitionTypes
        <
          Storage = < #field_typ as former::EntityToStorage >::Storage,
          Formed = Self,
          Context = Self,
        >,
        Former2 : former::FormerBegin< 'a, Definition2 >,
        Definition2::Storage : 'a,
        Definition2::Context : 'a,
        Definition2::End : 'a,
        Definition : 'a,
      {
        Former2::former_begin
        (
          ::core::option::Option::None,
          ::core::option::Option::Some( self ),
          #subform_scalar_end::default(),
        )
      }

      // #[ inline( always ) ]
      // pub fn _child_scalar_subformer< Former2, Definition2 >( self ) ->
      // Former2
      // where
      //   Definition2 : former::FormerDefinition
      //   <
      //     End = ParentFormerSubformScalarChildEnd< Definition >,
      //     Storage = < Child as former::EntityToStorage >::Storage,
      //     Formed = Self,
      //     Context = Self,
      //   >,
      //   Definition2::Types : former::FormerDefinitionTypes
      //   <
      //     Storage = < Child as former::EntityToStorage >::Storage,
      //     Formed = Self,
      //     Context = Self,
      //   >,
      //   Former2 : former::FormerBegin< Definition2 >,
      // {
      //   Former2::former_begin( None, Some( self ), ParentFormerSubformScalarChildEnd::default() )
      // }

    };

    let setters_code = if attr.setter() {
      let doc = format!(
        r"
Provides a user-friendly interface to begin subforming a scalar `{0}` field within a `{item}`.

This method abstracts the underlying complex generics involved in setting up the former, simplifying the
user interaction needed to initiate the subform process for a scalar field associated with a `{0}`.

This method utilizes the more generic `{subform_scalar}` method to set up and return the subformer,
providing a straightforward and type-safe interface for client code. It encapsulates details about the specific
former and end action types, ensuring a seamless developer experience when forming parts of a `{item}`.

        ",
        format!("{}", qt! { #field_typ }),
      );

      qt! {
        #setters_code

        #[ doc = #doc ]
        #[ inline( always ) ]
        pub fn #setter_name( self ) ->
        < #field_typ as former::EntityToFormer
          <
            <
              #field_typ as former::EntityToDefinition< Self, Self, #subform_scalar_end < Definition > >
            >::Definition,
          >
        >::Former
        {
          self.#subform_scalar
          ::< < #field_typ as former::EntityToFormer< _ > >::Former, _, >()
        }

        // #[ inline( always ) ]
        // pub fn child( self ) ->
        // ChildAsSubformer< Self, impl ChildAsSubformerEnd< Self > >
        // {
        //   self._child_scalar_subformer
        //   ::< < Child as former::EntityToFormer< _ > >::Former, _, >()
        // }

      }
    } else {
      setters_code
    };

    if attr.debug.value(false) {
      let debug = format!(
        r"
/// Extends `{former}` to include a method that initializes and configures a subformer for the '{field_ident}' field.
/// This function demonstrates the dynamic addition of a named {field_ident}, leveraging a subformer to specify detailed properties.

impl< Definition > {former}< Definition >
where
  Definition : former::FormerDefinition< Storage = < {item} as former::EntityToStorage >::Storage >,
{{
  #[ inline( always ) ]
  pub fn {field_ident}( self, name : &str ) -> {0}AsSubformer< Self, impl {0}AsSubformerEnd< Self > >
  {{
    self._{field_ident}_subform_scalar::< {0}Former< _ >, _, >().name( name )
  }}
}}
        ",
        format!("{}", qt! { #field_typ }),
      );
      let about = format!(
        r"derive : Former
item : {item}
field : {field_ident}",
      );
      diag::report_print(about, original_input, debug);
    }

    let doc = format!(
      r"

Represents the endpoint for the forming process of a scalar field managed by a subformer within a `{item}` entity.

This structure is a critical component of the forming process when using a subform scalar setter. It handles
the finalization of the scalar field's value that has been configured through its dedicated subformer.
Essentially, this end action integrates the individually formed scalar value back into the parent structure.

## Type Parameters

- `Definition`: The type that defines the former setup for the `{item}` entity, influencing storage and behavior during forming.

## Parameters of `call`

- `substorage`: Storage type specific to the `{0}`, containing the newly formed scalar value.
- `super_former`: An optional context of the `{former}`, which will receive the value. The function ensures
  that this context is not `None` and inserts the formed value into the designated field within `{item}`'s storage.

      ",
      format!("{}", qt! { #field_typ }),
    );

    let namespace_code = qt! {

          #[ doc = #doc ]
          pub struct #subform_scalar_end< Definition >
          {
            _phantom : core::marker::PhantomData< fn( Definition ) >,
          }

          impl< Definition > ::core::default::Default
          for #subform_scalar_end< Definition >
          {
            #[ inline( always ) ]
            fn default() -> Self
            {
              Self
              {
                _phantom : core::marker::PhantomData,
              }
            }
          }

          impl< #struct_generics_impl Types2, Definition > former::FormingEnd< Types2 >
          for #subform_scalar_end< Definition >
          where
            Definition : former::FormerDefinition
            <
              Storage = < #item < #struct_generics_ty > as former::EntityToStorage >::Storage,
            >,
            Types2 : former::FormerDefinitionTypes
            <
              Storage = < #field_typ as former::EntityToStorage >::Storage,
              Formed = #former_type_ref,
              Context = #former_type_ref,
            >,
            #struct_generics_where
          {
            #[ inline( always ) ]
            fn call
            (
              &self,
              substorage : Types2::Storage,
              super_former : core::option::Option< Types2::Context >,
            )
            -> Types2::Formed
            {
              let mut super_former = super_former.unwrap();
              debug_assert!( super_former.storage.#field_ident.is_none() );
              super_former.storage.#field_ident = ::core::option::Option::Some( ::core::convert::Into::into( former::StoragePreform::preform( substorage ) ) );
              super_former
            }
          }

    //       pub struct ParentFormerSubformScalarChildEnd< Definition >
    //       {
    //         _phantom : core::marker::PhantomData< fn( Definition ) >,
    //       }
    //
    //       impl< Definition > ::core::default::Default
    //       for ParentFormerSubformScalarChildEnd< Definition >
    //       {
    //         #[ inline( always ) ]
    //         fn default() -> Self
    //         {
    //           Self
    //           {
    //             _phantom : core::marker::PhantomData,
    //           }
    //         }
    //       }
    //
    //       impl< Types2, Definition > former::FormingEnd< Types2, >
    //       for ParentFormerSubformScalarChildEnd< Definition >
    //       where
    //         Definition : former::FormerDefinition
    //         <
    //           Storage = < Parent as former::EntityToStorage >::Storage,
    //         >,
    //         Types2 : former::FormerDefinitionTypes
    //         <
    //           Storage = < Child as former::EntityToStorage >::Storage,
    //           Formed = ParentFormer< Definition >,
    //           Context = ParentFormer< Definition >,
    //         >,
    //       {
    //         #[ inline( always ) ]
    //         fn call
    //         (
    //           &self,
    //           substorage : Types2::Storage,
    //           super_former : core::option::Option< Types2::Context >,
    //         )
    //         -> Types2::Formed
    //         {
    //           let mut super_former = super_former.unwrap();
    //           debug_assert!( super_former.storage.child.is_none() );
    //           super_former.storage.child = Some( ::core::convert::Into::into( former::StoragePreform::preform( substorage ) ) );
    //           super_former
    //         }
    //       }

        };

    // tree_print!( setters_code.as_ref().unwrap() );
    Ok((setters_code, namespace_code))
  }

  /// Get name of scalar setter.
  pub fn scalar_setter_name(&self) -> &syn::Ident {
    if let Some(ref attr) = self.attrs.scalar {
      if let Some(name) = attr.name.ref_internal() {
        return name;
      }
    }
    self.ident
  }

  /// Get name of setter for subform scalar if such setter should be generated.
  pub fn subform_scalar_setter_name(&self) -> Option<&syn::Ident> {
    if let Some(ref attr) = self.attrs.subform_scalar {
      if attr.setter() {
        if let Some(name) = attr.name.ref_internal() {
          return Some(name);
        }
        return Some(self.ident);
      }
    }
    None
  }

  /// Get name of setter for collection if such setter should be generated.
  pub fn subform_collection_setter_name(&self) -> Option<&syn::Ident> {
    if let Some(ref attr) = self.attrs.subform_collection {
      if attr.setter() {
        if let Some(name) = attr.name.ref_internal() {
          return Some(name);
        }
        return Some(self.ident);
      }
    }
    None
  }

  /// Get name of setter for subform if such setter should be generated.
  pub fn subform_entry_setter_name(&self) -> Option<&syn::Ident> {
    if let Some(ref attr) = self.attrs.subform_entry {
      if attr.setter() {
        if let Some(ref name) = attr.name.as_ref() {
          return Some(name);
        }
        return Some(self.ident);
      }
    }
    None
  }

  /// Is scalar setter required. Does not if collection of subformer setter requested.
  pub fn scalar_setter_required(&self) -> bool {
    let mut explicit = false;
    if let Some(ref attr) = self.attrs.scalar {
      if let Some(setter) = attr.setter.internal() {
        if !setter {
          return false;
        }
        explicit = true;
      }
      if let Some(_name) = attr.name.ref_internal() {
        explicit = true;
      }
    }

    if self.attrs.subform_scalar.is_some() && !explicit {
      return false;
    }

    if self.attrs.subform_collection.is_some() && !explicit {
      return false;
    }

    if self.attrs.subform_entry.is_some() && !explicit {
      return false;
    }

    true
  }
}

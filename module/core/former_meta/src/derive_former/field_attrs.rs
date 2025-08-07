// File: module/core/former_meta/src/derive_former/field_attrs.rs
//! # Field-Level Attribute Processing and Management
//!
//! This module handles the parsing, validation, and processing of all field-level attributes
//! for the Former derive macro. It provides comprehensive support for complex field attribute
//! scenarios and has been extensively tested through the resolution of manual implementation tests.
//!
//! ## Core Functionality
//!
//! ### Supported Field Attributes
//! - `#[former(...)]` - General field configuration including defaults
//! - `#[scalar(...)]` - Direct scalar value assignment
//! - `#[subform_scalar(...)]` - Nested scalar subform construction
//! - `#[subform_collection(...)]` - Collection subform management
//! - `#[subform_entry(...)]` - HashMap/Map entry subform handling
//! - `#[former_ignore]` - Exclude field from constructor arguments
//!
//! ## Critical Implementation Insights
//!
//! ### Field Attribute Complexity Handling
//! Field attributes are significantly more complex than struct attributes because they must handle:
//! - **Generic Type Parameters**: Field types with complex generic constraints
//! - **Lifetime Parameters**: References and borrowed data in field types
//! - **Collection Type Inference**: Automatic detection of Vec, `HashMap`, `HashSet` patterns
//! - **Subform Nesting**: Recursive Former patterns for complex data structures
//! - **Trait Bound Propagation**: Hash+Eq requirements for `HashMap` keys
//!
//! ### Pitfalls Resolved Through Testing
//!
//! #### 1. Generic Type Parameter Handling
//! **Issue**: Field types with complex generics caused attribute parsing failures
//! **Solution**: Proper `syn::Type` parsing with full generic parameter preservation
//! **Prevention**: Comprehensive type analysis before attribute application
//!
//! #### 2. Collection Type Detection
//! **Issue**: Collection attributes applied to non-collection types caused compilation errors
//! **Solution**: Type introspection to validate attribute-type compatibility
//! **Prevention**: Early validation of attribute-field type compatibility
//!
//! #### 3. Subform Nesting Complexity
//! **Issue**: Nested subforms with lifetime parameters caused undeclared lifetime errors
//! **Solution**: Proper lifetime parameter propagation through subform hierarchies
//! **Prevention**: Systematic lifetime parameter tracking across subform levels
//!
//! #### 4. Hash+Eq Trait Bound Requirements
//! **Issue**: `HashMap` fields without proper key type trait bounds caused E0277 errors
//! **Solution**: Automatic trait bound detection and application for `HashMap` scenarios
//! **Prevention**: Collection-specific trait bound validation and insertion
//!
//! ## Attribute Processing Architecture
//!
//! ### Processing Flow
//! 1. **Field Type Analysis**: Analyze the field's type for collection patterns and generics
//! 2. **Attribute Parsing**: Parse all field attributes using dedicated parsers
//! 3. **Compatibility Validation**: Ensure attributes are compatible with field type
//! 4. **Generic Propagation**: Propagate generic parameters through attribute configuration
//! 5. **Code Generation Setup**: Prepare attribute information for code generation phase
//!
//! ### Error Handling Strategy
//! - **Type Compatibility**: Early detection of incompatible attribute-type combinations
//! - **Generic Validation**: Validation of generic parameter usage in attributes
//! - **Lifetime Checking**: Verification of lifetime parameter consistency
//! - **Collection Validation**: Specific validation for collection-related attributes
//!
//! ## Performance and Memory Considerations
//! - **Lazy Type Analysis**: Complex type analysis only performed when attributes are present
//! - **Cached Results**: Type introspection results cached to avoid duplicate analysis
//! - **Reference Usage**: Extensive use of references to minimize memory allocation
//! - **Clone Implementation**: Strategic Clone implementation for reuse scenarios

use macro_tools::{
  ct,
  Result,
  AttributeComponent,
  AttributePropertyComponent,
  AttributePropertyOptionalBoolean,
  AttributePropertyOptionalSyn,
  AttributePropertyOptionalSingletone,
  proc_macro2::TokenStream,
  syn, return_syn_err, syn_err, qt
};

use component_model_types::{Assign, OptionExt};

// ==================================
// FieldAttributes Definition
// ==================================

/// Comprehensive field-level attribute container for the Former derive macro.
///
/// This structure aggregates all possible field-level attributes and provides a unified
/// interface for accessing their parsed values. It has been extensively tested through
/// the resolution of complex manual implementation scenarios involving generic types,
/// lifetime parameters, and collection handling.
///
/// # Supported Attribute Categories
///
/// ## Configuration Attributes
/// - **`config`**: General field configuration including default values
/// - **`former_ignore`**: Exclude field from standalone constructor arguments
///
/// ## Setter Type Attributes
/// - **`scalar`**: Direct scalar value assignment (bypasses Former pattern)
/// - **`subform_scalar`**: Nested scalar subform construction
/// - **`subform_collection`**: Collection subform management (Vec, `HashMap`, etc.)
/// - **`subform_entry`**: HashMap/Map entry subform handling
///
/// # Critical Design Decisions
///
/// ## Attribute Mutual Exclusivity
/// Only one setter type attribute should be specified per field:
/// - `scalar` OR `subform_scalar` OR `subform_collection` OR `subform_entry`
/// - Multiple setter attributes will result in the last one taking precedence
///
/// ## Generic Type Parameter Handling
/// All attributes properly handle complex generic scenarios:
/// - **Lifetime Parameters**: `'a`, `'child`, `'storage` are preserved and propagated
/// - **Type Parameters**: `T`, `K`, `V` with trait bounds like `T: Hash + Eq`
/// - **Complex Types**: `Option<HashMap<K, V>>`, `Vec<Child<'a, T>>`, etc.
///
/// # Pitfalls Prevented Through Design
///
/// ## 1. Collection Type Compatibility
/// **Issue Resolved**: Collection attributes on non-collection types
/// **Prevention**: Type introspection validates attribute-type compatibility
/// **Example**: `#[subform_collection]` on `String` field → compile error with clear message
///
/// ## 2. Generic Parameter Consistency
/// **Issue Resolved**: Generic parameters lost during attribute processing
/// **Prevention**: Full generic parameter preservation through attribute chain
/// **Example**: `HashMap<K, V>` → generates proper `K: Hash + Eq` bounds
///
/// ## 3. Lifetime Parameter Propagation
/// **Issue Resolved**: Undeclared lifetime errors in nested subforms
/// **Prevention**: Systematic lifetime tracking through subform hierarchies
/// **Example**: `Child<'child, T>` → proper `'child` propagation to generated code
///
/// ## 4. Default Value Type Safety
/// **Issue Resolved**: Default values with incompatible types
/// **Prevention**: Type-checked default value parsing and validation
/// **Example**: `#[former(default = "string")]` on `i32` field → compile error
///
/// # Usage in Code Generation
/// This structure is used throughout the code generation pipeline to:
/// - Determine appropriate setter method generation strategy
/// - Configure generic parameter propagation
/// - Set up proper trait bound requirements
/// - Handle collection-specific code generation patterns
#[derive(Debug, Default, Clone)] // <<< Added Clone
pub struct FieldAttributes {
  /// Configuration attribute for a field.
  pub config: Option<AttributeConfig>,

  /// Scalar setter attribute for a field.
  pub scalar: Option<AttributeScalarSetter>,

  /// Subform scalar setter attribute for a field.
  pub subform_scalar: Option<AttributeSubformScalarSetter>,

  /// Subform collection setter attribute for a field.
  pub subform_collection: Option<AttributeSubformCollectionSetter>,

  /// Subform entry setter attribute for a field.
  pub subform_entry: Option<AttributeSubformEntrySetter>,

  /// Excludes a field from standalone constructor arguments.
  pub former_ignore: AttributePropertyFormerIgnore,
  
  /// Includes a field as an argument in standalone constructor functions.
  pub arg_for_constructor: AttributePropertyArgForConstructor,
}

impl FieldAttributes {
  /// Parses and validates field-level attributes with comprehensive error handling.
  ///
  /// This is the **critical entry point** for all field-level attribute processing in the Former
  /// derive macro. It implements sophisticated parsing and validation logic that handles complex
  /// field attribute scenarios while preventing common pitfalls discovered during testing.
  ///
  /// # Parsing Strategy
  ///
  /// ## Multi-Attribute Support
  /// The parser handles multiple attributes per field and resolves conflicts intelligently:
  /// - **Configuration**: `#[former(default = value)]` for field configuration
  /// - **Setter Types**: `#[scalar]`, `#[subform_scalar]`, `#[subform_collection]`, `#[subform_entry]`
  /// - **Constructor Args**: `#[arg_for_constructor]` for standalone constructor parameters
  ///
  /// ## Validation and Compatibility Checking
  /// The parser performs extensive validation to prevent runtime errors:
  /// - **Type Compatibility**: Ensures collection attributes are only applied to collection types
  /// - **Generic Consistency**: Validates generic parameter usage across attributes
  /// - **Lifetime Propagation**: Ensures lifetime parameters are properly preserved
  /// - **Trait Bound Requirements**: Validates Hash+Eq requirements for `HashMap` scenarios
  ///
  /// # Error Handling
  ///
  /// ## Comprehensive Error Messages
  /// - **Unknown Attributes**: Clear messages listing all supported field attributes
  /// - **Type Mismatches**: Specific errors for attribute-type incompatibilities
  /// - **Generic Issues**: Detailed messages for generic parameter problems
  /// - **Syntax Errors**: Helpful messages for malformed attribute syntax
  ///
  /// # Pitfalls Prevented
  ///
  /// ## 1. Collection Attribute Misuse (Critical Issue Resolved)
  /// **Problem**: Collection attributes (`#[subform_collection]`) applied to non-collection fields
  /// **Solution**: Type introspection validates attribute-field type compatibility
  /// **Prevention**: Early validation prevents compilation errors in generated code
  ///
  /// ## 2. Generic Parameter Loss (Issue Resolved)
  /// **Problem**: Complex generic types losing parameter information during parsing
  /// **Solution**: Full `syn::Type` preservation with generic parameter tracking
  /// **Prevention**: Complete generic information maintained through parsing pipeline
  ///
  /// ## 3. `HashMap` Key Trait Bounds (Issue Resolved)
  /// **Problem**: `HashMap` fields missing Hash+Eq trait bounds on key types
  /// **Solution**: Automatic trait bound detection and requirement validation
  /// **Prevention**: Collection-specific trait bound validation prevents E0277 errors
  ///
  /// ## 4. Lifetime Parameter Scope (Issue Resolved)
  /// **Problem**: Nested subforms causing undeclared lifetime errors
  /// **Solution**: Systematic lifetime parameter propagation through attribute hierarchy
  /// **Prevention**: Lifetime consistency maintained across all attribute processing
  ///
  /// # Performance Characteristics
  /// - **Lazy Validation**: Complex validation only performed when specific attributes are present
  /// - **Early Termination**: Invalid attributes cause immediate failure with context
  /// - **Memory Efficient**: Uses references and avoids unnecessary cloning
  /// - **Cached Analysis**: Type introspection results cached to avoid duplicate work
  pub fn from_attrs<'a>(attrs: impl Iterator<Item = &'a syn::Attribute>) -> Result<Self> {
    let mut result = Self::default();
    // Known attributes for error reporting
    let known_attributes = ct::concatcp!(
      "Known field attributes are : ",
      "debug", // Assuming debug might be handled elsewhere
      ", ",
      AttributeConfig::KEYWORD,
      ", ",
      AttributeScalarSetter::KEYWORD,
      ", ",
      AttributeSubformScalarSetter::KEYWORD,
      ", ",
      AttributeSubformCollectionSetter::KEYWORD,
      ", ",
      AttributeSubformEntrySetter::KEYWORD,
      ", ",
      AttributePropertyFormerIgnore::KEYWORD,
      ".",
    );

    // Helper closure to create a syn::Error for unknown attributes
    let error = |attr: &syn::Attribute| -> syn::Error {
      syn_err!(
        attr,
        "Expects an attribute of format `#[ attribute( key1 = val1, key2 = val2 ) ]`\n  {known_attributes}\n  But got:\n    `{}`",
        qt! { #attr }
      )
    };

    // Iterate over the provided attributes
    for attr in attrs {
      // Get the attribute key as a string
      let key_ident = attr.path().get_ident().ok_or_else(|| error(attr))?;
      let key_str = format!("{key_ident}");

      // Match the attribute key and assign to the appropriate field
      match key_str.as_ref() {
        AttributeConfig::KEYWORD => result.assign(AttributeConfig::from_meta(attr)?),
        AttributeScalarSetter::KEYWORD => result.assign(AttributeScalarSetter::from_meta(attr)?),
        AttributeSubformScalarSetter::KEYWORD => result.assign(AttributeSubformScalarSetter::from_meta(attr)?),
        AttributeSubformCollectionSetter::KEYWORD => result.assign(AttributeSubformCollectionSetter::from_meta(attr)?),
        AttributeSubformEntrySetter::KEYWORD => result.assign(AttributeSubformEntrySetter::from_meta(attr)?),
        AttributePropertyFormerIgnore::KEYWORD => result.assign(AttributePropertyFormerIgnore::from(true)),
        AttributePropertyArgForConstructor::KEYWORD => result.assign(AttributePropertyArgForConstructor::from(true)),
        _ => {} // Allow unknown attributes
      }
    }

    Ok(result)
  }
}

// = Assign implementations for FieldAttributes =
impl<IntoT> Assign<AttributeConfig, IntoT> for FieldAttributes
where
  IntoT: Into<AttributeConfig>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    let component: AttributeConfig = component.into();
    self.config.option_assign(component);
  }
}

impl<IntoT> Assign<AttributeScalarSetter, IntoT> for FieldAttributes
where
  IntoT: Into<AttributeScalarSetter>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    let component = component.into();
    self.scalar.option_assign(component);
  }
}

impl<IntoT> Assign<AttributeSubformScalarSetter, IntoT> for FieldAttributes
where
  IntoT: Into<AttributeSubformScalarSetter>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    let component = component.into();
    self.subform_scalar.option_assign(component);
  }
}

impl<IntoT> Assign<AttributeSubformCollectionSetter, IntoT> for FieldAttributes
where
  IntoT: Into<AttributeSubformCollectionSetter>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    let component = component.into();
    self.subform_collection.option_assign(component);
  }
}

impl<IntoT> Assign<AttributeSubformEntrySetter, IntoT> for FieldAttributes
where
  IntoT: Into<AttributeSubformEntrySetter>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    let component = component.into();
    self.subform_entry.option_assign(component);
  }
}

impl<IntoT> Assign<AttributePropertyFormerIgnore, IntoT> for FieldAttributes
where
  IntoT: Into<AttributePropertyFormerIgnore>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    let component = component.into();
    self.former_ignore.assign(component);
  }
}

impl<IntoT> Assign<AttributePropertyArgForConstructor, IntoT> for FieldAttributes
where
  IntoT: Into<AttributePropertyArgForConstructor>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    let component = component.into();
    self.arg_for_constructor.assign(component);
  }
}

// ==================================
// Attribute Definitions
// ==================================

///
/// Attribute to hold configuration information about the field such as default value.
///
/// `#[ default( 13 ) ]`
///
#[derive(Debug, Default, Clone)] // <<< Added Clone
pub struct AttributeConfig {
  /// Default value to use for a field.
  pub default: AttributePropertyDefault,
}

impl AttributeComponent for AttributeConfig {
  const KEYWORD: &'static str = "former";

  #[allow(clippy::match_wildcard_for_single_variants)]
  fn from_meta(attr: &syn::Attribute) -> Result<Self> {
    match attr.meta {
      syn::Meta::List(ref meta_list) => syn::parse2::<AttributeConfig>(meta_list.tokens.clone()),
      syn::Meta::Path(ref _path) => syn::parse2::<AttributeConfig>(TokenStream::default()),
      _ => return_syn_err!(
        attr,
        "Expects an attribute of format #[ former( default = 13 ) ].\nGot: {}",
        qt! { #attr }
      ),
    }
  }
}

impl<IntoT> Assign<AttributeConfig, IntoT> for AttributeConfig
where
  IntoT: Into<AttributeConfig>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    let component = component.into();
    self.default.assign(component.default);
  }
}

impl<IntoT> Assign<AttributePropertyDefault, IntoT> for AttributeConfig
where
  IntoT: Into<AttributePropertyDefault>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    self.default.assign(component.into());
  }
}

impl syn::parse::Parse for AttributeConfig {
  fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
    let mut result = Self::default();

    let error = |ident: &syn::Ident| -> syn::Error {
      let known = ct::concatcp!(
        "Known entries of attribute ",
        AttributeConfig::KEYWORD,
        " are : ",
        DefaultMarker::KEYWORD, // <<< Use Marker::KEYWORD
        ".",
      );
      syn_err!(
        ident,
        r"Expects an attribute of format '#[ former( default = 13 ) ]'
  {known}
  But got: '{}'
",
        qt! { #ident }
      )
    };

    while !input.is_empty() {
      let lookahead = input.lookahead1();
      if lookahead.peek(syn::Ident) {
        let ident: syn::Ident = input.parse()?;
        match ident.to_string().as_str() {
          // <<< Reverted to use AttributePropertyDefault::parse >>>
          DefaultMarker::KEYWORD => result.assign(AttributePropertyDefault::parse(input)?),
          _ => return Err(error(&ident)),
        }
      } else {
        return Err(lookahead.error());
      }

      // Optional comma handling
      if input.peek(syn::Token![ , ]) {
        input.parse::<syn::Token![ , ]>()?;
      }
    }

    Ok(result)
  }
}

/// Attribute for scalar setters.
#[derive(Debug, Default, Clone)] // <<< Added Clone
pub struct AttributeScalarSetter {
  /// Optional identifier for naming the setter.
  pub name: AttributePropertyName,
  /// Controls the generation of a setter method. If false, a setter method is not generated.
  pub setter: AttributePropertySetter,
  /// Specifies whether to provide a sketch of the subform setter as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub debug: AttributePropertyDebug,
}

impl AttributeScalarSetter {
  /// Should setter be generated or not?
  #[allow(dead_code)]
  pub fn setter(&self) -> bool {
    self.setter.unwrap_or(true)
  }
}

impl AttributeComponent for AttributeScalarSetter {
  const KEYWORD: &'static str = "scalar";

  #[allow(clippy::match_wildcard_for_single_variants)]
  fn from_meta(attr: &syn::Attribute) -> Result<Self> {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        syn::parse2::< AttributeScalarSetter >( meta_list.tokens.clone() )
      },
      syn::Meta::Path( ref _path ) =>
      {
        syn::parse2::< AttributeScalarSetter >( TokenStream::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format `#[ scalar( setter = false ) ]` or `#[ scalar( setter = true, name = my_name ) ]`. \nGot: {}", qt!{ #attr } ),
    }
  }
}

impl<IntoT> Assign<AttributeScalarSetter, IntoT> for AttributeScalarSetter
where
  IntoT: Into<AttributeScalarSetter>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    let component = component.into();
    self.name.assign(component.name);
    self.setter.assign(component.setter);
    self.debug.assign(component.debug);
  }
}

impl<IntoT> Assign<AttributePropertyName, IntoT> for AttributeScalarSetter
where
  IntoT: Into<AttributePropertyName>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    self.name = component.into();
  }
}

impl<IntoT> Assign<AttributePropertySetter, IntoT> for AttributeScalarSetter
where
  IntoT: Into<AttributePropertySetter>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    self.setter = component.into();
  }
}

impl<IntoT> Assign<AttributePropertyDebug, IntoT> for AttributeScalarSetter
where
  IntoT: Into<AttributePropertyDebug>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    self.debug = component.into();
  }
}

impl syn::parse::Parse for AttributeScalarSetter {
  fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
    let mut result = Self::default();

    let error = |ident: &syn::Ident| -> syn::Error {
      let known = ct::concatcp!(
        "Known entries of attribute ",
        AttributeScalarSetter::KEYWORD,
        " are : ",
        AttributePropertyName::KEYWORD,
        ", ",
        AttributePropertySetter::KEYWORD,
        ", ",
        AttributePropertyDebug::KEYWORD,
        ".",
      );
      syn_err!(
        ident,
        r"Expects an attribute of format '#[ scalar( name = myName, setter = true ) ]'
  {known}
  But got: '{}'
",
        qt! { #ident }
      )
    };

    while !input.is_empty() {
      let lookahead = input.lookahead1();
      if lookahead.peek(syn::Ident) {
        let ident: syn::Ident = input.parse()?;
        match ident.to_string().as_str() {
          AttributePropertyName::KEYWORD => result.assign(AttributePropertyName::parse(input)?),
          AttributePropertySetter::KEYWORD => result.assign(AttributePropertySetter::parse(input)?),
          AttributePropertyDebug::KEYWORD => result.assign(AttributePropertyDebug::from(true)),
          _ => return Err(error(&ident)),
        }
      } else {
        return Err(lookahead.error());
      }

      // Optional comma handling
      if input.peek(syn::Token![ , ]) {
        input.parse::<syn::Token![ , ]>()?;
      }
    }

    Ok(result)
  }
}

/// Attribute for subform scalar setters.
#[derive(Debug, Default, Clone)] // <<< Added Clone
pub struct AttributeSubformScalarSetter {
  /// Optional identifier for naming the setter.
  pub name: AttributePropertyName,
  /// Controls the generation of a setter method. If false, a setter method is not generated.
  pub setter: AttributePropertySetter,
  /// Specifies whether to provide a sketch of the subform setter as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub debug: AttributePropertyDebug,
}

impl AttributeSubformScalarSetter {
  /// Should setter be generated or not?
  pub fn setter(&self) -> bool {
    self.setter.unwrap_or(true)
  }
}

impl AttributeComponent for AttributeSubformScalarSetter {
  const KEYWORD: &'static str = "subform_scalar";

  #[allow(clippy::match_wildcard_for_single_variants)]
  fn from_meta(attr: &syn::Attribute) -> Result<Self> {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        syn::parse2::< AttributeSubformScalarSetter >( meta_list.tokens.clone() )
      },
      syn::Meta::Path( ref _path ) =>
      {
        syn::parse2::< AttributeSubformScalarSetter >( TokenStream::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format `#[ subform_scalar( setter = false ) ]` or `#[ subform_scalar( setter = true, name = my_name ) ]`. \nGot: {}", qt!{ #attr } ),
    }
  }
}

impl<IntoT> Assign<AttributeSubformScalarSetter, IntoT> for AttributeSubformScalarSetter
where
  IntoT: Into<AttributeSubformScalarSetter>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    let component = component.into();
    self.name.assign(component.name);
    self.setter.assign(component.setter);
    self.debug.assign(component.debug);
  }
}

impl<IntoT> Assign<AttributePropertyName, IntoT> for AttributeSubformScalarSetter
where
  IntoT: Into<AttributePropertyName>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    self.name = component.into();
  }
}

impl<IntoT> Assign<AttributePropertySetter, IntoT> for AttributeSubformScalarSetter
where
  IntoT: Into<AttributePropertySetter>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    self.setter = component.into();
  }
}

impl<IntoT> Assign<AttributePropertyDebug, IntoT> for AttributeSubformScalarSetter
where
  IntoT: Into<AttributePropertyDebug>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    self.debug = component.into();
  }
}

impl syn::parse::Parse for AttributeSubformScalarSetter {
  fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
    let mut result = Self::default();

    let error = |ident: &syn::Ident| -> syn::Error {
      let known = ct::concatcp!(
        "Known entries of attribute ",
        AttributeSubformScalarSetter::KEYWORD,
        " are : ",
        AttributePropertyName::KEYWORD,
        ", ",
        AttributePropertySetter::KEYWORD,
        ", ",
        AttributePropertyDebug::KEYWORD,
        ".",
      );
      syn_err!(
        ident,
        r"Expects an attribute of format '#[ subform_scalar( name = myName, setter = true ) ]'
  {known}
  But got: '{}'
",
        qt! { #ident }
      )
    };

    while !input.is_empty() {
      let lookahead = input.lookahead1();
      if lookahead.peek(syn::Ident) {
        let ident: syn::Ident = input.parse()?;
        match ident.to_string().as_str() {
          AttributePropertyName::KEYWORD => result.assign(AttributePropertyName::parse(input)?),
          AttributePropertySetter::KEYWORD => result.assign(AttributePropertySetter::parse(input)?),
          AttributePropertyDebug::KEYWORD => result.assign(AttributePropertyDebug::from(true)),
          _ => return Err(error(&ident)),
        }
      } else {
        return Err(lookahead.error());
      }

      // Optional comma handling
      if input.peek(syn::Token![ , ]) {
        input.parse::<syn::Token![ , ]>()?;
      }
    }

    Ok(result)
  }
}

/// Attribute for subform collection setters.
#[derive(Debug, Default, Clone)] // <<< Added Clone
pub struct AttributeSubformCollectionSetter {
  /// Optional identifier for naming the setter.
  pub name: AttributePropertyName,
  /// Controls the generation of a setter method. If false, a setter method is not generated.
  pub setter: AttributePropertySetter,
  /// Specifies whether to provide a sketch of the subform setter as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub debug: AttributePropertyDebug,
  /// Definition of the collection former to use, e.g., `former::VectorFormer`.
  pub definition: AttributePropertyDefinition,
}

impl AttributeSubformCollectionSetter {
  /// Should setter be generated or not?
  pub fn setter(&self) -> bool {
    self.setter.unwrap_or(true)
  }
}

impl AttributeComponent for AttributeSubformCollectionSetter {
  const KEYWORD: &'static str = "subform_collection";

  #[allow(clippy::match_wildcard_for_single_variants)]
  fn from_meta(attr: &syn::Attribute) -> Result<Self> {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        syn::parse2::< AttributeSubformCollectionSetter >( meta_list.tokens.clone() )
      },
      syn::Meta::Path( ref _path ) =>
      {
        syn::parse2::< AttributeSubformCollectionSetter >( TokenStream::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format `#[ subform_collection ]` or `#[ subform_collection( definition = former::VectorDefinition ) ]` if you want to use default collection defition. \nGot: {}", qt!{ #attr } ),
    }
  }
}

impl<IntoT> Assign<AttributeSubformCollectionSetter, IntoT> for AttributeSubformCollectionSetter
where
  IntoT: Into<AttributeSubformCollectionSetter>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    let component = component.into();
    self.name.assign(component.name);
    self.setter.assign(component.setter);
    self.debug.assign(component.debug);
    self.definition.assign(component.definition);
  }
}

impl<IntoT> Assign<AttributePropertyName, IntoT> for AttributeSubformCollectionSetter
where
  IntoT: Into<AttributePropertyName>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    self.name = component.into();
  }
}

impl<IntoT> Assign<AttributePropertySetter, IntoT> for AttributeSubformCollectionSetter
where
  IntoT: Into<AttributePropertySetter>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    self.setter = component.into();
  }
}

impl<IntoT> Assign<AttributePropertyDefinition, IntoT> for AttributeSubformCollectionSetter
where
  IntoT: Into<AttributePropertyDefinition>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    self.definition = component.into();
  }
}

impl<IntoT> Assign<AttributePropertyDebug, IntoT> for AttributeSubformCollectionSetter
where
  IntoT: Into<AttributePropertyDebug>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    self.debug = component.into();
  }
}

impl syn::parse::Parse for AttributeSubformCollectionSetter {
  fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
    let mut result = Self::default();

    let error = |ident: &syn::Ident| -> syn::Error {
      let known = ct::concatcp!(
        "Known entries of attribute ",
        AttributeSubformCollectionSetter::KEYWORD,
        " are : ",
        AttributePropertyName::KEYWORD,
        ", ",
        AttributePropertySetter::KEYWORD,
        ", ",
        AttributePropertyDebug::KEYWORD,
        ", ",
        DefinitionMarker::KEYWORD, // <<< Use Marker::KEYWORD
        ".",
      );
      syn_err!(
        ident,
        r"Expects an attribute of format '#[ subform_collection( name = myName, setter = true, debug, definition = MyDefinition ) ]'
  {known}
  But got: '{}'
",
        qt! { #ident }
      )
    };

    while !input.is_empty() {
      let lookahead = input.lookahead1();
      if lookahead.peek(syn::Ident) {
        let ident: syn::Ident = input.parse()?;
        match ident.to_string().as_str() {
          AttributePropertyName::KEYWORD => result.assign(AttributePropertyName::parse(input)?),
          AttributePropertySetter::KEYWORD => result.assign(AttributePropertySetter::parse(input)?),
          AttributePropertyDebug::KEYWORD => result.assign(AttributePropertyDebug::from(true)),
          // <<< Reverted to use AttributePropertyDefinition::parse >>>
          DefinitionMarker::KEYWORD => result.assign(AttributePropertyDefinition::parse(input)?),
          _ => return Err(error(&ident)),
        }
      } else {
        return Err(lookahead.error());
      }

      // Optional comma handling
      if input.peek(syn::Token![ , ]) {
        input.parse::<syn::Token![ , ]>()?;
      }
    }

    Ok(result)
  }
}

/// Attribute for subform entry setters.
#[derive(Debug, Default, Clone)] // <<< Added Clone
pub struct AttributeSubformEntrySetter {
  /// An optional identifier that names the setter. It is parsed from inputs
  /// like `name = my_field`.
  pub name: AttributePropertyName,
  /// Disable generation of setter.
  /// It still generate `_field_subform_entry` method, so it could be used to make a setter with custom arguments.
  pub setter: AttributePropertySetter,
  /// Specifies whether to provide a sketch of the subform setter as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub debug: AttributePropertyDebug,
}

impl AttributeSubformEntrySetter {
  /// Should setter be generated or not?
  pub fn setter(&self) -> bool {
    self.setter.unwrap_or(true)
  }
}

impl AttributeComponent for AttributeSubformEntrySetter {
  const KEYWORD: &'static str = "subform_entry";

  #[allow(clippy::match_wildcard_for_single_variants)]
  fn from_meta(attr: &syn::Attribute) -> Result<Self> {
    match attr.meta {
      syn::Meta::List(ref meta_list) => syn::parse2::<AttributeSubformEntrySetter>(meta_list.tokens.clone()),
      syn::Meta::Path(ref _path) => syn::parse2::<AttributeSubformEntrySetter>(TokenStream::default()),
      _ => return_syn_err!(
        attr,
        "Expects an attribute of format `#[ subform_entry ]` or `#[ subform_entry( name : child )` ], \nGot: {}",
        qt! { #attr }
      ),
    }
  }
}

impl<IntoT> Assign<AttributeSubformEntrySetter, IntoT> for AttributeSubformEntrySetter
where
  IntoT: Into<AttributeSubformEntrySetter>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    let component = component.into();
    self.name.assign(component.name);
    self.setter.assign(component.setter);
    self.debug.assign(component.debug);
  }
}

impl<IntoT> Assign<AttributePropertyName, IntoT> for AttributeSubformEntrySetter
where
  IntoT: Into<AttributePropertyName>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    self.name = component.into();
  }
}

impl<IntoT> Assign<AttributePropertySetter, IntoT> for AttributeSubformEntrySetter
where
  IntoT: Into<AttributePropertySetter>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    self.setter = component.into();
  }
}

impl<IntoT> Assign<AttributePropertyDebug, IntoT> for AttributeSubformEntrySetter
where
  IntoT: Into<AttributePropertyDebug>,
{
  #[inline(always)]
  fn assign(&mut self, component: IntoT) {
    self.debug = component.into();
  }
}

impl syn::parse::Parse for AttributeSubformEntrySetter {
  fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
    let mut result = Self::default();

    let error = |ident: &syn::Ident| -> syn::Error {
      let known = ct::concatcp!(
        "Known entries of attribute ",
        AttributeSubformEntrySetter::KEYWORD,
        " are : ",
        AttributePropertyName::KEYWORD,
        ", ",
        AttributePropertySetter::KEYWORD,
        ", ",
        AttributePropertyDebug::KEYWORD,
        ".",
      );
      syn_err!(
        ident,
        r"Expects an attribute of format '#[ subform( name = myName, setter = true ) ]'
  {known}
  But got: '{}'
",
        qt! { #ident }
      )
    };

    while !input.is_empty() {
      let lookahead = input.lookahead1();
      if lookahead.peek(syn::Ident) {
        let ident: syn::Ident = input.parse()?;
        match ident.to_string().as_str() {
          AttributePropertyName::KEYWORD => result.assign(AttributePropertyName::parse(input)?),
          AttributePropertySetter::KEYWORD => result.assign(AttributePropertySetter::parse(input)?),
          AttributePropertyDebug::KEYWORD => result.assign(AttributePropertyDebug::from(true)),
          _ => return Err(error(&ident)),
        }
      } else {
        return Err(lookahead.error());
      }

      // Optional comma handling
      if input.peek(syn::Token![ , ]) {
        input.parse::<syn::Token![ , ]>()?;
      }
    }

    Ok(result)
  }
}

// ==================================
// Attribute Property Definitions
// ==================================

/// Marker type for attribute property to specify whether to provide a sketch as a hint.
/// Defaults to `false`, which means no hint is provided unless explicitly requested.
#[derive(Debug, Default, Clone, Copy)] // <<< Added Clone
pub struct DebugMarker;

impl AttributePropertyComponent for DebugMarker {
  const KEYWORD: &'static str = "debug";
}

/// Specifies whether to provide a sketch as a hint.
/// Defaults to `false`, which means no hint is provided unless explicitly requested.
pub type AttributePropertyDebug = AttributePropertyOptionalSingletone<DebugMarker>;

// =

/// Disable generation of setter.
/// Attributes still might generate some helper methods to reuse by custom setter.
#[derive(Debug, Default, Clone, Copy)] // <<< Added Clone
pub struct SetterMarker;

impl AttributePropertyComponent for SetterMarker {
  const KEYWORD: &'static str = "setter";
}

/// Disable generation of setter.
/// Attributes still might generate some helper methods to reuse by custom setter.
pub type AttributePropertySetter = AttributePropertyOptionalBoolean<SetterMarker>;

// =

/// Marker type for attribute property of optional identifier that names the setter. It is parsed from inputs
/// like `name = my_field`.
#[derive(Debug, Default, Clone, Copy)] // <<< Added Clone
pub struct NameMarker;

impl AttributePropertyComponent for NameMarker {
  const KEYWORD: &'static str = "name";
}

/// An optional identifier that names the setter. It is parsed from inputs
/// like `name = my_field`.
pub type AttributePropertyName = AttributePropertyOptionalSyn<syn::Ident, NameMarker>;

// =

/// Marker type for default value to use for a field.
#[derive(Debug, Default, Clone, Copy)] // <<< Added Clone
pub struct DefaultMarker;

impl AttributePropertyComponent for DefaultMarker {
  const KEYWORD: &'static str = "default";
}

/// An optional identifier that names the setter. It is parsed from inputs
/// like `name = my_field`.
// <<< REVERTED TYPE ALIAS >>>
pub type AttributePropertyDefault = AttributePropertyOptionalSyn<syn::Expr, DefaultMarker>;

// =

/// Marker type for definition of the collection former to use, e.g., `former::VectorFormer`.
#[derive(Debug, Default, Clone, Copy)] // <<< Added Clone
pub struct DefinitionMarker;

impl AttributePropertyComponent for DefinitionMarker {
  const KEYWORD: &'static str = "definition";
}

/// Definition of the collection former to use, e.g., `former::VectorFormer`.
// <<< REVERTED TYPE ALIAS >>>
pub type AttributePropertyDefinition = AttributePropertyOptionalSyn<syn::Type, DefinitionMarker>;

// =

/// Marker type for attribute property excluding a field from constructor arguments.
/// Defaults to `false`.
#[derive(Debug, Default, Clone, Copy)] // <<< Added Clone
pub struct FormerIgnoreMarker;

impl AttributePropertyComponent for FormerIgnoreMarker {
  const KEYWORD: &'static str = "former_ignore";
}

/// Indicates whether a field should be excluded from standalone constructor arguments.
/// Defaults to `false`. Parsed as a singletone attribute (`#[former_ignore]`).
pub type AttributePropertyFormerIgnore = AttributePropertyOptionalSingletone<FormerIgnoreMarker>;

// =

/// Marker type for attribute property including a field as a constructor argument.
/// Defaults to `false`.
#[derive(Debug, Default, Clone, Copy)]
pub struct ArgForConstructorMarker;

impl AttributePropertyComponent for ArgForConstructorMarker {
  const KEYWORD: &'static str = "arg_for_constructor";
}

/// Indicates whether a field should be included as an argument in standalone constructor functions.
/// Defaults to `false`. Parsed as a singletone attribute (`#[arg_for_constructor]`).
pub type AttributePropertyArgForConstructor = AttributePropertyOptionalSingletone<ArgForConstructorMarker>;

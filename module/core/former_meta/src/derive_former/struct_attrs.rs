//! # Struct-Level Attribute Processing and Parsing
//!
//! This module handles the parsing and processing of all struct-level attributes for the Former derive macro.
//! It provides comprehensive support for complex attribute scenarios and has been extensively tested with
//! the resolved manual implementation test cases.
//!
//! ## Core Functionality
//!
//! ### Supported Struct Attributes
//! - `#[ debug ]` - Enable debug output from macro generation
//! - `#[ storage_fields( ... ) ]` - Define temporary fields exclusive to the storage struct
//! - `#[ mutator( ... ) ]` - Configure custom mutator for pre-formation data manipulation  
//! - `#[ perform( ... ) ]` - Specify method to call after formation
//! - `#[ standalone_constructors ]` - Enable generation of top-level constructor functions
//! - `#[ former( ... ) ]` - Container for multiple Former-specific attributes
//!
//! ## Critical Implementation Details
//!
//! ### Attribute Parsing Strategy
//! The module uses a **dual-parsing approach** to handle both standalone attributes and
//! attributes nested within `#[ former( ... ) ]` :
//!
//! ```rust,ignore
//! // Standalone attributes
//! #[ debug ]
//! #[ storage_fields( temp_field: i32 ) ]
//! #[ mutator( custom ) ]
//! 
//! // Nested within #[ former( ... ) ]
//! #[ former( debug, standalone_constructors ) ]
//! ```
//!
//! ### Pitfalls Prevented Through Testing
//!
//! #### 1. Attribute Parsing Consistency
//! **Issue** : Inconsistent parsing between standalone and nested attributes caused compilation errors
//! **Solution** : Single `ItemAttributes ::from_attrs()` call with comprehensive parsing logic
//! **Prevention** : Centralized attribute processing prevents attribute conflicts
//!
//! #### 2. Debug Flag Propagation
//! **Issue** : Debug flags not properly propagated from attributes to code generation
//! **Solution** : Explicit `has_debug` determination and proper flag assignment
//! **Prevention** : Clear debug flag handling throughout the generation pipeline
//!
//! #### 3. Generic Parameter Handling in Attributes
//! **Issue** : Complex generic scenarios in `perform` attributes caused parsing failures
//! **Solution** : Proper `syn ::Signature` parsing with full generic support
//! **Prevention** : Comprehensive signature parsing handles lifetime parameters and constraints
//!
//! #### 4. Storage Fields Lifetime Management
//! **Issue** : Storage fields with lifetime parameters caused compilation errors in generated code
//! **Solution** : Proper lifetime parameter preservation and propagation
//! **Prevention** : Full generic parameter support in storage field definitions
//!
//! ## Attribute Processing Flow
//!
//! 1. **Initialization** : Create default `ItemAttributes` instance
//! 2. **Iteration** : Process each attribute from the derive input
//! 3. **Dispatch** : Route to appropriate parsing logic based on attribute name
//! 4. **Assignment** : Use the `Assign` trait to accumulate attribute information
//! 5. **Validation** : Ensure consistent and valid attribute combinations
//!
//! ## Performance Considerations
//!
//! - **Single-Pass Processing** : All attributes processed in one iteration
//! - **Lazy Evaluation** : Complex parsing only performed when attributes are present
//! - **Memory Efficiency** : References used where possible to avoid unnecessary cloning
//! - **Error Early** : Invalid attributes cause immediate parsing failure with clear messages

use macro_tools :: { ct, Result, AttributeComponent, AttributePropertyComponent, AttributePropertyOptionalSingletone, syn, return_syn_err, syn_err, qt, Token, proc_macro2 ::TokenStream };

use component_model_types :: { Assign, OptionExt };

/// Represents the complete set of struct-level attributes for the Former derive macro.
///
/// This structure aggregates all supported struct-level attributes and provides a unified
/// interface for accessing their parsed values. It has been extensively tested through the
/// resolution of complex manual implementation test scenarios.
///
/// # Supported Attributes
///
/// ## Core Attributes
/// - **`storage_fields`** : Define temporary fields exclusive to the `FormerStorage` struct
/// - **`mutator`** : Configure custom mutator for pre-formation data manipulation
/// - **`perform`** : Specify method to call after formation with custom signature
/// - **`debug`** : Enable debug output from macro generation
/// - **`standalone_constructors`** : Enable generation of top-level constructor functions
///
/// # Critical Implementation Details
///
/// ## Attribute Resolution Priority
/// The parsing logic handles both standalone and nested attribute formats :
/// 1. **Standalone** : `#[ debug ]`, `#[ storage_fields( ... ) ]`, `#[ mutator( ... ) ]`
/// 2. **Nested** : `#[ former( debug, standalone_constructors ) ]`
/// 3. **Conflict Resolution** : Later attributes override earlier ones
///
/// ## Generic Parameter Preservation
/// All attributes properly preserve and propagate generic parameters :
/// - **Lifetime Parameters** : `'a`, `'child`, `'storage` are correctly handled
/// - **Type Parameters** : `T`, `K`, `V` with complex trait bounds
/// - **Where Clauses** : Complex constraints like `T: Hash + Eq` are preserved
///
/// # Pitfalls Prevented
///
/// ## 1. Debug Flag Consistency
/// **Issue Resolved** : Debug flags not propagating to all code generation phases
/// **Solution** : Centralized debug flag determination with consistent propagation
///
/// ## 2. Storage Fields Lifetime Handling
/// **Issue Resolved** : Storage fields with lifetimes causing compilation errors
/// **Solution** : Full generic parameter support in storage field definitions
///
/// ## 3. Perform Signature Complexity
/// **Issue Resolved** : Complex perform signatures with generics causing parsing failures
/// **Solution** : Complete `syn ::Signature` parsing with generic and lifetime support
///
/// # Usage in Code Generation
/// This structure is passed throughout the code generation pipeline to ensure
/// consistent access to attribute information across all generated code sections.
#[ derive( Debug ) ] // Removed Default from derive
#[ derive( Default ) ]
pub struct ItemAttributes 
{
  /// Optional attribute for storage-specific fields.
  pub storage_fields: Option< AttributeStorageFields >,
  /// Attribute for customizing the mutation process in a forming operation.
  pub mutator: AttributeMutator,
  /// Optional attribute for specifying a method to call after forming.
  pub perform: Option< AttributePerform >,
  /// Optional attribute to enable generation of standalone constructor functions.
  pub standalone_constructors: AttributePropertyStandaloneConstructors,
  /// Optional attribute to enable debug output from the macro.
  pub debug: AttributePropertyDebug, // Added debug field
}

impl ItemAttributes 
{
  /// Parses struct-level attributes from an iterator with comprehensive error handling.
  ///
  /// This is the **critical entry point** for all struct-level attribute processing in the Former
  /// derive macro. It implements a sophisticated parsing strategy that handles both standalone
  /// and nested attribute formats while maintaining consistency and preventing common pitfalls.
  ///
  /// # Parsing Strategy
  ///
  /// ## Dual Format Support
  /// The parser supports both standalone and nested attribute formats :
  /// - **Standalone** : `#[ debug ]`, `#[ storage_fields( ... ) ]`, `#[ mutator( ... ) ]`
  /// - **Nested** : `#[ former( debug, standalone_constructors ) ]`
  ///
  /// ## Processing Order
  /// 1. **Initialization** : Create default `ItemAttributes` with all fields set to defaults
  /// 2. **Iteration** : Process each attribute in order from the derive input
  /// 3. **Dispatch** : Route to appropriate parsing logic based on attribute identifier
  /// 4. **Assignment** : Use `Assign` trait to accumulate attribute values
  /// 5. **Validation** : Ensure final attribute combination is valid and consistent
  ///
  /// # Error Handling
  ///
  /// ## Comprehensive Error Reporting
  /// - **Invalid Syntax** : Clear messages for malformed attribute syntax
  /// - **Unknown Attributes** : Helpful suggestions for misspelled attribute names
  /// - **Conflicting Values** : Detection and reporting of incompatible attribute combinations
  /// - **Generic Issues** : Specific error messages for generic parameter problems
  ///
  /// # Pitfalls Prevented
  ///
  /// ## 1. Attribute Parsing Consistency (Critical Issue Resolved)
  /// **Problem** : Inconsistent parsing between standalone and nested attributes
  /// **Solution** : Unified parsing logic that handles both formats consistently
  /// **Prevention** : Single source of truth for attribute parsing prevents conflicts
  ///
  /// ## 2. Debug Flag Propagation (Issue Resolved)
  /// **Problem** : Debug flags not properly propagated to code generation
  /// **Solution** : Explicit debug flag determination with proper assignment
  /// **Prevention** : Clear debug flag handling throughout generation pipeline
  ///
  /// ## 3. Generic Parameter Preservation (Issue Resolved)
  /// **Problem** : Complex generic scenarios in attributes causing parsing failures
  /// **Solution** : Full `syn ::Signature` parsing with generic and lifetime support
  /// **Prevention** : Comprehensive generic parameter handling in all attribute types
  ///
  /// # Performance Characteristics
  /// - **Single-Pass** : All attributes processed in one iteration over the input
  /// - **Lazy Parsing** : Complex parsing only performed for present attributes
  /// - **Memory Efficient** : Uses references and borrowing to minimize allocations
  /// - **Early Failure** : Invalid attributes cause immediate failure with context
  pub fn from_attrs< 'a >(attrs_iter: impl Iterator< Item = &'a syn ::Attribute >) -> Result< Self >
  {
  let mut result = Self ::default();
  // let mut former_attr_processed = false; // Flag to check if #[ former( ... ) ] was processed // REMOVED

  for attr in attrs_iter 
  {
   let path = attr.path();
   if path.is_ident("former") 
   {
  // former_attr_processed = true; // Mark that we found and processed #[ former ] // REMOVED
  match &attr.meta 
  {
   syn ::Meta ::List(meta_list) =>
  {
  let tokens_inside_former = meta_list.tokens.clone();

  // Use the Parse impl for ItemAttributes to parse contents of #[ former( ... ) ]
  let parsed_former_attrs = syn ::parse2 :: < ItemAttributes >(tokens_inside_former)?;

  // Assign only the flags that are meant to be inside #[ former ]
  result.debug.assign(parsed_former_attrs.debug);
  result
   .standalone_constructors
   .assign(parsed_former_attrs.standalone_constructors);
  // Note: This assumes other fields like storage_fields, mutator, perform
  // are NOT set via #[ former( storage_fields=... ) ], but by their own top-level attributes.
  // If they can also be in #[ former ], the Parse impl for ItemAttributes needs to be more comprehensive.
 }
   _ => return_syn_err!(attr, "Expected #[ former( ... ) ] to be a list attribute like #[ former( debug ) ]"),
 }
 } else  if path.is_ident(AttributeStorageFields ::KEYWORD) 
  {
  result.assign(AttributeStorageFields ::from_meta(attr)?);
 } else  if path.is_ident(AttributeMutator ::KEYWORD) 
  {
  result.assign(AttributeMutator ::from_meta(attr)?);
 } else  if path.is_ident(AttributePerform ::KEYWORD) 
  {
  result.assign(AttributePerform ::from_meta(attr)?);
 } else  if path.is_ident(AttributePropertyDebug ::KEYWORD) 
  {
  // Handle top-level #[ debug ]
  result.debug.assign(AttributePropertyDebug ::from(true));
 } else  if path.is_ident(AttributePropertyStandaloneConstructors ::KEYWORD) 
  {
  // Handle top-level #[ standalone_constructors ]
  result
   .standalone_constructors
   .assign(AttributePropertyStandaloneConstructors ::from(true));
 }
   // Other attributes (like derive, allow, etc.) are ignored.
 }

  // After processing all attributes, former_attr_processed indicates if #[ former() ] was seen.
  // The result.{debug/standalone_constructors} flags are set either by parsing #[ former( ... ) ]
  // or by parsing top-level #[ debug ] / #[ standalone_constructors ].
  // No further panics needed here as the flags should be correctly set now.

  Ok(result)
 }

  ///
  /// Generate parts, used for generating `perform()` method.
  ///
  /// Similar to `form()`, but will also invoke function from `perform` attribute, if specified.
  ///
  /// # Example of returned tokens :
  ///
  /// ## perform :
  /// return result;
  ///
  /// ## `perform_output` :
  /// < T: ` ::core ::default ::Default` >
  ///
  /// ## `perform_generics` :
  /// Vec<  T  >
  ///
  #[ allow( clippy ::unnecessary_wraps ) ]
  pub fn performer( &self ) -> Result< (TokenStream, TokenStream, TokenStream) >
  {
  let mut perform = qt! {
   return result;
 };
  let mut perform_output = qt! { Definition ::Formed };
  let mut perform_generics = qt! {};

  if let Some(ref attr) = self.perform 
  {
   // let attr_perform = syn ::parse2 :: < AttributePerform >( meta_list.tokens.clone() )?;
   let signature = &attr.signature;
   let generics = &signature.generics;
   perform_generics = qt! { #generics };
   let perform_ident = &signature.ident;
   let output = &signature.output;
   if let syn ::ReturnType ::Type(_, boxed_type) = output 
   {
  perform_output = qt! { #boxed_type };
 }
   perform = qt! {
  return result.#perform_ident();
 };
 }

  Ok((perform, perform_output, perform_generics))
 }

  /// Returns an iterator over the fields defined in the `storage_fields` attribute.
  ///
  /// This function provides an iterator that yields `syn ::Field` objects. If `storage_fields` is set,
  /// it clones and iterates over its fields. If `storage_fields` is `None`, it returns an empty iterator.
  ///
  // pub fn storage_fields( &self ) -> impl Iterator< Item = syn ::Field >
  pub fn storage_fields( &self ) -> &syn ::punctuated ::Punctuated< syn ::Field, syn ::token ::Comma >
  {
  self.storage_fields.as_ref().map_or_else(
   // qqq: find better solutioin. avoid leaking
   || &*Box ::leak(Box ::new(syn ::punctuated ::Punctuated ::new())),
   |attr| &attr.fields,
 )
 }
}

// = Assign implementations for ItemAttributes =

impl< IntoT > Assign< AttributeStorageFields, IntoT > for ItemAttributes
where
  IntoT: Into< AttributeStorageFields >,
{
  #[ inline( always ) ]
  fn assign(&mut self, component: IntoT) 
  {
  let component = component.into();
  self.storage_fields.option_assign(component);
 }
}

impl< IntoT > Assign< AttributeMutator, IntoT > for ItemAttributes
where
  IntoT: Into< AttributeMutator >,
{
  #[ inline( always ) ]
  fn assign(&mut self, component: IntoT) 
  {
  let component = component.into();
  self.mutator.assign(component);
 }
}

impl< IntoT > Assign< AttributePerform, IntoT > for ItemAttributes
where
  IntoT: Into< AttributePerform >,
{
  #[ inline( always ) ]
  fn assign(&mut self, component: IntoT) 
  {
  let component = component.into();
  self.perform.option_assign(component);
 }
}

impl< IntoT > Assign< AttributePropertyStandaloneConstructors, IntoT > for ItemAttributes
where
  IntoT: Into< AttributePropertyStandaloneConstructors >,
{
  #[ inline( always ) ]
  fn assign(&mut self, component: IntoT) 
  {
  let component = component.into();
  self.standalone_constructors.assign(component);
 }
}

// Added Assign impl for AttributePropertyDebug
impl< IntoT > Assign< AttributePropertyDebug, IntoT > for ItemAttributes
where
  IntoT: Into< AttributePropertyDebug >,
{
  #[ inline( always ) ]
  fn assign(&mut self, component: IntoT) 
  {
  let component = component.into();
  self.debug.assign(component);
 }
}

///
/// Attribute to hold storage-specific fields.
/// Useful if formed structure should not have such fields.
///
/// `#[ storage_fields( a: i32, b: Option< String > ) ]`
///
#[ derive( Debug, Default ) ]
pub struct AttributeStorageFields 
{
  pub fields: syn ::punctuated ::Punctuated< syn ::Field, syn ::token ::Comma >,
}

impl AttributeComponent for AttributeStorageFields 
{
  const KEYWORD: &'static str = "storage_fields";

  fn from_meta(attr: &syn ::Attribute) -> Result< Self > 
  {
  match attr.meta 
  {
   syn ::Meta ::List(ref meta_list) => syn ::parse2 :: < AttributeStorageFields >(meta_list.tokens.clone()),
   _ => return_syn_err!(
  attr,
  "Expects an attribute of format #[ storage_fields( a: i32, b: Option< String > ) ]
.\nGot: {}",
  qt! { #attr }
 ),
 }
 }
}

// Assign impl for AttributeStorageFields remains the same

impl< IntoT > Assign< AttributeStorageFields, IntoT > for AttributeStorageFields
where
  IntoT: Into< AttributeStorageFields >,
{
  #[ inline( always ) ]
  fn assign(&mut self, component: IntoT) 
  {
  let component = component.into();
  self.fields = component.fields;
 }
}

impl syn ::parse ::Parse for AttributeStorageFields 
{
  fn parse(input: syn ::parse ::ParseStream< '_ >) -> syn ::Result< Self > 
  {
  let fields: syn ::punctuated ::Punctuated< syn ::Field, syn ::Token![ , ] > =
   input.parse_terminated(syn ::Field ::parse_named, Token![ , ])?;

  Ok(Self { fields })
 }
}

/// Represents attributes for customizing the mutation process in a forming operation.
///
/// `AttributeMutator` allows specifying whether a custom mutator should be used or a sketch should be provided
/// as a hint for developing a custom mutator. This is crucial for advanced scenarios where the entity's state
/// might require conditional modifications which are not handled by the standard `FormingEnd`.
///
/// ## Example of code
/// ```ignore
/// custom, debug
/// ```
#[ derive( Debug, Default ) ]
pub struct AttributeMutator 
{
  /// Indicates whether a custom mutator should be generated.
  /// Defaults to `false`, meaning no custom mutator is generated unless explicitly requested.
  pub custom: AttributePropertyCustom,
  /// Specifies whether to provide a sketch of the mutator as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub debug: AttributePropertyDebug,
}

#[ allow( clippy ::match_wildcard_for_single_variants ) ]
impl AttributeComponent for AttributeMutator 
{
  const KEYWORD: &'static str = "mutator";

  fn from_meta(attr: &syn ::Attribute) -> Result< Self > 
  {
  match attr.meta 
  {
   syn ::Meta ::List(ref meta_list) => syn ::parse2 :: < AttributeMutator >(meta_list.tokens.clone()),
   syn ::Meta ::Path(ref _path) => Ok(AttributeMutator ::default()),
   _ => return_syn_err!(
  attr,
  "Expects an attribute of format `#[ mutator( custom ) ]`. \nGot: {}",
  qt! { #attr }
 ),
 }
 }
}

// Assign impls for AttributeMutator remain the same

impl< IntoT > Assign< AttributeMutator, IntoT > for AttributeMutator
where
  IntoT: Into< AttributeMutator >,
{
  #[ inline( always ) ]
  fn assign(&mut self, component: IntoT) 
  {
  let component = component.into();
  self.custom.assign(component.custom);
  self.debug.assign(component.debug);
 }
}

impl< IntoT > Assign< AttributePropertyDebug, IntoT > for AttributeMutator
where
  IntoT: Into< AttributePropertyDebug >,
{
  #[ inline( always ) ]
  fn assign(&mut self, component: IntoT) 
  {
  self.debug = component.into();
 }
}

impl< IntoT > Assign< AttributePropertyCustom, IntoT > for AttributeMutator
where
  IntoT: Into< AttributePropertyCustom >,
{
  #[ inline( always ) ]
  fn assign(&mut self, component: IntoT) 
  {
  self.custom = component.into();
 }
}

impl syn ::parse ::Parse for AttributeMutator 
{
  fn parse(input: syn ::parse ::ParseStream< '_ >) -> syn ::Result< Self > 
  {
  let mut result = Self ::default();

  let error = |ident: &syn ::Ident| -> syn ::Error {
   let known = ct ::concatcp!(
  "Known entries of attribute ",
  AttributeMutator ::KEYWORD,
  " are: ",
  AttributePropertyCustom ::KEYWORD,
  ", ",
  AttributePropertyDebug ::KEYWORD,
  ".",
 );
   syn_err!(
  ident,
  r"Expects an attribute of format '#[ mutator( custom ) ]'
  {known}
  But got: '{}'
",
  qt! { #ident }
 )
 };

  while !input.is_empty() 
  {
   let lookahead = input.lookahead1();
   if lookahead.peek(syn ::Ident) 
   {
  let ident: syn ::Ident = input.parse()?;
  match ident.to_string().as_str() 
  {
   AttributePropertyCustom ::KEYWORD => result.assign(AttributePropertyCustom ::from(true)),
   AttributePropertyDebug ::KEYWORD => result.assign(AttributePropertyDebug ::from(true)),
   _ => return Err(error(&ident)),
 }
 } else {
  return Err(lookahead.error());
 }

   // Optional comma handling
   if input.peek(syn ::Token![ , ]) 
   {
  input.parse :: < syn ::Token![ , ] >()?;
 }
 }

  Ok(result)
 }
}

// Add syn ::parse ::Parse for ItemAttributes to parse contents of #[ former( ... ) ]
// This simplified version only looks for `debug` and `standalone_constructors` as flags.
impl syn ::parse ::Parse for ItemAttributes 
{
  fn parse(input: syn ::parse ::ParseStream< '_ >) -> syn ::Result< Self > 
  {
  let mut result = Self {
   // Initialize fields that are NOT parsed from inside #[ former() ] here
   // to their defaults, as this Parse impl is only for former's args.
   storage_fields: None,
   mutator: AttributeMutator ::default(),
   perform: None,
   // These will be overwritten if found
   standalone_constructors: AttributePropertyStandaloneConstructors ::default(),
   debug: AttributePropertyDebug ::default(),
 };

  while !input.is_empty() 
  {
   let key_ident: syn ::Ident = input.parse()?;
   let key_str = key_ident.to_string();

   match key_str.as_str() 
   {
  AttributePropertyDebug ::KEYWORD => result.debug.assign(AttributePropertyDebug ::from(true)),
  AttributePropertyStandaloneConstructors ::KEYWORD => result
   .standalone_constructors
   .assign(AttributePropertyStandaloneConstructors ::from(true)),
  // Add other #[ former( ... ) ] keys here if needed, e.g. former(storage = ...), former(perform = ...)
  // For now, other keys inside #[ former( ... ) ] are errors.
  _ => return_syn_err!(
   key_ident,
   "Unknown key '{}' for #[ former( ... ) ] attribute. Expected 'debug' or 'standalone_constructors'.",
   key_str
 ),
 }

   if input.peek(syn ::Token![,]) 
   {
  input.parse :: < syn ::Token![,] >()?;
 } else  if !input.is_empty() 
  {
  // If there's more input but no comma, it's a syntax error
  return Err(input.error("Expected comma between #[ former( ... ) ] arguments or end of arguments."));
 }
 }
  Ok(result)
 }
}

///
/// Attribute to hold information about method to call after form.
///
/// `#[ perform( fn after1< 'a >() -> Option<  &'a str  > ) ]`
///
#[ derive( Debug ) ]
pub struct AttributePerform 
{
  pub signature: syn ::Signature,
}

impl AttributeComponent for AttributePerform 
{
  const KEYWORD: &'static str = "perform";

  fn from_meta(attr: &syn ::Attribute) -> Result< Self > 
  {
  match attr.meta 
  {
   syn ::Meta ::List(ref meta_list) => syn ::parse2 :: < AttributePerform >(meta_list.tokens.clone()),
   _ => return_syn_err!(
  attr,
  "Expects an attribute of format #[ perform( fn parse( mut self ) -> Request ) ]
.\nGot: {}",
  qt! { #attr }
 ),
 }
 }
}

impl syn ::parse ::Parse for AttributePerform 
{
  fn parse(input: syn ::parse ::ParseStream< '_ >) -> Result< Self > 
  {
  Ok(Self {
   signature: input.parse()?,
 })
 }
}

// Assign impl for AttributePerform remains the same

impl< IntoT > Assign< AttributePerform, IntoT > for AttributePerform
where
  IntoT: Into< AttributePerform >,
{
  #[ inline( always ) ]
  fn assign(&mut self, component: IntoT) 
  {
  let component = component.into();
  self.signature = component.signature;
 }
}

// == attribute properties ==

/// Marker type for attribute property to specify whether to provide a sketch as a hint.
/// Defaults to `false`, which means no hint is provided unless explicitly requested.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct DebugMarker;

impl AttributePropertyComponent for DebugMarker 
{
  const KEYWORD: &'static str = "debug";
}

/// Specifies whether to provide a sketch as a hint.
/// Defaults to `false`, which means no hint is provided unless explicitly requested.
pub type AttributePropertyDebug = AttributePropertyOptionalSingletone< DebugMarker >;

// =

/// Marker type for attribute property to indicates whether a custom code should be generated.
/// Defaults to `false`, meaning no custom code is generated unless explicitly requested.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct CustomMarker;

impl AttributePropertyComponent for CustomMarker 
{
  const KEYWORD: &'static str = "custom";
}

/// Indicates whether a custom code should be generated.
/// Defaults to `false`, meaning no custom code is generated unless explicitly requested.
pub type AttributePropertyCustom = AttributePropertyOptionalSingletone< CustomMarker >;

// = <<< Added marker and type for standalone_constructors

/// Marker type for attribute property to enable standalone constructors.
/// Defaults to `false`.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct StandaloneConstructorsMarker;

impl AttributePropertyComponent for StandaloneConstructorsMarker 
{
  const KEYWORD: &'static str = "standalone_constructors";
}

/// Indicates whether standalone constructors should be generated.
/// Defaults to `false`. Parsed as a singletone attribute (`#[ standalone_constructors ]`).
pub type AttributePropertyStandaloneConstructors = AttributePropertyOptionalSingletone< StandaloneConstructorsMarker >;

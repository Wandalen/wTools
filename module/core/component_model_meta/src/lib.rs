#![doc(html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png")]
#![doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
)]
#![doc(html_root_url = "https://docs.rs/component_model_derive_meta/latest/component_model_derive_meta/")]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Component model macro support" ) ]

#[ allow( unused_imports ) ]
use macro_tools::prelude::*;

/// Popular type support for derive macro generation
mod popular_types;

#[ cfg( feature = "enabled" ) ]
#[ cfg( any(
  feature = "derive_components",
  feature = "derive_component_from",
  feature = "derive_from_components",
  feature = "derive_component_assign",
  feature = "derive_component_assign",
  feature = "derive_components_assign"
))]
mod component
{

  //!
  //! Implement couple of derives of general-purpose.
  //!

  #[ allow( unused_imports ) ]
  use macro_tools::prelude::*;

  #[ cfg( feature = "derive_component_assign" ) ]
  pub mod component_assign;
  #[ cfg( feature = "derive_component_from" ) ]
  pub mod component_from;
  #[ cfg(all(feature = "derive_component_assign", feature = "derive_components_assign")) ]
  pub mod components_assign;
  #[ cfg( feature = "derive_from_components" ) ]
  pub mod from_components;
  #[ cfg( feature = "derive_component_model" ) ]
  pub mod component_model;
}

///
/// Macro to implement `From` for each component (field) of a structure.
/// This macro simplifies the creation of `From` trait implementations for struct fields,
/// enabling easy conversion from a struct reference to its field types.
///
/// # Features
///
/// - Requires the `derive_component_from` feature to be enabled for use.
/// - The `ComponentFrom` derive macro can be applied to structs to automatically generate
///   `From` implementations for each field.
///
/// # Attributes
///
/// - `debug` : Optional attribute to enable debug-level output during the macro expansion process.
///
/// # Examples
///
/// Assuming the `derive_component_from` feature is enabled in your `Cargo.toml`, you can use the macro as follows :
///
/// ```rust
/// # fn main()
/// # {
/// use component_model_meta ::ComponentFrom;
///
/// #[ derive( ComponentFrom ) ]
/// struct Person
/// {
///   pub age: i32,
///   pub name: String,
/// }
///
/// let my_struct = Person { age: 10, name: "Hello".into() };
/// let age: i32 = From ::from( &my_struct );
/// let name: String = From ::from( &my_struct );
/// dbg!( age );
/// dbg!( name );
/// // > age = 10
/// // > name = "Hello"
/// # }
/// ```
///
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_component_from" ) ]
#[ proc_macro_derive(ComponentFrom, attributes(debug)) ]
pub fn component_from(input: proc_macro::TokenStream) -> proc_macro::TokenStream 
{
  let result = component::component_from::component_from(input);
  match result 
  {
  Ok(stream) => stream.into(),
  Err(err) => err.to_compile_error().into(),
 }
}

/// Derives the `Assign` trait for struct fields, allowing each field to be set
/// with a value that can be converted into the field's type.
///
/// This macro facilitates the automatic implementation of the `Assign` trait for all
/// fields within a struct, leveraging the power of Rust's type system to ensure type safety
/// and conversion logic. It is particularly useful for builder patterns or mutating instances
/// of data structures in a fluent and ergonomic manner.
///
/// # Attributes
///
/// - `debug` : An optional attribute to enable debugging of the trait derivation process.
///
/// # Conditions
///
/// - This macro is only enabled when the `derive_component_assign` feature is active in your `Cargo.toml`.
///
/// # Input Code Example
///
/// Given a struct definition annotated with `#[ derive( Assign ) ]` :
///
/// ```rust
/// use component_model_types ::Assign;
/// use component_model_meta ::Assign;
///
/// #[ derive( Default, PartialEq, Debug, Assign ) ]
/// struct Person
/// {
///   age: i32,
///   name: String,
/// }
///
/// let mut person: Person = Default ::default();
/// person.assign( 13 );
/// person.assign( "John" );
/// assert_eq!( person, Person { age: 13, name: "John".to_string() } );
/// ```
///
/// # Generated Code Example
///
/// The procedural macro generates the following implementations for `Person` :
///
/// ```rust
/// use component_model_types ::Assign;
/// use component_model_meta ::Assign;
///
/// #[ derive( Default, PartialEq, Debug ) ]
/// struct Person
/// {
///   age: i32,
///   name: String,
/// }
///
/// impl< IntoT > Assign< i32, IntoT > for Person
/// where
///   IntoT: Into< i32 >,
/// {
///   fn assign( &mut self, component: IntoT )
///   {
///     self.age = component.into();
/// }
/// }
///
/// impl< IntoT > Assign< String, IntoT > for Person
/// where
///   IntoT: Into< String >,
/// {
///   fn assign( &mut self, component: IntoT )
///   {
///     self.name = component.into();
/// }
/// }
///
/// let mut person: Person = Default ::default();
/// person.assign( 13 );
/// person.assign( "John" );
/// assert_eq!( person, Person { age: 13, name: "John".to_string() } );
/// ```
/// This allows any type that can be converted into an `i32` or `String` to be set as
/// the value of the `age` or `name` fields of `Person` instances, respectively.
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_component_assign" ) ]
#[ proc_macro_derive(Assign, attributes(debug)) ]
pub fn component_assign(input: proc_macro::TokenStream) -> proc_macro::TokenStream 
{
  let result = component::component_assign::component_assign(input);
  match result 
  {
  Ok(stream) => stream.into(),
  Err(err) => err.to_compile_error().into(),
 }
}

///
/// Derives the `ComponentsAssign` trait for a struct, enabling `components_assign` which set all fields at once.
///
/// This will work only if every field can be acquired from the passed value.
/// In other words, the type passed as an argument to `components_assign` must implement `Into< T >` for each field type.
///
/// # Attributes
///
/// - `debug` : An optional attribute to enable debugging of the trait derivation process.
///
/// # Conditions
///
/// - This macro is only enabled when the `derive_components_assign` feature is active in your `Cargo.toml`.
/// - The type must implement `Assign` (`derive( Assign )`)
///
/// # Limitations
/// This trait cannot be derived, if the struct has fields with identical types
///
/// # Input Code Example
///
/// An example when we encapsulate parameters passed to a function in a struct.
///
/// ```rust, ignore
/// use component_model :: { Assign, ComponentsAssign };
///
/// #[ derive( Default, Assign, ComponentsAssign ) ]
/// struct BigOpts
/// {
///   cond: bool,
///   int: i32,
///   str: String,
/// }
///
/// #[ derive( Default, Assign, ComponentsAssign ) ]
/// struct SmallerOpts
/// {
///   cond: bool,
///   int: i32,
/// }
///
/// impl From< &BigOpts > for bool
/// {
///   fn from( value: &BigOpts ) -> Self
///   {
///     value.cond
/// }
/// }
///
/// impl From< &BigOpts > for i32
/// {
///   fn from( value: &BigOpts ) -> Self
///   {
///     value.int
/// }
/// }
///
/// fn take_big_opts( options: &BigOpts ) -> &String
/// {
///   &options.str
/// }
///
/// fn take_smaller_opts( options: &SmallerOpts ) -> bool
/// {
///   !options.cond
/// }
///
/// let options1 = BigOpts
/// {
///   cond: true,
///   int: -14,
///   ..Default ::default()
/// };
/// take_big_opts( &options1 );
///
/// let mut options2 = SmallerOpts ::default();
/// options2.smaller_opts_assign( &options1 );
/// take_smaller_opts( &options2 );
/// ```
///
/// Which expands approximately into :
///
/// ```rust, ignore
/// use component_model :: { Assign, ComponentsAssign };
///
/// #[ derive( Default ) ]
/// struct BigOpts
/// {
///   cond: bool,
///   int: i32,
///   str: String,
/// }
///
/// impl< IntoT > Assign< bool, IntoT > for BigOpts
/// where
///   IntoT: Into< bool >,
/// {
///   fn assign( &mut self, component: IntoT )
///   {
///     self.cond = component.into();
/// }
/// }
///
/// impl< IntoT > Assign< i32, IntoT > for BigOpts
/// where
///   IntoT: Into< i32 >,
/// {
///   fn assign( &mut self, component: IntoT )
///   {
///     self.int = component.into();
/// }
/// }
///
/// impl< IntoT > Assign< String, IntoT > for BigOpts
/// where
///   IntoT: Into< String >,
/// {
///   fn assign( &mut self, component: IntoT )
///   {
///     self.str = component.into();
/// }
/// }
///
/// pub trait BigOptsComponentsAssign< IntoT >
/// where
///   IntoT: Into< bool >,
///   IntoT: Into< i32 >,
///   IntoT: Into< String >,
///   IntoT: Clone,
/// {
///   fn components_assign( &mut self, component: IntoT );
/// }
///
/// impl< T, IntoT > BigOptsComponentsAssign< IntoT > for T
/// where
///   T: component_model ::Assign< bool, IntoT >,
///   T: component_model ::Assign< i32, IntoT >,
///   T: component_model ::Assign< String, IntoT >,
///   IntoT: Into< bool >,
///   IntoT: Into< i32 >,
///   IntoT: Into< String >,
///   IntoT: Clone,
/// {
///   fn components_assign( &mut self, component: IntoT )
///   {
///     component_model ::Assign :: < bool, _ > ::assign( self, component.clone() );
///     component_model ::Assign :: < i32, _ > ::assign( self, component.clone() );
///     component_model ::Assign :: < String, _ > ::assign( self, component.clone() );
/// }
/// }
///
/// #[ derive( Default ) ]
/// struct SmallerOpts
/// {
///   cond: bool,
///   int: i32,
/// }
///
/// impl< IntoT > Assign< bool, IntoT > for SmallerOpts
/// where
///   IntoT: Into< bool >,
/// {
///   fn assign( &mut self, component: IntoT )
///   {
///     self.cond = component.into();
/// }
/// }
///
/// impl< IntoT > Assign< i32, IntoT > for SmallerOpts
/// where
///     IntoT: Into< i32 >,
/// {
///   fn assign( &mut self, component: IntoT )
///   {
///     self.int = component.into();
/// }
/// }
///
/// pub trait SmallerOptsComponentsAssign< IntoT >
/// where
///   IntoT: Into< bool >,
///   IntoT: Into< i32 >,
///   IntoT: Clone,
/// {
///   fn smaller_opts_assign( &mut self, component: IntoT );
/// }
///
/// impl< T, IntoT > SmallerOptsComponentsAssign< IntoT > for T
/// where
///   T: component_model ::Assign< bool, IntoT >,
///   T: component_model ::Assign< i32, IntoT >,
///   IntoT: Into< bool >,
///   IntoT: Into< i32 >,
///   IntoT: Clone,
/// {
///   fn smaller_opts_assign( &mut self, component: IntoT )
///   {
///     component_model ::Assign :: < bool, _ > ::assign( self, component.clone() );
///     component_model ::Assign :: < i32, _ > ::assign( self, component.clone() );
/// }
/// }
///
/// impl From< &BigOpts > for bool
/// {
///   fn from( value: &BigOpts ) -> Self
///   {
///     value.cond
/// }
/// }
///
/// impl From< &BigOpts > for i32
/// {
///   fn from( value: &BigOpts ) -> Self
///   {
///     value.int
/// }
/// }
///
/// fn take_big_opts( options: &BigOpts ) -> &String
/// {
///   &options.str
/// }
///
/// fn take_smaller_opts( options: &SmallerOpts ) -> bool
/// {
///   !options.cond
/// }
///
/// let options1 = BigOpts
/// {
///   cond: true,
///   int: -14,
///   ..Default ::default()
/// };
/// take_big_opts( &options1 );
/// let mut options2 = SmallerOpts ::default();
/// options2.smaller_opts_assign( &options1 );
/// take_smaller_opts( &options2 );
/// ```
///
#[ cfg( feature = "enabled" ) ]
#[ cfg(all(feature = "derive_component_assign", feature = "derive_components_assign")) ]
#[ proc_macro_derive(ComponentsAssign, attributes(debug)) ]
pub fn components_assign(input: proc_macro::TokenStream) -> proc_macro::TokenStream 
{
  let result = component::components_assign::components_assign(input);
  match result 
  {
  Ok(stream) => stream.into(),
  Err(err) => err.to_compile_error().into(),
 }
}

/// A procedural macro to automatically derive the `From< T >` trait implementation for a struct,
/// enabling instances of one type to be converted from instances of another type.
///
/// It is part of type-based forming approach which requires each field having an unique type. Each field
/// of the target struct must be capable of being individually converted from the source type `T`.
/// This macro simplifies the implementation of type conversions, particularly useful for
/// constructing a struct from another type with compatible fields. The source type `T` must
/// implement `Into< FieldType >` for each field type of the target struct.
///
/// # Attributes
///
/// - `debug` : Optional. Enables debug printing during macro expansion.
///
/// # Requirements
///
/// - Available only when the feature flags `enabled` and `derive_from_components`
///   are activated in your Cargo.toml. It's activated by default.
///
/// # Examples
///
/// Given the structs `Options1` and `Options2`, where `Options2` is a subset of `Options1` :
///
/// ```rust
/// use component_model_meta ::FromComponents;
///
/// #[ derive( Debug, Default, PartialEq ) ]
/// pub struct Options1
/// {
///   field1: i32,
///   field2: String,
///   field3: f32,
/// }
///
/// impl From< &Options1 > for i32
/// {
///   #[ inline( always ) ]
///   fn from( src: &Options1 ) -> Self
///   {
///     src.field1.clone()
/// }
/// }
///
/// impl From< &Options1 > for String
/// {
///   #[ inline( always ) ]
///   fn from( src: &Options1 ) -> Self
///   {
///     src.field2.clone()
/// }
/// }
///
/// impl From< &Options1 > for f32
/// {
///   #[ inline( always ) ]
///   fn from( src: &Options1 ) -> Self
///   {
///     src.field3.clone()
/// }
/// }
///
/// #[ derive( Debug, Default, PartialEq, FromComponents ) ]
/// pub struct Options2
/// {
///   field1: i32,
///   field2: String,
/// }
///
/// let o1 = Options1 { field1: 42, field2: "Hello, world!".to_string(), field3: 13.01 };
///
/// // Demonstrating conversion from Options1 to Options2
/// let o2: Options2 = Into :: < Options2 > ::into( &o1 );
/// let expected = Options2 { field1: 42, field2: "Hello, world!".to_string() };
/// assert_eq!( o2, expected );
///
/// // Alternative way using `.into()`
/// let o2: Options2 = ( &o1 ).into();
/// assert_eq!( o2, expected );
///
/// // Alternative way using `.from()`
/// let o2 = Options2 ::from( &o1 );
/// assert_eq!( o2, expected );
/// ```
///
/// This demonstrates how `Options2` can be derived from `Options1` using the `FromComponents` macro,
/// automatically generating the necessary `From< &Options1 >` implementation for `Options2`, facilitating
/// an easy conversion between these types based on their compatible fields.
///
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_from_components" ) ]
#[ proc_macro_derive(FromComponents, attributes(debug)) ]
pub fn from_components(input: proc_macro::TokenStream) -> proc_macro::TokenStream 
{
  let result = component::from_components::from_components(input);
  match result 
  {
  Ok(stream) => stream.into(),
  Err(err) => err.to_compile_error().into(),
 }
}

/// Unified derive macro that combines all component model functionality into a single annotation.
/// 
/// The `ComponentModel` derive automatically generates implementations for :
/// - `Assign` : Basic component assignment with type-safe field setting
/// - `ComponentsAssign` : Multiple component assignment from tuples (when applicable) 
/// - `ComponentFrom` : Create objects from single components (when applicable)
/// - `FromComponents` : Create objects from multiple components (when applicable)
/// 
/// This eliminates the need to apply multiple individual derives and reduces boilerplate.
/// 
/// # Features
/// 
/// - Requires the `derive_component_model` feature to be enabled for use.
/// - Automatically detects which trait implementations are appropriate for the struct.
/// - Handles type conflicts gracefully by skipping conflicting implementations.
/// 
/// # Attributes
/// 
/// - `debug` : Optional attribute to enable debug-level output during macro expansion.
/// - `component` : Optional field-level attribute for customizing component behavior.
/// 
/// # Examples
/// 
/// ```rust
/// use component_model_meta ::ComponentModel;
/// use component_model_types ::Assign;
/// 
/// #[ derive( Default, ComponentModel ) ]
/// struct Config
/// {
///   host: String,
///   port: i32,
///   enabled: bool,
/// }
/// 
/// let mut config = Config ::default();
/// 
/// // Use Assign trait (auto-generated)
/// config.assign( "localhost".to_string() );
/// config.assign( 8080i32 );
/// config.enabled_set( true ); // Use field-specific method to avoid type ambiguity
/// 
/// // Use fluent builder pattern (auto-generated)
/// let config2 = Config ::default()
///   .impute( "api.example.com".to_string() )
///   .impute( 3000i32 )
///   .enabled_with( false ); // Use field-specific method to avoid type ambiguity
/// ```
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_component_model" ) ]
#[ proc_macro_derive(ComponentModel, attributes(debug, component)) ]
pub fn component_model(input: proc_macro::TokenStream) -> proc_macro::TokenStream 
{
  let result = component::component_model::component_model(input);
  match result 
  {
  Ok(stream) => stream.into(),
  Err(err) => err.to_compile_error().into(),
 }
}

/// Unified derive macro that combines all component model functionality into a single annotation.
/// 
/// The `ComponentModel` derive automatically generates implementations for:
/// - `Assign`: Basic component assignment with type-safe field setting
/// - `ComponentsAssign`: Multiple component assignment from tuples (when applicable) 
/// - `ComponentFrom`: Create objects from single components (when applicable)
/// - `FromComponents`: Create objects from multiple components (when applicable)
/// 
/// This eliminates the need to apply multiple individual derives and reduces boilerplate.
/// 
/// # Features
/// 
/// - Requires the `derive_component_model` feature to be enabled for use.
/// - Automatically detects which trait implementations are appropriate for the struct.
/// - Handles type conflicts gracefully by skipping conflicting implementations.
/// 
/// # Attributes
/// 
/// - `debug` : Optional attribute to enable debug-level output during macro expansion.
/// - `component` : Optional field-level attribute for customizing component behavior.
/// 
/// # Examples
/// 
/// ```rust
/// use component_model_meta::ComponentModel;
/// use component_model_types::Assign;
/// 
/// #[ derive( Default, ComponentModel ) ]
/// struct Config
/// {
///   host : String,
///   port : i32,
///   enabled : bool,
/// }
/// 
/// let mut config = Config::default();
/// 
/// // Use Assign trait (auto-generated)
/// config.assign( "localhost".to_string() );
/// config.assign( 8080i32 );
/// config.enabled_set( true ); // Use field-specific method to avoid type ambiguity
/// 
/// // Use fluent builder pattern (auto-generated)
/// let config2 = Config::default()
///   .impute( "api.example.com".to_string() )
///   .impute( 3000i32 )
///   .enabled_with( false ); // Use field-specific method to avoid type ambiguity
/// ```
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_component_model" ) ]
#[proc_macro_derive(ComponentModel, attributes(debug, component))]
pub fn component_model(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let result = component::component_model::component_model(input);
  match result {
    Ok(stream) => stream.into(),
    Err(err) => err.to_compile_error().into(),
  }
}

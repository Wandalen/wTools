#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/former_derive_meta/latest/former_derive_meta/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ cfg( feature = "enabled" ) ]
mod derive
{

  //!
  //! Implement couple of derives of general-purpose.
  //!

  #[ allow( unused_imports ) ]
  use macro_tools::prelude::*;

  #[ cfg( feature = "derive_former" ) ]
  pub mod former;
  #[ cfg( feature = "derive_component_from" ) ]
  pub mod component_from;
  #[ cfg( feature = "derive_set_component" ) ]
  pub mod set_component;
  #[ cfg( feature = "derive_set_components" ) ]
  pub mod set_components;

}

///
/// Derive macro to generate former for a structure. Former is variation of Builder Pattern.
///

/// Derives a 'Former' for a struct, implementing a variation of the Builder Pattern.
///
/// This macro simplifies the creation of builder patterns for structs by automatically
/// generating a 'former' (builder) struct and implementation. It supports customization
/// through attributes to control default values, setter generation, subformer inclusion,
/// and field aliases.
///
/// # Attributes :
/// - `perform` : Specifies a method to call on the built object immediately after its construction.
/// - `default` : Sets a default value for a field.
/// - `setter` : Enables or disables the generation of a setter method for a field.
/// - `subformer` : Defines a sub-former for complex field types, allowing nested builders.
/// - `alias` : Creates an alias for a field setter.
/// - `doc` : Adds documentation to the generated setter methods. (deprecated)
///
/// # Input Example :
///
/// ```rust
///   use former::Former;
///
///   #[ derive( Debug, PartialEq, Former ) ]
///   #[ perform( fn greet_user() ) ]
///   pub struct UserProfile
///   {
///     #[default(1)]
///     age : i32,
///     
///     username : String,
///     
///     #[alias(bio)]
///     bio_optional : Option< String >, // Fields could be optional
///   }
/// 
///   impl UserProfile
///   {
///     fn greet_user(self) -> Self
///     {
///       println!("Hello, {}", self.username);
///       self
///     }
///   }
///
///   let profile = UserProfile::former()
///   .age( 30 )
///   .username( "JohnDoe".to_string() )
///   .bio_optional( "Software Developer".to_string() ) // Optionally provide a bio
///   .form();
///   // .perform(); // same as `form()` but will execute method passed to perform attribute
///
///   dbg!( &profile );
///   // Expected output:
///   // &profile = UserProfile {
///   //   age: 30,
///   //   username: "JohnDoe",
///   //   bio_optional: Some("Software Developer"),
///   // }
/// ```
///
/// # Generated Code Example :
///
/// Assuming the struct above, the macro generates something like this :
///
/// ```rust
/// # #[ cfg( feature = "enabled" ) ]
/// # #[ allow( dead_code ) ]
/// # fn main()
/// # {
///
///   #[ derive( Debug, PartialEq ) ]
///   pub struct UserProfile
///   {
///     age : i32,
///     username : String,
///     bio_optional : Option< String >, // Fields could be optional
///   }
///
///   impl UserProfile
///   {
///     #[ inline( always ) ]
///     pub fn former() -> UserProfileFormer< UserProfile, former::ReturnContainer >
///     {
///       UserProfileFormer::< UserProfile, former::ReturnContainer >::new()
///     }
///   }
///
///   #[ derive( Debug, Default ) ]
///   pub struct UserProfileFormerContainer
///   {
///     age : Option< i32 >,
///     username : Option< String >,
///     bio_optional : Option< String >,
///   }
///
///   pub struct UserProfileFormer
///   <
///     FormerContext = UserProfile,
///     FormerEnd = former::ReturnContainer,
///   >
///   where
///     FormerEnd : former::ToSuperFormer< UserProfile, FormerContext >,
///   {
///     container : UserProfileFormerContainer,
///     context : Option< FormerContext >,
///     on_end : Option< FormerEnd >,
///   }
///
///   impl< FormerContext, FormerEnd > UserProfileFormer< FormerContext, FormerEnd >
///   where
///     FormerEnd : former::ToSuperFormer< UserProfile, FormerContext >,
///   {
///     #[ inline( always ) ]
///     pub fn form( mut self ) -> UserProfile
///     {
///       let age = if self.container.age.is_some()
///       {
///         self.container.age.take().unwrap()
///       }
///       else
///       {
///         (1).into()
///       };
///       let username = if self.container.username.is_some()
///       {
///         self.container.username.take().unwrap()
///       }
///       else
///       {
///         String::default()
///       };
///       let bio_optional = if self.container.bio_optional.is_some()
///       {
///         Some( self.container.bio_optional.take().unwrap() )
///       }
///       else
///       {
///         None
///       };
///       UserProfile { age, username, bio_optional }
///     }
///
///     #[ inline( always ) ]
///     pub fn perform( self ) -> UserProfile
///     {
///       let result = self.form();
///       return result.greet_user();
///     }
///
///      #[ inline( always ) ]
///      pub fn new() -> UserProfileFormer< UserProfile, former::ReturnContainer >
///      {
///        UserProfileFormer::< UserProfile, former::ReturnContainer >::begin( None, former::ReturnContainer )
///      }
///
///     #[ inline( always ) ]
///     pub fn begin( context : Option< FormerContext >, on_end : FormerEnd ) -> Self
///     {
///       Self
///       {
///         container : Default::default(),
///         context,
///         on_end : Some( on_end ),
///       }
///     }
///
///     #[ inline( always ) ]
///     pub fn end( mut self ) -> FormerContext
///     {
///       let on_end = self.on_end.take().unwrap();
///       let context = self.context.take();
///       let container = self.form();
///       on_end.call( container, context )
///     }
///
///     #[ inline ]
///     pub fn age< Src >( mut self, src : Src ) -> Self
///     where
///       Src : Into< i32 >,
///     {
///       self.container.age = Some( src.into() );
///       self
///     }
///
///     #[ inline ]
///     pub fn username< Src >( mut self, src : Src ) -> Self
///     where
///       Src : Into< String >,
///     {
///       self.container.username = Some( src.into() );
///       self
///     }
///
///     #[ inline ]
///     pub fn bio_optional< Src >( mut self, src : Src ) -> Self
///     where
///       Src : Into< String >,
///     {
///       self.container.bio_optional = Some( src.into() );
///       self
///     }
/// 
///     #[inline]
///     pub fn bio< Src >( mut self, src : Src ) -> Self
///     where
///       Src : Into< String >,
///     {
///       self.container.bio_optional = Some( src.into() );
///       self
///     }
///   }
///
///   let profile = UserProfile::former()
///   .age( 30 )
///   .username( "JohnDoe".to_string() )
///   .bio_optional( "Software Developer".to_string() )
///   .form();
///
///   dbg!( &profile );
///   // Expected output:
///   // &profile = UserProfile {
///   //   age: 30,
///   //   username: "JohnDoe",
///   //   bio_optional: Some("Software Developer"),
///   // }
/// # }
/// ```
///
/// This generated code allows building an instance of `MyStruct` fluently, with optional customization for each field.

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_former" ) ]
#[ proc_macro_derive( Former, attributes( debug, perform, default, setter, subformer, alias, doc ) ) ]
pub fn former( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::former::former( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
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
/// #[ derive( former::ComponentFrom ) ]
/// struct Person
/// {
///   pub age : i32,
///   pub name : String,
/// }
///
/// let my_struct = Person { age : 10, name : "Hello".into() };
/// let age : i32 = From::from( &my_struct );
/// let name : String = From::from( &my_struct );
/// dbg!( age );
/// dbg!( name );
/// // > age = 10
/// // > name = "Hello"
/// # }
/// ```
///

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_component_from" ) ]
#[ proc_macro_derive( ComponentFrom, attributes( debug ) ) ]
pub fn component_from( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::component_from::component_from( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

/// Derives the `SetComponent` trait for struct fields, allowing each field to be set
/// with a value that can be converted into the field's type.
///
/// This macro facilitates the automatic implementation of the `SetComponent` trait for all
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
/// - This macro is only enabled when the `derive_set_component` feature is active in your `Cargo.toml`.
///
/// # Input Code Example
///
/// Given a struct definition annotated with `#[ derive( SetComponent ) ]` :
///
/// ```rust
/// use former::SetComponent;
///
/// #[ derive( Default, PartialEq, Debug, former::SetComponent ) ]
/// struct Person
/// {
///   age : i32,
///   name : String,
/// }
///
/// let mut person : Person = Default::default();
/// person.set( 13 );
/// person.set( "John" );
/// assert_eq!( person, Person { age : 13, name : "John".to_string() } );
/// ```
///
/// # Generated Code Example
///
/// The procedural macro generates the following implementations for `Person` :
///
/// ```rust
/// use former::SetComponent;
///
/// #[ derive( Default, PartialEq, Debug ) ]
/// struct Person
/// {
///   age : i32,
///   name : String,
/// }
///
/// impl< IntoT > SetComponent< i32, IntoT > for Person
/// where
///   IntoT : Into< i32 >,
/// {
///   fn set( &mut self, component : IntoT )
///   {
///     self.age = component.into();
///   }
/// }
///
/// impl< IntoT > SetComponent< String, IntoT > for Person
/// where
///   IntoT : Into< String >,
/// {
///   fn set( &mut self, component : IntoT )
///   {
///     self.name = component.into();
///   }
/// }
///
/// let mut person : Person = Default::default();
/// person.set( 13 );
/// person.set( "John" );
/// assert_eq!( person, Person { age : 13, name : "John".to_string() } );
/// ```
/// This allows any type that can be converted into an `i32` or `String` to be set as
/// the value of the `age` or `name` fields of `Person` instances, respectively.

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_set_component" ) ]
#[ proc_macro_derive( SetComponent, attributes( debug ) ) ]
pub fn set_component( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::set_component::set_component( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

///
/// Derives the `SetComponents` trait for a struct, enabling `components_set` which set all fields at once.
///
/// This will work only if every field can be acquired from the passed value.
/// In other words, the type passed as an argument to `components_set` must implement Into<T> for each field type.
///
/// # Attributes
///
/// - `debug` : An optional attribute to enable debugging of the trait derivation process.
///
/// # Conditions
///
/// - This macro is only enabled when the `derive_set_components` feature is active in your `Cargo.toml`.
/// - The type must implement `SetComponent` (`derive( SetComponent )`)
///
/// # Limitations
/// This trait cannot be derived, if the struct has fields with identical types
///
/// # Input Code Example
///
/// An example when we encapsulate parameters passed to a function in a struct.
///
/// ```rust
/// use former::{ SetComponent, SetComponents };
///
/// #[ derive( Default, SetComponent, SetComponents ) ]
/// struct BigOptions
/// {
///   cond : bool,
///   int : i32,
///   str : String,
///   vec : Vec< u8 >,
/// }
///
/// #[ derive( Default, SetComponent, SetComponents ) ]
/// struct SubBigOptions
/// {
///   cond: bool,
///   int: i32,
/// }
///
/// impl From< &BigOptions > for bool
/// {
///   fn from( value : &BigOptions ) -> Self
///   {
///     value.cond
///   }
/// }
///
/// impl From< &BigOptions > for i32
/// {
///   fn from( value: &BigOptions ) -> Self
///   {
///     value.int
///   }
/// }
///
/// fn boo( options : &BigOptions ) -> &Vec< u8 >
/// {
///   &options.vec
/// }
///
/// fn foo( options : &SubBigOptions ) -> bool
/// {
///   !options.cond
/// }
///
/// let options1 = BigOptions
/// {
///   cond : true,
///   int : -14,
///   ..Default::default()
/// };
/// boo( &options1 );
///
/// let mut options2 = SubBigOptions::default();
/// options2.components_set( &options1 );
/// foo( &options2 );
/// ```
///
/// Which expands approximately into :
///
/// ```rust
/// use former::{ SetComponent, SetComponents };
/// 
/// struct BigOptions
/// {
///   cond : bool,
///   int : i32,
///   str : String,
///   vec : Vec< u8 >,
/// }
/// 
/// impl< IntoT > SetComponent< bool, IntoT > for BigOptions
/// where
///   IntoT : Into< bool >,
/// {
///   #[ inline( always ) ]
///   fn set( &mut self, component : IntoT )
///   {
///     self.cond = component.into();
///   }
/// }
/// 
/// #[ allow( non_snake_case ) ]
/// impl< IntoT > SetComponent< i32, IntoT > for BigOptions
/// where
///   IntoT : Into< i32 >,
/// {
///   #[ inline( always ) ]
///   fn set( &mut self, component : IntoT )
///   {
///     self.int = component.into();
///   }
/// }
/// 
/// #[ allow( non_snake_case ) ]
/// impl< IntoT > SetComponent< String, IntoT > for BigOptions
/// where
///   IntoT : Into< String >,
/// {
///   #[ inline( always ) ]
///   fn set( &mut self, component : IntoT )
///   {
///     self.str = component.into();
///   }
/// }
/// 
/// #[ allow( non_snake_case ) ]
/// impl< IntoT > SetComponent< Vec< u8 >, IntoT > for BigOptions
/// where
///   IntoT : Into< Vec< u8 > >,
/// {
///   #[ inline( always ) ]
///   fn set( &mut self, component : IntoT )
///   {
///     self.vec = component.into();
///   }
/// }
/// 
/// pub trait BigOptionsSetComponents< IntoT >
/// where
///   IntoT : Into< bool >,
///   IntoT : Into< i32 >,
///   IntoT : Into< String >,
///   IntoT : Into< Vec< u8 > >,
///   IntoT : Clone,
/// {
///   fn components_set( &mut self, component : IntoT );
/// }
/// 
/// impl< T, IntoT > BigOptionsSetComponents< IntoT > for T
/// where
///   T : former::SetComponent< bool, IntoT >,
///   T : former::SetComponent< i32, IntoT >,
///   T : former::SetComponent< String, IntoT >,
///   T : former::SetComponent< Vec< u8 >, IntoT >,
///   IntoT : Into< bool >,
///   IntoT : Into< i32 >,
///   IntoT : Into< String >,
///   IntoT : Into< Vec< u8 > >,
///   IntoT : Clone,
/// {
///   #[ inline( always ) ]
///   fn components_set( &mut self, component : IntoT )
///   {
///     former::SetComponent::< bool, _ >::set( self, component.clone() );
///     former::SetComponent::< i32, _ >::set( self, component.clone() );
///     former::SetComponent::< String, _ >::set( self, component.clone() );
///     former::SetComponent::< Vec< u8 >, _ >::set( self, component.clone() );
///   }
/// }
///
/// struct SubBigOptions
/// {
///   cond : bool,
///   int : i32,
/// }
///
/// #[ allow( non_snake_case ) ]
/// impl< IntoT > SetComponent< bool, IntoT > for SubBigOptions
/// where
///   IntoT : Into< bool >,
/// {
///   #[ inline( always ) ]
///   fn set( &mut self, component : IntoT )
///   {
///     self.cond = component.into();
///   }
/// }
///
/// #[ allow( non_snake_case ) ]
/// impl< IntoT > SetComponent< i32, IntoT > for SubBigOptions
/// where
///     IntoT : Into< i32 >,
/// {
///   #[ inline( always ) ]
///   fn set( &mut self, component : IntoT )
///   {
///     self.int = component.into();
///   }
/// }
///
/// pub trait SubBigOptionsSetComponents< IntoT >
/// where
///   IntoT : Into< bool >,
///   IntoT : Into< i32 >,
///   IntoT : Clone,
/// {
///   fn components_set( &mut self, component : IntoT );
/// }
///
/// impl< T, IntoT > SubBigOptionsSetComponents< IntoT > for T
/// where
///   T : former::SetComponent< bool, IntoT >,
///   T : former::SetComponent< i32, IntoT >,
///   IntoT : Into< bool >,
///   IntoT : Into< i32 >,
///   IntoT : Clone,
/// {
///   #[ inline( always ) ]
///   fn components_set( &mut self, component : IntoT )
///   {
///     former::SetComponent::< bool, _ >::set( self, component.clone() );
///     former::SetComponent::< i32, _ >::set( self, component.clone() );
///   }
/// }
///
/// impl From< &BigOptions > for bool
/// {
///   fn from( value : &BigOptions ) -> Self
///   {
///     value.cond
///   }
/// }
///
/// impl From< &BigOptions > for i32
/// {
///   fn from( value : &BigOptions ) -> Self
///   {
///     value.int
///   }
/// }
///
/// fn boo( options : &BigOptions ) -> &Vec< u8 >
/// {
///   &options.vec
/// }
///
/// fn foo( options : &SubBigOptions ) -> bool
/// {
///   !options.cond
/// }
///
/// let options1 = BigOptions
/// {
///   cond : true,
///   int : -14,
///   ..Default::default()
/// };
/// boo( &options1 );
/// let mut options2 = SubBigOptions::default();
/// options2.components_set( &options1 );
/// foo( &options2 );
/// ```
///
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_set_components" ) ]
#[ proc_macro_derive( SetComponents, attributes( debug ) ) ]
pub fn set_components( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::set_components::set_components( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

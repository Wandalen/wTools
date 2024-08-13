// #![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/clone_dyn_meta/latest/clone_dyn_meta/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ cfg
(
  any
  (
    feature = "derive_as_mut",
    feature = "derive_as_ref",
    feature = "derive_deref",
    feature = "derive_deref_mut",
    feature = "derive_from",
    feature = "derive_index",
    feature = "derive_index_mut",
    feature = "derive_inner_from",
    feature = "derive_new",
    feature = "derive_variadic_from",
    feature = "derive_not",
    feature = "derive_phantom"
  )
)]
#[ cfg( feature = "enabled" ) ]
mod derive;
// #[ cfg
// (
//   any
//   (
//     feature = "derive_as_mut",
//     feature = "derive_as_ref",
//     feature = "derive_deref",
//     feature = "derive_deref_mut",
//     feature = "derive_from",
//     feature = "derive_index",
//     feature = "derive_index_mut",
//     feature = "derive_inner_from",
//     feature = "derive_new",
//     feature = "derive_variadic_from",
//     feature = "derive_not",
//     feature = "derive_phantom"
//   )
// )]
// #[ cfg( feature = "enabled" ) ]
// use derive::*;


///
/// Provides an automatic `From` implementation for struct wrapping a single value.
///
/// This macro simplifies the conversion of an inner type to an outer struct type
/// when the outer type is a simple wrapper around the inner type.
///
/// ## Example Usage
///
/// Instead of manually implementing `From< bool >` for `IsTransparent`:
///
/// ```rust
/// pub struct IsTransparent( bool );
///
/// impl From< bool > for IsTransparent
/// {
///   #[ inline( always ) ]
///   fn from( src : bool ) -> Self
///   {
///     Self( src )
///   }
/// }
/// ```
///
/// Use `#[ derive( From ) ]` to automatically generate the implementation:
///
/// ```rust
/// # use derive_tools_meta::*;
/// #[ derive( From ) ]
/// pub struct IsTransparent( bool );
/// ```
///
/// The macro facilitates the conversion without additional boilerplate code.
///

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_from" ) ]
#[ proc_macro_derive
(
  From,
  attributes
  (
    debug, // item
    from, // field
  )
)]
pub fn from( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::from::from( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

///
/// Provides an automatic `new` implementation for struct wrapping a single value.
///
/// This macro simplifies the conversion of an inner type to an outer struct type
/// when the outer type is a simple wrapper around the inner type.
///
/// ## Example Usage
///
/// Instead of manually implementing `new` for `IsTransparent`:
///
/// ```rust
/// pub struct IsTransparent( bool );
///
/// impl IsTransparent
/// {
///   #[ inline( always ) ]
///   fn new( src : bool ) -> Self
///   {
///     Self( src )
///   }
/// }
/// ```
///
/// Use `#[ derive( New ) ]` to automatically generate the implementation:
///
/// ```rust
/// # use derive_tools_meta::*;
/// #[ derive( New ) ]
/// pub struct IsTransparent( bool );
/// ```
///
/// The macro facilitates the conversion without additional boilerplate code.
///

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_new" ) ]
#[ proc_macro_derive
(
  New,
  attributes
  (
    debug, // item
    new, // field
  )
)]
pub fn new( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::new::new( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

// ///
// /// Alias for derive `From`. Provides an automatic `From` implementation for struct wrapping a single value.
// ///
// /// This macro simplifies the conversion of an inner type to an outer struct type
// /// when the outer type is a simple wrapper around the inner type.
// ///
// /// ## Example Usage
// ///
// /// Instead of manually implementing `From< bool >` for `IsTransparent`:
// ///
// /// ```rust
// /// pub struct IsTransparent( bool );
// ///
// /// impl From< bool > for IsTransparent
// /// {
// ///   #[ inline( always ) ]
// ///   fn from( src : bool ) -> Self
// ///   {
// ///     Self( src )
// ///   }
// /// }
// /// ```
// ///
// /// Use `#[ derive( FromInner ) ]` to automatically generate the implementation:
// ///
// /// ```rust
// /// # use derive_tools_meta::*;
// /// #[ derive( FromInner ) ]
// /// pub struct IsTransparent( bool );
// /// ```
// ///
// /// The macro facilitates the conversion without additional boilerplate code.
// ///
//
// #[ cfg( feature = "enabled" ) ]
// #[ cfg( feature = "derive_from" ) ]
// #[ proc_macro_derive( FromInner, attributes( debug ) ) ]
// pub fn from( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
// {
//   let result = derive::from::from( input );
//   match result
//   {
//     Ok( stream ) => stream.into(),
//     Err( err ) => err.to_compile_error().into(),
//   }
// }

///
/// Derive macro to implement From converting outer type into inner when-ever it's possible to do automatically.
///
/// ### Sample :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// # use derive_tools_meta::*;
/// #[ derive( InnerFrom ) ]
/// pub struct IsTransparent( bool );
/// ```
///
/// Instead of this
///
/// ```rust
/// pub struct IsTransparent( bool );
/// impl From< IsTransparent > for bool
/// {
///   #[ inline( always ) ]
///   fn from( src : IsTransparent ) -> Self
///   {
///     src.0
///   }
/// }
/// ```

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_inner_from" ) ]
#[ proc_macro_derive( InnerFrom, attributes( debug ) ) ]
pub fn inner_from( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::inner_from::inner_from( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

///
/// Derive macro to implement Deref when-ever it's possible to do automatically.
///
/// ### Sample :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// # use derive_tools_meta::*;
/// #[ derive( Deref ) ]
/// pub struct IsTransparent( bool );
/// ```
///
/// Instead of this
///
/// ```rust
/// pub struct IsTransparent( bool );
/// impl core::ops::Deref for IsTransparent
/// {
///   type Target = bool;
///   #[ inline( always ) ]
///   fn deref( &self ) -> &Self::Target
///   {
///     &self.0
///   }
/// }
/// ```

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_deref" ) ]
#[ proc_macro_derive( Deref, attributes( debug ) ) ]
pub fn deref( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::deref::deref( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

///
/// Derive macro to implement Deref when-ever it's possible to do automatically.
///
/// ### Sample :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// # use derive_tools_meta::DerefMut;
/// #[ derive( DerefMut ) ]
/// pub struct IsTransparent( bool );
///
/// impl ::core::ops::Deref for IsTransparent
/// {
///   type Target = bool;
///   #[ inline( always ) ]
///   fn deref( &self ) -> &Self::Target
///   {
///     &self.0
///   }
/// }
/// ```
///
/// Instead of this
///
/// ```rust
/// pub struct IsTransparent( bool );
/// impl ::core::ops::Deref for IsTransparent
/// {
///   type Target = bool;
///   #[ inline( always ) ]
///   fn deref( &self ) -> &Self::Target
///   {
///     &self.0
///   }
/// }
/// impl ::core::ops::DerefMut for IsTransparent
/// {
///   #[ inline( always ) ]
///   fn deref_mut( &mut self ) -> &mut Self::Target
///   {
///     &mut self.0
///   }
/// }
///
/// ```

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_deref_mut" ) ]
#[ proc_macro_derive( DerefMut, attributes( debug ) ) ]
pub fn deref_mut( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::deref_mut::deref_mut( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

///
/// Derive macro to implement AsRef when-ever it's possible to do automatically.
///
/// ### Sample :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// # use derive_tools_meta::*;
/// #[ derive( AsRef ) ]
/// pub struct IsTransparent( bool );
/// ```
///
/// Instead of this
///
/// ```rust
/// pub struct IsTransparent( bool );
/// impl AsRef< bool > for IsTransparent
/// {
///   fn as_ref( &self ) -> &bool
///   {
///     &self.0
///   }
/// }
/// ```

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_as_ref" ) ]
#[ proc_macro_derive( AsRef, attributes( debug ) ) ]
pub fn as_ref( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::as_ref::as_ref( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

///
/// Derive macro to implement AsMut when-ever it's possible to do automatically.
///
/// ### Sample :: struct instead of macro.
///
/// Write this
///
/// ```rust
/// # use derive_tools_meta::*;
/// #[ derive( AsMut ) ]
/// pub struct IsTransparent( bool );
/// ```
///
/// Instead of this
///
/// ```rust
/// pub struct IsTransparent( bool );
/// impl AsMut< bool > for IsTransparent
/// {
///   fn as_mut( &mut self ) -> &mut bool
///   {
///     &mut self.0
///   }
/// }
///
/// ```

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_as_mut" ) ]
#[ proc_macro_derive( AsMut, attributes( debug ) ) ]
pub fn as_mut( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::as_mut::as_mut( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

///
/// The `derive_variadic_from` macro is designed to provide a way to implement the `From`-like
/// traits for structs with a variable number of fields, allowing them to be constructed from
/// tuples of different lengths or from individual arguments. This functionality is particularly
/// useful for creating flexible constructors that enable different methods of instantiation for
/// a struct. By automating the implementation of traits, this macro reduces boilerplate code
/// and enhances code readability and maintainability.
///
/// ### Key Features
///
/// - **Flexible Construction**: Allows a struct to be constructed from different numbers of
///   arguments, converting each to the appropriate type.
/// - **Tuple Conversion**: Enables the struct to be constructed from tuples, leveraging the
///   `From` and `Into` traits for seamless conversion.
/// - **Code Generation**: Automates the implementation of these traits, reducing the need for
///   manual coding and ensuring consistent constructors.
///
/// ### Limitations
///
/// Currently, the macro supports up to 3 arguments. If your struct has more than 3 fields, the
/// derive macro will generate no implementation. It supports tuple conversion, allowing structs
/// to be instantiated from tuples by leveraging the `From` and `Into` traits for seamless conversion.
///
/// ### Example Usage
///
/// This example demonstrates the use of the `variadic_from` macro to implement flexible
/// constructors for a struct, allowing it to be instantiated from different numbers of
/// arguments or tuples. It also showcases how to derive common traits like `Debug`,
/// `PartialEq`, `Default`, and `VariadicFrom` for the struct.
///
/// ```rust
/// #[ cfg( not( all(feature = "enabled", feature = "type_variadic_from", feature = "derive_variadic_from" ) ) ) ]
/// fn main(){}
/// #[ cfg( all( feature = "enabled", feature = "type_variadic_from", feature = "derive_variadic_from" ) )]
/// fn main()
/// {
///   use variadic_from::exposed::*;
///
///   // Define a struct `MyStruct` with fields `a` and `b`.
///   // The struct derives common traits like `Debug`, `PartialEq`, `Default`, and `VariadicFrom`.
///   #[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
///   // Use `#[ debug ]` to expand and debug generate code.
///   // #[ debug ]
///   struct MyStruct
///   {
///     a : i32,
///     b : i32,
///   }
///
///   // Implement the `From1` trait for `MyStruct`, which allows constructing a `MyStruct` instance
///   // from a single `i32` value by assigning it to both `a` and `b` fields.
///   impl From1< i32 > for MyStruct
///   {
///     fn from1( a : i32 ) -> Self { Self { a, b : a } }
///   }
///
///   let got : MyStruct = from!();
///   let exp = MyStruct { a : 0, b : 0 };
///   assert_eq!( got, exp );
///
///   let got : MyStruct = from!( 13 );
///   let exp = MyStruct { a : 13, b : 13 };
///   assert_eq!( got, exp );
///
///   let got : MyStruct = from!( 13, 14 );
///   let exp = MyStruct { a : 13, b : 14 };
///   assert_eq!( got, exp );
///
///   dbg!( exp );
///   //> MyStruct {
///   //>   a : 13,
///   //>   b : 14,
///   //> }
/// }
/// ```
///
/// ### Debugging
///
/// If your struct has a `debug` attribute, the macro will print information about the generated code for diagnostic purposes.
///
/// ```rust, ignore
/// #[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
/// // Use `#[ debug ]` to expand and debug generate code.
/// // #[ debug ]
/// item MyStruct
/// {
///   a : i32,
///   b : i32,
/// }
/// ```
///

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_variadic_from" ) ]
#[ proc_macro_derive( VariadicFrom, attributes( debug ) ) ]
pub fn derive_variadic_from( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::variadic_from::variadic_from( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

/// Provides an automatic [Not](core::ops::Not) trait  implementation for struct.
///
/// This macro simplifies the creation of a logical negation or complement operation
/// for structs that encapsulate values which support the `!` operator.
///
/// ## Example Usage
///
/// Instead of manually implementing [Not](core::ops::Not) for [IsActive]:
///
/// ```rust
/// use core::ops::Not;
///
/// pub struct IsActive( bool );
///
/// impl Not for IsActive
/// {
///   type Output = IsActive;
///
///   fn not(self) -> Self::Output
///   {
///     IsActive(!self.0)
///   }
/// }
/// ```
///
/// Use `#[ derive( Not ) ]` to automatically generate the implementation:
///
/// ```rust
/// # use derive_tools_meta::*;
/// #[ derive( Not ) ]
/// pub struct IsActive( bool );
/// ```
///
/// The macro automatically implements the [not](core::ops::Not::not) method, reducing boilerplate code and potential errors.
///
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_not" ) ]
#[ proc_macro_derive
(
  Not,
  attributes
  (
    debug, // item
    not, // field
  )
)]
pub fn derive_not( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::not::not( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

///
/// Provides an automatic `PhantomData` field for a struct based on its generic types.
///
/// This macro simplifies the addition of a `PhantomData` field to a struct
/// to indicate that the struct logically owns instances of the generic types,
/// even though it does not store them.
///
/// ## Example Usage
///
/// Instead of manually adding `PhantomData<T>` to `MyStruct`:
///
/// ```rust
/// use std::marker::PhantomData;
///
/// pub struct MyStruct<T>
/// {
///     data: i32,
///     _phantom: PhantomData<T>,
/// }
/// ```
///
/// Use `#[ phantom ]` to automatically generate the `PhantomData` field:
///
/// ```rust
/// use derive_tools_meta::*;
///
/// #[ phantom ]
/// pub struct MyStruct< T >
/// {
///     data: i32,
/// }
/// ```
///
/// The macro facilitates the addition of the `PhantomData` field without additional boilerplate code.
///

#[ cfg( feature = "enabled" ) ]
#[ cfg ( feature = "derive_phantom" ) ]
#[ proc_macro_attribute ]
pub fn phantom( _attr: proc_macro::TokenStream, input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::phantom::phantom( _attr, input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

///
/// Provides an automatic [Index](core::ops::Index) trait implementation when-ever it's possible.
///
/// This macro simplifies the indexing syntax of struct type.
///
/// ## Example Usage
//
/// Instead of manually implementing `Index< T >` for `IsTransparent`:
///
/// ```rust
/// use core::ops::Index;
///
/// pub struct IsTransparent< T >
/// {
///     a : Vec< T >,
/// }
///
/// impl< T > Index< usize > for IsTransparent< T > 
/// {
///   type Output = T;
///
///   #[ inline( always ) ]
///   fn index( &self, index : usize ) -> &Self::Output 
///   {
///     &self.a[ index ]
///   }
/// }
/// ```
///
/// Use `#[ index ]` to automatically generate the implementation:
///
/// ```rust
/// use derive_tools_meta::*;
/// 
/// #[ derive( Index ) ]
/// pub struct IsTransparent< T > 
/// {
///  #[ index ]
///   a : Vec< T >  
/// };
/// ```
///

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_index" ) ]
#[ proc_macro_derive
( 
  Index, 
  attributes
  ( 
    debug, // item 
    index, // field
  ) 
)]
pub fn derive_index( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::index::index( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

///
/// Provides an automatic [IndexMut](core::ops::IndexMut) trait implementation when-ever it's possible.
///
/// This macro simplifies the indexing syntax of struct type.
///
/// ## Example Usage
//
/// Instead of manually implementing `IndexMut< T >` for `IsTransparent`:
///
/// ```rust
/// use core::ops::{ Index, IndexMut };
/// pub struct IsTransparent< T >
/// {
///     a : Vec< T >,
/// }
///
/// impl< T > Index< usize > for IsTransparent< T > 
/// {
///   type Output = T;
///
///   #[ inline( always ) ]
///   fn index( &self, index : usize ) -> &Self::Output 
///   {
///     &self.a[ index ]
///   }
/// }
///
/// impl< T > IndexMut< usize > for IsTransparent< T >
/// {
///   fn index_mut( &mut self, index : usize ) -> &mut Self::Output 
///   {
///     &mut self.a[ index ]
///   }
/// }
/// ```
///
/// Use `#[ index ]` on field or `#[ index( name = field_name )]` on named items to automatically generate the implementation:
///
/// ```rust
/// use derive_tools_meta::*;
/// #[derive( IndexMut )]
/// pub struct IsTransparent< T > 
/// { 
///   #[ index ]
///   a : Vec< T >  
/// };
/// ```
///

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_index_mut" ) ]
#[ proc_macro_derive
( 
  IndexMut, 
  attributes
  ( 
    debug, // item 
    index, // field
  ) 
)]
pub fn derive_index_mut( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = derive::index_mut::index_mut( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}


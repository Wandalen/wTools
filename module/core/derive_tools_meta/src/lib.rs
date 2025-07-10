#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_3_black.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_3_black.png" ) ]
#![ doc( html_root_url = "https://docs.rs/derive_tools_meta/latest/derive_tools_meta/" ) ]
#![ deny( rust_2018_idioms ) ]
#![ deny( future_incompatible ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]
#![ deny( unsafe_code ) ]
#![ allow( clippy::upper_case_acronyms ) ]
#![ warn( clippy::unwrap_used ) ]
#![ warn( clippy::default_trait_access ) ]
#![ warn( clippy::wildcard_imports ) ]

//!
//! Collection of derive macros for `derive_tools`.
//!

mod derive;


///
/// Implement `AsMut` for a structure.
///
/// ### Sample.
///
/// ```text
/// use derive_tools::AsMut;
///
/// #[ derive( AsMut ) ]
/// struct MyStruct
/// {
///   #[ as_mut( original ) ]
///   a : i32,
///   b : i32,
/// }
///
/// let mut my_struct = MyStruct { a : 1, b : 2 };
/// *my_struct.as_mut() += 1;
/// dbg!( my_struct.a );
/// ```
///
/// To learn more about the feature, study the module [`derive_tools::AsMut`](https://docs.rs/derive_tools/latest/derive_tools/as_mut/index.html).
///
#[ proc_macro_derive( AsMut, attributes( as_mut ) ) ]
pub fn as_mut( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  derive::as_mut::as_mut( input ).unwrap_or_else( macro_tools::syn::Error::into_compile_error ).into()
}

///
/// Implement `AsRef` for a structure.
///
/// ### Sample.
///
/// ```text
/// use derive_tools::AsRef;
///
/// #[ derive( AsRef ) ]
/// struct MyStruct
/// {
///   #[ as_ref( original ) ]
///   a : i32,
///   b : i32,
/// }
///
/// let my_struct = MyStruct { a : 1, b : 2 };
/// dbg!( my_struct.as_ref() );
/// ```
///
/// To learn more about the feature, study the module [`derive_tools::AsRef`](https://docs.rs/derive_tools/latest/derive_tools/as_ref/index.html).
///
#[ proc_macro_derive( AsRef, attributes( as_ref ) ) ]
pub fn as_ref( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  derive::as_ref::as_ref( input ).unwrap_or_else( macro_tools::syn::Error::into_compile_error ).into()
}

///
/// Implement `Deref` for a structure.
///
/// ### Sample.
///
/// ```text
/// use derive_tools::Deref;
///
/// #[ derive( Deref ) ]
/// struct MyStruct
/// {
///   #[ deref( original ) ]
///   a : i32,
///   b : i32,
/// }
///
/// let my_struct = MyStruct { a : 1, b : 2 };
/// dbg!( *my_struct );
/// ```
///
/// To learn more about the feature, study the module [`derive_tools::Deref`](https://docs.rs/derive_tools/latest/derive_tools/deref/index.html).
///
#[ proc_macro_derive( Deref, attributes( deref, debug ) ) ]
pub fn deref( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  derive::deref::deref( input ).unwrap_or_else( macro_tools::syn::Error::into_compile_error ).into()
}

///
/// Implement `DerefMut` for a structure.
///
/// ### Sample.
///
/// ```text
/// use derive_tools::DerefMut;
///
/// #[ derive( DerefMut ) ]
/// struct MyStruct
/// {
///   #[ deref_mut( original ) ]
///   a : i32,
///   b : i32,
/// }
///
/// let mut my_struct = MyStruct { a : 1, b : 2 };
/// *my_struct += 1;
/// dbg!( my_struct.a );
/// ```
///
/// To learn more about the feature, study the module [`derive_tools::DerefMut`](https://docs.rs/derive_tools/latest/derive_tools/deref_mut/index.html).
///
#[ proc_macro_derive( DerefMut, attributes( deref_mut ) ) ]
pub fn deref_mut( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  derive::deref_mut::deref_mut( input ).unwrap_or_else( macro_tools::syn::Error::into_compile_error ).into()
}

///
/// Implement `From` for a structure.
///
/// ### Sample.
///
/// ```text
/// use derive_tools::From;
///
/// #[ derive( From ) ]
/// struct MyStruct( i32 );
///
/// let my_struct = MyStruct::from( 13 );
/// dbg!( my_struct.0 );
/// ```
///
/// To learn more about the feature, study the module [`derive_tools::From`](https://docs.rs/derive_tools/latest/derive_tools/from/index.html).
///
#[ proc_macro_derive( From, attributes( from ) ) ]
pub fn from( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  derive::from::from( input ).unwrap_or_else( macro_tools::syn::Error::into_compile_error ).into()
}

///
/// Implement `Index` for a structure.
///
/// ### Sample.
///
/// ```text
/// use derive_tools::Index;
///
/// #[ derive( Index ) ]
/// struct MyStruct( i32 );
///
/// let my_struct = MyStruct( 13 );
/// dbg!( my_struct[ 0 ] );
/// ```
///
/// To learn more about the feature, study the module [`derive_tools::Index`](https://docs.rs/derive_tools/latest/derive_tools/index/index.html).
///
#[ proc_macro_derive( Index, attributes( index ) ) ]
pub fn index( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  derive::index::index( input ).unwrap_or_else( macro_tools::syn::Error::into_compile_error ).into()
}

///
/// Implement `IndexMut` for a structure.
///
/// ### Sample.
///
/// ```text
/// use derive_tools::IndexMut;
///
/// #[ derive( IndexMut ) ]
/// struct MyStruct( i32 );
///
/// let mut my_struct = MyStruct( 13 );
/// my_struct[ 0 ] += 1;
/// dbg!( my_struct.0 );
/// ```
///
/// To learn more about the feature, study the module [`derive_tools::IndexMut`](https://docs.rs/derive_tools/latest/derive_tools/index_mut/index.html).
///
#[ proc_macro_derive( IndexMut, attributes( index_mut ) ) ]
pub fn index_mut( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  derive::index_mut::index_mut( input ).unwrap_or_else( macro_tools::syn::Error::into_compile_error ).into()
}

///
/// Implement `InnerFrom` for a structure.
///
/// ### Sample.
///
/// ```text
/// use derive_tools::InnerFrom;
///
/// #[ derive( InnerFrom ) ]
/// struct MyStruct( i32 );
///
/// let my_struct = MyStruct::inner_from( 13 );
/// dbg!( my_struct.0 );
/// ```
///
/// To learn more about the feature, study the module [`derive_tools::InnerFrom`](https://docs.rs/derive_tools/latest/derive_tools/inner_from/index.html).
///
#[ proc_macro_derive( InnerFrom, attributes( inner_from ) ) ]
pub fn inner_from( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  derive::inner_from::inner_from( input ).unwrap_or_else( macro_tools::syn::Error::into_compile_error ).into()
}

///
/// Implement `New` for a structure.
///
/// ### Sample.
///
/// ```text
/// use derive_tools::New;
///
/// #[ derive( New ) ]
/// struct MyStruct;
///
/// let my_struct = MyStruct::new();
/// dbg!( my_struct );
/// ```
///
/// To learn more about the feature, study the module [`derive_tools::New`](https://docs.rs/derive_tools/latest/derive_tools/new/index.html).
///
#[ proc_macro_derive( New, attributes( new ) ) ]
pub fn new( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  derive::new::new( input ).unwrap_or_else( macro_tools::syn::Error::into_compile_error ).into()
}

///
/// Implement `Not` for a structure.
///
/// ### Sample.
///
/// ```text
/// use derive_tools::Not;
///
/// #[ derive( Not ) ]
/// struct MyStruct( bool );
///
/// let my_struct = MyStruct( true );
/// dbg!( !my_struct );
/// ```
///
/// To learn more about the feature, study the module [`derive_tools::Not`](https://docs.rs/derive_tools/latest/derive_tools/not/index.html).
///
#[ proc_macro_derive( Not, attributes( not ) ) ]
pub fn not( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  derive::not::not( input ).unwrap_or_else( macro_tools::syn::Error::into_compile_error ).into()
}

// ///\n// /// Implement `PhantomData` for a structure.\n// ///\n// /// ### Sample.\n// ///\n// /// ```text\n// /// use derive_tools::PhantomData;\n// ///\n// /// #\[ derive\( PhantomData \) \]\n// /// struct MyStruct< T >\( core::marker::PhantomData< T > \);\n// ///\n// /// let my_struct = MyStruct::\< i32 >\( core::marker::PhantomData \);\n// /// dbg!\( my_struct \);\n// /// ```\n// ///\n// /// To learn more about the feature, study the module \[`derive_tools::PhantomData`\]\(https://docs.rs/derive_tools/latest/derive_tools/phantom_data/index.html\)\.
// qqq: This derive is currently generating invalid code by attempting to implement `core::marker::PhantomData` as a trait.
// It needs to be re-designed to correctly handle `PhantomData` usage, likely by adding a field to the struct.
// Temporarily disabling to allow other tests to pass.
// #[ proc_macro_derive( PhantomData, attributes( phantom_data ) ]
// pub fn phantom_data( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
// {
//   derive::phantom::phantom( input ).unwrap_or_else( macro_tools::syn::Error::into_compile_error ).into()
// }

///
/// Implement `VariadicFrom` for a structure.
///
/// ### Sample.
///
/// ```text
/// use derive_tools::VariadicFrom;
///
/// #[ derive( VariadicFrom ) ]
/// struct MyStruct( i32 );
///
/// let my_struct = MyStruct::variadic_from( 13 );
/// dbg!( my_struct.0 );
/// ```
///
/// To learn more about the feature, study the module [`derive_tools::VariadicFrom`](https://docs.rs/derive_tools/latest/derive_tools/variadic_from/index.html).
///
#[ proc_macro_derive( VariadicFrom, attributes( variadic_from ) ) ]
pub fn variadic_from( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  derive::variadic_from::variadic_from( input ).unwrap_or_else( macro_tools::syn::Error::into_compile_error ).into()
}

/// 
/// # Specification `#[ derive ( Add ) ]`
///
/// ## Overview
/// This macro generates an implementation of the [`core::ops::Add`] trait
/// for structs whose fields all implement `Add`. It supports both named and tuple-style structs.
///
/// ## Supported Structures
/// - Named structs: `struct My { a : T, b : T }`
/// - Tuple structs: `struct My( T, T )`
/// - Enums — **unimplemented**
/// - Unit structs: `struct My;` — **not supported**
///
/// ## Item-Level Attributes
/// The macro recognizes the following struct-level attributes:
///
/// | Attribute                  | Target        | Description                                                           |
/// |----------------------------|---------------|-----------------------------------------------------------------------|
/// | `#[ derive( Add ) ]`       | Struct        | Enables generation of `Add` implementation                            |
/// | `#[add(error = SomeType)]` | Struct / Enum | Overrides the default error type (`String`) used in `Result<Self, E>` |
/// 
/// /// ### Notes:
/// - `SomeType` must be a valid Rust type (e.g., `MyError`, `Box<dyn std::error::Error>`, etc.).
/// - If omitted, the default error type is `String`.
/// - The provided error type must implement `From<String>` or be manually handled in the generated code.
/// 
/// ## Field-Level Attributes 
///
/// No field-level attributes supported yet.
///
/// ## Generated Output
/// ```ignore
/// impl Add for MyStruct 
/// {
///     type Output = Self;
///     fn add(self, other: Self) -> Self::Output 
///     {
///         Self 
///         {
///             field1: self.field1 + other.field1,
///             field2: self.field2 + other.field2,
///             ...
///         }
///     }
/// }
/// ```
/// Or for tuple structs:
/// ```ignore
/// Self( self.0 + other.0, self.1 + other.1 )
/// ```
///
/// ## Requirements
/// All fields must implement [`core::ops::Add`].
///
/// ## Test Plan
///
/// | ID   | Struct Type              | Fields                      | Should Compile?  | Should Work at Runtime?| Notes                                |
/// |------|--------------------------|-----------------------------|------------------|------------------------|--------------------------------------|
/// | T1.1 | Named                    | `{x: i32, y: i32}`          | +                | +                      | Basic case                           |
/// | T1.2 | Tuple                    | `(i32)`                     | +                | +                      | Tuple struct                         |
/// | T1.3 | Unit                     | `()`                        | -                | —                      | Should be rejected                   |
/// | T1.4 | Named with String        | `{x: String}`               | -                | —                      | String doesn't implement `Add<Output = String>` in all cases |
/// | T1.5 | Generic                  | `{x: T}`                    | +                | + (if T: Add)          | Test with bounds                     |
/// | T1.6 | Enum,                    | `enum E { One(i32) }`       | -                | -                      | Unimplemented yet                    |
/// 
/// ## Input / Expected Output
///
/// | ID   | Input Expression                              | Expected Output / Behavior                          | Notes                                           |
/// |------|-----------------------------------------------|-----------------------------------------------------|-------------------------------------------------|
/// | T2.1 | `E::One(3) + E::One(3)`                       | `Ok(E::One(6))`                                     | Basic enum case                                 |
/// | T2.2 | `E::One(i32) + E::Two(i32)`                   | `Error("Mismatched variant")`                       | Mismatched variant                              |
/// | T2.3 | `S { x: 2, y: 3 } + S { x: 4, y: 1 }`         | `S { x: 6, y: 4 }`                                  | Basic case                                      |
/// | T2.4 | `Empty {} + Empty {}`                         | Compile error                                       | Struct has zero fields                          |
/// | T2.5 | `S(String::from("1")) + S(String::from("2"))` | Compile error                                       | String doesn't implement `Add<Output = String>` |
/// 
/// ## Example Usage
/// ```
/// use derive_tools_meta::Add;
/// 
/// #[ derive( Add ) ]
/// struct MyNamedStruct
/// {
///   x: i32
/// };
/// 
/// #[ derive( Add ) ]
/// struct MyTupleStruct( i32 );
///
/// type Er = Box<dyn std::error::Error>;
/// #[ derive( Add ) ]
/// #[ add(error = Er)]
/// enum MyEnum 
/// {
///   One,
///   Two( i32 ),
/// }
/// 
/// let my_struct1 = MyNamedStruct { x : 3 };
/// let my_struct2 = MyNamedStruct { x : 3 };
/// 
/// let result = ( my_struct1 + my_struct2 );
/// 
/// assert_eq!( result.x, 6 );
/// 
/// let my_struct1 = MyTupleStruct ( 3 );
/// let my_struct2 = MyTupleStruct ( 3 );
/// 
/// let result = ( my_struct1 + my_struct2 );
/// assert_eq!( result.0, 6 );
/// 
/// let my_enum1 = MyEnum::Two( 3 );
/// let my_enum2 = MyEnum::Two( 3 );
/// 
/// let result = ( my_enum1 + my_enum2 );
///
/// assert!( result.is_ok() );
/// let value = if let MyEnum::Two( val ) = result.unwrap() { val } else { panic!( "Expected MyEnum::Two variant" ) };
/// assert_eq!( value, 6 );
/// ```
///
#[ proc_macro_derive( Add, attributes(add) ) ]
pub fn add( input : proc_macro::TokenStream ) -> proc_macro::TokenStream 
{
  derive::add::add( input ).unwrap_or_else( macro_tools::syn::Error::into_compile_error ).into()
} 

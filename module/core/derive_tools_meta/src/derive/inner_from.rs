
use super::*;
use macro_tools::{ type_struct, Result };

//

pub fn inner_from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< type_struct::TypeStructParsed >( input )?;
  let field_types = parsed.field_types();
  let field_names = parsed.field_names();
  let item_name = parsed.item_name.clone();
  let result =
  match ( field_types.len(), field_names )
  {
    ( 0, _ ) => unit( item_name ),
    ( 1, Some( field_names ) ) =>
    {
      let field_name = field_names.get( 0 ).unwrap();
      let field_type = field_types.get( 0 ).unwrap();
      from_impl_named( item_name, field_type, field_name )
    }
    ( 1, None ) =>
    {
      let field_type = field_types.get( 0 ).unwrap();
      from_impl( item_name, field_type )
    }
    ( _, Some( field_names ) ) =>
    {
      let params : Vec< proc_macro2::TokenStream > = field_names.iter()
      .map( | field_name | qt! { src.#field_name } )
      .collect();
      from_impl_multiple_fields( item_name, &field_types, &params )
    }
    ( _, None ) =>
    {
      let params : Vec< proc_macro2::TokenStream > = ( 0..field_types.len() )
      .map( | index |
      {
        let index : proc_macro2::TokenStream = index.to_string().parse().unwrap();
        qt! { src.#index }
      })
      .collect();
      from_impl_multiple_fields( item_name, &field_types, &params )
    }
  };
  Ok( result )
}

// qqq  : document, add example of generated code
/// Generates `From` implementation for the inner type regarding bounded type
/// Works with structs with a single named field
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::InnerFrom;
/// #[ derive( InnerFrom ) ]
/// pub struct Struct
/// {
///   value : bool,
/// }
/// ```
///
/// ## Output
/// ```rust
/// pub struct Struct
/// {
///   value : bool,
/// }
/// #[ allow( non_local_definitions ) ]
/// #[ automatically_derived ]
/// impl From< Struct > for bool
/// {
///   #[ inline( always ) ]
///   fn from( src : Struct ) -> Self
///   {
///     src.value
///   }
/// }
/// ```
///
fn from_impl_named
(
  item_name : syn::Ident,
  field_type : &syn::Type,
  field_name : &syn::Ident,
) -> proc_macro2::TokenStream
{
  qt!
  {
    #[ allow( non_local_definitions ) ]
    #[ automatically_derived ]
    impl From< #item_name > for #field_type
    {
      #[ inline( always ) ]
      fn from( src: #item_name ) -> Self
      {
        src.#field_name
      }
    }
  }
}

// qqq  : document, add example of generated code -- done
/// Generates `From` implementation for the only contained type regarding the bounded type
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::InnerFrom;
/// #[ derive( InnerFrom ) ]
/// pub struct Struct( bool );
/// ```
///
/// ## Output
/// ```rust
/// pub struct Struct( bool );
/// #[ allow( non_local_definitions ) ]
/// #[ automatically_derived ]
/// impl From< Struct > for bool
/// {
///   #[ inline( always ) ]
///   fn from( src : Struct ) -> Self
///   {
///     src.0
///   }
/// }
/// ```
///
fn from_impl
(
  item_name : syn::Ident,
  field_type : &syn::Type,
) -> proc_macro2::TokenStream
{
  qt!
  {
    #[ allow( non_local_definitions ) ]
    #[ automatically_derived ]
    impl From< #item_name > for #field_type
    {
      #[ inline( always ) ]
      fn from( src: #item_name ) -> Self
      {
        src.0
      }
    }
  }
}

// qqq  : document, add example of generated code -- done
/// Generates `From` implementation for the tuple type containing all the inner types regarding the bounded type
/// Can generate implementations both for structs with named fields and tuple structs.
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::InnerFrom;
/// #[ derive( InnerFrom ) ]
/// pub struct Struct( bool, i32 );
/// ```
///
/// ## Output
/// ```rust
/// pub struct Struct( bool, i32 );
/// #[ allow( non_local_definitions ) ]
/// #[ automatically_derived ]
/// impl From< Struct > for ( bool, i32 )
/// {
///   #[ inline( always ) ]
///   fn from( src : Struct ) -> Self
///   {
///     ( src.0, src.1 )
///   }
/// }
/// ```
///
fn from_impl_multiple_fields
(
  item_name : syn::Ident,
  field_types : &Vec< &syn::Type >,
  params : &Vec< proc_macro2::TokenStream >,
) -> proc_macro2::TokenStream
{
  qt!
  {
    #[ allow( non_local_definitions ) ]
    #[ automatically_derived ]
    impl From< #item_name > for ( #( #field_types ), *)
    {
      #[ inline( always ) ]
      fn from( src : #item_name ) -> Self
      {
        ( #( #params ), * )
      }
    }
  }
}

// qqq  : document, add example of generated code -- done
/// Generates `From` implementation for the unit type regarding the bound type
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::InnerFrom;
/// #[ derive( InnerFrom ) ]
/// pub struct Struct;
/// ```
///
/// ## Output
/// ```rust
/// pub struct Struct;
/// #[ allow( non_local_definitions ) ]
/// #[ allow( clippy::unused_imports ) ]
/// #[ automatically_derived]
/// impl From< Struct > for ()
/// {
///   #[ inline( always ) ]
///   fn from( src : Struct ) -> ()
///   {
///     ()
///   }
/// }
/// ```
///
fn unit( item_name : syn::Ident ) -> proc_macro2::TokenStream
{
  qt!
  {
    #[ allow( non_local_definitions ) ]
    #[ allow( clippy::unused_imports ) ]
    #[ automatically_derived ]
    impl From< #item_name > for ()
    {
      #[ inline( always ) ]
      fn from( src: #item_name ) -> ()
      {
        ()
      }
    }
  }
}

use super::*;
use macro_tools::{ type_struct, Result };

//

pub fn from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< type_struct::TypeStructParsed >( input )?;
  let field_types = parsed.field_types();
  let field_names = parsed.field_names();
  let item_name = parsed.item_name.clone();
  let result =
  match ( field_types.len(), field_names )
  {
    ( 0, _ ) => { generate_unit(item_name) },
    ( 1, Some( field_names ) ) => generate_from_single_field_named( &field_types[ 0 ], &field_names[ 0 ], item_name ),
    ( 1, None ) => generate_from_single_field( &field_types[ 0 ], item_name ),
    ( _, Some( field_names ) ) => generate_from_multiple_fields_named( &field_types, &field_names, item_name ),
    ( _, None ) => generate_from_multiple_fields( &field_types, item_name ),
  };

  Ok( result )
}

// qqq  : document, add example of generated code -- done
/// Generates `From` implementation for tuple structs with a single field
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::From;
/// #[ derive( From ) ]
/// pub struct IsTransparent
/// {
///   value : bool,
/// }
/// ```
///
/// ## Output
/// ```rust
/// pub struct IsTransparent
/// {
///   value : bool,
/// }
/// #[ automatically_derived ]
/// impl From< bool > for IsTransparent
/// {
///   #[ inline( always ) ]
///   fn from( src : bool ) -> Self
///   {
///     Self { value : src }
///   }
/// }
/// ```
///
fn generate_from_single_field_named
(
  field_type : &syn::Type,
  field_name : &syn::Ident,
  item_name : syn::Ident,
) -> proc_macro2::TokenStream
{
  qt!
  {
    #[ automatically_derived ]
    impl From< #field_type > for #item_name
    {
      #[ inline( always ) ]
      fn from( src : #field_type ) -> Self
      {
        Self { #field_name : src }
      }
    }
  }
}

// qqq  : document, add example of generated code -- done
/// Generates `From`` implementation for structs with a single named field
///
/// # Example of generated code
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::From;
/// #[ derive( From ) ]
/// pub struct IsTransparent( bool );
/// ```
/// 
/// ## Output
/// ```rust
/// pub struct IsTransparent( bool );
/// #[ automatically_derived ]
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
fn generate_from_single_field
(
  field_type : &syn::Type,
  item_name : syn::Ident,
) -> proc_macro2::TokenStream
{
  qt!
  {
    #[automatically_derived]
    impl From< #field_type > for #item_name
    {
      #[ inline( always ) ]
      fn from( src : #field_type ) -> Self
      {
        Self( src )
      }
    }
  }
}

// qqq : document, add example of generated code -- done
/// Generates `From` implementation for structs with multiple named fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::From;
/// #[ derive( From ) ]
/// pub struct Struct
/// {
///   value1 : bool,
///   value2 : i32,
/// }
/// ```
///
/// ## Output
/// ```rust
/// pub struct Struct
/// {
///   value1 : bool,
///   value2 : i32,
/// }
/// impl From< ( bool, i32 ) > for Struct
/// {
///   #[ inline( always ) ]
///   fn from( src : ( bool, i32 ) ) -> Self
///   {
///     Struct
///     {
///       value1 : src.0,
///       value2 : src.1,
///     }
///   }
/// }
/// ```
fn generate_from_multiple_fields_named
(
  field_types : &Vec< &syn::Type >,
  field_names : &Vec< syn::Ident >,
  item_name : syn::Ident
) -> proc_macro2::TokenStream
{
  let params: Vec< proc_macro2::TokenStream > = field_names
    .iter()
    .enumerate()
    .map(| ( index, field_name ) |
      {
        let index = index.to_string().parse::< proc_macro2::TokenStream >().unwrap();
        qt! { #field_name : src.#index }
      })
      .collect();

  qt!
  {
    impl From< ( #( #field_types ), * ) > for #item_name
    {
      #[ inline( always ) ]
      fn from( src : ( #( #field_types ), * ) ) -> Self
      {
        #item_name { #( #params ), * }
      }
    }
  }
}

// qqq  : document, add example of generated code -- done
/// Generates `From` implementation for tuple structs with multiple fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::From;
/// #[ derive( From ) ]
/// pub struct Struct( bool, i32 );
/// ```
///
/// ## Output
/// ```rust
/// pub struct Struct( bool, i32 );
/// impl From< ( bool, i32 ) > for Struct
/// {
///   #[ inline( always ) ]
///   fn from( src : ( bool, i32 ) ) -> Self
///   {
///     Struct( src.0, src.1 )
///   }
/// }
/// ```
fn generate_from_multiple_fields
(
  field_types : &Vec< &syn::Type >,
  item_name : syn::Ident,
) -> proc_macro2::TokenStream
{
  let params : Vec< proc_macro2::TokenStream > = ( 0..field_types.len() )
  .map( | index |
    {
      let index = index.to_string().parse::< proc_macro2::TokenStream >().unwrap();
        qt!( src.#index )
    } )
  .collect();

  qt!
  {
    impl From< ( #( #field_types ), * ) > for #item_name
    {
      #[ inline( always ) ]
      fn from( src : ( #( #field_types ), * ) ) -> Self
      {
        #item_name( #( #params ), * )
      }
    }
  }
}

// qqq  : document, add example of generated code -- done
/// Generates `From` implementation for unit structs
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::From;
/// #[ derive( From ) ]
/// pub struct IsTransparent;
/// ```
///
/// ## Output
/// ```rust
/// pub struct IsTransparent;
/// impl From< () > for IsTransparent
/// {
///   #[ inline( always ) ]
///   fn from( src : () ) -> Self
///   {
///     Self
///   }
/// }
/// ```
///
fn generate_unit( item_name : syn::Ident ) -> proc_macro2::TokenStream
{
  qt!
  {
    impl From< () > for #item_name
    {
      #[ inline( always ) ]
      fn from( src : () ) -> Self
      {
        Self
      }
    }
  }
}
use super::*;
use macro_tools::
{
  attr, 
  diag, 
  generic_params, 
  struct_like::StructLike, 
  Result
};

pub fn index( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream > {
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where ) 
  = generic_params::decompose( &parsed.generics() );

  let result = match parsed 
  {
    StructLike::Struct( ref item ) => 
    generate_struct
      ( 
        item_name, 
        &generics_impl, 
        &generics_ty, 
        &generics_where, 
        &item.fields 
      ),
    StructLike::Enum(_) => 
    todo!(),
    StructLike::Unit(_) => 
    todo!(),
  }?;

  if has_debug 
  {
    let about = format!( "derive : Not\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// An aggregator function to generate `Index` implementation for unit, tuple structs and the ones with named fields
fn generate_struct
(
  item_name: &syn::Ident,
  generics_impl: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields: &syn::Fields,
) 
-> Result< proc_macro2::TokenStream > 
{
  match fields 
  {
    
    syn::Fields::Named( fields ) => 
    generate_struct_named_fields
    (
      item_name, 
      generics_impl, 
      generics_ty, 
      generics_where, 
      fields
    ),
    
    syn::Fields::Unnamed( fields ) => 
    generate_struct_tuple_fields
    (
      item_name, 
      generics_impl, 
      generics_ty, 
      generics_where, 
      fields
    ),
  
    syn::Fields::Unit => Err( 
      syn::Error::new(
        fields.span(),
        "cannot infer type: Empty type cannot be indexed",
      ) 
    ),
  }
}

/// Generates `Index` implementation for structs with tuple fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::Index;
/// #[ derive( Index ) ]
/// pub struct Struct< T >( T );
/// ```
///
/// ## Output
/// ```rust
/// pub struct Struct< T >( T );
/// #[ automatically_derived ]
/// impl< T > ::core::ops::Index< usize > for Struct< T >
/// {
///   type Output = T;
///   #[ inline( always ) ]
///   fn index( &self, index: usize ) -> &Self::Output
///   {
///     match index 
///     {
///        0 => &self.0,
///        _ => panic!( "Index out of bounds" ),
///     }
///   }
/// }
/// ```
///

fn generate_struct_tuple_fields
(
  item_name: &syn::Ident,
  generics_impl: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields: &syn::FieldsUnnamed,
) 
-> Result< proc_macro2::TokenStream > 
{
  let fields = fields.unnamed.clone();

  // Get the type of the first field as the output type
  let field_type = match fields.first() {
    Some(field) => &field.ty,
    None => return Err(
        syn::Error::new( fields.span(), "cannot infer type: Empty type cannot be indexed" )
      ),
  };

  // Generate match arms for each field
  let match_arms = fields.iter().enumerate().map(|( index, field )| 
    {
      let field_name = &field.ident;
      qt! 
      {
        #index => &self.#field_name
      }
    }
  );

  Ok
  (
    qt! 
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::Index< usize > for #item_name< #generics_ty >
      where
        #generics_where
      {
        type Output = #field_type;
        #[ inline( always ) ]
        fn index( &self, index: usize ) -> &Self::Output
        {
           match index 
           {
             #(#match_arms,)*
             _ => panic!( "Index out of bounds" ),
           }
        }
      }
    }
  )
}

/// Generates `Index` implementation for structs with tuple fields
///
/// # Example
///
/// ## Input
/// ```rust
/// # use derive_tools_meta::Index;
/// #[ derive( Index ) ]
/// pub struct Struct< T > i
/// {
///   a: T,    
/// }
/// ```
///
/// ## Output
/// ```rust
/// pub struct Struct< T >
/// {
///   a: T,
/// };
///
/// #[ automatically_derived ]
/// impl< T > ::core::ops::Index< usize > for Struct< T >
/// {
///   type Output = T;
///   #[ inline( always ) ]
///   fn index( &self, index: usize ) -> &Self::Output
///   {
///   match index 
///     {
///        0 => &self.a,
///        _ => panic!( "Index out of bounds" ),
///     }
///   }
/// }
/// ```
///

fn generate_struct_named_fields
(
  item_name: &syn::Ident,
  generics_impl: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields: &syn::FieldsNamed,
) 
-> Result< proc_macro2::TokenStream > 
{
  let fields = fields.named.clone();

  // Get the type of the first field as the output type
  let field_type = match fields.first() {
    Some(field) => &field.ty,
    None => return Err(
      syn::Error::new(fields.span(), "Empty type cannot be indexed")
    ),
  };

  // Generate match arms for each field
  let match_arms = fields.iter().enumerate().map(|( index, field )| 
    {
      let field_name = &field.ident;
      qt! 
      {
        #index => &self.#field_name
      }
    }
  );

  Ok
  (
    qt! 
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::Index< usize > for #item_name< #generics_ty >
      where
        #generics_where
      {
        type Output = #field_type;
        #[ inline( always ) ]
        fn index( &self, index: usize ) -> &Self::Output
        {
          match index 
          {
            #(#match_arms,)*
            _ => panic!( "Index out of bounds" ),
          }
        }
      }
    }
  )
}

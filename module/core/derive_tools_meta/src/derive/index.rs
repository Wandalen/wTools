use super::*;
use macro_tools::{ attr, diag, generic_params, struct_like::StructLike, Result };


pub fn index( input : proc_macro::TokenStream  ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( &parsed.generics() );

 let result = match parsed
  {
 StructLike::Struct( ref item ) =>
    {
      generate_struct
      (
        item_name,
        &generics_impl,
        &generics_ty,
        &generics_where,
        &item.fields,
      )
    },
    StructLike::Unit(_) | StructLike::Enum(_) => todo!()
   }?;




  if has_debug
  {
    let about = format!( "derive : Not\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}



/// An aggregator function to generate `AsRef` implementation for unit, tuple structs and the ones with named fields
fn generate_struct
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields : &syn::Fields,
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
      fields,
    ),
     &syn::Fields::Unnamed(_) | &syn::Fields::Unit => todo!()
  }
}




fn generate_struct_named_fields
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields : &syn::FieldsNamed,
)
-> Result< proc_macro2::TokenStream >
{
  let fields = fields.named.clone();
  let ( field_name, field_type ) = match fields.first()
  {
    Some( field ) => ( field.ident.as_ref().unwrap(), &field.ty ),
    None => todo!()
      };

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
        
        fn index( &self, index: usize ) -> &Self::Output
        {
           match index {
            0 => &self.#field_name,
            _ => panic!("Index out of bounds"),
        }

        }
      }
    }
  )
}






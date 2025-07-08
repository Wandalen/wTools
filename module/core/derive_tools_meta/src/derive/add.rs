use macro_tools::
{
  diag,
  generic_params,
  struct_like::StructLike,
  Result,
  qt,
  attr,
  syn,
  proc_macro2,
  return_syn_err,
  Spanned,
};

use super::FieldAccess;
use super::field_attributes::FieldAttributes;
use super::item_attributes::ItemAttributes;

pub fn add( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream > 
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let item_attrs = ItemAttributes::from_attrs( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();
  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( parsed.generics() );

  let result = match parsed 
  {
    StructLike::Unit( ref item ) => 
    {
      return_syn_err!( item.span(), "Add cannot be derived for unit structs. It is only applicable to structs with at least one field." );
    },
    
    StructLike::Struct( ref item ) =>
    {

    let fields_result: Result< Vec < ( FieldAccess, syn::Type )> > = item
    .fields
    .iter()
    .enumerate()
    .map( | ( index, field ) | {
        let _attrs = FieldAttributes::from_attrs( field.attrs.iter() )?;
        let access = match &field.ident 
        {
            Some( ident ) => FieldAccess::Named( ident.clone() ),
            None => FieldAccess::Unnamed( syn::Index::from( index ) ),
        };
        let field_type = field.ty.clone();
        Ok( ( access, field_type ) )
    })
    .collect();

      let fields = fields_result?;

      if fields.is_empty() 
      {
        return_syn_err!( item.span(), "Add requires at least one field in the struct" );
      }

      generate_struct
      (
        item_name,
        &generics_impl,
        &generics_ty,
        &generics_where,
        &fields,
      )
      
    },
    StructLike::Enum( ref item ) =>
    {
      todo!()
    }
    
  };
  
  Ok( result )
}

fn generate_struct
(
  item_name: &syn::Ident,
  generics_impl: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty: &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields: &[ ( FieldAccess, syn::Type ) ],
) 
-> proc_macro2::TokenStream 
{
  let additions = fields.iter().map( | ( access, _ty ) | 
  {
    match access 
    {
      FieldAccess::Named( ident ) => qt! { #ident: self.#ident + other.#ident },
      FieldAccess::Unnamed( index ) => qt! { self.#index + other.#index },
    }
  });

  let body = if matches!( fields.first(), Some( ( FieldAccess::Named( _ ), _ ) ) ) 
  {
    qt! { Self { #( #additions ),* } }
  } 
  else 
  {
    qt! { Self ( #( #additions ),* ) }
  };

  qt! {
    #[ automatically_derived ]
    impl< #generics_impl > std::ops::Add for #item_name< #generics_ty >
    where #generics_where
    {
      type Output = Self;

      # [ inline ( always ) ]
      fn add( self, other: Self ) -> Self::Output 
      {
        #body
      }
    }
  }
}
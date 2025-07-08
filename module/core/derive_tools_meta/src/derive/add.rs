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

use super::field_attributes::{ FieldAttributes };
use super::item_attributes::{ ItemAttributes };

pub fn add( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream > {
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let item_attrs = ItemAttributes::from_attrs( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();
  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( parsed.generics() );

  let result = match parsed {
    StructLike::Unit( ref _item ) => 
    {
      todo!()
    },
     StructLike::Struct( ref item ) =>
    {
    let fields_result: Result<Vec<(syn::Ident, syn::Type)>> = item
        .fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
          let _attrs = FieldAttributes::from_attrs(field.attrs.iter())?;
          let field_name = field
            .ident
            .clone()
            .unwrap_or_else(|| {
              syn::Ident::new(&format!("_{index}"), field.span())
            });
          let field_type = field.ty.clone();
          Ok((field_name, field_type))
        })
        .collect();

      let fields = fields_result?;

      if fields.is_empty() 
      {
        return_syn_err!( "Expected at least one field" ); // TODO: change error message
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
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields : &[ ( syn::Ident, syn::Type ) ],
) 
-> proc_macro2::TokenStream
{
  let additions = fields.iter().map( | ( ident, _ty ) | {
    qt! 
    {
      #ident: self.#ident + other.#ident
    }
  });

  qt!
  {
    #[ automatically_derived ]
    impl< #generics_impl > std::ops::Add for #item_name< #generics_ty >
    where 
      #generics_where
    {
      type Output = Self;

      #[ inline ( always )]
      fn add( self, other : Self ) -> Self::Output
      {
        Self 
        {
          #( #additions ),*
        }
      }
    }
  }
}


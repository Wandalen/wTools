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
use crate::derive::syn::Variant;
use super::FieldAccess;
use super::item_attributes::{ ItemAttributes };

pub fn div( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream > 
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();
  let item_attrs = ItemAttributes::from_attrs( parsed.attrs().iter() )?;
  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( parsed.generics() );

  let result = match parsed 
  {
    StructLike::Unit( ref item ) =>
    {
      return_syn_err!( item.span(), "Div cannot be derived for unit structs. It is only applicable to structs with at least one field." );
    },
    
    StructLike::Struct( ref item ) =>
    {

    let fields_result : Result< Vec < ( FieldAccess, syn::Type ) > > = item
    .fields
    .iter()
    .enumerate()
    .map( | ( index, field ) | {
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
        return_syn_err!( item.span(), "Div requires at least one field in the struct" );
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
        let non_empty_variants: Vec< &Variant > = item.variants.iter().filter( | variant| !variant.fields.is_empty() ).collect();
        if non_empty_variants.is_empty() 
        {
          return_syn_err!
          (
            item.span(),
            "Div cannot be derived for enums without any variants containing fields. At least one variant must have fields."
          );
        }
        
        generate_enum(
          item_name,
          &item_attrs,
          &generics_impl,
          &generics_ty,
          &generics_where,
          &non_empty_variants,
        )
    }
  };

  if has_debug
  {
    let about = format!( "derive : Div\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }
  
  Ok( result )
}

fn generate_enum
(
  item_name : &syn::Ident,
  item_attrs : &ItemAttributes,
  generics_impl : & syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : & syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where : & syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  variants : &[ &syn::Variant ],
)
-> proc_macro2::TokenStream 
{
  let op_expr = | a_ident : &syn::Ident, b_ident : &syn::Ident | -> proc_macro2::TokenStream {
    qt! { #a_ident / #b_ident}
  };

  let error_type: proc_macro2::TokenStream = if let Some( ty ) = &item_attrs.error_type 
  {
    qt! { #ty }
  } 
  else 
  {
    qt! { String }
  };

  let enum_match = super::generate_enum_match_body( item_name, variants, item_attrs, op_expr );
  let body :  proc_macro2::TokenStream = 
    qt! { #enum_match };

   qt! 
   {
      #[ automatically_derived ]
      impl< #generics_impl > std::ops::Div for #item_name< #generics_ty >
      where #generics_where
      {

      type Output = Result< Self, #error_type >;

      # [ inline ( always ) ]
      fn div( self, other: Self ) -> Self::Output 
      {
        #body
      }
    }
  }
}

fn generate_struct
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields: &[ ( FieldAccess, syn::Type ) ],
) 
-> proc_macro2::TokenStream 
{
  let divs = fields.iter().map( | ( access, _ty ) | 
  {
    match access 
    {
      FieldAccess::Named( ident ) => qt! { #ident: self.#ident / other.#ident },
      FieldAccess::Unnamed( index ) => qt! { self.#index / other.#index },
    }
  });

  let body = if matches!( fields.first(), Some( ( FieldAccess::Named( _ ), _ ) ) ) 
  {
    qt! { Self { #( #divs ), * } }
  } 
  else 
  {
    qt! { Self ( #( #divs ), * ) }
  };

  qt! 
  {
    #[ automatically_derived ]
    impl< #generics_impl > std::ops::Div for #item_name< #generics_ty >
    where #generics_where
    {
      type Output = Self;

      # [ inline ( always ) ]
      fn div( self, other : Self ) -> Self::Output 
      {
        #body
      }
    }
  }
}
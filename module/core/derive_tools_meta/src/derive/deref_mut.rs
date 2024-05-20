
use super::*;
use macro_tools::{ attr, diag, generic_params, item_struct, Result, struct_like::StructLike };

//

pub fn deref_mut( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( &parsed.generics() );

  let result = match parsed
  {
    StructLike::Unit( ref item ) | StructLike::Struct( ref item ) =>
    {
      let field_types = item_struct::field_types( &item );
      let field_names = item_struct::field_names( &item );

      match ( field_types.len(), field_names )
      {
        ( 0, _ ) =>
        generate_unit
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
        ),
        ( _, Some( mut field_names ) ) =>
        generate_named_field
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
          field_names.next().unwrap(),
        ),
        ( _, None ) =>
        generate_tuple_field
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
        ),
      }
    }
    StructLike::Enum( ref item ) =>
    {
      todo!()
    }
  };

  if has_debug
  {
    let about = format!( "derive : DerefMut\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

// qqq : docs
fn generate_unit
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
)
-> proc_macro2::TokenStream
{
  qt!
  {
    #[ automatically_derived ]
    impl< #generics_impl > ::core::ops::DerefMut for #item_name< #generics_ty >
    where
      #generics_where
    {
      fn deref_mut( &mut self ) -> &mut Self::Target
      {
        &mut ()
      }
    }
  }
}

// qqq : docs
fn generate_named_field
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_name : &syn::Ident,
)
-> proc_macro2::TokenStream
{
  qt!
  {
    #[ automatically_derived ]
    impl< #generics_impl > ::core::ops::DerefMut for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      fn deref_mut( &mut self ) -> &mut Self::Target
      {
        &mut self.#field_name
      }
    }
  }
}

// qqq : docs
fn generate_tuple_field
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
)
-> proc_macro2::TokenStream
{
  qt!
  {
    #[ automatically_derived ]
    impl< #generics_impl > ::core::ops::DerefMut for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      fn deref_mut( &mut self ) -> &mut Self::Target
      {
        &mut self.0
      }
    }
  }
}

// qqq : docs
fn generate_variant
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  variant : &syn::Variant,
)
-> proc_macro2::TokenStream
{
  // TODO
  todo!()
  // let variant_name = &variant.ident;
  // let fields = &variant.fields;

  // if fields.len() <= 0
  // {
  //   return qt!{};
  // }

  // let ( args, use_src ) = if fields.len() == 1
  // {
  //   let field = fields.iter().next().unwrap();
  //   (
  //     qt!{ #field },
  //     qt!{ src },
  //   )
  // }
  // else
  // {
  //   let src_i = ( 0..fields.len() ).map( | e |
  //   {
  //     let i = syn::Index::from( e );
  //     qt!{ src.#i, }
  //   });
  //   (
  //     qt!{ #fields },
  //     qt!{ #( #src_i )* },
  //     // qt!{ src.0, src.1 },
  //   )
  // };

  // qt!
  // {
  //   #[ automatically_derived ]
  //   impl< #generics_impl > DerefMut< #args > for #item_name< #generics_ty >
  //   where
  //     #generics_where
  //   {
  //     #[ inline ]
  //     fn deref_mut( src : #args ) -> Self
  //     {
  //       Self::#variant_name( #use_src )
  //     }
  //   }
  // }

}

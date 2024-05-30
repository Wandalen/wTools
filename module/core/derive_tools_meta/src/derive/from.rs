use super::*;
use macro_tools::
{
  attr,
  diag,
  generic_params,
  item_struct,
  struct_like::StructLike,
  Result,
};

mod field_attributes;
use field_attributes::*;
mod item_attributes;
use item_attributes::*;

//

pub fn from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  use macro_tools::quote::ToTokens;

  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_attrs = ItemAttributes::from_attrs( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( &parsed.generics() );

  let result = match parsed
  {
    StructLike::Unit( ref item ) | StructLike::Struct( ref item ) =>
    {

      let mut field_types = item_struct::field_types( &item );
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
        ( 1, Some( mut field_names ) ) =>
        generate_from_single_field_named
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
          field_names.next().unwrap(),
          &field_types.next().unwrap(),
        ),
        ( 1, None ) =>
        generate_from_single_field
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
          &field_types.next().unwrap(),
        ),
        ( _, Some( field_names ) ) =>
        generate_from_multiple_fields_named
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
          field_names,
          field_types,
        ),
        ( _, None ) =>
        generate_from_multiple_fields
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
          field_types,
        ),
      }

    },
    StructLike::Enum( ref item ) =>
    {

      let mut map = std::collections::HashMap::new();
      item.variants.iter().for_each( | variant |
      {
        map
        .entry( variant.fields.to_token_stream().to_string() )
        .and_modify( | e | *e += 1 )
        .or_insert( 1 );
      });

      let variants_result : Result< Vec< proc_macro2::TokenStream > > = item.variants.iter().map( | variant |
      {
        // don't do automatic off
        // if map[ & variant.fields.to_token_stream().to_string() ] <= 1
        if true
        {
          variant_generate
          (
            item_name,
            &item_attrs,
            &generics_impl,
            &generics_ty,
            &generics_where,
            variant,
            &original_input,
          )
        }
        else
        {
          Ok( qt!{} )
        }
      }).collect();

      let variants = variants_result?;

      qt!
      {
        #( #variants )*
      }
    },
  };

  if has_debug
  {
    let about = format!( "derive : From\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

// qqq  : document, add example of generated code
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
    // impl From< () > for UnitStruct
    impl< #generics_impl > From< () > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      fn from( src : () ) -> Self
      {
        Self
      }
    }
  }
}

// qqq  : document, add example of generated code
fn generate_from_single_field_named
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_name : &syn::Ident,
  field_type : &syn::Type,
)
-> proc_macro2::TokenStream
{
  qt!
  {
    #[ automatically_derived ]
    // impl From < i32 > for MyStruct
    impl< #generics_impl > From< #field_type > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      // fn from( src: i32 ) -> Self
      fn from( src: #field_type ) -> Self
      {
        // Self { a: src }
        Self { #field_name: src }
      }
    }
  }
}

// qqq  : document, add example of generated code
fn generate_from_single_field
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_type : &syn::Type,
)
-> proc_macro2::TokenStream
{

  qt!
  {
    #[automatically_derived]
    // impl From< bool > for IsTransparent
    impl< #generics_impl > From< #field_type > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      // fn from( src: bool ) -> Self
      fn from( src: #field_type ) -> Self
      {
        // Self(src)
        Self(src)
      }
    }
  }
}

// qqq : for Petro : document, add example of generated code
fn generate_from_multiple_fields_named< 'a >
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_names : Box< dyn macro_tools::IterTrait< 'a, &'a syn::Ident > + '_ >,
  field_types : impl macro_tools::IterTrait< 'a, &'a syn::Type >,
)
-> proc_macro2::TokenStream
{

  let params : Vec< proc_macro2::TokenStream > = field_names
  .enumerate()
  .map(| ( index, field_name ) |
  {
    let index = index.to_string().parse::< proc_macro2::TokenStream >().unwrap();
    qt! { #field_name : src.#index }
  })
  .collect();

  let field_types : Vec< _ > = field_types.collect();
  qt!
  {
    // impl From< (i32, bool) > for StructNamedFields
    impl< #generics_impl > From< (# ( #field_types ),* ) > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      // fn from( src: (i32, bool) ) -> Self
      fn from( src : ( #( #field_types ),* ) ) -> Self
      {
        // StructNamedFields{ a: src.0, b: src.1 }
        #item_name { #(#params),* }
      }
    }
  }

}

// qqq  : document, add example of generated code
fn generate_from_multiple_fields< 'a >
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_types : impl macro_tools::IterTrait< 'a, &'a macro_tools::syn::Type >,
)
-> proc_macro2::TokenStream
{

  let params : Vec< proc_macro2::TokenStream > = ( 0..field_types.len() )
  .map( | index |
  {
    let index = index.to_string().parse::< proc_macro2::TokenStream >().unwrap();
    qt!( src.#index )
  })
  .collect();

  let field_types : Vec< _ > = field_types.collect();

  qt!
  {
    // impl From< (i32, bool) > for StructWithManyFields
    impl< #generics_impl > From< (# ( #field_types ),* ) > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      // fn from( src: (i32, bool) ) -> Self
      fn from( src : ( #( #field_types ),* ) ) -> Self
      {
        // StructWithManyFields( src.0, src.1 )
        #item_name( #( #params ),* )
      }
    }
  }
}

// qqq  : document, add example of generated code
fn variant_generate
(
  item_name : &syn::Ident,
  item_attrs : &ItemAttributes,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  variant : &syn::Variant,
  original_input : &proc_macro::TokenStream,
)
-> Result< proc_macro2::TokenStream >
{
  let variant_name = &variant.ident;
  let fields = &variant.fields;
  let attrs = FieldAttributes::from_attrs( variant.attrs.iter() )?;

  if !attrs.config.enabled.value( item_attrs.config.enabled.value( true ) )
  {
    return Ok( qt!{} )
  }

  if fields.len() <= 0
  {
    return Ok( qt!{} )
  }

  let ( args, use_src ) = if fields.len() == 1
  {
    let field = fields.iter().next().unwrap();
    (
      qt!{ #field },
      qt!{ src },
    )
  }
  else
  {
    let src_i = ( 0..fields.len() ).map( | e |
    {
      let i = syn::Index::from( e );
      qt!{ src.#i, }
    });
    (
      qt!{ #fields },
      qt!{ #( #src_i )* },
      // qt!{ src.0, src.1 },
    )
  };

  if attrs.config.debug.value( false )
  {
    let debug = format!
    (
      r#"
#[ automatically_derived ]
impl< {0} > From< {args} > for {item_name}< {1} >
where
  {2}
{{
  #[ inline ]
  fn from( src : {args} ) -> Self
  {{
    Self::{variant_name}( {use_src} )
  }}
}}
      "#,
      format!( "{}", qt!{ #generics_impl } ),
      format!( "{}", qt!{ #generics_ty } ),
      format!( "{}", qt!{ #generics_where } ),
    );
    let about = format!
    (
r#"derive : From
item : {item_name}
field : {variant_name}"#,
    );
    diag::report_print( about, original_input, debug );
  }

  Ok
  (
    qt!
    {
      #[ automatically_derived ]
      impl< #generics_impl > From< #args > for #item_name< #generics_ty >
      where
        #generics_where
      {
        #[ inline ]
        fn from( src : #args ) -> Self
        {
          Self::#variant_name( #use_src )
        }
      }
    }
  )

}

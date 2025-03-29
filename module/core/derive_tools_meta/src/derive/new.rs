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

#[ path = "from/field_attributes.rs" ]
mod field_attributes;
use field_attributes::*;
#[ path = "from/item_attributes.rs" ]
mod item_attributes;
use item_attributes::*;

//

// zzz : qqq : implement
pub fn new( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  // use macro_tools::quote::ToTokens;

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
        generate_single_field_named
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
          field_names.next().unwrap(),
          &field_types.next().unwrap(),
        ),
        ( 1, None ) =>
        generate_single_field
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
          &field_types.next().unwrap(),
        ),
        ( _, Some( field_names ) ) =>
        generate_multiple_fields_named
        (
          item_name,
          &generics_impl,
          &generics_ty,
          &generics_where,
          field_names,
          field_types,
        ),
        ( _, None ) =>
        generate_multiple_fields
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

      let variants_result : Result< Vec< proc_macro2::TokenStream > > = item.variants.iter().map( | variant |
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
    let about = format!( "derive : New\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

// zzz : qqq : implement
// qqq : document, add example of generated code
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
    // impl UnitStruct
    impl< #generics_impl > #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      pub fn new() -> Self
      {
        Self
      }
    }
  }
}

// zzz : qqq : implement
// qqq : document, add example of generated code
fn generate_single_field_named
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
    // impl MyStruct
    impl< #generics_impl > #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      // pub fn new( src : i32 ) -> Self
      pub fn new( src : #field_type ) -> Self
      {
        // Self { a : src }
        Self { #field_name: src }
      }
    }
  }
}

// zzz : qqq : implement
// qqq : document, add example of generated code
fn generate_single_field
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
    // impl IsTransparent
    impl< #generics_impl > #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      // pub fn new( src : bool ) -> Self
      pub fn new( src : #field_type ) -> Self
      {
        // Self( src )
        Self( src )
      }
    }
  }
}

// zzz : qqq : implement
// qqq : document, add example of generated code
fn generate_multiple_fields_named< 'a >
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_names : impl macro_tools::IterTrait< 'a, &'a syn::Ident >,
  field_types : impl macro_tools::IterTrait< 'a, &'a syn::Type >,
)
-> proc_macro2::TokenStream
{

  let val_type = field_names
  .clone()
  .zip( field_types )
  .enumerate()
  .map(| ( _index, ( field_name, field_type ) ) |
  {
    qt! { #field_name : #field_type }
  });

  qt!
  {
    // impl StructNamedFields
    impl< #generics_impl > #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      // pub fn new( src : ( i32, bool ) ) -> Self
      pub fn new( #( #val_type ),* ) -> Self
      {
        // StructNamedFields{ a : src.0, b : src.1 }
        #item_name { #( #field_names ),* }
      }
    }
  }

}

// zzz : qqq : implement
// qqq : document, add example of generated code
fn generate_multiple_fields< 'a >
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_types : impl macro_tools::IterTrait< 'a, &'a macro_tools::syn::Type >,
)
-> proc_macro2::TokenStream
{

  let params = ( 0..field_types.len() )
  .map( | index |
  {
    let index = index.to_string().parse::< proc_macro2::TokenStream >().unwrap();
    qt!( src.#index )
  });

  qt!
  {
    // impl StructWithManyFields
    impl< #generics_impl > #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      // pub fn new( src : (i32, bool) ) -> Self
      pub fn new( src : ( #( #field_types ),* ) ) -> Self
      {
        // StructWithManyFields( src.0, src.1 )
        #item_name( #( #params ),* )
      }
    }
  }
}

// zzz : qqq : implement
// qqq : document, add example of generated code
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

  // qqq : make `debug` working for all branches
  if attrs.config.debug.value( false )
  {
    let debug = format!
    (
      r#"
#[ automatically_derived ]
impl< {0} > {item_name}< {1} >
where
  {2}
{{
  #[ inline ]
  pub fn new( src : {args} ) -> Self
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
r#"derive : New
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
      impl< #generics_impl > #item_name< #generics_ty >
      where
        #generics_where
      {
        #[ inline ]
        pub fn new( src : #args ) -> Self
        {
          Self::#variant_name( #use_src )
        }
      }
    }
  )

}

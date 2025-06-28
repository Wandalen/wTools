use super::*;
use macro_tools::
{
  attr,
  diag,
  generic_params,
  struct_like::StructLike,
  Result,
  qt,
};

#[ path = "from/field_attributes.rs" ]
mod field_attributes;
use field_attributes::*;
#[ path = "from/item_attributes.rs" ]
mod item_attributes;
use item_attributes::*;

use macro_tools::phantom;

///
/// Provides an automatic `PhantomData` field for a struct based on its generic types.
///
pub fn phantom( attr_input : proc_macro::TokenStream, input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_attrs = ItemAttributes::from_attrs( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( &parsed.generics() );

  let result = match parsed
  {
    StructLike::Unit( ref _item ) =>
    {
      return_syn_err!( parsed.span(), "Expects a structure with fields" );
    },
    StructLike::Struct( ref item ) =>
    {
      let mut result = item.clone();
      phantom::add_to_item( &mut result );
      qt!{ #result }
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

      let variants = match variants_result
      {
        Ok( v ) => v,
        Err( e ) => return Err( e ),
      };

      qt!
      {
        #( #variants )*
      }
    },
  };

  if has_debug
  {
    let about = format!( "derive : Phantom\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// Generates `PhantomData` field for enum variants.
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

  if fields.is_empty()
  {
    return Ok( qt!{} )
  }

  if fields.len() != 1
  {
    return_syn_err!( fields.span(), "Expects a single field to derive Phantom" );
  }

  let field = fields.iter().next().unwrap();
  let field_type = &field.ty;
  let field_name = &field.ident;

  if attrs.config.debug.value( false )
  {
    let debug = format_args!
    (
      r#"
#[ automatically_derived ]
impl< {} > {}< {} >
where
  {}
{{
  // PhantomData field added
}}
      "#,
      qt!{ #generics_impl },
      item_name,
      qt!{ #generics_ty },
      qt!{ #generics_where },
    );
    let about = format!
    (
r#"derive : Phantom
item : {item_name}
field : {variant_name}"#,
    );
    diag::report_print( about, original_input, debug.to_string() );
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
        // PhantomData field added
      }
    }
  )

}

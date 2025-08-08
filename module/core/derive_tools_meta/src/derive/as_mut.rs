use macro_tools::{
  diag,
  generic_params,
  // item_struct, // Removed unused import
  struct_like::StructLike,
  Result,
  qt,
  attr,
  syn,
  proc_macro2,
  return_syn_err,
  Spanned,
};

use super::field_attributes::{FieldAttributes};
use super::item_attributes::{ItemAttributes};

///
/// Derive macro to implement `AsMut` when-ever it's possible to do automatically.
///
pub fn as_mut(input: proc_macro::TokenStream) -> Result< proc_macro2::TokenStream > {
  let original_input = input.clone();
  let parsed = syn::parse::<StructLike>(input)?;
  let has_debug = attr::has_debug(parsed.attrs().iter())?;
  let item_attrs = ItemAttributes::from_attrs(parsed.attrs().iter())?;
  let item_name = &parsed.ident();

  let (_generics_with_defaults, generics_impl, generics_ty, generics_where) = generic_params::decompose(parsed.generics());

  let result = match parsed {
    StructLike::Unit(ref _item) => {
      return_syn_err!(parsed.span(), "Expects a structure with one field");
    }
    StructLike::Struct(ref item) => {
      let mut field_type = None;
      let mut field_name = None;
      let mut found_field = false;

      let fields = match &item.fields {
        syn::Fields::Named(fields) => &fields.named,
        syn::Fields::Unnamed(fields) => &fields.unnamed,
        syn::Fields::Unit => return_syn_err!(item.span(), "Expects a structure with one field"),
      };

      for f in fields {
        if attr::has_as_mut(f.attrs.iter())? {
          if found_field {
            return_syn_err!(f.span(), "Multiple `#[ as_mut ]` attributes are not allowed");
          }
          field_type = Some(&f.ty);
          field_name = f.ident.as_ref();
          found_field = true;
        }
      }

      let (field_type, field_name) = if let Some(ft) = field_type {
        (ft, field_name)
      } else if fields.len() == 1 {
        let f = fields.iter().next().expect("Expects a single field to derive AsMut");
        (&f.ty, f.ident.as_ref())
      } else {
        return_syn_err!(
          item.span(),
          "Expected `#[ as_mut ]` attribute on one field or a single-field struct"
        );
      };

      generate(
        item_name,
        &generics_impl,
        &generics_ty,
        &generics_where,
        field_type,
        field_name,
      )
    }
    StructLike::Enum(ref item) => {
      let variants_result: Result<Vec< proc_macro2::TokenStream >> = item
        .variants
        .iter()
        .map(|variant| {
          variant_generate(
            item_name,
            &item_attrs,
            &generics_impl,
            &generics_ty,
            &generics_where,
            variant,
            &original_input,
          )
        })
        .collect();

      let variants = variants_result?;

      qt! {
        #( #variants )*
      }
    }
  };

  if has_debug {
    let about = format!("derive : AsMut\nstructure : {item_name}");
    diag::report_print(about, &original_input, &result);
  }

  Ok(result)
}

/// Generates `AsMut` implementation for structs.
///
/// Example of generated code:
/// ```text
/// impl AsMut< bool > for IsTransparent
/// {
///   fn as_mut( &mut self ) -> &mut bool
/// ///   {
/// ///     &mut self.0
/// ///   }
/// /// }
/// ```
fn generate(
  item_name: &syn::Ident,
  generics_impl: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  generics_ty: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  generics_where: &syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>,
  field_type: &syn::Type,
  field_name: Option< &syn::Ident >,
) -> proc_macro2::TokenStream {
  let body = if let Some(field_name) = field_name {
    qt! { &mut self.#field_name }
  } else {
    qt! { &mut self.0 }
  };

  qt! {
    #[ automatically_derived ]
    impl< #generics_impl > core::convert::AsMut< #field_type > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      fn as_mut( &mut self ) -> &mut #field_type
      {
        #body
      }
    }
  }
}

/// Generates `AsMut` implementation for enum variants.
///
/// Example of generated code:
/// ```text
/// impl AsMut< i32 > for MyEnum
/// {
///   fn as_mut( &mut self ) -> &mut i32
/// ///   {
/// ///     &mut self.0
/// ///   }
/// /// }
/// ```
fn variant_generate(
  item_name: &syn::Ident,
  item_attrs: &ItemAttributes,
  generics_impl: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  generics_ty: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  generics_where: &syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>,
  variant: &syn::Variant,
  original_input: &proc_macro::TokenStream,
) -> Result< proc_macro2::TokenStream > {
  let variant_name = &variant.ident;
  let fields = &variant.fields;
  let attrs = FieldAttributes::from_attrs(variant.attrs.iter())?;

  if !attrs.enabled.value(item_attrs.enabled.value(true)) {
    return Ok(qt! {});
  }

  if fields.is_empty() {
    return Ok(qt! {});
  }

  if fields.len() != 1 {
    return_syn_err!(fields.span(), "Expects a single field to derive AsMut");
  }

  let field = fields.iter().next().expect("Expects a single field to derive AsMut");
  let field_type = &field.ty;
  let field_name = &field.ident;

  let body = if let Some(field_name) = field_name {
    qt! { &mut self.#field_name }
  } else {
    qt! { &mut self.0 }
  };

  if attrs.debug.value(false) {
    let debug = format!(
      r"
#[ automatically_derived ]
impl< {} > core::convert::AsMut< {} > for {}< {} >
where
  {}
{{
  #[ inline ]
  fn as_mut( &mut self ) -> &mut {}
  {{
    {}
  }}
}}
      ",
      qt! { #generics_impl },
      qt! { #field_type },
      item_name,
      qt! { #generics_ty },
      qt! { #generics_where },
      qt! { #field_type },
      body,
    );
    let about = format!(
      r"derive : AsMut
item : {item_name}
field : {variant_name}",
    );
    diag::report_print(about, original_input, debug.to_string());
  }

  Ok(qt! {
    #[ automatically_derived ]
    impl< #generics_impl > core::convert::AsMut< #field_type > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline ]
      fn as_mut( &mut self ) -> &mut #field_type
      {
        #body
      }
    }
  })
}

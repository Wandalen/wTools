#![allow(clippy::assigning_clones)]
use macro_tools::{
  diag, // Uncommented
  generic_params,
  struct_like::StructLike,
  Result,
  qt,
  attr,
  syn,
  proc_macro2,
  return_syn_err,
  syn_err,
  Spanned,
};

use super::field_attributes::{FieldAttributes};
use super::item_attributes::{ItemAttributes};

///
/// Derive macro to implement From when-ever it's possible to do automatically.
///
pub fn from(input: proc_macro::TokenStream) -> Result< proc_macro2::TokenStream > {
  let original_input = input.clone();
  let parsed = syn::parse::<StructLike>(input)?;
  let has_debug = attr::has_debug(parsed.attrs().iter())?;
  let item_attrs = ItemAttributes::from_attrs(parsed.attrs().iter())?;
  let item_name = &parsed.ident();

  let (_generics_with_defaults, generics_impl, generics_ty, generics_where_punctuated) =
    generic_params::decompose(parsed.generics());
  let where_clause_owned = if generics_where_punctuated.is_empty() {
    None
  } else {
    Some(syn::WhereClause {
      where_token: <syn::token::Where as Default>::default(),
      predicates: generics_where_punctuated.clone(),
    })
  };
  let generics_where = where_clause_owned.as_ref();

  if has_debug {
    diag::report_print("generics_impl_raw", &original_input, qt! { #generics_impl }.to_string());
    diag::report_print("generics_ty_raw", &original_input, qt! { #generics_ty }.to_string());
    diag::report_print(
      "generics_where_punctuated_raw",
      &original_input,
      qt! { #generics_where_punctuated }.to_string(),
    );
  }

  let result = match parsed {
    StructLike::Unit(ref _item) => {
      return_syn_err!(parsed.span(), "Expects a structure with one field");
    }
    StructLike::Struct(ref item) => {
      let context = StructFieldHandlingContext {
        item,
        item_name,
        has_debug,
        generics_impl: &generics_impl,
        generics_ty: &generics_ty,
        generics_where,
        original_input: &original_input,
      };
      handle_struct_fields(&context)? // Propagate error
    }
    StructLike::Enum(ref item) => {
      let variants_result: Result<Vec< proc_macro2::TokenStream >> = item
        .variants
        .iter()
        .map(|variant| {
          let context = VariantGenerateContext {
            item_name,
            item_attrs: &item_attrs,
            has_debug,
            generics_impl: &generics_impl,
            generics_ty: &generics_ty,
            generics_where,
            variant,
            original_input: &original_input,
          };
          variant_generate(&context)
        })
        .collect();

      let variants = variants_result?;

      qt! {
        #( #variants )*
      }
    }
  };

  if has_debug {
    let about = format!("derive : From\nstructure : {item_name}");
    diag::report_print(about, &original_input, &result);
  }

  Ok(result)
}

/// Context for handling struct fields in `From` derive.
struct StructFieldHandlingContext<'a> {
  item: &'a syn::ItemStruct,
  item_name: &'a syn::Ident,
  has_debug: bool,
  generics_impl: &'a syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  generics_ty: &'a syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  generics_where: Option< &'a syn::WhereClause >,
  original_input: &'a proc_macro::TokenStream,
}

/// Handles the generation of `From` implementation for structs.
fn handle_struct_fields(context: &StructFieldHandlingContext<'_>) -> Result< proc_macro2::TokenStream > // Change return type here
{
  let fields_count = context.item.fields.len();
  let mut target_field_type = None;
  let mut target_field_name = None;
  let mut target_field_index = None;

  let mut from_attr_count = 0;

  if fields_count == 0 {
    return_syn_err!(context.item.span(), "From cannot be derived for structs with no fields.");
  } else if fields_count == 1 {
    // Single field struct: automatically from to that field
    let field = context
      .item
      .fields
      .iter()
      .next()
      .expect("Expects a single field to derive From");
    target_field_type = Some(field.ty.clone());
    target_field_name = field.ident.clone();
    target_field_index = Some(0);
  } else {
    // Multi-field struct: require #[ from ] attribute on one field
    for (i, field) in context.item.fields.iter().enumerate() {
      if attr::has_from(field.attrs.iter())? {
        from_attr_count += 1;
        target_field_type = Some(field.ty.clone());
        target_field_name = field.ident.clone();
        target_field_index = Some(i);
      }
    }

    if from_attr_count == 0 {
      return_syn_err!(
        context.item.span(),
        "From cannot be derived for multi-field structs without a `#[ from ]` attribute on one field."
      );
    } else if from_attr_count > 1 {
      return_syn_err!(context.item.span(), "Only one field can have the `#[ from ]` attribute.");
    }
  }

  let field_type =
    target_field_type.ok_or_else(|| syn_err!(context.item.span(), "Could not determine target field type for From."))?;
  let field_name = target_field_name;

  Ok(generate(&GenerateContext {
    item_name: context.item_name,
    has_debug: context.has_debug,
    generics_impl: context.generics_impl,
    generics_ty: context.generics_ty,
    generics_where: context.generics_where,
    field_type: &field_type,
    field_name: field_name.as_ref(),
    all_fields: &context.item.fields,
    field_index: target_field_index,
    original_input: context.original_input,
  }))
}

/// Context for generating `From` implementation.
struct GenerateContext<'a> {
  item_name: &'a syn::Ident,
  has_debug: bool,
  generics_impl: &'a syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  generics_ty: &'a syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  generics_where: Option< &'a syn::WhereClause >,
  field_type: &'a syn::Type,
  field_name: Option< &'a syn::Ident >,
  all_fields: &'a syn::Fields,
  field_index: Option< usize >,
  original_input: &'a proc_macro::TokenStream,
}

/// Generates `From` implementation for structs.
///
/// Example of generated code:
/// ```text
/// /// impl From< bool > for IsTransparent
/// /// {
/// ///   fn from( src : bool ) -> Self
/// ///   {
/// ///     Self( src )
/// ///   }
/// /// }
/// ```
fn generate(context: &GenerateContext<'_>) -> proc_macro2::TokenStream {
  let item_name = context.item_name;
  let has_debug = context.has_debug;
  let generics_impl = context.generics_impl;
  let generics_ty = context.generics_ty;
  let generics_where = context.generics_where;
  let field_type = context.field_type;
  let field_name = context.field_name;
  let all_fields = context.all_fields;
  let field_index = context.field_index;
  let original_input = context.original_input;

  let where_clause_tokens = {
    let mut predicates_vec = Vec::new();

    if let Some(generics_where) = generics_where {
      for p in &generics_where.predicates {
        predicates_vec.push(macro_tools::quote::quote_spanned! { p.span() => #p });
      }
    }

    for param in generics_impl {
      if let syn::GenericParam::Const(const_param) = param {
        let const_ident = &const_param.ident;
        predicates_vec.push(macro_tools::quote::quote_spanned! { const_param.span() => [(); #const_ident]: Sized });
      }
    }

    if predicates_vec.is_empty() {
      proc_macro2::TokenStream::new()
    } else {
      let mut joined_predicates = proc_macro2::TokenStream::new();
      for (i, p) in predicates_vec.into_iter().enumerate() {
        if i > 0 {
          joined_predicates.extend(qt! { , });
        }
        joined_predicates.extend(p);
      }
      qt! { where #joined_predicates }
    }
  };

  let body = generate_struct_body_tokens(field_name, all_fields, field_index, has_debug, original_input);

  if has_debug {
    // Use has_debug directly
    diag::report_print(
      "generated_where_clause_tokens_struct",
      original_input,
      where_clause_tokens.to_string(),
    ); // Uncommented
  }

  let generics_ty_filtered = {
    let mut params = Vec::new();
    for param in generics_ty {
      params.push(qt! { #param }); // Include all parameters
    }
    let mut joined_params = proc_macro2::TokenStream::new();
    for (i, p) in params.into_iter().enumerate() {
      if i > 0 {
        joined_params.extend(qt! { , });
      }
      joined_params.extend(p);
    }
    joined_params
  };

  let generics_impl_filtered = {
    let mut params = Vec::new();
    for param in generics_impl {
      params.push(qt! { #param });
    }
    let mut joined_params = proc_macro2::TokenStream::new();
    for (i, p) in params.into_iter().enumerate() {
      if i > 0 {
        joined_params.extend(qt! { , });
      }
      joined_params.extend(p);
    }
    joined_params
  };

  qt! {
    #[ automatically_derived ]
    impl< #generics_impl_filtered > ::core::convert::From< #field_type > for #item_name< #generics_ty_filtered > #where_clause_tokens
    {
      #[ inline( always ) ]
      fn from( src : #field_type ) -> Self
      {
        #body
      }
    }
  }
}

/// Generates the body tokens for a struct's `From` implementation.
fn generate_struct_body_tokens(
  field_name: Option< &syn::Ident >,
  all_fields: &syn::Fields,
  field_index: Option< usize >,
  has_debug: bool,
  original_input: &proc_macro::TokenStream,
) -> proc_macro2::TokenStream {
  let body_tokens = if let Some(field_name) = field_name {
    // Named struct
    qt! { Self { #field_name : src } }
  } else {
    // Tuple struct
    let fields_tokens = generate_tuple_struct_fields_tokens(all_fields, field_index);
    qt! { Self( #fields_tokens ) } // Wrap the generated fields with Self(...)
  };

  if has_debug {
    // Use has_debug directly
    diag::report_print("generated_body_tokens_struct", original_input, body_tokens.to_string());
    // Uncommented
  }
  body_tokens
}

/// Generates the field tokens for a tuple struct's `From` implementation.
fn generate_tuple_struct_fields_tokens(all_fields: &syn::Fields, field_index: Option< usize >) -> proc_macro2::TokenStream {
  let mut fields_tokens = proc_macro2::TokenStream::new();
  let mut first = true;
  for (i, field) in all_fields.into_iter().enumerate() {
    if !first {
      fields_tokens.extend(qt! { , });
    }
    if Some(i) == field_index {
      fields_tokens.extend(qt! { src });
    } else {
      let field_type_path = if let syn::Type::Path(type_path) = &field.ty {
        Some(type_path)
      } else {
        None
      };

      if let Some(type_path) = field_type_path {
        let last_segment = type_path.path.segments.last();
        if let Some(segment) = last_segment {
          if segment.ident == "PhantomData" {
            // Extract the type argument from PhantomData
            if let syn::PathArguments::AngleBracketed(ref args) = segment.arguments {
              if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
                fields_tokens.extend(qt! { ::core::marker::PhantomData::< #ty > });
              } else {
                fields_tokens.extend(qt! { ::core::marker::PhantomData }); // Fallback
              }
            } else {
              fields_tokens.extend(qt! { ::core::marker::PhantomData }); // Fallback
            }
          } else {
            fields_tokens.extend(qt! { Default::default() });
          }
        } else {
          fields_tokens.extend(qt! { _ });
        }
      } else {
        fields_tokens.extend(qt! { _ });
      }
    }
    first = false;
  }
  fields_tokens
}

/// Context for generating `From` implementation for enum variants.
struct VariantGenerateContext<'a> {
  item_name: &'a syn::Ident,
  item_attrs: &'a ItemAttributes,
  has_debug: bool,
  generics_impl: &'a syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  generics_ty: &'a syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  generics_where: Option< &'a syn::WhereClause >,
  variant: &'a syn::Variant,
  original_input: &'a proc_macro::TokenStream,
}

/// Generates `From` implementation for enum variants.
///
/// Example of generated code:
/// ```text
/// /// impl From< i32 > for MyEnum
/// /// {
/// ///   fn from( src : i32 ) -> Self
/// ///   {
/// ///     Self::Variant( src )
/// ///   }
/// /// }
/// ```
fn variant_generate(context: &VariantGenerateContext<'_>) -> Result< proc_macro2::TokenStream > {
  let item_name = context.item_name;
  let item_attrs = context.item_attrs;
  let has_debug = context.has_debug;
  let generics_impl = context.generics_impl;
  let generics_ty = context.generics_ty;
  let generics_where = context.generics_where;
  let variant = context.variant;
  let original_input = context.original_input;

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
    return_syn_err!(fields.span(), "Expects a single field to derive From");
  }

  let field = fields.iter().next().expect("Expects a single field to derive From");
  let field_type = &field.ty;
  let field_name = &field.ident;

  let body = if let Some(field_name) = field_name {
    qt! { Self::#variant_name { #field_name : src } }
  } else {
    qt! { Self::#variant_name( src ) }
  };

  let where_clause_tokens = generate_variant_where_clause_tokens(generics_where, generics_impl);
  let generics_ty_filtered = generate_variant_generics_ty_filtered(generics_ty);
  let generics_impl_filtered = generate_variant_generics_impl_filtered(generics_impl);

  if has_debug
  // Use has_debug directly
  {
    diag::report_print(
      "generated_where_clause_tokens_enum",
      original_input,
      where_clause_tokens.to_string(),
    ); // Uncommented
    diag::report_print("generated_body_tokens_enum", original_input, body.to_string()); // Uncommented
    let debug = format!(
      r"
#[ automatically_derived ]
impl< {} > ::core::convert::From< {} > for {}< {} >
{}
{{
  #[ inline ]
  fn from( src : {} ) -> Self
  {{
    {}
  }}
}}
      ",
      qt! { #generics_impl_filtered }, // Use filtered generics_impl
      qt! { #field_type },
      item_name,
      qt! { #generics_ty_filtered }, // Use filtered generics_ty
      where_clause_tokens,
      qt! { #field_type }, // This was the problem, it should be `src`
      body,
    );
    let about = format!(
      r"derive : From
item : {item_name}
field : {variant_name}",
    );
    diag::report_print(about, original_input, debug.to_string()); // Uncommented
  }

  Ok(qt! {
    #[ automatically_derived ]
    impl< #generics_impl_filtered > ::core::convert::From< #field_type > for #item_name< #generics_ty_filtered > #where_clause_tokens
    {
      #[ inline ]
      fn from( src : #field_type ) -> Self
      {
        #body
      }
    }
  })
}

/// Generates the where clause tokens for an enum variant's `From` implementation.
fn generate_variant_where_clause_tokens(
  generics_where: Option< &syn::WhereClause >,
  generics_impl: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
) -> proc_macro2::TokenStream {
  let mut predicates_vec = Vec::new();

  if let Some(generics_where) = generics_where {
    for p in &generics_where.predicates {
      predicates_vec.push(macro_tools::quote::quote_spanned! { p.span() => #p });
    }
  }

  for param in generics_impl {
    if let syn::GenericParam::Const(const_param) = param {
      let const_ident = &const_param.ident;
      predicates_vec.push(macro_tools::quote::quote_spanned! { const_param.span() => [(); #const_ident]: Sized });
    }
  }

  if predicates_vec.is_empty() {
    proc_macro2::TokenStream::new()
  } else {
    let mut joined_predicates = proc_macro2::TokenStream::new();
    for (i, p) in predicates_vec.into_iter().enumerate() {
      if i > 0 {
        joined_predicates.extend(qt! { , });
      }
      joined_predicates.extend(p);
    }
    qt! { where #joined_predicates }
  }
}

/// Generates the filtered generics type tokens for an enum variant's `From` implementation.
fn generate_variant_generics_ty_filtered(
  generics_ty: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
) -> proc_macro2::TokenStream {
  let mut params = Vec::new();
  for param in generics_ty {
    params.push(qt! { #param });
  }
  let mut joined_params = proc_macro2::TokenStream::new();
  for (i, p) in params.into_iter().enumerate() {
    if i > 0 {
      joined_params.extend(qt! { , });
    }
    joined_params.extend(p);
  }
  joined_params
}

/// Generates the filtered generics implementation tokens for an enum variant's `From` implementation.
fn generate_variant_generics_impl_filtered(
  generics_impl: &syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
) -> proc_macro2::TokenStream {
  let mut params = Vec::new();
  for param in generics_impl {
    params.push(qt! { #param });
  }
  let mut joined_params = proc_macro2::TokenStream::new();
  for (i, p) in params.into_iter().enumerate() {
    if i > 0 {
      joined_params.extend(qt! { , });
    }
    joined_params.extend(p);
  }
  joined_params
}

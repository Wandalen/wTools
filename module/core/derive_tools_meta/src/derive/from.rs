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
  syn_err,
  Spanned,
};

use super::field_attributes::{ FieldAttributes };
use super::item_attributes::{ ItemAttributes };

///
/// Derive macro to implement From when-ever it's possible to do automatically.
///
pub fn from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_attrs = ItemAttributes::from_attrs( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where_punctuated )
  = generic_params::decompose( parsed.generics() );
  let generics_where = if generics_where_punctuated.is_empty() {
    None
  } else {
    Some( &syn::WhereClause {
      where_token: <syn::token::Where as Default>::default(),
      predicates: generics_where_punctuated.clone(),
    })
  };

  if has_debug
  {
    diag::report_print( "generics_impl_raw", &original_input, &qt!{ #generics_impl }.to_string() );
    diag::report_print( "generics_ty_raw", &original_input, &qt!{ #generics_ty }.to_string() );
    diag::report_print( "generics_where_punctuated_raw", &original_input, &qt!{ #generics_where_punctuated }.to_string() );
  }

  let result = match parsed
  {
    StructLike::Unit( ref _item ) =>
    {
      return_syn_err!( parsed.span(), "Expects a structure with one field" );
    },
    StructLike::Struct( ref item ) =>
    {
      let fields_count = item.fields.len();
      let mut target_field_type = None;
      let mut target_field_name = None;
      let mut target_field_index = None;

      let mut from_attr_count = 0;

      if fields_count == 0 {
        return_syn_err!( item.span(), "From cannot be derived for structs with no fields." );
      } else if fields_count == 1 {
        // Single field struct: automatically from to that field
        let field = item.fields.iter().next().unwrap();
        target_field_type = Some( field.ty.clone() );
        target_field_name = field.ident.clone();
        target_field_index = Some( 0 );
      } else {
        // Multi-field struct: require #[from] attribute on one field
        for ( i, field ) in item.fields.iter().enumerate() {
          if attr::has_from( field.attrs.iter() )? {
            from_attr_count += 1;
            target_field_type = Some( field.ty.clone() );
            target_field_name = field.ident.clone();
            target_field_index = Some( i );
          }
        }

        if from_attr_count == 0 {
          return_syn_err!( item.span(), "From cannot be derived for multi-field structs without a `#[from]` attribute on one field." );
        } else if from_attr_count > 1 {
          return_syn_err!( item.span(), "Only one field can have the `#[from]` attribute." );
        }
      }

      let field_type = target_field_type.ok_or_else(|| syn_err!( item.span(), "Could not determine target field type for From." ))?;
      let field_name = target_field_name;

      generate
      (
        item_name,
        &item_attrs, // Pass item_attrs
        has_debug,   // Pass has_debug
        &generics_impl,
        &generics_ty,
        generics_where,
        &field_type,
        field_name.as_ref(),
        &item.fields,
        target_field_index,
        &original_input,
      )
    },
    StructLike::Enum( ref item ) =>
    {
      let variants_result : Result< Vec< proc_macro2::TokenStream > > = item.variants.iter().map( | variant |
      {
        variant_generate
        (
          item_name,
          &item_attrs, // Pass item_attrs
          has_debug,   // Pass has_debug
          &generics_impl,
          &generics_ty,
          generics_where,
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
    let about = format!( "derive : From\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// Generates `From` implementation for structs.
///
/// Example of generated code:
/// ```text
/// impl From< bool > for IsTransparent
/// {
///   fn from( src : bool ) -> Self
///   {
///     Self( src )
///   }
/// }
/// ```
fn generate
(
  item_name : &syn::Ident,
  _item_attrs : &ItemAttributes, // Prefix with _ as it's not used for logic here
  has_debug : bool,             // Add has_debug
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: Option< &syn::WhereClause >,
  field_type : &syn::Type,
  field_name : Option< &syn::Ident >,
  all_fields : &syn::Fields,
  field_index : Option< usize >,
  original_input : &proc_macro::TokenStream,
)
-> proc_macro2::TokenStream
{
  let where_clause_tokens = {
    let mut predicates_vec = Vec::new();

    if let Some( generics_where ) = generics_where {
        for p in generics_where.predicates.iter() {
            predicates_vec.push(macro_tools::quote::quote_spanned!{ p.span() => #p });
        }
    }

    for param in generics_impl.iter() {
        if let syn::GenericParam::Const( const_param ) = param {
            let const_ident = &const_param.ident;
            predicates_vec.push(macro_tools::quote::quote_spanned!{ const_param.span() => [(); #const_ident]: Sized });
        }
    }

    if !predicates_vec.is_empty() {
        let mut joined_predicates = proc_macro2::TokenStream::new();
        for (i, p) in predicates_vec.into_iter().enumerate() {
            if i > 0 {
                joined_predicates.extend(qt!{ , });
            }
            joined_predicates.extend(p);
        }
        qt!{ where #joined_predicates }
    } else {
        proc_macro2::TokenStream::new()
    }
  };

  let body = if let Some( field_name ) = field_name
  {
    // Named struct
    qt!{ Self { #field_name : src } }
  }
  else
  {
    // Tuple struct
    let mut fields_tokens = proc_macro2::TokenStream::new();
    let mut first = true;
    for ( i, field ) in all_fields.iter().enumerate() {
        if !first {
            fields_tokens.extend( qt!{ , } );
        }
        if Some( i ) == field_index {
            fields_tokens.extend( qt!{ src } );
        } else {
            let field_type_path = if let syn::Type::Path( type_path ) = &field.ty {
                Some( type_path )
            } else {
                None
            };

            if let Some( type_path ) = field_type_path {
                let last_segment = type_path.path.segments.last();
                if let Some( segment ) = last_segment {
                    if segment.ident == "PhantomData" {
                        // Extract the type argument from PhantomData
                        if let syn::PathArguments::AngleBracketed( ref args ) = segment.arguments {
                            if let Some( syn::GenericArgument::Type( ty ) ) = args.args.first() {
                                fields_tokens.extend( qt!{ ::core::marker::PhantomData::< #ty > } );
                            } else {
                                fields_tokens.extend( qt!{ ::core::marker::PhantomData } ); // Fallback
                            }
                        } else {
                            fields_tokens.extend( qt!{ ::core::marker::PhantomData } ); // Fallback
                        }
                    } else {
                        fields_tokens.extend( qt!{ Default::default() } );
                    }
                } else {
                    fields_tokens.extend( qt!{ _ } );
                }
            } else {
                fields_tokens.extend( qt!{ _ } );
            }
        }
        first = false;
    }
    let body_tokens = qt!{ Self( #fields_tokens ) };
    if has_debug { // Use has_debug directly
        diag::report_print( "generated_body_tokens_struct", original_input, &body_tokens.to_string() );
    }
    body_tokens
  };

  if has_debug { // Use has_debug directly
      diag::report_print( "generated_where_clause_tokens_struct", original_input, &where_clause_tokens.to_string() );
  }

  let generics_ty_filtered = {
      let mut params = Vec::new();
      for param in generics_ty.iter() {
          params.push(qt!{ #param }); // Include all parameters
      }
      let mut joined_params = proc_macro2::TokenStream::new();
      for (i, p) in params.into_iter().enumerate() {
          if i > 0 {
              joined_params.extend(qt!{ , });
          }
          joined_params.extend(p);
      }
      joined_params
  };

  let generics_impl_filtered = {
      let mut params = Vec::new();
      for param in generics_impl.iter() {
          params.push(qt!{ #param });
      }
      let mut joined_params = proc_macro2::TokenStream::new();
      for (i, p) in params.into_iter().enumerate() {
          if i > 0 {
              joined_params.extend(qt!{ , });
          }
          joined_params.extend(p);
      }
      joined_params
  };

  qt!
  {
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
fn variant_generate
(
  item_name : &syn::Ident,
  item_attrs : &ItemAttributes, // Keep item_attrs
  has_debug : bool,             // Add has_debug
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: Option< &syn::WhereClause >,
  variant : &syn::Variant,
  original_input : &proc_macro::TokenStream,
)
-> Result< proc_macro2::TokenStream >
{
  let variant_name = &variant.ident;
  let fields = &variant.fields;
  let attrs = FieldAttributes::from_attrs( variant.attrs.iter() )?;

  if !attrs.enabled.value( item_attrs.enabled.value( true ) )
  {
    return Ok( qt!{} )
  }

  if fields.is_empty()
  {
    return Ok( qt!{} )
  }

  if fields.len() != 1
  {
    return_syn_err!( fields.span(), "Expects a single field to derive From" );
  }

  let field = fields.iter().next().expect( "Expects a single field to derive From" );
  let field_type = &field.ty;
  let field_name = &field.ident;

  let body = if let Some( field_name ) = field_name
  {
    qt!{ Self::#variant_name { #field_name : src } }
  }
  else
  {
    qt!{ Self::#variant_name( src ) }
  };

  let where_clause_tokens = {
    let mut predicates_vec = Vec::new();

    if let Some( generics_where ) = generics_where {
        for p in generics_where.predicates.iter() {
            predicates_vec.push(macro_tools::quote::quote_spanned!{ p.span() => #p });
        }
    }

    for param in generics_impl.iter() {
        if let syn::GenericParam::Const( const_param ) = param {
            let const_ident = &const_param.ident;
            predicates_vec.push(macro_tools::quote::quote_spanned!{ const_param.span() => [(); #const_ident]: Sized });
        }
    }

    if !predicates_vec.is_empty() {
        let mut joined_predicates = proc_macro2::TokenStream::new();
        for (i, p) in predicates_vec.into_iter().enumerate() {
            if i > 0 {
                joined_predicates.extend(qt!{ , });
            }
            joined_predicates.extend(p);
        }
        qt!{ where #joined_predicates }
    } else {
        proc_macro2::TokenStream::new()
    }
  };

  let generics_ty_filtered = {
      let mut params = Vec::new();
      for param in generics_ty.iter() {
          params.push(qt!{ #param });
      }
      let mut joined_params = proc_macro2::TokenStream::new();
      for (i, p) in params.into_iter().enumerate() {
          if i > 0 {
              joined_params.extend(qt!{ , });
          }
          joined_params.extend(p);
      }
      joined_params
  };

  let generics_impl_filtered = {
      let mut params = Vec::new();
      for param in generics_impl.iter() {
          params.push(qt!{ #param });
      }
      let mut joined_params = proc_macro2::TokenStream::new();
      for (i, p) in params.into_iter().enumerate() {
          if i > 0 {
              joined_params.extend(qt!{ , });
          }
          joined_params.extend(p);
      }
      joined_params
  };

  if has_debug // Use has_debug directly
  {
    diag::report_print( "generated_where_clause_tokens_enum", original_input, &where_clause_tokens.to_string() );
    diag::report_print( "generated_body_tokens_enum", original_input, &body.to_string() );
    let debug = format!
    (
      r#"
#[ automatically_derived ]
impl< {0} > ::core::convert::From< {1} > for {2}< {3} >
{4}
{{
  #[ inline ]
  fn from( src : {1} ) -> Self
  {{
    {5}
  }}
}}
      "#,
      qt!{ #generics_impl_filtered }.to_string(), // Use filtered generics_impl
      qt!{ #field_type }.to_string(),
      item_name.to_string(),
      generics_ty_filtered.to_string(), // Use filtered generics_ty
      where_clause_tokens.to_string(),
      body.to_string(),
    );
    let about = format!
    (
r"derive : From
item : {item_name}
field : {variant_name}",
    );
    diag::report_print( about, original_input, debug.to_string() );
  }

  Ok
  (
    qt!
    {
      #[ automatically_derived ]
      impl< #generics_impl_filtered > ::core::convert::From< #field_type > for #item_name< #generics_ty_filtered > #where_clause_tokens
      {
        #[ inline ]
        fn from( src : #field_type ) -> Self
        {
          #body
        }
      }
    }
  )
}

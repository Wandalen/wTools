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

///
/// Provides an automatic [Not](core::ops::Not) trait  implementation for struct.
///
pub fn not( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
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
      return_syn_err!( parsed.span(), "Expects a structure with one field" );
    },
    StructLike::Struct( ref item ) =>
    {
      let field_type = item.fields.iter().next().map( | e | &e.ty );
      let field_name = item.fields.iter().next().map( | e | &e.ident );
      generate
      (
        item_name,
        &generics_impl,
        &generics_ty,
        &generics_where,
        field_type.unwrap(),
        field_name.flatten(),
      )
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
    let about = format!( "derive : Not\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// Generates `Not` implementation for structs.
///
/// Example of generated code:
/// ```rust
/// impl core::ops::Not for IsActive
/// {
///   type Output = IsActive;
///
///   fn not(self) -> Self::Output
///   {
///     IsActive(!self.0)
///   }
/// }
/// ```
fn generate
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_type : &syn::Type,
  field_name : Option< &syn::Ident >,
)
-> proc_macro2::TokenStream
{
  let body = if let Some( field_name ) = field_name
  {
    qt!{ !self.#field_name }
  }
  else
  {
    qt!{ !self.0 }
  };

  qt!
  {
    #[ automatically_derived ]
    impl< #generics_impl > core::ops::Not for #item_name< #generics_ty >
    where
      #generics_where
    {
      type Output = Self;
      #[ inline( always ) ]
      fn not( self ) -> Self::Output
      {
        #item_name( #body )
      }
    }
  }
}

/// Generates `Not` implementation for enum variants.
///
/// Example of generated code:
/// ```rust
/// impl core::ops::Not for MyEnum
/// {
///   type Output = MyEnum;
///
///   fn not(self) -> Self::Output
///   {
///     MyEnum::Variant(!self.0)
///   }
/// }
/// ```
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
    return_syn_err!( fields.span(), "Expects a single field to derive Not" );
  }

  let field = fields.iter().next().unwrap();
  let field_type = &field.ty;
  let field_name = &field.ident;

  let body = if let Some( field_name ) = field_name
  {
    qt!{ !self.#field_name }
  }
  else
  {
    qt!{ !self.0 }
  };

  if attrs.config.debug.value( false )
  {
    let debug = format_args!
    (
      r#"
#[ automatically_derived ]
impl< {} > core::ops::Not for {}< {} >
where
  {}
{{
  type Output = Self;
  #[ inline ]
  fn not( self ) -> Self::Output
  {{
    {}( {} )
  }}
}}
      "#,
      qt!{ #generics_impl },
      item_name,
      qt!{ #generics_ty },
      qt!{ #generics_where },
      item_name,
      body,
    );
    let about = format!
    (
r#"derive : Not
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
      impl< #generics_impl > core::ops::Not for #item_name< #generics_ty >
      where
        #generics_where
      {
        type Output = Self;
        #[ inline ]
        fn not( self ) -> Self::Output
        {
          Self::#variant_name( #body )
        }
      }
    }
  )

}

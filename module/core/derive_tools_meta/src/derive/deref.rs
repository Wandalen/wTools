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
  Spanned,
};
use macro_tools::diag::prelude::*;


///
/// Derive macro to implement Deref when-ever it's possible to do automatically.
///
pub fn deref( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( parsed.generics() );

  let result = match parsed
  {
    StructLike::Unit( ref item ) =>
    {
      return_syn_err!( item.span(), "Deref cannot be derived for unit structs. It is only applicable to structs with at least one field." );
    },
    StructLike::Struct( ref item ) =>
    {
      let fields_count = item.fields.len();
      let mut target_field_type = None;
      let mut target_field_name = None;
      let mut deref_attr_count = 0;

      if fields_count == 0 {
        return_syn_err!( item.span(), "Deref cannot be derived for structs with no fields." );
      } else if fields_count == 1 {
        // Single field struct: automatically deref to that field
        let field = item.fields.iter().next().unwrap();
        target_field_type = Some( field.ty.clone() );
        target_field_name = field.ident.clone();
      } else {
        // Multi-field struct: require #[deref] attribute on one field
        for field in item.fields.iter() {
          if attr::has_deref( field.attrs.iter() )? {
            deref_attr_count += 1;
            target_field_type = Some( field.ty.clone() );
            target_field_name = field.ident.clone();
          }
        }

        if deref_attr_count == 0 {
          return_syn_err!( item.span(), "Deref cannot be derived for multi-field structs without a `#[deref]` attribute on one field." );
        } else if deref_attr_count > 1 {
          return_syn_err!( item.span(), "Only one field can have the `#[deref]` attribute." );
        }
      }

      let field_type = target_field_type.ok_or_else(|| syn_err!( item.span(), "Could not determine target field type for Deref." ))?;
      let field_name = target_field_name;

      generate
      (
        item_name,
        &generics_impl,
        &generics_ty,
        &generics_where,
        &field_type,
        field_name.as_ref(),
        &original_input,
      )
    },
    StructLike::Enum( ref item ) =>
    {
      return_syn_err!( item.span(), "Deref cannot be derived for enums. It is only applicable to structs with a single field or a field with `#[deref]` attribute." );
    },
  };

  if has_debug
  {
    let about = format!( "derive : Deref\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// Generates `Deref` implementation for structs.
///
/// Example of generated code:
/// ```text
/// impl Deref for IsTransparent
/// {
///   type Target = bool;
///   fn deref( &self ) -> &bool
/// ///   {
/// ///     &self.0
/// ///   }
/// /// }
/// ```
fn generate
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_type : &syn::Type,
  field_name : Option< &syn::Ident >,
  original_input : &proc_macro::TokenStream,
)
-> proc_macro2::TokenStream
{
  let body = if let Some( field_name ) = field_name
  {
    qt!{ &self.#field_name }
  }
  else
  {
    qt!{ &self.0 }
  };

  let debug = format!
  (
    r"
#[ automatically_derived ]
impl< {} > core::ops::Deref for {}< {} >
where
  {}
{{
  type Target = {};
  #[ inline ]
  fn deref( &self ) -> &{}
  {{
    {}
  }}
}}
    ",
    qt!{ #generics_impl },
    item_name,
    qt!{ #generics_ty },
    qt!{ #generics_where },
    qt!{ #field_type },
    qt!{ #field_type },
    body,
  );
  let about = format!
  (
r"derive : Deref
item : {item_name}
field_type : {field_type:?}
field_name : {field_name:?}",
  );
  diag::report_print( about, original_input, debug.to_string() );

  qt!
  {
    #[ automatically_derived ]
    impl< #generics_impl > core::ops::Deref for #item_name< #generics_ty >
    where
      #generics_where
    {
      type Target = #field_type;
      #[ inline( always ) ]
      fn deref( &self ) -> & #field_type
      {
        #body
      }
    }
  }
}

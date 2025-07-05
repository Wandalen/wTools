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




///
/// Derive macro to implement `DerefMut` when-ever it's possible to do automatically.
///
pub fn deref_mut( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
    let item_name = &parsed.ident();

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( parsed.generics() );

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
      let mut deref_mut_attr_count = 0;

      if fields_count == 0 {
        return_syn_err!( item.span(), "DerefMut cannot be derived for structs with no fields." );
      } else if fields_count == 1 {
        // Single field struct: automatically deref_mut to that field
        let field = item.fields.iter().next().unwrap();
        target_field_type = Some( field.ty.clone() );
        target_field_name = field.ident.clone();
      } else {
        // Multi-field struct: require #[deref_mut] attribute on one field
        for field in item.fields.iter() {
          if attr::has_deref_mut( field.attrs.iter() )? {
            deref_mut_attr_count += 1;
            target_field_type = Some( field.ty.clone() );
            target_field_name = field.ident.clone();
          }
        }

        if deref_mut_attr_count == 0 {
          return_syn_err!( item.span(), "DerefMut cannot be derived for multi-field structs without a `#[deref_mut]` attribute on one field." );
        } else if deref_mut_attr_count > 1 {
          return_syn_err!( item.span(), "Only one field can have the `#[deref_mut]` attribute." );
        }
      }

      let field_type = target_field_type.ok_or_else(|| syn_err!( item.span(), "Could not determine target field type for DerefMut." ))?;
      let field_name = target_field_name;

      generate
      (
        item_name,
        &generics_impl,
        &generics_ty,
        &generics_where,
        &field_type,
        field_name.as_ref(),
      )
    },
    StructLike::Enum( ref item ) =>
    {
      return_syn_err!( item.span(), "DerefMut cannot be derived for enums. It is only applicable to structs with a single field." );
    },
  };

  if has_debug
  {
    let about = format!( "derive : DerefMut\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// Generates `DerefMut` implementation for structs.
///
/// Example of generated code:
/// ```text
/// impl DerefMut for IsTransparent
/// {
///   fn deref_mut( &mut self ) -> &mut bool
/// ///   {
/// ///     &mut self.0
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
)
-> proc_macro2::TokenStream
{
  let body = if let Some( field_name ) = field_name
  {
    qt!{ &mut self.#field_name }
  }
  else
  {
    qt!{ &mut self.0 }
  };

  qt!
  {
    #[ automatically_derived ]
    impl #generics_impl ::core::ops::DerefMut for #item_name #generics_ty
    where
      #generics_where
    {
      fn deref_mut( &mut self ) -> &mut #field_type
      {
        #body
      }
    }
  }
}

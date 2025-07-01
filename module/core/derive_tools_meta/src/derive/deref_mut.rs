use macro_tools::
{
  diag,
  generic_params,
  item_struct,
  struct_like::StructLike,
  Result,
  qt,
  attr,
  syn,
  proc_macro2,
  return_syn_err,
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
      let field_type = item_struct::first_field_type( item )?;
      let field_name = item_struct::first_field_name( item ).ok().flatten();
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
    use core::ops;
    #[ automatically_derived ]
    impl< #generics_impl > ops::DerefMut for #item_name< #generics_ty >
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

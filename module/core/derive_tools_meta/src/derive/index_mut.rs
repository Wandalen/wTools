use macro_tools::
{
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

use super::item_attributes::{ ItemAttributes };

///
/// Derive macro to implement `IndexMut` when-ever it's possible to do automatically.
///
pub fn index_mut( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let _item_attrs = ItemAttributes::from_attrs( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( parsed.generics() );

  let result = match parsed
  {
    StructLike::Unit( ref _item ) =>
    {
      return_syn_err!( parsed.span(), "IndexMut can be applied only to a structure with one field" );
    },
    StructLike::Struct( ref item ) =>
    {
      let mut field_type = None;
      let mut field_name = None;
      let mut found_field = false;

      let fields = match &item.fields {
          syn::Fields::Named(fields) => &fields.named,
          syn::Fields::Unnamed(fields) => &fields.unnamed,
          syn::Fields::Unit => return_syn_err!( item.span(), "IndexMut can be applied only to a structure with one field" ),
      };

      for f in fields.iter()
      {
        if attr::has_index_mut( f.attrs.iter() )?
        {
          if found_field
          {
            return_syn_err!( f.span(), "Multiple `#[index_mut]` attributes are not allowed" );
          }
          field_type = Some( &f.ty );
          field_name = f.ident.as_ref();
          found_field = true;
        }
      }

      let ( field_type, field_name ) = if let Some( ft ) = field_type
      {
        ( ft, field_name )
      }
      else if fields.len() == 1
      {
        let f = fields.iter().next().unwrap();
        ( &f.ty, f.ident.as_ref() )
      }
      else
      {
        return_syn_err!( item.span(), "Expected `#[index_mut]` attribute on one field or a single-field struct" );
      };

      generate
      (
        item_name,
        &generics_impl,
        &generics_ty,
        &generics_where,
        field_type,
        field_name,
      )
    },
    StructLike::Enum( ref item ) =>
    {
      return_syn_err!( item.span(), "IndexMut can be applied only to a structure" );
    },
  };

  if has_debug
  {
    let about = format!( "derive : IndexMut\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// Generates `IndexMut` implementation for structs.
///
/// Example of generated code:
/// ```text
/// impl IndexMut< usize > for IsTransparent
/// {
///   fn index_mut( &mut self, index : usize ) -> &mut bool
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
  let body_ref = if let Some( field_name ) = field_name
  {
    qt!{ & self.#field_name }
  }
  else
  {
    qt!{ & self.0 }
  };

  let body_mut = if let Some( field_name ) = field_name
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
    impl< #generics_impl > core::ops::Index< usize > for #item_name< #generics_ty >
    where
      #generics_where
    {
      type Output = #field_type;
      #[ inline( always ) ]
      fn index( &self, _index : usize ) -> & #field_type
      {
        #body_ref
      }
    }

    #[ automatically_derived ]
    impl< #generics_impl > core::ops::IndexMut< usize > for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      fn index_mut( &mut self, _index : usize ) -> &mut #field_type
      {
        #body_mut
      }
    }
  }
}

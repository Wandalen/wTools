
use super::*;
use iter_tools::{ Itertools, process_results };
use macro_tools::{ attr, diag, generic_params, generic_args, container_kind, typ, Result };
use proc_macro2::TokenStream;

///
/// Definition of a field.
///

#[ allow( dead_code ) ]
pub struct FormerField< 'a >
{
  pub attrs : FieldAttributes,
  pub vis : &'a syn::Visibility,
  pub ident : &'a syn::Ident,
  pub colon_token : &'a Option< syn::token::Colon >,
  pub ty : &'a syn::Type,
  pub non_optional_ty : &'a syn::Type,
  pub is_optional : bool,
  pub of_type : container_kind::ContainerKind,
}

impl< 'a > FormerField< 'a >
{

  /// Get name of scalar setter.
  pub fn scalar_setter_name( &self ) -> &syn::Ident
  {
    if let Some( ref attr ) = self.attrs.scalar
    {
      if let Some( ref name ) = attr.name
      {
        return name
      }
    }
    return &self.ident;
  }

  /// Get name of setter for container if such setter should be generated.
  pub fn container_setter_name( &self ) -> Option< &syn::Ident >
  {

    if let Some( ref attr ) = self.attrs.container
    {
      if attr.setter()
      {
        if let Some( ref name ) = attr.name
        {
          return Some( &name )
        }
        else
        {
          return Some( &self.ident )
        }
      }
    }

    return None;
  }

  /// Get name of setter for subform if such setter should be generated.
  pub fn subform_setter_name( &self ) -> Option< &syn::Ident >
  {

    if let Some( ref attr ) = self.attrs.subform
    {
      if attr.setter()
      {
        if let Some( ref name ) = attr.name
        {
          return Some( &name )
        }
        else
        {
          return Some( &self.ident )
        }
      }
    }

    return None;
  }

  /// Is scalar setter required. Does not if container of subformer setter requested.
  pub fn scalar_setter_required( &self ) -> bool
  {

    let mut explicit = false;
    if let Some( ref attr ) = self.attrs.scalar
    {
      if let Some( setter ) = attr.setter
      {
        if setter == false
        {
          return false
        }
        explicit = true;
      }
      if let Some( ref _name ) = attr.name
      {
        explicit = true;
      }
    }

    if self.attrs.container.is_some() && !explicit
    {
      return false;
    }

    if self.attrs.subform.is_some() && !explicit
    {
      return false;
    }

    return true;
  }

}

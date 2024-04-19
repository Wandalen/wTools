//! xxx : update documentation of file

/// Internal namespace.
pub( crate ) mod private
{
  use super::super::*;

  pub fn phantom_add( input : &syn::ItemStruct ) -> syn::ItemStruct
  {
    use proc_macro2::Span;

    // Clone the input struct to work on a modifiable copy
    let mut input = input.clone();

    // Prepare the tuple type for PhantomData based on the struct's generics
    let generics_tuple = if !input.generics.params.is_empty()
    {
      let generics_list = input.generics.params.iter().map( | param |
      {
        match param
        {
          syn::GenericParam::Type( type_param ) =>
          {
            syn::Type::Path( syn::TypePath
            {
              qself : None,
              path : type_param.ident.clone().into(),
            })
          },
          syn::GenericParam::Lifetime( lifetime_param ) =>
          {
            syn::Type::Path( syn::TypePath
            {
              qself : None,
              path : lifetime_param.lifetime.ident.clone().into(),
            })
          },
          syn::GenericParam::Const( const_param ) =>
          {
            syn::Type::Path( syn::TypePath
            {
              qself : None,
              path : const_param.ident.clone().into(),
            })
          },
        }
      }).collect::<syn::punctuated::Punctuated<_, syn::token::Comma>>();

      syn::Type::Tuple( syn::TypeTuple
      {
        paren_token : syn::token::Paren( Span::call_site() ),
        elems : generics_list,
      })
    }
    else
    {
      // Use unit type if there are no generics
      syn::Type::Tuple( syn::TypeTuple
      {
        paren_token : syn::token::Paren( Span::call_site() ),
        elems : syn::punctuated::Punctuated::new(),
      })
    };

    // Create the PhantomData field
    let phantom_field = syn::Field
    {
      attrs : Vec::new(),
      vis : syn::Visibility::Inherited,
      ident : Some( syn::Ident::new( "_phantom", Span::call_site() ) ),
      colon_token : Some( Default::default() ),
      mutability : syn::FieldMutability::None,
      ty : syn::Type::Path( syn::TypePath
      {
        qself : None,
        path : syn::Path
        {
          leading_colon : None,
          segments :
          {
            let mut segments = syn::punctuated::Punctuated::new();
            segments.push_value( syn::PathSegment
            {
              ident : syn::Ident::new( "std", Span::call_site() ),
              arguments : syn::PathArguments::None,
            });
            segments.push_punct( Default::default() );
            segments.push_value( syn::PathSegment
            {
              ident : syn::Ident::new( "marker", Span::call_site() ),
              arguments : syn::PathArguments::None,
            });
            segments.push_punct( Default::default() );
            segments.push_value( syn::PathSegment
            {
              ident : syn::Ident::new( "PhantomData", Span::call_site() ),
              arguments : syn::PathArguments::AngleBracketed( syn::AngleBracketedGenericArguments
              {
                colon2_token : None,
                lt_token : Default::default(),
                args : syn::punctuated::Punctuated::from_iter( vec![ syn::GenericArgument::Type( generics_tuple )] ),
                gt_token : Default::default(),
              }),
            });
            segments
          },
        },
      }),
    };

    // Add the new field to the existing fields of the struct
    if let syn::Fields::Named( ref mut fields ) = input.fields
    {
      fields.named.push( phantom_field );
    }

    input
  }

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    phantom_add,
  };
}

// xxx : external attr instead of internal?
/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::protected as item;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    prelude::*,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}

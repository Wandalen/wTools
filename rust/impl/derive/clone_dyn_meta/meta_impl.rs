
use proc_macro_tools::*;
pub type Result< T > = std::result::Result< T, syn::Error >;

//

pub fn clone_dyn( _attr : proc_macro::TokenStream, item : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let item_parsed = match syn::parse::< syn::ItemTrait >( item )
  {
    Ok( original ) => original,
    Err( err ) => return Err( err ),
  };

  let name_ident = &item_parsed.ident;

  let result = qt!
  {
    #item_parsed

    impl < 'c > Clone
    for Box< dyn #name_ident + 'c >
    {
      #[ inline ]
      fn clone( &self ) -> Self { clone_dyn::_clone_boxed( &**self ) }
    }

    impl < 'c > Clone
    for Box< dyn #name_ident + Send + 'c >
    {
      #[ inline ]
      fn clone( &self ) -> Self { clone_dyn::_clone_boxed( &**self ) }
    }

    impl < 'c > Clone
    for Box< dyn #name_ident + Sync + 'c >
    {
      #[ inline ]
      fn clone( &self ) -> Self { clone_dyn::_clone_boxed( &**self ) }
    }

    impl < 'c > Clone
    for Box< dyn #name_ident + Send + Sync + 'c >
    {
      #[ inline ]
      fn clone( &self ) -> Self { clone_dyn::_clone_boxed( &**self ) }
    }

  };

  Ok( result )
}

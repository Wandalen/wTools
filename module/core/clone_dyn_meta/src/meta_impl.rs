
use macro_tools::prelude::*;
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
  // let generics = &item_parsed.generics;
  let generics_analyzed = item_parsed.generics_analyze();
  let generics_params = &generics_analyzed.generics.params;
  let generics_where = &generics_analyzed.generics.where_clause;
  let generics_names = &generics_analyzed.names;

  let result = qt!
  {
    #item_parsed

    impl < 'c, #generics_params > Clone
    for Box< dyn #name_ident< #( #generics_names ),* > + 'c >
    // where
      #generics_where
    {
      #[ inline ]
      fn clone( &self ) -> Self { clone_dyn::_clone_boxed( &**self ) }
    }

    impl < 'c, #generics_params > Clone
    for Box< dyn #name_ident< #( #generics_names ),* > + Send + 'c >
    // where
      #generics_where
    {
      #[ inline ]
      fn clone( &self ) -> Self { clone_dyn::_clone_boxed( &**self ) }
    }

    impl < 'c, #generics_params > Clone
    for Box< dyn #name_ident< #( #generics_names ),* > + Sync + 'c >
    // where
      #generics_where
    {
      #[ inline ]
      fn clone( &self ) -> Self { clone_dyn::_clone_boxed( &**self ) }
    }

    impl < 'c, #generics_params > Clone
    for Box< dyn #name_ident< #( #generics_names ),* > + Send + Sync + 'c >
    // where
      #generics_where
    {
      #[ inline ]
      fn clone( &self ) -> Self { clone_dyn::_clone_boxed( &**self ) }
    }

  };

  Ok( result )
}

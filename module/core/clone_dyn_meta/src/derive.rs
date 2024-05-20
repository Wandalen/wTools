
use macro_tools::prelude::*;
use macro_tools::{ Result, generic_params };

//

pub fn clone_dyn( _attr : proc_macro::TokenStream, item : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let item_parsed = match syn::parse::< syn::ItemTrait >( item )
  {
    Ok( original ) => original,
    Err( err ) => return Err( err ),
  };

  let name_ident = &item_parsed.ident;

  let generic_params = &item_parsed.generics.params;
  let generics_where = &item_parsed.generics.where_clause;
  let generics_names : Vec< _ > = generic_params::names( &item_parsed.generics ).collect();

  let result = qt!
  {
    #item_parsed

    #[ allow( non_local_definitions ) ]
    impl < 'c, #generic_params > Clone
    for Box< dyn #name_ident< #( #generics_names ),* > + 'c >
    // where
      #generics_where
    {
      #[ inline ]
      fn clone( &self ) -> Self { clone_dyn::_clone_boxed( &**self ) }
    }

    #[ allow( non_local_definitions ) ]
    impl < 'c, #generic_params > Clone
    for Box< dyn #name_ident< #( #generics_names ),* > + Send + 'c >
    // where
      #generics_where
    {
      #[ inline ]
      fn clone( &self ) -> Self { clone_dyn::_clone_boxed( &**self ) }
    }

    #[ allow( non_local_definitions ) ]
    impl < 'c, #generic_params > Clone
    for Box< dyn #name_ident< #( #generics_names ),* > + Sync + 'c >
    // where
      #generics_where
    {
      #[ inline ]
      fn clone( &self ) -> Self { clone_dyn::_clone_boxed( &**self ) }
    }

    #[ allow( non_local_definitions ) ]
    impl < 'c, #generic_params > Clone
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

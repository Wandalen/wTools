/// Internal namespace.
mod internal
{
  // use crate::exposed::*;
  // use crate::*;
  use crate::*;
  // use proc_macro_tools::prelude::*;
  // use core::hash::{ Hash, Hasher };

  // #[ allow( unused_imports ) ]
  use proc_macro_tools::exposed::*;
  // #[ allow( unused_imports ) ]
  // use proc_macro_tools::{ Result };

  // use proc_macro_tools::syn::
  // {
  //   ext::IdentExt,
  //   parse::discouraged::Speculative,
  // };
  // use core::hash::{ Hash, Hasher };

  ///
  /// Protocol of modularity unifying interface of a module.
  ///

  pub fn mod_interface( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
  {
    use std::collections::HashMap; /* xxx : include into prelude of wtools */

    let records = syn::parse::< Records >( input )?;

    //let mut mods = vec![];
    let mut immediates : Vec< proc_macro2::TokenStream > = vec![];

    // use inspect_type::*;
    // inspect_type_of!( immediates );

    let mut fixes : HashMap< _ , Vec< proc_macro2::TokenStream > > = HashMap::new();
    fixes.insert( VisPrivate::Kind(), Vec::new() );
    fixes.insert( VisProtected::Kind(), Vec::new() );
    fixes.insert( VisOrphan::Kind(), Vec::new() );
    fixes.insert( VisExposed::Kind(), Vec::new() );
    fixes.insert( VisPrelude::Kind(), Vec::new() );

    let mut err = None;

    records.0.iter().for_each( | record |
    {
      record.elements.iter().for_each( | element |
      {
        //mods.push( record.ident.clone() );
  //      let ident = &record.ident;
        let ident = &element.1;

        if record.mod_token.is_some()
        {
          immediates.push( qt!{ pub mod #ident; } );
          let fixes = fixes.get_mut( &record.vis.kind() ).unwrap();
          fixes.push( qt!{ pub use super::#ident; } );

          // xxx : test
          if !record.vis.can_be_used_for_non_standard_mod()
          {
            err = Some( syn_err!
            (
              record, "To include a non-standard module use either [ protected, orphan, exposed, prelude ] visibility:\n  {}",
              qt!{ #record }
            ));
          }
        }
      });

    });

    if let Some( _err ) = err
    {
      return Err( _err );
    }

    let _private_fix = fixes.get( &VisPrivate::Kind() ).unwrap();
    let protected_fix = fixes.get( &VisProtected::Kind() ).unwrap();
    let orphan_fix = fixes.get( &VisOrphan::Kind() ).unwrap();
    let exposed_fix = fixes.get( &VisExposed::Kind() ).unwrap();
    let prelude_fix = fixes.get( &VisPrelude::Kind() ).unwrap();

    let result = qt!
    {

      #( #immediates )*

      /// Protected namespace of the module.
      pub mod protected
      {
        #[ doc( inline ) ]
        pub use super::orphan::*;

        #( #protected_fix )*

      }

      #[ doc( inline ) ]
      pub use protected::*;

      /// Orphan namespace of the module.
      pub mod orphan
      {
        #[ doc( inline ) ]
        pub use super::exposed::*;

        #( #orphan_fix )*

      }

      /// Exposed namespace of the module.
      pub mod exposed
      {
        #[ doc( inline ) ]
        pub use super::prelude::*;

        #( #exposed_fix )*

      }

      /// Prelude to use: `use wtools::prelude::*`.
      pub mod prelude
      {

        #( #prelude_fix )*

      }

    };

    Ok( result )
  }

}

/// Own namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;

  pub use super::internal::
  {
  };

}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::internal::
  {
    mod_interface,
  };
}

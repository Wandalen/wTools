/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use proc_macro_tools::exposed::*;


// = use

  // x
  // use private::Type1;
  // use private::{ Type1, Type2 };
  // protected use private::Type1;
  // prelude use private::Type1;

// = ?

  // x
  // protected protected1;
  // orphan orphan1;
  // exposed exposed1;
  // prelude prelude1;
  // prelude { prelude1, prelude2 };

// = macro module

  // x
  // macromod mod1;
  // macromod mod2;
  // macromod { mod1, mod2 };

  // - narrowing

  // x
  // orphan macromod mod_orphan1;
  // : protected -> protected
  // : orphan -> orphan
  // : exposed -> orphan
  // : prelude -> orphan

  // - extending

  // x
  // prelude exposed macromod mod_protected1;
  // : protected -> exposed
  // : orphan -> exposed
  // : exposed -> exposed
  // : prelude -> prelude

  // x
  // prelude protected macromod mod_exposed1;
  // : protected -> protected
  // : orphan -> orphan
  // : exposed -> exposed
  // : prelude -> prelude

  // - selective

  // x
  // exposed exposed macromod mod_exposed1;
  // : protected -> exposed
  // : orphan -> exposed
  // : exposed -> exposed
  // : prelude -> exposed

  // x
  // exposed orphan macromod mod_exposed1;
  // : protected -> orphan
  // : orphan -> orphan
  // : exposed -> exposed
  // : prelude -> exposed

// = micro module

  // x
  // mod mod1;
  // mod mod2;
  // mod { mod1, mod2 };

  // +
  // protected mod mod_protected1;
  // orphan mod mod_orphan1;
  // exposed mod mod_exposed1;
  // prelude mod mod_prelude1;

  // +
  // protected mod { mod_protected1, mod_protected2 };
  // orphan mod { mod_orphan1, mod_orphan2 };
  // exposed mod { mod_exposed1, mod_exposed2 };
  // prelude mod { mod_prelude1, mod_prelude2 };

  ///
  /// Protocol of modularity unifying interface of a module.
  ///

  pub fn mod_interface( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
  {
    use std::collections::HashMap; /* xxx : include into prelude of wtools */

    let records = syn::parse::< Records >( input )?;
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
        // let ident = element.first().unwrap();
        let ident = element;

        if record.element_type.is_micro_module()
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

      /// Prelude to use essentials: `use my_module::prelude::*`.
      pub mod prelude
      {

        #( #prelude_fix )*

      }

    };

    Ok( result )
  }

}

/// Protected namespace of the module.
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

  pub use super::private::
  {
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
    mod_interface,
  };
}

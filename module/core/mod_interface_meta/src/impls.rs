/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use macro_tools::exposed::*;
  use std::collections::HashMap;

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

  // zzz : clause should not expect the first argument

  /// Context for handlign a record. Cotnains clauses map and debug attribute.
  #[ allow( dead_code ) ]
  pub struct RecordContext< 'clauses_map >
  {
    pub has_debug : bool,
    pub clauses_map : &'clauses_map mut HashMap< ClauseKind , Vec< proc_macro2::TokenStream > >,
  }

  ///
  /// Handle record "use" with implicit visibility.
  ///
  #[ allow ( dead_code ) ]
  fn record_use_implicit
  (
    record : &Record,
    c : &'_ mut RecordContext< '_ >,
    // clauses_map : &mut HashMap< u32, Vec< proc_macro2::TokenStream > >,
  )
  ->
  syn::Result< () >
  {

    let attrs1 = &record.attrs;
    let path = record.use_elements.as_ref().unwrap();
    // let vis = record.vis.clone();

    // if vis == Visibility::Inherited

    // xxx

    // let _path;
    // let path2 = if path.prefix_is_needed()
    // {
    //   _path = parse_qt!{ super::private::#path };
    //   &_path
    // }
    // else
    // {
    //   path
    // };

    let adjsuted_path = path.adjsuted_implicit_path()?;

    // println!( "adjsuted_path : {}", qt!{ #adjsuted_path } );

    if let Some( rename ) = &path.rename
    {
      let pure_path = path.pure_without_super_path()?;
      c.clauses_map.get_mut( &ClauseImmediates::Kind() ).unwrap().push( qt!
      {
        pub use #pure_path as #rename;
      });
    }

    c.clauses_map.get_mut( &VisProtected::Kind() ).unwrap().push( qt!
    {
      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      #attrs1
      pub use #adjsuted_path::orphan::*;
    });

    c.clauses_map.get_mut( &VisExposed::Kind() ).unwrap().push( qt!
    {
      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      #attrs1
      pub use #adjsuted_path::exposed::*;
    });

    c.clauses_map.get_mut( &VisPrelude::Kind() ).unwrap().push( qt!
    {
      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      #attrs1
      pub use #adjsuted_path::prelude::*;
    });

    Ok( () )
  }

  ///
  /// Handle record "use" with explicit visibility.
  ///
  #[ allow ( dead_code ) ]
  fn record_use_explicit
  (
    record : &Record,
    c : &'_ mut RecordContext< '_ >,
    // clauses_map : &mut HashMap< u32, Vec< proc_macro2::TokenStream > >,
  )
  ->
  syn::Result< () >
  {
    let attrs1 = &record.attrs;
    let path = record.use_elements.as_ref().unwrap();
    let vis = record.vis.clone();

    if !vis.valid_sub_namespace()
    {
      return Err( syn_err!
      (
        record,
        "Use either {} visibility:\n  {}",
        VALID_VISIBILITY_LIST_STR,
        qt!{ #record },
      ));
    }

    let adjsuted_path = path.adjsuted_explicit_path();

    let vis2 = if vis.restriction().is_some()
    {
      qt!{ pub( crate ) }
    }
    else
    {
      qt!{ pub }
    };

    c.clauses_map.get_mut( &vis.kind() ).unwrap().push( qt!
    {
      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      #attrs1
      #vis2 use #adjsuted_path;
    });

    Ok( () )
  }

  ///
  /// Handle record micro module.
  ///

  fn record_micro_module
  (
    record : &Record,
    element : &Pair< AttributesOuter, syn::Path >,
    c : &'_ mut RecordContext< '_ >,
  )
  ->
  syn::Result< () >
  {
    let attrs1 = &record.attrs;
    let attrs2 = &element.0;
    let path = &element.1;

    c.clauses_map.get_mut( &ClauseImmediates::Kind() ).unwrap().push( qt!
    {
      #attrs1
      #attrs2
      pub mod #path;
    });

    if !record.vis.valid_sub_namespace()
    {
      return Err( syn_err!
      (
        record,
        "To include a non-standard module use either {} visibility:\n  {}",
        VALID_VISIBILITY_LIST_STR,
        qt!{ #record },
      ));
    }

    c.clauses_map.get_mut( &record.vis.kind() ).unwrap().push( qt!
    {
      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      #attrs1
      #attrs2
      pub use super::#path;
    });

    Ok( () )
  }

  ///
  /// Handle record micro module.
  ///
  #[ allow ( dead_code ) ]
  fn record_layer
  (
    record : &Record,
    element : &Pair< AttributesOuter, syn::Path >,
    c : &'_ mut RecordContext< '_ >,
  )
  ->
  syn::Result< () >
  {
    let attrs1 = &record.attrs;
    let attrs2 = &element.0;
    let path = &element.1;

    if record.vis != Visibility::Inherited
    {
      return Err( syn_err!
      (
        record,
        "Layer should not have explicitly defined visibility because all its subnamespaces are used.\n  {}",
        qt!{ #record },
      ));
    }

    c.clauses_map.get_mut( &ClauseImmediates::Kind() ).unwrap().push( qt!
    {
      #attrs1
      #attrs2
      pub mod #path;
    });

    c.clauses_map.get_mut( &VisProtected::Kind() ).unwrap().push( qt!
    {
      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      #attrs1
      #attrs2
      pub use super::#path::orphan::*;
    });

    c.clauses_map.get_mut( &VisExposed::Kind() ).unwrap().push( qt!
    {
      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      #attrs1
      #attrs2
      pub use super::#path::exposed::*;
    });

    c.clauses_map.get_mut( &VisPrelude::Kind() ).unwrap().push( qt!
    {
      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      #attrs1
      #attrs2
      pub use super::#path::prelude::*;
    });

    Ok( () )
  }

  ///
  /// Protocol of modularity unifying interface of a module and introducing layers.
  ///
  #[ allow ( dead_code ) ]
  pub fn mod_interface( input : proc_macro::TokenStream ) -> syn::Result< proc_macro2::TokenStream >
  {
    use ElementType::*;

    let original_input = input.clone();
    let document = syn::parse::< Thesis >( input )?;
    document.inner_attributes_validate()?;
    let has_debug = document.has_debug();

    // use inspect_type::*;
    // inspect_type_of!( immediates );

    let mut clauses_map : HashMap< _ , Vec< proc_macro2::TokenStream > > = HashMap::new();
    clauses_map.insert( ClauseImmediates::Kind(), Vec::new() );
    //clauses_map.insert( VisPrivate::Kind(), Vec::new() );
    clauses_map.insert( VisProtected::Kind(), Vec::new() );
    clauses_map.insert( VisOrphan::Kind(), Vec::new() );
    clauses_map.insert( VisExposed::Kind(), Vec::new() );
    clauses_map.insert( VisPrelude::Kind(), Vec::new() );

    // zzz : test case with several attrs

    let mut record_context = RecordContext::< '_ >
    {
      has_debug,
      clauses_map : &mut clauses_map,
    };

    document.records.0.iter().try_for_each( | record |
    {

      match record.element_type
      {
        Use( _ ) =>
        {
          let vis = &record.vis;
          if vis == &Visibility::Inherited
          {
            record_use_implicit( record, &mut record_context )?;
          }
          else
          {
            record_use_explicit( record, &mut record_context )?;
          }
        },
        _ =>
        {
          record.elements.iter().try_for_each( | element | -> syn::Result::< () >
          {
            match record.element_type
            {
              MicroModule( _ ) =>
              {
                record_micro_module( record, element, &mut record_context )?;
              },
              Layer( _ ) =>
              {
                record_layer( record, element, &mut record_context )?;
              },
              Use( _ ) =>
              {
              },
            }
            syn::Result::Ok( () )
          })?;
        }
      };

      syn::Result::Ok( () )
    })?;

    let immediates_clause = clauses_map.get( &ClauseImmediates::Kind() ).unwrap();
    let protected_clause = clauses_map.get( &VisProtected::Kind() ).unwrap();
    let orphan_clause = clauses_map.get( &VisOrphan::Kind() ).unwrap();
    let exposed_clause = clauses_map.get( &VisExposed::Kind() ).unwrap();
    let prelude_clause = clauses_map.get( &VisPrelude::Kind() ).unwrap();

    let result = qt!
    {

      #( #immediates_clause )*

      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      pub use protected::*;

      /// Protected namespace of the module.
      pub mod protected
      {
        #[ doc( inline ) ]
        #[ allow( unused_imports ) ]
        pub use super::orphan::*;
        #( #protected_clause )*
      }

      /// Orphan namespace of the module.
      pub mod orphan
      {
        #[ doc( inline ) ]
        #[ allow( unused_imports ) ]
        pub use super::exposed::*;
        #( #orphan_clause )*
      }

      /// Exposed namespace of the module.
      pub mod exposed
      {
        #[ doc( inline ) ]
        #[ allow( unused_imports ) ]
        pub use super::prelude::*;
        #( #exposed_clause )*
      }

      /// Prelude to use essentials: `use my_module::prelude::*`.
      pub mod prelude
      {
        #( #prelude_clause )*
      }

    };

    if has_debug
    {
      let about = format!( "derive : mod_interface" );
      diag::report_print( about, &original_input, &result );
    }

    // if has_debug
    // {
    //   diag::report_print( "derive : mod_interface", original_input, &result );
    // }

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
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use super::prelude::*;
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    mod_interface,
  };
}

// xxx : clean up, ad solve problems
// - example based on simpified version of test::layer_have_layer with single sublayer
// - example with attribute `#![ debug ]`

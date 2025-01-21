/// Define a private namespace for all its items.
mod private
{
  #[ allow( clippy::wildcard_imports ) ]
  use crate::*;
  #[ allow( clippy::wildcard_imports ) ]
  use macro_tools::exposed::*;
  use std::collections::HashMap;

// = use

  // x
  // use private::Type1;
  // use private::{ Type1, Type2 };
  // own use private::Type1;
  // prelude use private::Type1;

// = ?

  // x
  // own own1;
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
  // : own -> own
  // : orphan -> orphan
  // : exposed -> orphan
  // : prelude -> orphan

  // - extending

  // x
  // prelude exposed macromod mod_own1;
  // : own -> exposed
  // : orphan -> exposed
  // : exposed -> exposed
  // : prelude -> prelude

  // x
  // prelude own macromod mod_exposed1;
  // : own -> own
  // : orphan -> orphan
  // : exposed -> exposed
  // : prelude -> prelude

  // - selective

  // x
  // exposed exposed macromod mod_exposed1;
  // : own -> exposed
  // : orphan -> exposed
  // : exposed -> exposed
  // : prelude -> exposed

  // x
  // exposed orphan macromod mod_exposed1;
  // : own -> orphan
  // : orphan -> orphan
  // : exposed -> exposed
  // : prelude -> exposed

// = micro module

  // x
  // mod mod1;
  // mod mod2;
  // mod { mod1, mod2 };

  // +
  // own mod mod_own1;
  // orphan mod mod_orphan1;
  // exposed mod mod_exposed1;
  // prelude mod mod_prelude1;

  // +
  // own mod { mod_own1, mod_own2 };
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
  fn record_reuse_implicit
  (
    record : &Record,
    c : &'_ mut RecordContext< '_ >,
  )
  ->
  syn::Result< () >
  {

    let attrs1 = &record.attrs;
    let path = record.use_elements.as_ref().unwrap();

    let path = if let Some( rename ) = &path.rename
    {
      let pure_path = path.pure_without_super_path()?;
      c.clauses_map.get_mut( &ClauseImmediates::Kind() ).unwrap().push( qt!
      {
        pub use #pure_path as #rename;
      });
      parse_qt!{ #rename }
    }
    else
    {
      path.clone()
    };

    let adjsuted_path = path.prefixed_with_all();

    c.clauses_map.get_mut( &VisOwn::Kind() ).unwrap().push( qt!
    {
      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      #attrs1
      pub use #adjsuted_path::own::*;
    });

    c.clauses_map.get_mut( &VisOrphan::Kind() ).unwrap().push( qt!
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
  /// Handle record "use" with implicit visibility.
  ///
  fn record_use_implicit
  (
    record : &Record,
    c : &'_ mut RecordContext< '_ >,
  )
  ->
  syn::Result< () >
  {

    let attrs1 = &record.attrs;
    let path = record.use_elements.as_ref().unwrap();

    let path = if let Some( rename ) = &path.rename
    {
      let pure_path = path.pure_without_super_path()?;
      c.clauses_map.get_mut( &ClauseImmediates::Kind() ).unwrap().push( qt!
      {
        pub use #pure_path as #rename;
      });
      parse_qt!{ #rename }
    }
    else
    {
      path.clone()
    };

    let adjsuted_path = path.prefixed_with_all();

    c.clauses_map.get_mut( &VisOwn::Kind() ).unwrap().push( qt!
    {
      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      #attrs1
      pub use #adjsuted_path::orphan::*;
    });

    // export layer as own field of current layer
    let prefixed_with_super_maybe = path.prefixed_with_super_maybe();
    c.clauses_map.get_mut( &VisOwn::Kind() ).unwrap().push( qt!
    {
      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      #attrs1
      pub use #prefixed_with_super_maybe;
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
  fn record_use_explicit
  (
    record : &Record,
    c : &'_ mut RecordContext< '_ >,
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

    let adjsuted_path = path.prefixed_with_all();
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
      return Err
      (
        syn_err!
        (
          record,
          "To include a non-standard module use either {} visibility:\n  {}",
          VALID_VISIBILITY_LIST_STR,
          qt!{ #record },
        )
      );
    }

    c.clauses_map.get_mut( &record.vis.kind() ).unwrap().push( qt!
    {
      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      #attrs1
      #attrs2
      pub use __all__::#path;
      // pub use super::#path;
      // xxx : remove super?
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

    c.clauses_map.get_mut( &VisOwn::Kind() ).unwrap().push( qt!
    {
      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      #attrs1
      #attrs2
      pub use __all__::#path::orphan::*;
    });

    // export layer as own field of current layer
    // let prefixed_with_super_maybe = path.prefixed_with_super_maybe();
    c.clauses_map.get_mut( &VisOwn::Kind() ).unwrap().push( qt!
    {
      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      #attrs1
      pub use super::#path;
    });

    c.clauses_map.get_mut( &VisExposed::Kind() ).unwrap().push( qt!
    {
      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      #attrs1
      #attrs2
      pub use __all__::#path::exposed::*;
    });

    c.clauses_map.get_mut( &VisPrelude::Kind() ).unwrap().push( qt!
    {
      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      #attrs1
      #attrs2
      pub use __all__::#path::prelude::*;
    });

    Ok( () )
  }

  ///
  /// Protocol of modularity unifying interface of a module and introducing layers.
  ///
  #[ allow ( dead_code, clippy::too_many_lines ) ]
  pub fn mod_interface( input : proc_macro::TokenStream ) -> syn::Result< proc_macro2::TokenStream >
  {
    #[ allow( clippy::enum_glob_use ) ]
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
    clauses_map.insert( VisOwn::Kind(), Vec::new() );
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
        Reuse( _ ) =>
        {
          let vis = &record.vis;
          if vis == &Visibility::Inherited
          {
            record_reuse_implicit( record, &mut record_context )?;
          }
          else
          {
            return Err( syn_err!
            (
              record,
              "Using visibility usesd before `reuse` is illegal\n{}",
              qt!{ #record },
            ));
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
              _ =>
              {
                panic!( "Unexpected" )
              },
            }
            syn::Result::Ok( () )
          })?;
        }
      };

      syn::Result::Ok( () )
    })?;

    let immediates_clause = clauses_map.get( &ClauseImmediates::Kind() ).unwrap();
    let own_clause = clauses_map.get( &VisOwn::Kind() ).unwrap();
    let orphan_clause = clauses_map.get( &VisOrphan::Kind() ).unwrap();
    let exposed_clause = clauses_map.get( &VisExposed::Kind() ).unwrap();
    let prelude_clause = clauses_map.get( &VisPrelude::Kind() ).unwrap();

    let result = qt!
    {

      #( #immediates_clause )*

      // use private as __private__; // this line is necessary for readable error in case private namespace is not present

      #[ doc( inline ) ]
      #[ allow( unused_imports ) ]
      pub use own::*;

      /// Own namespace of the module.
      #[ allow( unused_imports ) ]
      pub mod own
      {
        // There must be internal private namespace
        // Because it's not possible to direcly make `use super::*;`
        // Because then items from super can't be exposed publicly complaining:
        // `error[E0428]: the name `mod1` is defined multiple times`
        // use super::*;
        use super::private; // this line is necessary for readable error in case private namespace is not present
        mod __all__
        {
          pub use super::super::*;
          pub use super::super::private::*;
        }
        #[ doc( inline ) ]
        pub use super::orphan::*;
        #( #own_clause )*
      }

      /// Orphan namespace of the module.
      #[ allow( unused_imports ) ]
      pub mod orphan
      {
        // use super::*;
        mod __all__
        {
          pub use super::super::*;
          pub use super::super::private::*;
        }
        #[ doc( inline ) ]
        pub use super::exposed::*;
        #( #orphan_clause )*
      }

      /// Exposed namespace of the module.
      #[ allow( unused_imports ) ]
      pub mod exposed
      {
        // use super::*;
        mod __all__
        {
          pub use super::super::*;
          pub use super::super::private::*;
        }
        #[ doc( inline ) ]
        pub use super::prelude::*;
        #( #exposed_clause )*
      }

      /// Prelude to use essentials: `use my_module::prelude::*`.
      #[ allow( unused_imports ) ]
      pub mod prelude
      {
        // use super::*;
        mod __all__
        {
          pub use super::super::*;
          pub use super::super::private::*;
        }
        #( #prelude_clause )*
      }

    };

    if has_debug
    {
      let about = "derive : mod_interface";
      diag::report_print( about, &original_input, &result );
    }

    // if has_debug
    // {
    //   diag::report_print( "derive : mod_interface", original_input, &result );
    // }

    Ok( result )
  }

}

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  pub use orphan::*;
}

pub use own::*;

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  pub use prelude::*;
  pub use private::
  {
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  pub use private::
  {
    mod_interface,
  };
}

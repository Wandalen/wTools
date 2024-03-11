//!
//! Analyze generic to provide more information than trivial syntax node.
//!

// xxx : is it used?

/// Internal namespace.
pub( crate ) mod private
{

  // xxx : qqq : examples. documentation
  /// Result of generics analyze.
  #[ derive( Debug ) ]
  pub struct GenericsAnalysis
  {
    /// Original generics.
    pub generics : syn::Generics,
    /// Array of names.
    pub names : Vec< syn::Ident >,
  }

  /// To analyze generics.
  pub trait GenericsAnalyze
  {

    /// Analyze generic.
    fn generics_analyze( &self ) -> GenericsAnalysis;

  }

  impl GenericsAnalyze for syn::ItemTrait
  {
    fn generics_analyze( &self ) -> GenericsAnalysis
    {
      let mut names = vec![];
      let generics = self.generics.clone();

      for param in &generics.params
      {
        match param
        {
          syn::GenericParam::Type( type_param ) => names.push( type_param.ident.clone() ),
          syn::GenericParam::Lifetime( lifetime_def ) => names.push( lifetime_def.lifetime.ident.clone() ),
          syn::GenericParam::Const( const_param ) => names.push( const_param.ident.clone() ),
        }
      }

      GenericsAnalysis
      {
        generics,
        names,
      }
    }
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
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    prelude::*,
    private::GenericsAnalysis,
  };
  pub use super::protected as generic_analyze;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    private::GenericsAnalyze,
  };
}

//!
//! Analyze generic to provide more information than trivial syntax node.
//!

/// Internal namespace.
pub( crate ) mod private
{

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
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::
  {
    prelude::*,
    private::GenericsAnalysis,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::
  {
    private::GenericsAnalyze,
  };
}

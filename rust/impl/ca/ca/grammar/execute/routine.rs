pub( crate ) mod private
{
  use crate::Context;

  use wtools::{ HashMap, Result };

  use std::{ fmt::Formatter, rc::Rc };

  /// Command Args
  #[ derive( Debug ) ]
  pub struct Args( pub Vec< String > );

  impl core::ops::Deref for Args
  {
    type Target = Vec< String >;
    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  /// Command Properties
  #[ derive( Debug ) ]
  pub struct Props( pub HashMap< String, String > );

  impl core::ops::Deref for Props
  {
    type Target = HashMap< String, String > ;
    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  type RoutineWithoutContextFn = dyn Fn(( Args, Props )) -> Result< () >;
  type RoutineWithContextFn = dyn Fn( ( Args, Props ), Context ) -> Result< () >;

  ///
  /// Routine handle.
  ///
  
  #[ derive( Clone ) ]
  pub enum Routine
  {
    /// Routine without context
    WithoutContext( Rc< RoutineWithoutContextFn > ),
    /// Routine with context
    WithContext( Rc< RoutineWithContextFn > ),
  }

  impl Routine
  {
    ///
    /// Create new routine.
    ///

    pub fn new< F >( callback : F ) -> Self
    where
      F : Fn(( Args, Props )) -> Result< () > + 'static,
    {
      Routine::WithoutContext( Rc::new( callback ) )
    }

    ///
    /// Create new routine with context.
    ///

    pub fn new_with_ctx< F >( callback : F ) -> Self
    where
      F : Fn( ( Args, Props ), Context ) -> Result< () > + 'static,
    {
      Routine::WithContext( Rc::new( callback ) )
    }
  }

  impl std::fmt::Debug for Routine
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      f.write_str( "Routine" )
    }
  }

  impl PartialEq for Routine
  {
    fn eq( &self, other : &Self ) -> bool
    {
      // We can't compare closures. Because every closure has a separate type, even if they're identical.
      // Therefore, we check that the two Rc's point to the same closure (allocation).
      #[ allow( clippy::vtable_address_comparisons ) ]
      match ( self, other )
      {
        ( Routine::WithContext( this ), Routine::WithContext( other ) ) => Rc::ptr_eq( this, other ),
        ( Routine::WithoutContext( this ), Routine::WithoutContext( other ) ) => Rc::ptr_eq( this, other ),
        _ => false
      }
    }
  }

  impl Eq for Routine {}
}

//

crate::mod_interface!
{
  prelude use Routine;
  prelude use Args;
  prelude use Props;
}

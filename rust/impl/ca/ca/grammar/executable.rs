pub( crate ) mod private
{
  use wtools::
  {
    HashMap,
    Result,
  };

  use std::{ fmt::Formatter, rc::Rc };

  /// Commands that be executed
  #[ derive( Debug ) ]
  pub struct ExecutableCommand
  {
    /// subjects values
    pub subjects : Vec< String >,
    /// properties value
    pub properties : HashMap< String, String >,
    /// function that will be called
    pub routine : Routine,
  }

  // ! TEMP !
  pub struct Context;

  pub struct Args;
  pub struct Props;
  // ! ---

  type RoutineWithoutContextFn = dyn Fn(( &Args, &Props )) -> Result< () >;
  type RoutineWithContextFn = dyn Fn( ( &Args, &Props ), Context ) -> Result< () >;

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
      F : Fn(( &Args, &Props )) -> Result< () > + 'static,
    {
      Routine::WithoutContext( Rc::new( callback ) )
    }

    ///
    /// Create new routine with context.
    ///

    pub fn new_with_ctx< F >( callback : F ) -> Self
    where
      F : Fn( ( &Args, &Props ), Context ) -> Result< () > + 'static,
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
  prelude use ExecutableCommand;
  prelude use Routine;
  prelude use Args;
  prelude use Props;
  prelude use Context;
}
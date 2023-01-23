
pub( crate ) mod private
{
  use std::rc::Rc;
  use core::cell::RefCell;
  use core::any::Any;

  /// Execution statement of a program
  #[ derive( Debug ) ]
  pub struct ProgramState
  {
    /// Current instruction number
    pub current_pos : usize
  }

  #[ derive( Debug, Clone ) ]
  /// Container for contexts values
  pub struct Context
  {
    inner : Rc< RefCell< anymap::AnyMap > >
  }

  impl Context
  {
    /// Create context
    pub fn new< T : Any >( value : T ) -> Self
    {
      let mut contexts = anymap::AnyMap::new();
      contexts.insert( value );

      // Execution context
      // ? Is it OK?
      let state = ProgramState { current_pos: 0 } ;
      contexts.insert( state );

      Self { inner : Rc::new( RefCell::new( contexts ) ) }
    }

    /// Insert the T value to the context. If it is alredy exists - replace it
    pub fn insert< T : Any >( &self, value : T )
    {
      self.inner.borrow_mut().insert( value );
    }

    /// Removes the T value from the context
    pub fn remove< T : Any >( &mut self )
    {
      self.inner.borrow_mut().remove::< T >();
    }

    /// Return immutable reference on interior object. ! Unsafe !
    pub fn get_ref< T : Any >( &self ) -> Option< &T >
    {
      // ! how do it better?
      unsafe{ self.inner.as_ptr().as_ref()?.get() }
    }

    /// Return mutable reference on interior object. ! Unsafe !
    pub fn get_mut< T : Any >( &self ) -> Option< &mut T >
    {
      // ! how do it better?
      unsafe { self.inner.as_ptr().as_mut()?.get_mut() }
    }
  }

  impl PartialEq for Context
  {
    fn eq( &self, _other : &Self ) -> bool
    {
      false
    }
  }
}

crate::mod_interface!
{
  prelude use Context;
  prelude use ProgramState;
}

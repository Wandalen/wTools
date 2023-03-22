
pub( crate ) mod private
{
  use std::rc::Rc;
  use core::cell::RefCell;
  use core::any::Any;

  use crate::ProgramState;

  #[ derive( Debug, Clone ) ]
  /// Container for contexts values
  pub struct Context
  {
    inner : Rc< RefCell< anymap::AnyMap > >
  }

  impl Default for Context
  {
    fn default() -> Self
    {
      let mut contexts = anymap::AnyMap::new();
      
      let state = ProgramState::default();
      contexts.insert( state );

      Self { inner : Rc::new( RefCell::new( contexts ) ) }
    }
  }

  impl Context
  {
    /// Create context
    pub fn new< T : Any >( value : T ) -> Self
    {
      let contexts = Self::default();
      contexts.insert( value );

      contexts
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

    /// Insert the value if it doesn't exists, or take an existing value and return mutable reference to it
    pub fn get_or_insert< T : Any >( &self, value : T ) -> &mut T
    {
      if let Some( value ) = self.get_mut()
      {
        value
      }
      else
      {
        self.insert( value );
        self.get_mut().unwrap()
      }
    }

    /// Insert default value if it doesn't exists, or take an existing value and return mutable reference to it
    pub fn get_or_default< T : Any + Default >( &self ) -> &mut T
    {
      self.get_or_insert( T::default() )
    }
  }

  impl PartialEq for Context
  {
    fn eq( &self, other : &Self ) -> bool
    {
      self == other
    }
  }
}

//

crate::mod_interface!
{
  prelude use Context;
}

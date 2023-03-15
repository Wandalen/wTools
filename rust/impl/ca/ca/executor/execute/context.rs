pub( crate ) mod private
{
  use std::{ sync::Arc, cell::RefCell };

  use anymap::{ Map, any::CloneAny };

  // CloneAny needs to deep clone of Context
  #[ derive( Debug, Clone, former::Former ) ]
  /// Container for contexts values
  pub struct Context
  {
    inner : Arc< RefCell< Map::< dyn CloneAny > > >
  }

  impl ContextFormer
  {
    pub fn with< T : CloneAny >( mut self, value : T ) -> Self
    {
      if self.inner.is_none()
      {
        self.inner = Some( Arc::new( RefCell::new( Map::< dyn CloneAny >::new() ) ) );
      }
      self.inner.as_ref().map( | inner | inner.borrow_mut().insert( value ) );
      self
    }
  }

  impl Default for Context
  {
    fn default() -> Self
    {
      Self { inner : Arc::new( RefCell::new( Map::< dyn CloneAny >::new() ) ) }
    }
  }

  impl Context
  {
     /// Insert the T value to the context. If it is alredy exists - replace it
     pub fn insert< T : CloneAny >( &self, value : T )
     {
       self.inner.borrow_mut().insert( value );
     }
 
     /// Removes the T value from the context
     pub fn remove< T : CloneAny >( &mut self ) -> Option< T >
     {
       self.inner.borrow_mut().remove::< T >()
     }

    /// Return immutable reference on interior object. ! Unsafe !
    pub fn get_ref< T : CloneAny >( &self ) -> Option< &T >
    {
      unsafe{ self.inner.as_ptr().as_ref()?.get() }
    }

    /// Return mutable reference on interior object. ! Unsafe !
    pub fn get_mut< T : CloneAny >( &self ) -> Option< &mut T >
    {
      unsafe { self.inner.as_ptr().as_mut()?.get_mut() }
    }

    /// Insert the value if it doesn't exists, or take an existing value and return mutable reference to it
    pub fn get_or_insert< T : CloneAny >( &self, value : T ) -> &mut T
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
    pub fn get_or_default< T : CloneAny + Default >( &self ) -> &mut T
    {
      self.get_or_insert( T::default() )
    }

    /// Make a deep clone of the context
    pub( crate ) fn deep_clone( &self ) -> Self
    {
      Self { inner : Arc::new( RefCell::new( ( *self.inner ).borrow_mut().clone() ) ) }
    }
  }
}

//

crate::mod_interface!
{
  prelude use Context;
}

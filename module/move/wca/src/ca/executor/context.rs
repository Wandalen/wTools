pub( crate ) mod private
{
  use std::{ sync::Arc, cell::RefCell };
  use anymap::{ Map, any::CloneAny };

  /// Container for contexts values
  ///
  /// # Examples:
  ///
  /// ```
  /// use wca::Context;
  ///
  /// let ctx = Context::default();
  ///
  /// ctx.insert( 42 );
  /// assert_eq!( 42, *ctx.get_ref().unwrap() );
  /// ```
  ///
  /// ```
  /// # use wca::{ Routine, Context, Value, Args, Props };
  /// let routine = Routine::new_with_ctx
  /// (
  ///   | ( args, props ), ctx |
  ///   {
  ///     let first_arg : i32 = args.get_owned( 0 ).unwrap_or_default();
  ///     let ctx_value : &mut i32 = ctx.get_or_default();
  ///
  ///     *ctx_value += first_arg;
  ///
  ///     Ok( () )
  ///   }
  /// );
  /// let ctx = Context::default();
  /// if let Routine::WithContext( callback ) = routine
  /// {
  ///   callback( ( Args( vec![ Value::Number( 1.0 ) ] ), Props( Default::default() ) ), ctx.clone() ).unwrap();
  /// }
  /// assert_eq!( 1, *ctx.get_ref().unwrap() );
  /// ```
  // CloneAny needs to deep clone of Context
  // qqq : ?
  #[ derive( Debug, Clone, former::Former ) ]
  pub struct Context
  {
    inner : Arc< RefCell< Map::< dyn CloneAny > > >
  }

  impl ContextFormer
  {
    /// Initialize Context with some value
    pub fn with< T : CloneAny >( mut self, value : T ) -> Self
    {
      if self.storage.inner.is_none()
      {
        self.storage.inner = Some( Arc::new( RefCell::new( Map::< dyn CloneAny >::new() ) ) );
      }
      self.storage.inner.as_ref().map( | inner | inner.borrow_mut().insert( value ) );
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
     /// Insert the T value to the context. If it is already exists - replace it
     pub fn insert< T : CloneAny >( &self, value : T )
     {
       self.inner.borrow_mut().insert( value );
     }

     /// Removes the T value from the context
     pub fn remove< T : CloneAny >( &mut self ) -> Option< T >
     {
       self.inner.borrow_mut().remove::< T >()
     }

    // qqq : Bohdan : why unsafe?
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
    // qqq : for Bohdan : why is it deep? how is it deep?
    // qqq : how is it useful? Is it? Examples?
    pub( crate ) fn deep_clone( &self ) -> Self
    {
      Self { inner : Arc::new( RefCell::new( ( *self.inner ).borrow_mut().clone() ) ) }
    }
  }
}

//

crate::mod_interface!
{
  exposed use Context;
}

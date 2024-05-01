pub( crate ) mod private
{
  use std::sync::Arc;

  /// Container for contexts values
  ///
  /// # Examples:
  ///
  /// ```
  /// # use wca::{ Routine, Handler, Context, Value, Args, Props, VerifiedCommand };
  /// # use std::sync::{ Arc, Mutex };
  /// let routine = Routine::from( Handler::from
  /// (
  ///   | ctx : Context, o : VerifiedCommand |
  ///   {
  ///     let first_arg : i32 = o.args.get_owned( 0 ).unwrap_or_default();
  ///     let ctx_value : Arc< Mutex< i32 > > = ctx.get().unwrap();
  ///
  ///     *ctx_value.lock().unwrap() += first_arg;
  ///   }
  /// ) );
  /// let ctx = Context::new( Mutex::new( 0 ) );
  /// if let Routine::WithContext( callback ) = routine
  /// {
  ///   let w_command = VerifiedCommand
  ///   {
  ///     phrase : "command".into(),
  ///     internal_command : false,
  ///     args : Args( vec![ Value::Number( 1.0 ) ] ),
  ///     props : Props( Default::default() ),
  ///   };
  ///   callback( ctx.clone(), w_command ).unwrap();
  /// }
  /// assert_eq!( 1, *ctx.get::< Mutex< i32 > >().unwrap().lock().unwrap() );
  /// ```
  // qqq : ?
  #[ derive( Debug, Clone ) ]
  pub struct Context
  {
    inner : Arc< dyn std::any::Any + Send + Sync >,
  }
  
  impl Default for Context
  {
    fn default() -> Self
    {
      Self::new( () )
    }
  }
  
  impl Context
  {
    /// Creates a new `Context` object with the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be stored in the `Context`. The value must implement the `Send` and `Sync` traits.
    /// ```
    // `'static` means that the object must be owned or live at least as a `Context'
    pub fn new< T : Send + Sync + 'static >( value : T ) -> Self
    {
      Self { inner : Arc::new( value ) }
    }
  }

  impl Context
  {
    /// This method retrieves a shared reference to an object of type `T` from the context.
    ///
    /// # Arguments
    ///
    /// * `&self` - The context object.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the object to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference-counted smart pointer (`Arc`) to the object of type `T` if it exists in the context.
    /// `None` is returned if the object does not exist or if it cannot be downcasted to type `T`.
    // `'static` means that the object must be owned or live at least as a `Context'
    pub fn get< T : Send + Sync + 'static >( &self ) -> Option< Arc< T > >
    {
      self.inner.clone().downcast::< T >().ok()
    }
  }
}

//

crate::mod_interface!
{
  exposed use Context;
}

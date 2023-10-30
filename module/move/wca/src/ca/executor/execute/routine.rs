pub( crate ) mod private
{
  use crate::{ Context, Value };

  use std::collections::HashMap;
  use wtools::error::Result;

  use std::{ fmt::Formatter, rc::Rc };

  /// Command Args
  ///
  /// Used to contain subjects of a command and allow the user to retrieve them in comfortable way.
  ///
  /// # Example:
  ///
  /// ```
  /// use wca::prelude::*;
  ///
  /// let args = Args( vec![ Value::String( "Hello, World!".to_string() ) ] );
  ///
  /// let first_arg : &str = args.get_owned( 0 ).unwrap();
  /// assert_eq!( "Hello, World!", first_arg );
  ///
  /// let first_arg : &str = args[ 0 ].clone().into();
  /// assert_eq!( "Hello, World!", first_arg );
  /// ```
  ///
  /// ## Use case
  /// ```
  /// # use wca::{ Routine, Args };
  /// let routine = Routine::new
  /// (
  ///   |( args, _ ) : ( Args, _ )|
  ///   {
  ///     let first_arg : i32 = args.get_owned( 0 ).unwrap();
  ///
  ///     Ok( () )
  ///   }
  /// );
  /// ```
  #[ derive( Debug ) ]
  pub struct Args( pub Vec< Value > );

  impl Args
  {
    /// Returns owned casted value by its index
    ///
    /// ```
    /// # use wca::prelude::*;
    ///
    /// let args = Args( vec![ Value::String( "Hello, World!".to_string() ) ] );
    ///
    /// let first_arg : &str = args.get_owned( 0 ).unwrap();
    /// assert_eq!( "Hello, World!", first_arg );
    ///
    /// let first_arg : &str = args[ 0 ].clone().into();
    /// assert_eq!( "Hello, World!", first_arg );
    /// ```
    pub fn get_owned< T : From< Value > >( &self, index : usize ) -> Option< T >
    {
      self.0.get( index ).map( | arg | arg.to_owned().into() )
    }
  }

  impl core::ops::Deref for Args
  {
    type Target = Vec< Value >;
    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  /// Command Properties
  ///
  /// Used to contain properties of a command and allow the user to retrieve them in comfortable way.
  ///
  /// # Example:
  ///
  /// ```
  /// use wca::prelude::*;
  ///
  /// let props = Props( [ ( "hello".to_string(), Value::String( "World!".to_string() ) ) ].into() );
  /// let hello_prop : &str = props.get_owned( "hello" ).unwrap();
  ///
  /// assert_eq!( "World!", hello_prop );
  /// ```
  ///
  /// ## Use case
  /// ```
  /// # use wca::{ Routine, Props };
  /// let routine = Routine::new
  /// (
  ///   |( _, props ) : ( _, Props )|
  ///   {
  ///     let key_option : i32 = props.get_owned( "key" ).unwrap();
  ///
  ///     Ok( () )
  ///   }
  /// );
  /// ```
  #[ derive( Debug ) ]
  pub struct Props( pub HashMap< String, Value > );

  impl Props
  {
    /// Returns owned casted value by its key
    ///
    /// ```
    /// # use wca::prelude::*;
    ///
    /// let props = Props( [ ( "hello".to_string(), Value::String( "World!".to_string() ) ) ].into() );
    /// let hello_prop : &str = props.get_owned( "hello" ).unwrap();
    ///
    /// assert_eq!( "World!", hello_prop );
    /// ```
    pub fn get_owned< K : AsRef< str >, T : From< Value > >( &self, key : K ) -> Option< T >
    {
      self.0.get( key.as_ref() ).map( | arg | arg.to_owned().into() )
    }
  }

  impl core::ops::Deref for Props
  {
    type Target = HashMap< String, Value > ;
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
  /// ```
  /// # use wca::Routine;
  /// let routine = Routine::new
  /// (
  ///   |( args, props )|
  ///   {
  ///     // Do what you need to do
  ///
  ///     Ok( () )
  ///   }
  /// );
  /// ```
  ///
  /// ```
  /// # use wca::Routine;
  /// let routine = Routine::new_with_ctx
  /// (
  ///   | ( args, props ), ctx |
  ///   {
  ///     // Do what you need to do
  ///
  ///     Ok( () )
  ///   }
  /// );

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
    /// ```
    /// # use wca::Routine;
    /// let routine = Routine::new
    /// (
    ///   |( args, props )|
    ///   {
    ///     // Do what you need to do
    ///
    ///     Ok( () )
    ///   }
    /// );
    /// ```

    pub fn new< F >( callback : F ) -> Self
    where
      F : Fn(( Args, Props )) -> Result< () > + 'static,
    {
      Routine::WithoutContext( Rc::new( callback ) )
    }

    ///
    /// Create new routine with context.
    ///
    /// ```
    /// # use wca::Routine;
    /// let routine = Routine::new_with_ctx
    /// (
    ///   | ( args, props ), ctx |
    ///   {
    ///     // Do what you need to do
    ///
    ///     Ok( () )
    ///   }
    /// );
    /// ```

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

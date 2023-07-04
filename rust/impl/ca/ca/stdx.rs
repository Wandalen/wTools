pub( crate ) mod private
{
  use core::fmt::Debug;

  /// Macro for parsing WCA arguments.
  ///
  /// # Examples
  /// ```rust
  /// use wca::Value;
  ///
  /// let mut args = vec![Value::Number(42.), Value::String("Rust".into())].into_iter();
  /// stdx::parse_args!(args, n: f64, name: String);
  ///
  /// assert_eq!(n, 42.);
  /// assert_eq!(name, "Rust");
  /// ```
  #[macro_export]
  macro_rules! parse_args
  {
    ($args:ident, mut $b:ident: $ty:ident $( $rest:tt )* ) =>
    {
      let mut $b: $ty = std::convert::TryFrom::try_from( $args.next().unwrap() ).unwrap();
      $crate::parse_args!($args $( $rest )* )
    };
    ($args:ident, $b:ident: $ty:ident $( $rest:tt )* ) =>
    {
      let $b: $ty = std::convert::TryFrom::try_from( $args.next().unwrap() ).unwrap();
      $crate::parse_args!( $args $( $rest )* )
    };
    ($args:ident, $b:ident $( $rest:tt )* ) =>
    {
      let $b = $args.next().unwrap();
      $crate::parse_args!( $args $( $rest )* )
      };
    ($args:ident, mut $b:ident $( $rest:tt )* ) =>
    {
      let mut $b = $args.next().unwrap();
      $crate::parse_args!( $args $( $rest )* )
    };
    ($args:ident) =>
    {
      assert!( $args.next().is_none() );
    };
    ($args:ident,) =>
    {
      $crate::parse_args!( $args )
    };
  }

  /// Creates a command-line interface (CLI) builder with the given initial state.
  ///
  /// This function initializes a `CommandBuilder` with the provided `state` and
  /// returns it for further configuration of the CLI.
  pub fn cli< T >( state: T ) -> CommandBuilder< T >
  {
    CommandBuilder::with_state( state )
  }

    /// A struct representing a property.
    #[ derive( Debug, Clone ) ]
    pub struct Property< 'a >
    {
      /// The name of the property.
      pub name : &'a str,
      /// The hint for the property.
      pub hint : &'a str,
      /// The tag representing the property's type.
      pub tag : crate::Type,
    }

    /// A builder struct for constructing commands.
    #[ derive( Debug ) ]
    pub struct CommandBuilder< T >
    {
      state: T,
      commands: Vec< crate::Command >,
      handlers: Vec< ( String, crate::Routine ) >,
    }

    impl< T > CommandBuilder< T >
    {
      /// Constructs a `CommandBuilder` with the given state.
      pub fn with_state(state: T) -> Self
      {
        Self { state, handlers: vec![], commands: vec![] }
      }
    }

    #[ derive( Debug ) ]
    pub struct Builder< F >
    {
      handler : F,
      command : crate::Command,
    }

    impl< F > Builder< F >
    {
        pub fn new( handler: F ) -> Self
        {
            let name =
            {
                use wtools::Itertools as _;

                let name = std::any::type_name::< F >();
                let name = name.rfind( ':' ).map_or( name, | tail | &name[ tail + 1.. ] );
                name.split( '_' ).join( "." )
            };

            Self { handler, command : crate::Command::former().phrase( name ).form() }
        }

        pub fn arg( mut self, hint : &str, tag : crate::Type ) -> Self
        {
            self.command.subjects.push( crate::grammar::settings::ValueDescription
            {
              hint : hint.into(),
              kind : tag,
              optional : false,
            } );

            self
        }

        pub fn properties< const N: usize >( mut self, properties: [ Property; N ] ) -> Self
        {

          self.command.properties.reserve( properties.len() );

          for property in properties
          {
            self.command.properties.insert(
              property.name.to_owned(),
              crate::grammar::settings::ValueDescription
              {
                hint : property.hint.to_owned(),
                kind : property.tag,
                optional : true,
              },
            );
          }
          self
        }
    }

    impl< T: Copy + 'static > CommandBuilder< T > {
        /// Adds a command to the `CommandBuilder`.
        pub fn command< F: Fn( T, crate::Args, crate::Props ) -> Result<(), E> + 'static, E: Debug>(
          mut self,
          command: impl IntoBuilder<F, T>,
        ) -> Self
        {
          let Builder { handler, command } = command.into_builder();

          let handler = crate::Routine::new(move | ( args, props ) |
          {
            handler(self.state, args, props)
            .map_err( | report | crate::BasicError::new( format!( "{report:?}" ) ) )
          } );


          self.handlers.push( (command.phrase.clone(), handler ) );
          self.commands.push( command );

          self
        }

        /// Builds and returns a `wca::CommandsAggregator` instance.
        ///
        /// This method finalizes the construction of the `CommandBuilder` by
        /// creating a `wca::CommandsAggregator` instance with the accumulated
        /// commands and handlers.
        pub fn build(self) -> crate::CommandsAggregator
        {
          let handlers = std::collections::HashMap::from_iter( self.handlers );
          crate::CommandsAggregator::former().grammar( self.commands ).executor( handlers ).build()
        }
    }

  /// An extension trait for commands.
  ///
  /// This trait provides additional methods for enhancing commands, such as
  /// adding arguments and properties.
  pub trait CommandExt< T >: Sized
  {
    /// Adds an argument to the command.
    fn arg( self, hint : &str, tag : crate::Type ) -> Builder< Self >
    {
      Builder::new( self ).arg( hint, tag )
    }

    /// Adds properties to the command.
    fn properties< const N: usize >( self, properties: [ Property; N ] ) -> Builder< Self >
    {
      Builder::new( self ).properties( properties )
    }
  }

  impl< F: Fn( T, crate::Args, crate::Props ) -> Result<(), E>, T, E > CommandExt< T > for F {}

  /// A trait for converting a type into a `Builder`.
  pub trait IntoBuilder< F, T >: Sized
  {
    /// Converts the type into a `Builder` instance.
    fn into_builder( self ) -> Builder< F >;
  }

  impl< F, T > IntoBuilder< F, T > for Builder< F >
  {
    fn into_builder(self) -> Self
    {
      self
    }
  }

  impl< F: Fn( T, crate::Args, crate::Props ) -> Result<(), E>, T, E > IntoBuilder< F, T > for F
  {
    fn into_builder( self ) -> Builder< F >
    {
      Builder::new( self )
    }
  }
}

crate::mod_interface! {
  prelude use cli;
  prelude use IntoBuilder;
  prelude use CommandExt;
  prelude use CommandBuilder;
}

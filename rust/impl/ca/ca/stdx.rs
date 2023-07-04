pub( crate ) mod private
{
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

  /// A type alias for `miette::Result<T, E>`.
  pub type Result< T = (), E = miette::Report > = miette::Result< T, E >;

  /// Creates a command-line interface (CLI) builder with the given initial state.
  ///
  /// This function initializes a `CommandBuilder` with the provided `state` and
  /// returns it for further configuration of the CLI.
  pub fn cli< T >( state: T ) -> CommandBuilder< T, 0 > 
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
    pub struct CommandBuilder< T, const N : usize > 
    {
      state: T,
      commands: [ crate::Command; N ],
      handlers: [ ( String, crate::Routine ); N ],
    }
  
    impl< T > CommandBuilder< T, 0 > 
    {
      /// Constructs a `CommandBuilder` with the given state.
      pub fn with_state(state: T) -> Self 
      {
        Self { state, handlers: [], commands: [] }
      }
    }
  
    #[ derive( Debug ) ]
    pub struct Builder<F> 
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
                name.rfind( ':' ).map_or( name, | tail | &name[ tail + 1.. ] ).split( '_' ).join( "." )
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
            });
            self
        }
  
        pub fn properties< const N: usize >( mut self, properties: [ Property; N ] ) -> Self 
        {
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
  
    impl< T: Copy + 'static, const LEN: usize > CommandBuilder< T, LEN > {
        /// Adds a command to the `CommandBuilder`.
        pub fn command< F: Fn( T, crate::Args, crate::Props ) -> Result + 'static>(
          self,
          command: impl IntoBuilder<F, T>,
        ) -> CommandBuilder< T, { LEN + 1 } > 
        {
          let Builder { handler, command } = command.into_builder();
  
          let handler = crate::Routine::new(move | ( args, props ) | 
          {
            handler(self.state, args, props)
            .map_err(|report| crate::BasicError::new( format!( "{report:?}" ) ) )
          });
  
          CommandBuilder 
          {
            state: self.state,
            handlers: array_push( self.handlers, ( command.phrase.clone(), handler ) ),
            commands: array_push( self.commands, command ),
          }
        }
  
        /// Builds and returns a `wca::CommandsAggregator` instance.
        ///
        /// This method finalizes the construction of the `CommandBuilder` by
        /// creating a `wca::CommandsAggregator` instance with the accumulated
        /// commands and handlers.
        pub fn build(self) -> crate::CommandsAggregator 
        {
          crate::CommandsAggregator::former().grammar( self.commands ).executor( self.handlers ).build()
        }
    }
  
    fn array_push< const N: usize, T >( this: [T; N], item: T ) -> [ T; N + 1 ] 
    {
      use std::mem::MaybeUninit;
  
      unsafe 
      {
        let mut uninit = MaybeUninit::< [ T; N + 1 ] >::uninit();
  
        let ptr = uninit.as_mut_ptr() as *mut T;
        ( ptr as *mut [ T; N ] ).write( this );
        ( ptr.add( N ) as *mut [ T; 1 ] ).write( [ item ] );
  
        uninit.assume_init()
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
      Builder::new(self).arg(hint, tag)
    }

    /// Adds properties to the command.
    fn properties< const N: usize >( self, properties: [ Property; N ] ) -> Builder< Self >
    {
      Builder::new( self ).properties( properties )
    }
  }

  impl< F: Fn( T, crate::Args, crate::Props ) -> Result, T > CommandExt< T > for F {}

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

  impl< F: Fn( T, crate::Args, crate::Props ) -> Result, T > IntoBuilder< F, T > for F 
  {
    fn into_builder( self ) -> Builder< F > 
    {
      Builder::new( self )
    }
  }
}

crate::mod_interface! {}

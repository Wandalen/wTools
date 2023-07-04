pub( crate ) mod private
{
  /// A struct representing a property.
  #[ derive( Debug, Clone ) ]
  pub struct Property<'a> 
  {
    /// The name of the property.
    pub name : &'a str,
    /// The hint for the property.
    pub hint : &'a str,
    /// The tag representing the property's type.
    pub tag : wca::Type,
  }

  /// A builder struct for constructing commands.
  #[ derive( Debug ) ]
  pub struct CommandBuilder< T, const N: usize > 
  {
    state: T,
    commands: [ wca::Command; N ],
    handlers: [ ( String, wca::Routine ); N ],
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
    command : wca::Command,
  }

  impl< F > Builder< F > 
  {
      pub fn new( handler: F ) -> Self
      {
          let name = 
          {
              let name = std::any::type_name::< F >();
              name.rfind( ':' ).map_or( name, | tail | &name[ tail + 1.. ] ).split( '_' ).join( "." )
          };

          Self { handler, command : wca::Command::former().phrase( name ).form() }
      }

      pub fn arg( mut self, hint : &str, tag : wca::Type ) -> Self 
      {
          self.command.subjects.push( wca::grammar::settings::ValueDescription 
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
            wca::grammar::settings::ValueDescription 
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
      pub fn command< F: Fn( T, wca::Args, wca::Props ) -> Result + 'static>(
        self,
        command: impl IntoBuilder<F, T>,
      ) -> CommandBuilder< T, { LEN + 1 } > 
      {
        let Builder { handler, command } = command.into_builder();

        let handler = wca::Routine::new(move | ( args, props ) | 
        {
          handler(self.state, args, props)
          .map_err(|report| wca::BasicError::new( format!( "{report:?}" ) ) )
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
      pub fn build(self) -> wca::CommandsAggregator 
      {
        wca::CommandsAggregator::former().grammar( self.commands ).executor( self.handlers ).build()
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
}

crate::mod_interface!
{

}

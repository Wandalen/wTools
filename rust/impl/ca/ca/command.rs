pub( crate ) mod private
{
  use std::
  {
    collections::HashMap,
    rc::Rc,
  };
  use std::fmt::Formatter;
  use wtools::
  {
    error::{ Result, BasicError },
    meta::Former,
  };
  use wtools::string::parse_request::OpType;
  use crate::{ Instruction, Context };

  ///
  /// A type that implements ['Properties'] can be used as 'properties' in ['Args'].
  ///

  pub trait Properties : Sized
  {
    /// Parse properties.
    fn parse( properties : &HashMap< String, OpType< String > > ) -> Result< Self >;
  }

  ///
  /// A type that implements ['Subject'] can be used as 'subject' in ['Args'].
  ///

  pub trait Subject : Sized
  {
    /// Parse subject.
    fn parse( input : impl AsRef< str > ) -> Result< Self >;
  }

  ///
  /// Command descriptor.
  ///

  #[ derive( Clone, PartialEq, Debug ) ]
  #[ derive( Former ) ]
  pub struct Command
  {
    /// Command common hint.
    #[ alias( h ) ]
    pub hint : String,
    /// Command full hint.
    #[ alias( lh ) ]
    pub long_hint : String,
    /// Phrase descriptor for command.
    pub phrase : String,
    /// Command subject hint.
    pub subject_hint : String,
    /// Hints for command options.
    pub properties_hints : HashMap< String, String >,
    /// Map of aliases.
    pub properties_aliases : HashMap< String, Vec< String > >,
    /// Command routine.
    #[ setter( false ) ]
    pub routine : Routine,
  }

  type RoutineWithoutContextFn = dyn Fn( &Instruction ) -> Result< () >;
  type RoutineWithContextFn = dyn Fn( &Instruction, Context ) -> Result< () >;

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

  ///
  /// Command args. Used in ['Routine'] as the routine args.
  ///

  pub struct Args< S, P >
  {
    /// Subject of the command.
    pub subject : S,
    /// Properties of the command.
    pub properties : P,
  }

  ///
  /// Used in ['Args'] when a command doesn't expect properties.
  ///

  #[ derive( Debug, PartialEq, Eq ) ]
  pub struct NoProperties;

  ///
  /// Used in ['Args'] when a command doesn't expect subject.
  ///

  #[ derive( Debug, PartialEq, Eq ) ]
  pub struct NoSubject;

  impl Command
  {
    /// Generate short help for command.
    pub fn help_short( &self ) -> String
    {
      format!( ".{} - {}", self.phrase.replace( ' ', "." ), self.hint )
    }

    /// Generate short help for command.
    pub fn help_long( &self ) -> String
    {
      let properties_hints = self.properties_hints.iter().map( | ( key, value ) | format!( "  {} - {}", key, value ) ).collect::< Vec< _ > >();
      let properties_hints = properties_hints.join( "\n" );
      format!( ".{} - {}\n{}", self.phrase.replace( ' ', "." ), self.long_hint, properties_hints )
    }

    /// Execute callback.
    pub fn perform( &self, instruction : &crate::instruction::Instruction, context : Option< Context > ) -> Result< () >
    {
      if self.subject_hint.is_empty() && !instruction.subject.is_empty()
      {
        return Err( BasicError::new( "Unexpected subject." ) );
      }

      for ( key, _value ) in &instruction.properties_map
      {
        if self.properties_hints.get( key.as_str() ).is_none()
        {
          return Err( BasicError::new( "Unknown option." ) );
        }
      }

      self.routine.perform( instruction, context )
    }
  }

  impl CommandFormer
  {
    /// Setter for separate properties.
    pub fn property_hint< S : AsRef< str > >( mut self, key : S, hint : S ) -> Self
    {
      let key = key.as_ref();
      let hint = hint.as_ref();

      if self.properties_hints.is_none()
      {
        self.properties_hints = Some( HashMap::from([ ( key.into(), hint.into() ) ]) );
      }
      else
      {
        let hmap = self.properties_hints.as_mut().unwrap();
        hmap.insert( key.into(), hint.into() );
      }
      self
    }

    /// Setter for separate properties aliases.
    pub fn property_alias< S : AsRef< str > >( mut self, key : S, alias : S ) -> Self
    {
      let key = key.as_ref();
      let alias = alias.as_ref();

      if self.properties_aliases.is_none()
      {
        self.properties_aliases = Some( HashMap::from([ ( key.into(), vec![ alias.into() ] ) ]) );
      }
      else
      {
        let hmap = self.properties_aliases.as_mut().unwrap();
        if hmap.get( key ).is_some()
        {
          let vec_aliases = hmap.get_mut( key ).unwrap();
          vec_aliases.push( alias.into() );
        }
        else
        {
          hmap.insert( key.into(), vec![ alias.into() ] );
        }
      }
      self
    }

    ///
    /// Routine setter.
    ///

    pub fn routine< F, S, P >( mut self, callback: F ) -> Self
    where
      F : Fn( Args< S, P > ) -> Result< () > + 'static,
      S : Subject,
      P : Properties,
    {
      self.routine = Some( Routine::new( callback ) );
      self
    }

    ///
    /// Routine setter with context.
    ///

    pub fn routine_with_ctx< F, S, P >( mut self, callback: F ) -> Self
    where
      F : Fn( Args< S, P >, Context ) -> Result< () > + 'static,
      S : Subject,
      P : Properties,
    {
      self.routine = Some( Routine::new_with_ctx( callback ) );
      self
    }

    ///
    /// Alias for 'routine'.
    ///

    pub fn ro< F, S, P >( self, callback: F ) -> Self
      where
        F : Fn( Args< S, P > ) -> Result< () > + 'static,
        S : Subject,
        P : Properties,
    {
      self.routine( callback )
    }
  }

  impl Routine
  {
    ///
    /// Create new routine.
    ///

    pub fn new< F, S, P >( callback: F ) -> Self
    where
      F : Fn( Args< S, P > ) -> Result< () > + 'static,
      S : Subject,
      P : Properties,
    {
      let callback = move | instruction: &Instruction |
      {
        let subject = S::parse( &instruction.subject )?;
        let properties = P::parse( &instruction.properties_map )?;

        callback( Args { subject, properties } )
      };

      Routine::WithoutContext( Rc::new( callback ) )
    }

    ///
    /// Create new routine with context.
    ///

    pub fn new_with_ctx< F, S, P >( callback: F ) -> Self
    where
      F : Fn( Args< S, P >, Context ) -> Result< () > + 'static,
      S : Subject,
      P : Properties,
    {
      let callback = move | instruction: &Instruction, context : Context |
      {
        let subject = S::parse( &instruction.subject )?;
        let properties = P::parse( &instruction.properties_map )?;

        callback( Args { subject, properties }, context )
      };

      Routine::WithContext( Rc::new( callback ) )
    }

    /// Perform callback.
    pub fn perform( &self, instruction: &Instruction, context : Option< Context > ) -> Result< () >
    {
      match self
      {
        Routine::WithContext( func ) if context.is_some() => ( func )( instruction, context.unwrap() ),
        Routine::WithoutContext( func ) => ( func )( instruction ),
        _ => Err( BasicError::new( "Can not perform" ) )
      }
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

  impl< S, P > std::fmt::Debug for Args< S, P >
  where
    S: std::fmt::Debug,
    P: std::fmt::Debug,
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      f.debug_struct( "Args" )
      .field( "properties", &self.properties )
      .field( "subject", &self.subject )
      .finish()
    }
  }

  impl Properties for NoProperties
  {
    fn parse( properties : &HashMap< String, OpType< String > > ) -> Result< Self >
    {
      if properties.is_empty()
      {
        Ok( NoProperties )
      }
      else
      {
        Err( BasicError::new( "Non empty properties map" ) )
      }
    }
  }

  impl Subject for NoSubject
  {
    fn parse( input : impl AsRef< str > ) -> Result< Self >
    {
      if input.as_ref().is_empty()
      {
        Ok( NoSubject )
      }
      else
      {
        Err( BasicError::new( "Non empty subject" ) )
      }
    }
  }

  impl Subject for String
  {
    fn parse( input : impl AsRef< str > ) -> Result< Self >
    {
      Ok( input.as_ref().to_string() )
    }
  }
}

//

crate::mod_interface!
{
  prelude use Routine;
  prelude use Command;
  prelude use CommandFormer;
  prelude use Args;
  exposed use NoProperties;
  exposed use NoSubject;
  exposed use Properties;
  exposed use Subject;
}

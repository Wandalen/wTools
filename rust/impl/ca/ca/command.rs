pub( crate ) mod private
{
  use std::
  {
    collections::HashMap,
    rc::Rc,
    fmt,
  };
  use wtools::
  {
    error::{ Result, BasicError },
    meta::Former,
  };

  ///
  /// Handle for command routine.
  ///

  pub struct OnCommand( Option< Rc< dyn Fn( &crate::instruction::Instruction ) -> Result< () > > > );

  impl OnCommand
  {
    /// Checks that OnCommand has callback to call.
    pub fn callable( &self ) -> bool
    {
      if self.0.is_some()
      {
        true
      }
      else
      {
        false
      }
    }
    /// Perform callback.
    pub fn perform( &self, instruction : &crate::instruction::Instruction ) -> Result< () >
    {
      if self.0.is_some()
      {
        let r = self.0.as_ref().unwrap();
        return r( instruction );
      }

      Ok( () )
    }
  }

  impl Default for OnCommand
  {
    fn default() -> Self
    {
      Self ( Option::None )
    }
  }

  impl< T > From< &'static T > for OnCommand
  where
    T : Fn( &crate::instruction::Instruction ) -> Result< () >
  {
    fn from( src : &'static T ) -> Self
    {
      OnCommand( Some( Rc::new( src ) ) )
    }
  }

  impl Clone for OnCommand
  {
    fn clone( &self ) -> Self
    {
      match self
      {
        OnCommand ( Option::None ) => Self ( None ),
        OnCommand ( Option::Some( boxed ) ) => Self ( Some( boxed.clone() ) ),
      }
    }
  }

  impl fmt::Debug for OnCommand
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      match self
      {
        OnCommand ( Option::None ) => f.write_str( "None" ),
        OnCommand ( Option::Some( _rc ) ) => f.write_str( "OnCommand" ),
      }
    }
  }

  impl PartialEq for OnCommand
  {
    fn eq( &self, other : &Self ) -> bool
    {
      // We can't compare closures. Because every closure has a separate type, even if they're identical.
      // Therefore, we check that the two Rc's point to the same closure (allocation).
      if let ( Some( this_rc ), Some( other_rc ) ) = ( &self.0, &other.0 )
      {
        Rc::ptr_eq( this_rc, other_rc )
      }
      else
      {
        self.0.is_none() && other.0.is_none()
      }
    }
  }

  ///
  /// Command descriptor.
  ///

  #[ derive( Debug, Clone, PartialEq ) ]
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
    #[ alias( ro ) ]
    pub routine : OnCommand,
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
  }

  impl Command
  {
    /// Generate short help for command.
    pub fn help_short( &self ) -> String
    {
      format!( ".{} - {}", self.phrase.replace( " ", "." ), self.hint )
    }

    /// Generate short help for command.
    pub fn help_long( &self ) -> String
    {
      let properties_hints = self.properties_hints.iter().map( | ( key, value ) | format!( "  {} - {}", key, value ) ).collect::< Vec< _ > >();
      let properties_hints = properties_hints.join( "\n" );
      format!( ".{} - {}\n{}", self.phrase.replace( " ", "." ), self.long_hint, properties_hints )
    }

    /// Execute callback.
    pub fn perform( &self, instruction : &crate::instruction::Instruction ) -> Result< () >
    {
      if self.subject_hint.len() == 0 && instruction.subject.len() != 0
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
      if self.routine.callable()
      {
        return self.routine.perform( instruction );
      }

      Ok( () )
    }
  }
}

//

crate::mod_interface!
{
  prelude use OnCommand;
  prelude use Command;
  prelude use CommandFormer;
}

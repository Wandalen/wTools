
pub( crate ) mod private
{
  use std::collections::HashMap;
  use std::rc::Rc;
  use core::fmt;
  use crate::*;
  use wtools::error::{ Result, BasicError };

  // qqq : for Dima : adjust formatting /* aaa : Dmytro : adjusted */

  ///
  /// Handle for command routine.
  ///

  // qqq : for Dima : make alias for Result with BasicError /* aaa : Dmytro : done */
  pub struct OnCommand( Option< Rc< dyn Fn( &instruction::Instruction ) -> Result< () > > > );

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
    pub fn perform( &self, instruction : &instruction::Instruction ) -> Result< () >
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

  impl From<&'static dyn Fn( &instruction::Instruction ) -> Result< () >> for OnCommand
  {
    fn from( src : &'static dyn Fn( &instruction::Instruction ) -> Result< () > ) -> Self
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
    fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      match self
      {
        OnCommand ( Option::None ) => f.write_str( "None" ),
        OnCommand ( Option::Some( _rc ) ) => f.write_str( "OnCommand" ),
      }
    }
  }

  ///
  /// Command descriptor.
  ///

  #[derive( Default, Debug, Clone )]
  pub struct Command
  {
    /// Command common hint.
    pub hint : String,
    /// Command full hint.
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
    pub routine : OnCommand,
  }

  impl PartialEq for Command
  {
    /* qqq : for Dmytro : extend */
    fn eq( &self, other: &Self ) -> bool
    {
      self.hint == other.hint
      && self.long_hint == other.long_hint
      && self.subject_hint == other.subject_hint
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
    pub fn perform( &self, instruction : &instruction::Instruction ) -> Result< () >
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

  ///
  /// Options for command.
  ///

  #[derive( Debug, Clone, Default )]
  pub struct CommandOptions
  {
    ins : Command,
  }

  //

  // ro : null,
  // h : null,
  // lh : null,

  impl CommandOptions
  {
    field_str!{ hint }
    field_str!{ hint, h }
    field_str!{ long_hint }
    field_str!{ long_hint, lh }
    field_str!{ phrase }
    field_str!{ subject_hint }
    field_str!{ subject_hint, sh }
    field_map_str_str!{ properties_hints, property_hint }
    field_map_str_vec_str!{ properties_aliases, property_alias }
    field_routine!{ routine }
    field_routine!{ routine, ro }

    /// Command former.
    pub fn form( &self ) -> Command
    {
      self.ins.clone()
    }

  }
}

//

wtools::meta::mod_interface!
{
  prelude use OnCommand;
  prelude use Command;
  prelude use CommandOptions;
}

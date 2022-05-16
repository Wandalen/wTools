
mod internal
{
  use std::collections::HashMap;
  use std::rc::Rc;
  use std::fmt;
  use crate::*;
  use wtools::error::Error;

  //

  ///
  /// Handle for command routine.
  ///

  pub struct OnCommand( Option<Rc<dyn Fn( &instruction::Instruction ) -> Result<(), Error>>> );

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
    pub fn perform( &self, instruction : &instruction::Instruction ) -> Result<(), Error>
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

  impl From<&'static dyn Fn( &instruction::Instruction ) -> Result<(), Error>> for OnCommand
  {
    fn from( src : &'static dyn Fn( &instruction::Instruction ) -> Result<(), Error> ) -> Self
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

  impl Command
  {
    /// Execute callback.
    pub fn perform( &self, instruction : &instruction::Instruction ) -> Result<(), Error>
    {
      if self.subject_hint.len() == 0 && instruction.subject.len() != 0
      {
        return Err( Error::new( "Unexpected subject." ) );
      }

      for ( key, _value ) in &instruction.properties_map
      {
        if self.properties_hints.get( key.as_str() ).is_none()
        {
          return Err( Error::new( "Unknown option." ) );
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

    // pub fn hint< Str : AsRef< str > >( &mut self, src : Str ) -> &mut Self
    // where
    //   String : From<Str>
    // {
    //   self.ins.hint = src.into();
    //   self
    // }
    // pub fn long_hint< Str : AsRef< str > >( &mut self, src : Str ) -> &mut Self
    // where
    //   String : From<Str>
    // {
    //   self.ins.long_hint = src.into();
    //   self
    // }
    // pub fn phrase< Str : AsRef< str > >( &mut self, src : Str ) -> &mut Self
    // where
    //   String : From<Str>
    // {
    //   self.ins.phrase = src.into();
    //   self
    // }
    // pub fn subject_hint< Str : AsRef< str > >( &mut self, src : Str ) -> &mut Self
    // where
    //   String : From<Str>
    // {
    //   self.ins.subject_hint = src.into();
    //   self
    // }
    // pub fn property_hint< Str : AsRef< str > >( &mut self, property : Str, hint : Str ) -> &mut Self
    // where
    //   String : From<Str>
    // {
    //   self.ins.properties_hints.insert( property.into(), hint.into() );
    //   self
    // }
    // pub fn property_alias< Str : AsRef< str > >( &mut self, property : Str, alias : Str ) -> &mut Self
    // where
    //   String : From<Str>
    // {
    //   let entry = self.ins.properties_aliases.entry( property.into() ).or_insert_with( || -> Vec< String > { vec![] } );
    //   entry.push( alias.into() );
    //   self
    // }
    //
    // pub fn routine( &mut self, routine : &'static dyn Fn() ) -> &mut Self
    // {
    //   self.ins.routine = routine.into();
    //   self
    // }

    /// Command former.
    pub fn form( &self ) -> Command
    {
      self.ins.clone()
    }

  }

  // impl Options for CommandOptions
  // {
  //   type For = Command;
  // }
}

/// Protected namespace of the module.
pub mod protected
{
  use super::internal as i;

  pub use i::OnCommand;
  pub use i::Command;
  pub use i::CommandOptions;
}

pub use protected::*;

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;

  pub use i::OnCommand;
  pub use i::Command;
  pub use i::CommandOptions;
}

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  use super::internal as i;

  pub use i::OnCommand;
  pub use i::Command;
  pub use i::CommandOptions;
}


pub( crate ) mod private
{
  use crate::
  {
    Program, Namespace,

    GrammarCommand, ExecutableCommand,

    Routine, wtools,
  };

  use former::Former;
  use std::collections::HashMap;
  use wtools::{ error::Result, err };

  /// This is the struct that provides a way to convert a `GrammarCommand` to an `ExecutableCommand`.
  ///
  /// The conversion is done by looking up the `Routine` associated with the command in a HashMap of routines.
  ///
  /// ```
  /// # use wca::{ Command, Type, GrammarCommand, ExecutorConverter, Routine };
  /// # use std::collections::HashMap;
  /// # fn main() -> Result< (), Box< dyn std::error::Error > > {
  /// let executor_converter = ExecutorConverter::former()
  /// .routine( "command", Routine::new( |( args, props )| Ok( () ) ) )
  /// .form();
  ///
  /// let grammar_command = GrammarCommand
  /// {
  ///   phrase : "command".to_string(),
  ///   subjects : vec![],
  ///   properties : HashMap::new(),
  /// };
  ///
  /// let executable_command = executor_converter.to_command( grammar_command )?;
  /// # Ok( () ) }
  /// ```
  #[ derive( Debug ) ]
  #[ derive( Former ) ]
  pub struct ExecutorConverter
  {
    pub( crate ) routines : HashMap< String, Routine >,
  }

  impl ExecutorConverterFormer
  {
    /// Inserts routine to a routine dictionary
    pub fn routine< S >( mut self, phrase : S, routine : Routine ) -> Self
    where
      S : Into< String >,
      Routine : Into< Routine >
    {
      let mut routines = self.routines.unwrap_or_default();

      routines.insert( phrase.into(), routine );

      self.routines = Some( routines );
      self
    }
  }

  impl ExecutorConverter
  {
    /// Converts raw program to executable
    pub fn to_program( &self, raw_program : Program< Namespace< GrammarCommand > > ) -> Result< Program< Namespace< ExecutableCommand > > >
    {
      let namespaces = raw_program.namespaces
      .into_iter()
      .map( | n | self.to_namespace( n ) )
      .collect::< Result< Vec< Namespace< ExecutableCommand > > > >()?;

      Ok( Program { namespaces } )
    }

    /// Converts raw namespace to executable
    pub fn to_namespace( &self, raw_namespace : Namespace< GrammarCommand > ) -> Result< Namespace< ExecutableCommand > >
    {
      let commands = raw_namespace.commands
      .into_iter()
      .map( | c | self.to_command( c ) )
      .collect::< Result< Vec< ExecutableCommand > > >()?;

      Ok( Namespace { commands } )
    }

    /// Converts raw command to executable
    pub fn to_command( &self, command : GrammarCommand ) -> Result< ExecutableCommand >
    {
      self.routines
      .get( &command.phrase )
      .ok_or_else( || err!( "Can not found routine for command `{}`", command.phrase ) )
      .map(
        | routine |
        ExecutableCommand
        {
          subjects : command.subjects,
          properties : command.properties,
          routine : routine.clone(),
        }
      )
    }
  }
}

//

crate::mod_interface!
{
  prelude use ExecutorConverter;
}

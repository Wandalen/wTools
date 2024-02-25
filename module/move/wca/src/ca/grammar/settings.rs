pub( crate ) mod private
{
  use crate::Type;

  use std::collections::HashMap;
  use former::Former;

  /// A description of a Value in a command. Used to specify the expected type and provide a hint for the Value.
  ///
  /// This struct is used to describe a command's subject or property and validate it against the expected type. It contains a hint
  /// string that provides guidance to the user for entering a valid value, and a `Type` enum value that represents the expected
  /// type of the value.
  ///
  /// # Examples:
  ///
  /// ```
  /// # use wca::{ Type, grammar::settings::ValueDescription };
  /// let value_desc = ValueDescription { kind: Type::String, hint: "Enter your name".to_string(), optional: false };
  /// ```
  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  pub struct ValueDescription
  {
    /// providing guidance to the user for entering a valid value
    pub hint : String,
    /// expected type of a value
    pub kind : Type,
    /// subject optional parameter
    pub optional : bool,
  }

  /// Command descriptor.
  ///
  /// Based on this structure, the structure( `ParsedCommand` ) obtained after parsing will be validated and converted to `GrammarCommand`.
  ///
  /// # Example:
  ///
  /// ```
  /// # use wca::{ Command, Type };
  /// let command = Command::former()
  /// .hint( "hint" )
  /// .long_hint( "long_hint" )
  /// .phrase( "command" )
  /// .subject( "subject", Type::String, false )
  /// .form();
  /// ```

  #[ derive( Debug, Clone, PartialEq, Eq ) ]
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
    /// Command subjects hints and types.
    pub subjects : Vec< ValueDescription >,
    /// Hints and types for command options.
    pub properties : HashMap< String, ValueDescription >,
    /// Map of aliases.
    // Aliased key -> Original key
    pub properties_aliases : HashMap< String, String >,
  }

  impl CommandFormer
  {
    /// Setter for separate properties.
    pub fn subject< S : Into< String > >( mut self, hint : S, kind : Type, optional : bool ) -> Self
    {
      let hint = hint.into();
      let subject = ValueDescription { hint, kind, optional };

      let mut subjects = self.subjects.unwrap_or_default();

      subjects.push( subject );

      self.subjects = Some( subjects );
      self
    }

    /// Setter for separate properties.
    pub fn property< S : AsRef< str >, H : Into< String > >( mut self, key : S, hint : H, kind : Type, optional : bool ) -> Self
    {
      let key = key.as_ref();
      let hint = hint.into();
      let property = ValueDescription { hint, kind, optional };

      let mut properties = self.properties.unwrap_or_default();
      let properties_aliases = self.properties_aliases.unwrap_or_default();
      debug_assert!( !properties.contains_key( key ), "Property name `{key}` is already used for `{:?}`", properties[ key ] );
      debug_assert!( !properties_aliases.contains_key( key ), "Name `{key}` is already used for `{}` as alias", properties_aliases[ key ] );

      properties.insert( key.into(), property );

      self.properties = Some( properties );
      self.properties_aliases = Some( properties_aliases );
      self
    }

    // Setter for separate properties aliases.
    pub fn property_alias< S : Into< String > >( mut self, key : S, alias : S ) -> Self
    {
      let key = key.into();
      let alias = alias.into();
      let properties = self.properties.unwrap_or_default();
      let mut properties_aliases = self.properties_aliases.unwrap_or_default();
      debug_assert!( !properties.contains_key( &alias ), "Name `{key}` is already used for `{:?} as property name`", properties[ &alias ] );
      debug_assert!( !properties_aliases.contains_key( &alias ), "Alias `{alias}` is already used for `{}`", properties_aliases[ &alias ] );

      properties_aliases.insert( alias, key );

      self.properties = Some( properties );
      self.properties_aliases = Some( properties_aliases );
      self
    }
  }
}

//

crate::mod_interface!
{
  exposed use Command;
  protected use ValueDescription;
}

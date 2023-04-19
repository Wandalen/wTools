pub( crate ) mod private
{
  use crate::Type;

  use wtools::{ HashMap, Former };

  /// Command subject description
  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  pub struct ValueDescription
  {
    /// subject hint
    pub hint : String,
    /// subject type
    pub kind : Type,
  }

  ///
  /// Command descriptor.
  ///

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
    pub fn subject< S : Into< String > >( mut self, hint : S, kind : Type ) -> Self
    {
      let hint = hint.into();
      let subject = ValueDescription { hint, kind };

      let mut subjects = self.subjects.unwrap_or_default();

      subjects.push( subject );

      self.subjects = Some( subjects );
      self
    }

    /// Setter for separate properties.
    pub fn property< S : AsRef< str >, H : Into< String > >( mut self, key : S, hint : H, kind : Type ) -> Self
    {
      let key = key.as_ref();
      let hint = hint.into();
      let property = ValueDescription { hint, kind };

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
  prelude use Command;
  protected use ValueDescription;
}

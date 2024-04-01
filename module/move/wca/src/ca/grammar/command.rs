pub( crate ) mod private
{
  use crate::*;

  use { Handler, Routine, Type };

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
  /// # use wca::{ Type, ca::grammar::command::ValueDescription };
  /// let value_desc = ValueDescription { kind: Type::String, hint: "Enter your name".to_string(), optional: false };
  /// ```
  #[ derive( Debug, Clone, PartialEq, Eq, Former ) ]
  pub struct ValueDescription
  {
    /// providing guidance to the user for entering a valid value
    pub hint : String,
    /// expected type of a value
    pub kind : Type,
    /// subject optional parameter
    #[ default( false ) ]
    pub optional : bool,
  }

  #[ derive( Debug, Former ) ]
  pub struct PropertyDescription
  {
    name : String,
    // qqq : how to re-use ValueDescriptionFormer without additional end?
    // value : ValueDescription,
    /// providing guidance to the user for entering a valid value
    hint : String,
    /// expected type of a value
    kind : Type,
    /// subject optional parameter
    #[ default( false ) ]
    optional : bool,
    #[ setter( false ) ]
    #[ default( Vec::new() ) ]
    properties_aliases : Vec< String >,
  }

  impl< C, End > PropertyDescriptionFormer< C, End >
  where
    End : former::FormingEnd< PropertyDescription, C >,
  {
    pub fn alias< IntoName >( mut self, name : IntoName ) -> Self
    where
      IntoName : Into< String >,
    {
      let mut aliases = self.storage.properties_aliases.unwrap_or_default();
      aliases.push( name.into() );
      self.storage.properties_aliases = Some( aliases );

      self
    }
  }


  /// Command descriptor.
  ///
  /// Based on this structure, the structure( `ParsedCommand` ) obtained after parsing will be validated and converted to `VerifiedCommand`.
  ///
  /// # Example:
  ///
  /// ```
  /// # use wca::{ Command, Type };
  /// let command = Command::former()
  /// .hint( "hint" )
  /// .long_hint( "long_hint" )
  /// .phrase( "command" )
  /// .subject()
  ///   .kind( Type::String )
  ///   .end()
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
    // qqq : for Bohdan : routine should also be here
    // aaa : here it is
    // qqq : make it usable and remove default(?)
    /// The type `Routine` represents the specific implementation of the routine.
    #[ setter( false ) ]
    #[ default( Routine::from( Handler::from( || { panic!( "No routine available: A handler function for the command is missing" ) } ) ) ) ]
    pub routine : Routine,
  }

  impl< Context, End > CommandFormer< Context, End >
  where
    End : former::FormingEnd< Command, Context >,
  {
    /// Setter for separate properties aliases.
    pub fn property_alias< S : Into< String > >( mut self, key : S, alias : S ) -> Self
    {
      let key = key.into();
      let alias = alias.into();
      let properties = self.storage.properties.unwrap_or_default();
      let mut properties_aliases = self.storage.properties_aliases.unwrap_or_default();
      debug_assert!( !properties.contains_key( &alias ), "Name `{key}` is already used for `{:?} as property name`", properties[ &alias ] );
      debug_assert!( !properties_aliases.contains_key( &alias ), "Alias `{alias}` is already used for `{}`", properties_aliases[ &alias ] );

      properties_aliases.insert( alias, key );

      self.storage.properties = Some( properties );
      self.storage.properties_aliases = Some( properties_aliases );
      self
    }

    /// Sets the command routine.
    ///
    /// You can set the following types of command routines:
    /// - `fn()`: A command routine without any argument or property.
    /// - `fn(args)`: A command routine with arguments.
    /// - `fn(props)`: A command routine with properties.
    /// - `fn(args, props)`: A command routine with arguments and properties.
    /// - `fn(context)`: A command routine with a context.
    /// - `fn(context, args)`: A command routine with a context and arguments.
    /// - `fn(context, props)`: A command routine with a context and properties.
    /// - `fn(context, args, props)`: A command routine with a context, arguments, and properties.
    ///
    /// # Type Parameters
    ///
    /// * `I`: The input type for the handler function.
    /// * `R`: The return type for the handler function.
    /// * `F`: The function type that can be converted into a handler.
    ///
    /// # Parameters
    ///
    /// * `self`: The current `CommandFormer` instance. This instance will be consumed by this method.
    /// * `f`: The function that will be set as the command routine.
    ///
    /// # Returns
    ///
    /// Returns the `CommandFormer` instance with the new command routine set.
    pub fn routine< I, R, F : Into< Handler< I, R > > >( mut self, f : F ) -> Self
    where
      Routine: From< Handler< I, R > >,
    {
      let h = f.into();
      self.storage.routine = Some( h.into() );
      self
    }
  }

  impl< Context, End > CommandFormer< Context, End >
  where
    End : former::FormingEnd< Command, Context >,
  {
    /// Implements the `subject` method for a value.
    ///
    /// This method allows chaining, where `subject` is the current value and `ValueDescription` is the super-former.
    /// It returns a `ValueDescriptionFormer` which can be used to further build the super-former.
    pub fn subject( self ) -> ValueDescriptionFormer< Self, impl former::FormingEnd< ValueDescription, Self > >
    {
      let on_end = | subject : ValueDescription, super_former : Option< Self > | -> Self
      {
        let mut super_former = super_former.unwrap();
        let mut subjects = super_former.storage.subjects.unwrap_or_default();
        subjects.push( subject );

        super_former.storage.subjects = Some( subjects );

        super_former
      };
      ValueDescriptionFormer::begin( None, Some( self ), on_end )
    }

    /// Sets the name and other properties of the current property.
    ///
    /// This method takes ownership of `self` and the name of the property as input.
    /// It returns a `PropertyDescriptionFormer` instance that allows chaining of different property
    /// descriptions.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the property. It should implement the `Into< String >` trait.
    pub fn property< IntoName >( self, name : IntoName ) -> PropertyDescriptionFormer< Self, impl former::FormingEnd< PropertyDescription, Self > >
    where
      IntoName : Into< String >,
    {
      let on_end = | property : PropertyDescription, super_former : Option< Self > | -> Self
      {
        let mut super_former = super_former.unwrap();
        let mut properties = super_former.storage.properties.unwrap_or_default();
        let value = ValueDescription
        {
          hint : property.hint,
          kind : property.kind,
          optional : property.optional,
        };
        debug_assert!( !properties.contains_key( &property.name ), "Property name `{}` is already used for `{:?}`", property.name, properties[ &property.name ] );
        properties.insert( property.name.clone(), value );

        let mut aliases = super_former.storage.properties_aliases.unwrap_or_default();
        debug_assert!( !aliases.contains_key( &property.name ), "Name `{}` is already used for `{}` as alias", property.name, aliases[ &property.name ] );

        aliases.extend( property.properties_aliases.into_iter().map( | alias | ( alias, property.name.clone() ) ) );

        super_former.storage.properties = Some( properties );
        super_former.storage.properties_aliases = Some( aliases );

        super_former
      };
      let former = PropertyDescriptionFormer::begin( None, Some( self ), on_end );
      former.name( name )
    }
  }
}

//

crate::mod_interface!
{
  exposed use Command;
  exposed use CommandFormer;
  protected use ValueDescription;
}

// qqq : use orphan instead of exposed for ALL files in the folder, dont use prelude for structs
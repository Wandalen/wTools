pub( crate ) mod private
{
  use crate::*;

  use std::collections::{ HashMap };
  use indexmap::IndexMap;
  use former::{ Former, StoragePreform };
  use iter_tools::Itertools;

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
    #[ former( default = false ) ]
    pub optional : bool,
  }

  #[ derive( Debug, Former ) ]
  pub struct PropertyDescription
  {
    name : String,
    // qqq : how to re-use ValueDescriptionFormer without additional end?
    // #[subform_scalar]
    // value : ValueDescription,
    /// providing guidance to the user for entering a valid value
    hint : String,
    /// expected type of a value
    kind : Type,
    /// subject optional parameter
    #[ former( default = false ) ]
    optional : bool,
    #[ scalar( setter = false ) ]
    #[ former( default = Vec::new() ) ]
    properties_aliases : Vec< String >,
  }

  impl< Definition > PropertyDescriptionFormer< Definition >
  where
    Definition : former::FormerDefinition< Storage = < PropertyDescription as former::EntityToStorage >::Storage >,
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
    pub hint : String,
    /// Command full hint.
    pub long_hint : String,
    /// Phrase descriptor for command.
    pub phrase : String,
    /// Command subjects hints and types.
    #[ subform_entry( setter = true ) ]
    pub subjects : Vec< ValueDescription >,
    /// Hints and types for command options.
    pub properties : IndexMap< String, ValueDescription >,
    /// Map of aliases.
    // Aliased key -> Original key
    pub properties_aliases : HashMap< String, String >,
    // qqq : make it usable and remove default(?)
    /// The type `Routine` represents the specific implementation of the routine.
    #[ scalar( setter = false ) ]
    #[ former( default = Routine::from( Handler::from( || { panic!( "No routine available: A handler function for the command is missing" ) } ) ) ) ]
    pub routine : Routine,
  }

  impl Command
  {
    pub( crate ) fn properties( &self, order : Order ) -> Vec< ( &String, &ValueDescription ) >
    {
      match order
      {
        Order::Nature =>
        {
          self.properties.iter().map( | ( key, value ) | ( key, value ) ).collect()
        }
        Order::Lexicography =>
        {
          self.properties.iter().map( | ( key, value ) | ( key, value ) ).sorted_by_key( | ( k, _ ) | *k ).collect()
        }
      }
    }
  }

  impl< Definition > CommandFormer< Definition >
  where
    Definition : former::FormerDefinition< Storage = < Command as former::EntityToStorage >::Storage >,
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

  impl< Definition > CommandFormer< Definition >
  where
    Definition : former::FormerDefinition< Storage = < Command as former::EntityToStorage >::Storage >,
  {
    /// Implements the `subject` method for a value.
    ///
    /// This method allows chaining, where `subject` is the current value and `ValueDescription` is the super-former.
    /// It returns a `ValueDescriptionFormer` which can be used to further build the super-former.
    pub fn subject( self ) -> ValueDescriptionAsSubformer< Self, impl ValueDescriptionAsSubformerEnd< Self > >
    {
      self._subjects_subform_entry()
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
    pub fn property< IntoName >( self, name : IntoName ) -> PropertyDescriptionAsSubformer< Self, impl PropertyDescriptionAsSubformerEnd< Self > >
    where
      IntoName : Into< String >,
    {
      let on_end = | property : PropertyDescriptionFormerStorage, super_former : Option< Self > | -> Self
      {
        let mut super_former = super_former.unwrap();
        let mut properties = super_former.storage.properties.unwrap_or_default();
        let property = property.preform();

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
  own use ValueDescription;
}

// qqq : use orphan instead of exposed for ALL files in the folder, dont use prelude for structs
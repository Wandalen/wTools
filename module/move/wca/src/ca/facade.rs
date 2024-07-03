// pub( crate ) mod private
// {
//   use crate::*;
//   use core::fmt;
//   use ca::grammar;
//
//   /// Macro for parsing WCA arguments.
//   ///
//   /// # Examples
//   /// ```rust
//   /// use wca::Value;
//   ///
//   /// let mut args = vec![ Value::Number( 42. ), Value::String( "Rust".into() ) ].into_iter();
//   /// wca::parse_args!( args, n : f64, name : String );
//   ///
//   /// assert_eq!( n, 42. );
//   /// assert_eq!( name, "Rust" );
//   /// ```
//   #[macro_export]
//   macro_rules! parse_args
//   {
//     ( $args : ident, mut $b : ident : $ty : ident $( $rest : tt )* ) =>
//     {
//       let mut $b : $ty = std::convert::TryFrom::try_from( $args.next().unwrap() ).unwrap();
//       $crate::parse_args!( $args $( $rest )* )
//     };
//     ( $args : ident, $b : ident : $ty : ident $( $rest : tt )* ) =>
//     {
//       let $b : $ty = std::convert::TryFrom::try_from( $args.next().unwrap() ).unwrap();
//       $crate::parse_args!( $args $( $rest )* )
//     };
//     ( $args : ident, $b : ident $( $rest : tt )* ) =>
//     {
//       let $b = $args.next().unwrap();
//       $crate::parse_args!( $args $( $rest )* )
//     };
//     ( $args : ident, mut $b : ident $( $rest : tt )* ) =>
//     {
//       let mut $b = $args.next().unwrap();
//       $crate::parse_args!( $args $( $rest )* )
//     };
//     ( $args : ident ) =>
//     {
//       assert!( $args.next().is_none() );
//     };
//     ( $args : ident, ) =>
//     {
//       $crate::parse_args!( $args )
//     };
//   }
//
//   /// Creates a command-line interface (CLI) builder with the given initial state.
//   ///
//   /// This function initializes a `CommandBuilder` with the provided `state` and
//   /// returns it for further configuration of the CLI.
//   pub fn cui< T >( state : T ) -> CommandBuilder< T >
//   {
//     CommandBuilder::with_state( state )
//   }
//
//   /// A struct representing a property.
//   #[ derive( Debug, Clone ) ]
//   pub struct Property< 'a >
//   {
//     /// The name of the property.
//     pub name : &'a str,
//     /// The hint for the property.
//     pub debug : &'a str,
//     /// The tag representing the property's type.
//     pub tag : Type,
//   }
//
//   impl< 'a > Property< 'a >
//   {
//     /// Constructor of a property.
//     pub fn new( name : &'a str, hint : &'a str, tag : Type ) -> Self { Self { name, hint, tag } }
//   }
//
//   /// A builder struct for constructing commands.
//   #[ derive( Debug ) ]
//   pub struct CommandBuilder< T >
//   {
//     state : T,
//     commands : Vec< Command >,
//     handlers : std::collections::HashMap< String, Routine >,
//   }
//
//   impl< T > CommandBuilder< T >
//   {
//     /// Constructs a `CommandBuilder` with the given state.
//     pub fn with_state( state : T ) -> Self
//     {
//       Self { state, handlers : < _ >::default(), commands : vec![] }
//     }
//   }
//
//   #[ derive( Debug ) ]
//   pub struct Builder< F >
//   {
//     handler : F,
//     command : Command,
//   }
//
//   impl< F > Builder< F >
//   {
//     /// Creates a new instance of the command with the provided handler function.
//     ///
//     /// This method takes in a handler function `handler` and creates a new instance of the command.
//     /// The `handler` function is used to handle the execution logic associated with the command.
//     ///
//     /// # Arguments
//     ///
//     /// * `handler` - The handler function that will be invoked when the command is executed.
//     ///
//     /// # Returns
//     ///
//     /// A new instance of the command with the specified `handler`.
//     ///
//     #[ inline ]
//     pub fn new( handler: F ) -> Self
//     {
//       let name =
//       {
//         use iter_tools::Itertools as _;
//
//         let name = std::any::type_name::< F >();
//         let name = name.split("::").last().unwrap();
//         name.split( '_' ).join( "." )
//       };
//
//       Self { handler, command : Command::former().phrase( name ).form() }
//     }
//
//     /// Adds an argument to the command.
//     ///
//     /// This method takes in the `hint` and `tag` parameters to create a `ValueDescription` object
//     /// representing an argument. The `ValueDescription` object is then appended to the command's
//     /// `subjects` collection.
//     ///
//     /// # Arguments
//     ///
//     /// * `hint` - The hint for the argument, represented as a string slice (`&str`).
//     /// * `tag` - The type of the argument, represented by a `Type` object from the `Type` module.
//     ///
//     /// # Returns
//     ///
//     /// The modified command instance with the argument added.
//     ///
//     #[ inline ]
//     pub fn arg( mut self, hint : &str, tag : Type ) -> Self
//     {
//       self.command.subjects.push( grammar::command::ValueDescription
//       {
//         hint : hint.into(),
//         kind : tag,
//         optional : false,
//       });
//
//       self
//     }
//
//     /// Adds a property to the command.
//     ///
//     /// This method takes in the `name`, `hint`, and `kind` parameters to create a `ValueDescription`
//     /// object representing a property. The `ValueDescription` object is then inserted into the
//     /// command's properties collection using the `name` as the key.
//     ///
//     /// # Example
//     /// ```no_rust
//     /// let ca = cui(())
//     ///   .command(user.property("name", "Name property", Type::String))
//     ///   .build();
//     /// ```
//     ///
//     /// # Arguments
//     ///
//     /// * `name` - The name of the property. It should implement the `ToString` trait.
//     /// * `hint` - The hint for the property. It should implement the `ToString` trait.
//     /// * `kind` - The type of the property, represented by a `Type` object from the `Type` module.
//     ///
//     /// # Returns
//     ///
//     /// The modified command instance with the property added.
//     ///
//     #[ inline ]
//     pub fn property( mut self, name : impl ToString , hint : impl ToString, kind : Type ) -> Self
//     {
//       self.command.properties.insert
//       (
//         name.to_string(),
//         grammar::command::ValueDescription
//         {
//           hint : hint.to_string(),
//           kind,
//           optional : false,
//         }
//       );
//
//       self
//     }
//
//     /// Adds multiple properties to the command.
//     ///
//     /// This method takes in an array of `Property` objects and adds them to the command's properties.
//     /// The properties are provided in the `properties` parameter as an array of length `N`.
//     ///
//     /// ```without_std
//     /// let ca = cui(())
//     ///   .properties([
//     ///      Property::new("name", "Name property", Type::String),
//     ///      Property::new("age", "Age property", Type::Integer),
//     ///   ]).build();
//     /// ```
//     ///
//     /// # Arguments
//     ///
//     /// * `properties` - An array of `Property` objects representing the properties to be added.
//     ///
//     /// # Returns
//     ///
//     /// The modified command instance with the properties added.
//     ///
//     #[ inline ]
//     pub fn properties< const N: usize >( mut self, properties : [ Property< '_ >; N ] ) -> Self
//     {
//       self.command.properties.reserve( properties.len() );
//
//       for Property { name, hint, tag } in properties
//       {
//         self = self.property(name, hint, tag);
//       }
//
//       self
//     }
//   }
//
//   impl< T: Clone + 'static > CommandBuilder< T >
//   {
//     /// Adds a command to the `CommandBuilder`.
//     /// ```no_rust
//     /// let ca = cui( () ) // Add commands using the builder pattern
//     /// .command( command )
//     /// .command( command2 )
//     /// .command( echo.arg("string", Type::String ) ) // Customize your commands by chaining methods such as properties
//     ///                                            // property, and arg to add properties and arguments.
//     /// .build();
//     ///
//     /// ```
//     pub fn command< F, E >
//     (
//       mut self,
//       command : impl IntoBuilder< F, T >,
//     ) -> Self
//     where
//       F : Fn( T, Args, Props ) -> Result< (), E > + 'static + Copy,
//       E : fmt::Debug,
//     {
//       let Builder { handler, command } = command.into_builder();
//       let state = self.state.clone();
//
//       let closure = closure::closure!( | ( args, props ) |
//       {
//         handler( state.clone(), args, props )
//         .map_err( | report | BasicError::new( format!( "{report:?}" ) ).into() )
//       });
//
//       let handler = Routine::new( closure );
//
//       self.handlers.insert( command.phrase.clone(), handler );
//       self.commands.push( command );
//
//       self
//     }
//
//     /// Builds and returns a `wca::CommandsAggregator` instance.
//     ///
//     /// This method finalizes the construction of the `CommandBuilder` by
//     /// creating a `wca::CommandsAggregator` instance with the accumulated
//     /// commands and handlers.
//     pub fn build( self ) -> CommandsAggregator
//     {
//       CommandsAggregator::former().grammar( self.commands ).executor( self.handlers ).perform()
//     }
//   }
//
//   /// An extension trait for commands.
//   ///
//   /// This trait provides additional methods for enhancing commands, such as
//   /// adding arguments and properties.
//   pub trait CommandExt< T > : Sized
//   {
//     /// Adds an argument to the command.
//     fn arg( self, hint : &str, tag : Type ) -> Builder< Self >
//     {
//       Builder::new( self ).arg( hint, tag )
//     }
//
//     /// Adds property to the command.
//     fn property< const N: usize >( self, name : impl ToString , hint : impl ToString, kind : Type ) -> Builder< Self >
//     {
//       Builder::new( self ).property( name, hint, kind )
//     }
//
//     /// Adds properties to the command.
//     fn properties< const N: usize >( self, properties: [ Property< '_ >; N ] ) -> Builder< Self >
//     {
//       Builder::new( self ).properties( properties )
//     }
//   }
//
//   impl< F: Fn( T, Args, Props ) -> Result< (), E>, T, E > CommandExt< T > for F {}
//
//   /// A trait for converting a type into a `Builder`.
//   pub trait IntoBuilder< F, T > : Sized
//   {
//     /// Converts the type into a `Builder` instance.
//     fn into_builder( self ) -> Builder< F >;
//   }
//
//   impl< F, T > IntoBuilder< F, T > for Builder< F >
//   {
//     fn into_builder( self ) -> Self
//     {
//       self
//     }
//   }
//
//   impl< F: Fn( T, Args, Props ) -> Result< (), E >, T, E > IntoBuilder< F, T > for F
//   {
//     fn into_builder( self ) -> Builder< F >
//     {
//       Builder::new( self )
//     }
//   }
//
// }
//
// crate::mod_interface!
// {
//   exposed use cui;
//   exposed use CommandBuilder;
//   exposed use Property;
//   prelude use IntoBuilder;
//   prelude use CommandExt;
// }

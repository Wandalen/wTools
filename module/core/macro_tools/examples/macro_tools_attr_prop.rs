//!
//! ### Example: Attribute Properties
//!
//! This example demonstrates an approach to parsing attributes and their properties.
//! The attributes are collected into a struct that aggregates them, and attribute properties
//! are parsed using reusable components from a library. The example shows how to use
//! `AttributePropertyBoolean` for parsing boolean properties and the roles of the traits
//! `AttributePropertyComponent` and `AttributeComponent`. The `ComponentAssign` trait is
//! also used to simplify the logic of assigning fields.
//!
//! Attributes are collected into a `StructAttributes` struct, and attribute properties are parsed
//! using reusable components like `AttributePropertyBoolean`.
//!
//! - `AttributeComponent`: A trait that defines how an attribute should be parsed from a `syn::Attribute`.
//! - `AttributePropertyComponent`: A trait that defines a marker for attribute properties.
//! - `ComponentAssign`: A trait that simplifies the logic of assigning fields to a struct. Using a
//! component-based approach requires each field to have a unique type, which aligns with the
//! strengths of strongly-typed languages. This method ensures that the logic of
//! assigning values to fields is encapsulated within the fields themselves, promoting modularity
//! and reusability.
//!
//! The reusable property components from the library come with parameters that distinguish
//! different properties of the same type. This is useful when an attribute has multiple boolean
//! properties, for instance. Such an approach helps to avoid limitations where it is
//! always possible to define traits for custom types, while it may not be possible for types
//! defined in other crates.
//!

#[ cfg( not( all( feature = "enabled", debug_assertions ) )  ) ]
fn main(){}
#[ cfg( all( feature = "enabled", debug_assertions )  ) ]
fn main()
{

  use macro_tools::
  {
    attr,
    syn_err,
    return_syn_err,
    qt,
    Result,
    AttributeComponent,
    AttributePropertyComponent,
    AttributePropertyBoolean,
  };
  use former_types::ComponentAssign;

  /// Represents the attributes of a struct. Aggregates all its attributes.
  #[ derive( Debug, Default ) ]
  pub struct StructAttributes
  {
    /// Attribute for customizing the mutation process.
    pub mutator : AttributeMutator,
  }

  impl StructAttributes
  {
    /// Constructs a `StructAttributes` instance from an iterator of attributes.
    ///
    /// This function parses the provided attributes and assigns them to the
    /// appropriate fields in the `StructAttributes` struct.
    pub fn from_attrs< 'a >( attrs : impl Iterator< Item = & 'a syn::Attribute > ) -> Result< Self >
    {
      let mut result = Self::default();

      // Closure to generate an error message for unknown attributes.
      let error = | attr : & syn::Attribute | -> syn::Error
      {
        let known_attributes = const_format::concatcp!
        (
          "Known attributes are: ",
          "debug",
          ", ", AttributeMutator::KEYWORD,
          "."
        );
        syn_err!
        (
          attr,
          "Expects an attribute of format '#[ attribute( key1 = val1, key2 = val2 ) ]'\n  {known_attributes}\n  But got: '{}'",
          qt! { #attr }
        )
      };

      for attr in attrs
      {
        let key_ident = attr.path().get_ident().ok_or_else( || error( attr ) )?;
        let key_str = format!( "{}", key_ident );
        if attr::is_standard( & key_str )
        {
          continue;
        }
        match key_str.as_ref()
        {
          AttributeMutator::KEYWORD => result.assign( AttributeMutator::from_meta( attr )? ),
          "debug" => {},
          _ => return Err( error( attr ) ),
        }
      }

      Ok( result )
    }
  }

  /// Represents attributes for customizing the mutation process in a forming operation.
  ///
  /// ## Example of code
  ///
  /// ```ignore
  /// #[ mutator( custom = true, hint = true ) ]
  /// ```
  #[ derive( Debug, Default ) ]
  pub struct AttributeMutator
  {
    /// Indicates whether a custom mutator should be generated.
    /// Defaults to `false`, meaning no custom mutator is generated unless explicitly requested.
    pub custom : AttributePropertyCustom,
    /// Specifies whether to provide a sketch of the mutator as a hint.
    /// Defaults to `false`, which means no hint is provided unless explicitly requested.
    pub hint : AttributePropertyHint,
  }

  impl AttributeComponent for AttributeMutator
  {
    const KEYWORD : & 'static str = "mutator";

    /// Parses a `syn::Attribute` into an `AttributeMutator`.
    fn from_meta( attr : & syn::Attribute ) -> Result< Self >
    {
      match attr.meta
      {
        syn::Meta::List( ref meta_list ) =>
        {
          return syn::parse2::< AttributeMutator >( meta_list.tokens.clone() );
        },
        syn::Meta::Path( ref _path ) =>
        {
          return Ok( Default::default() )
        },
        _ => return_syn_err!
        (
          attr,
          "Expects an attribute of format `#[ mutator( custom = true, hint = true ) ]`. \nGot: {}",
          qt! { #attr }
        ),
      }
    }
  }

  // Implement `ComponentAssign` trait to allow assigning `AttributeMutator` to `StructAttributes`.
  impl< IntoT > ComponentAssign< AttributeMutator, IntoT > for StructAttributes
  where
    IntoT : Into< AttributeMutator >,
  {
    #[ inline( always ) ]
    fn assign( & mut self, component : IntoT )
    {
      self.mutator = component.into();
    }
  }

  // Implement `ComponentAssign` trait to allow assigning `AttributePropertyHint` to `AttributeMutator`.
  impl< IntoT > ComponentAssign< AttributePropertyHint, IntoT > for AttributeMutator
  where
    IntoT : Into< AttributePropertyHint >,
  {
    #[ inline( always ) ]
    fn assign( & mut self, component : IntoT )
    {
      self.hint = component.into();
    }
  }

  // Implement `ComponentAssign` trait to allow assigning `AttributePropertyCustom` to `AttributeMutator`.
  impl< IntoT > ComponentAssign< AttributePropertyCustom, IntoT > for AttributeMutator
  where
    IntoT : Into< AttributePropertyCustom >,
  {
    #[ inline( always ) ]
    fn assign( & mut self, component : IntoT )
    {
      self.custom = component.into();
    }
  }

  impl syn::parse::Parse for AttributeMutator
  {
    fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
    {
      let mut result = Self::default();

      let error = | ident : & syn::Ident | -> syn::Error
      {
        let known = const_format::concatcp!
        (
          "Known entries of attribute ", AttributeMutator::KEYWORD, " are: ",
          AttributePropertyCustom::KEYWORD,
          ", ", AttributePropertyHint::KEYWORD,
          "."
        );
        syn_err!
        (
          ident,
          r#"Expects an attribute of format '#[ mutator( custom = false, hint = false ) ]'
    {known}
    But got: '{}'
  "#,
          qt! { #ident }
        )
      };

      while !input.is_empty()
      {
        let lookahead = input.lookahead1();
        if lookahead.peek( syn::Ident )
        {
          let ident : syn::Ident = input.parse()?;

          match ident.to_string().as_str()
          {
            AttributePropertyCustom::KEYWORD => result.assign( AttributePropertyCustom::parse( input )? ),
            AttributePropertyHint::KEYWORD => result.assign( AttributePropertyHint::parse( input )? ),
            _ => return Err( error( & ident ) ),
          }
        }
        else
        {
          return Err( lookahead.error() );
        }

        // Optional comma handling
        if input.peek( syn::Token![,] )
        {
          input.parse::< syn::Token![,] >()?;
        }
      }

      Ok( result )
    }
  }

  // == Attribute properties

  /// Marker type for attribute property to specify whether to provide a sketch as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct AttributePropertyHintMarker;

  impl AttributePropertyComponent for AttributePropertyHintMarker
  {
    const KEYWORD : & 'static str = "hint";
  }

  /// Specifies whether to provide a sketch as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub type AttributePropertyHint = AttributePropertyBoolean< AttributePropertyHintMarker >;

  // ==

  /// Marker type for attribute property to indicate whether a custom code should be generated.
  /// Defaults to `false`, meaning no custom code is generated unless explicitly requested.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct AttributePropertyCustomMarker;

  impl AttributePropertyComponent for AttributePropertyCustomMarker
  {
    const KEYWORD : & 'static str = "custom";
  }

  /// Indicates whether a custom code should be generated.
  /// Defaults to `false`, meaning no custom code is generated unless explicitly requested.
  pub type AttributePropertyCustom = AttributePropertyBoolean< AttributePropertyCustomMarker >;

  // == test code

  // Parse an attribute and construct a `StructAttributes` instance.
  let input : syn::Attribute = syn::parse_quote!( #[ mutator( custom = true, hint = false ) ] );
  let attrs : StructAttributes = StructAttributes::from_attrs( std::iter::once( & input ) ).unwrap();
  println!( "{:?}", attrs );

  // Test `AttributePropertyBoolean` functionality.
  let attr : AttributePropertyBoolean< AttributePropertyHintMarker > = AttributePropertyBoolean::default();
  assert_eq!( attr.internal(), false );
  let attr : AttributePropertyBoolean< AttributePropertyHintMarker > = true.into();
  assert_eq!( attr.internal(), true );
  let attr : AttributePropertyBoolean< AttributePropertyHintMarker > = false.into();
  assert_eq!( attr.internal(), false );

}

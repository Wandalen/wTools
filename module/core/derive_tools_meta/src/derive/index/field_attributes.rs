
  use macro_tools::
  {
    ct,
    syn_err,
    syn,
    qt,
    Result,
    AttributePropertyComponent,
    AttributePropertyOptionalSingletone,
    Assign,
  };

  /// Represents the attributes of a struct. Aggregates all its attributes.
  #[ derive( Debug, Default ) ]
  pub struct FieldAttributes
    {
    /// Attribute for customizing the mutation process.
    pub index : AttributePropertyIndex,

    pub debug : AttributePropertyDebug,
  }

  impl FieldAttributes
  {
    /// Constructs a `ItemAttributes` instance from an iterator of attributes.
    ///
    /// This function parses the provided attributes and assigns them to the
    /// appropriate fields in the `ItemAttributes` struct.
    pub fn from_attrs< 'a >( attrs : impl Iterator< Item = & 'a syn::Attribute > ) -> Result< Self >
    {
      let mut result = Self::default();

      // Closure to generate an error message for unknown attributes.
      let error = | attr : & syn::Attribute | -> syn::Error
      {
        let known_attributes = ct::str::format!
        (
          "Known attributes are: {}, {}.",
          "debug",
          AttributePropertyIndex::KEYWORD,
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
        // if attr::is_standard( & key_str )
        // {
        //   continue;
        // }
        match key_str.as_ref()
        {
          AttributePropertyIndex::KEYWORD => result.assign( AttributePropertyIndex::from( true ) ),
          "debug" => {},
          _ => {},
          // _ => return Err( error( attr ) ),
        }
      }

      Ok( result )
    }
  }


impl< IntoT > Assign< AttributePropertyIndex, IntoT > for FieldAttributes
where
  IntoT : Into< AttributePropertyIndex >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.index.assign( component.into() );
  }
}





  // == Attribute properties

  /// Marker type for attribute property to specify whether to provide a sketch as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct AttributePropertyDebugMarker;

  impl AttributePropertyComponent for AttributePropertyDebugMarker
  {
    const KEYWORD : & 'static str = "debug";
  }

  /// Specifies whether to provide a sketch as a hint.
  /// Defaults to `false`, which means no hint is provided unless explicitly requested.
  pub type AttributePropertyDebug = AttributePropertyOptionalSingletone< AttributePropertyDebugMarker >;

  // ==

  /// Marker type for attribute property to indicate whether a custom code should be generated.
  /// Defaults to `false`, meaning no custom code is generated unless explicitly requested.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct AttributePropertyIndexMarker;

  impl AttributePropertyComponent for AttributePropertyIndexMarker
  {
    const KEYWORD : & 'static str = "index";
  }

  /// Indicates whether a custom code should be generated.
  /// Defaults to `false`, meaning no custom code is generated unless explicitly requested.
  pub type AttributePropertyIndex = AttributePropertyOptionalSingletone< AttributePropertyIndexMarker >;

  // == test code


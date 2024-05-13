//!
//! Attributes analyzys and manipulation.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use super::super::*;

  ///
  /// For attribute like `#[former( default = 31 ) ]` return key `default` and value `31`,
  /// as well as syn::Meta as the last element of result tuple.
  ///
  /// ### Basic use-case.
  /// ```rust
  /// use macro_tools::exposed::*;
  /// let attr : syn::Attribute = syn::parse_quote!( #[ former( default = 31 ) ] );
  /// // tree_print!( attr );
  /// let got = equation( &attr ).unwrap();
  /// assert_eq!( code_to_str!( got ), "default = 31".to_string() );
  /// ```

  pub fn equation( attr : &syn::Attribute ) -> Result< tokens::Equation >
  {
    let meta = &attr.meta;
    return match meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        let eq : tokens::Equation = syn::parse2( meta_list.tokens.clone() )?;
        Ok( eq )
      }
      _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected syn::Meta::List( meta_list )" ) ),
    };
  }

  /// Checks if the given iterator of attributes contains an attribute named `debug`.
  ///
  /// This function iterates over an input sequence of `syn::Attribute`, typically associated with a struct,
  /// enum, or other item in a Rust Abstract Syntax Tree ( AST ), and determines whether any of the attributes
  /// is exactly named `debug`.
  ///
  /// # Parameters
  /// - `attrs` : An iterator over `syn::Attribute`. This could be obtained from parsing Rust code
  ///   with the `syn` crate, where the iterator represents attributes applied to a Rust item ( like a struct or function ).
  ///
  /// # Returns
  /// - `Ok( true )` if the `debug` attribute is present.
  /// - `Ok( false )` if the `debug` attribute is not found.
  /// - `Err( syn::Error )` if an unknown or improperly formatted attribute is encountered.
  ///
  /// # Example
  ///
  /// Suppose you have the following struct definition in a procedural macro input:
  ///
  /// ```rust, ignore
  /// #[ derive( SomeDerive ) ]
  /// #[ debug ]
  /// struct MyStruct
  /// {
  ///   field : i32,
  /// }
  /// ```
  ///
  /// You can use `has_debug` to check for the presence of the `debug` attribute:
  ///
  /// ```rust
  /// use macro_tools::exposed::*;
  ///
  /// // Example struct attribute
  /// let attrs : Vec< syn::Attribute > = vec![ syn::parse_quote!( #[ debug ] ) ];
  ///
  /// // Checking for 'debug' attribute
  /// let contains_debug = attr::has_debug( ( &attrs ).into_iter() ).unwrap();
  ///
  /// assert!( contains_debug, "Expected to find 'debug' attribute" );
  /// ```
  ///

  pub fn has_debug< 'a >( attrs : impl Iterator< Item = &'a syn::Attribute > ) -> Result< bool >
  {
    for attr in attrs
    {
      if let Some( ident ) = attr.path().get_ident()
      {
        let ident_string = format!( "{}", ident );
        if ident_string == "debug"
        {
          return Ok( true )
        }
      }
      else
      {
        return_syn_err!( "Unknown structure attribute:\n{}", qt!{ attr } );
      }
    }
    return Ok( false )
  }

  /// Checks if the given attribute name is a standard Rust attribute.
  ///
  /// Standard Rust attributes are those which are recognized and processed
  /// directly by the Rust compiler. They influence various aspects of compilation,
  /// including but not limited to conditional compilation, optimization hints,
  /// code visibility, and procedural macro behavior.
  ///
  /// This function is useful when developing tools that need to interact with or
  /// understand the significance of specific attributes in Rust source code, such
  /// as linters, code analyzers, or procedural macros.
  ///
  /// This function does not cover all possible attributes but includes many of the
  /// common ones that are relevant to most Rust projects. Developers are encouraged
  /// to update this function as needed to suit more specialized needs, especially
  /// when dealing with nightly-only compiler attributes or deprecated ones.
  ///
  /// # Parameters
  /// - `attr_name`: A string slice that holds the name of the attribute to check.
  ///
  /// # Returns
  /// Returns `true` if `attr_name` is a recognized standard Rust attribute. Otherwise,
  /// returns `false`.
  ///
  /// # Examples
  ///
  /// Standard attributes:
  ///
  /// ```
  /// assert_eq!( macro_tools::attr::is_standard( "cfg" ), true );
  /// assert_eq!( macro_tools::attr::is_standard( "inline" ), true );
  /// assert_eq!( macro_tools::attr::is_standard( "derive" ), true );
  /// ```
  ///
  /// Non-standard or custom attributes:
  ///
  /// ```
  /// assert_eq!( macro_tools::attr::is_standard( "custom_attr" ), false );
  /// assert_eq!( macro_tools::attr::is_standard( "my_attribute" ), false );
  /// ```
  ///

  pub fn is_standard<'a>( attr_name : &'a str ) -> bool
  {
    match attr_name
    {
      // Conditional compilation
      "cfg" | "cfg_attr" => true,

      // Compiler instructions and optimizations
      "inline" | "repr" | "derive" | "allow" | "warn" | "deny" | "forbid" => true,

      // Testing attributes
      "test" | "bench" => true,

      // Documentation attributes
      "doc" => true,

      // Visibility and accessibility
      "pub" => true, // This would typically need context to be accurate

      // Safety and ABI
      "unsafe" | "no_mangle" | "extern" => true,

      // Module and Crate configuration
      "path" | "macro_use" | "crate_type" | "crate_name" => true,

      // Linking
      "link" | "link_name" | "link_section" => true,

      // Usage warnings
      "must_use" => true,

      // Other attributes
      "cold" | "export_name" | "global_allocator" => true,

      // Module handling
      "used" | "unused" => true,

      // Procedural macros and hygiene
      "proc_macro" | "proc_macro_derive" | "proc_macro_attribute" => true,

      // Stability attributes
      "stable" | "unstable" | "rustc_const_unstable" | "rustc_const_stable" |
      "rustc_diagnostic_item" | "rustc_deprecated" | "rustc_legacy_const_generics" => true,

      // Special compiler attributes
      "feature" | "non_exhaustive" => true,

      // Future compatibility
      "rustc_paren_sugar" | "rustc_insignificant_dtor" => true,

      // Type system extensions
      "opaque" => true,

      // Miscellaneous
      "track_caller" => true,

      // Default case
      _ => false,
    }
  }

  ///
  /// Attribute which is inner.
  ///
  /// For example: `// #![ deny( missing_docs ) ]`.
  ///

  #[ derive( Debug, PartialEq, Eq, Clone, Default ) ]
  pub struct AttributesInner( pub Vec< syn::Attribute > );

  impl From< Vec< syn::Attribute > > for AttributesInner
  {
    #[ inline( always ) ]
    fn from( src : Vec< syn::Attribute > ) -> Self
    {
      Self( src )
    }
  }

  impl From< AttributesInner > for Vec< syn::Attribute >
  {
    #[ inline( always ) ]
    fn from( src : AttributesInner ) -> Self
    {
      src.0
    }
  }

  impl AttributesInner
  {
    /// Iterator
    pub fn iter( &self ) -> core::slice::Iter< '_, syn::Attribute >
    {
      self.0.iter()
    }
  }

  impl syn::parse::Parse
  for AttributesInner
  {
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      // let mut result : Self = from!();
      let mut result : Self = Default::default();
      loop
      {
        if !input.peek( Token![ # ] ) || !input.peek2( Token![ ! ] )
        {
          break;
        }
        let input2;
        let element = syn::Attribute
        {
          pound_token : input.parse()?,
          style : syn::AttrStyle::Inner( input.parse()? ),
          bracket_token : bracketed!( input2 in input ),
          // path : input2.call( syn::Path::parse_mod_style )?,
          // tokens : input2.parse()?,
          meta : input2.parse()?,
        };
        result.0.push( element );
      }
      Ok( result )
    }
  }

  impl quote::ToTokens
  for AttributesInner
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      use crate::quote::TokenStreamExt;
      tokens.append_all( self.0.iter() );
    }
  }

  //

  // types!
  // {

    ///
    /// Attribute which is outer.
    ///
    /// For example: `#[ derive( Copy ) ]`.
    ///

    // #[ derive( Debug, PartialEq, Eq, Clone, Default ) ]
    // pub many AttributesOuter : syn::Attribute;
    // xxx : apply maybe collection of derives for TDD

    #[ derive( Debug, PartialEq, Eq, Clone, Default ) ]
    pub struct AttributesOuter( pub Vec< syn::Attribute > );

  // }

  impl From< Vec< syn::Attribute > > for AttributesOuter
  {
    #[ inline( always ) ]
    fn from( src : Vec< syn::Attribute > ) -> Self
    {
      Self( src )
    }
  }

  impl From< AttributesOuter > for Vec< syn::Attribute >
  {
    #[ inline( always ) ]
    fn from( src : AttributesOuter ) -> Self
    {
      src.0
    }
  }

  impl AttributesOuter
  {
    /// Iterator
    pub fn iter( &self ) -> core::slice::Iter< '_, syn::Attribute >
    {
      self.0.iter()
    }
  }

  impl syn::parse::Parse
  for AttributesOuter
  {
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      let mut result : Self = Default::default();
      loop
      {
        if !input.peek( Token![ # ] ) || input.peek2( Token![ ! ] )
        {
          break;
        }
        let input2;
        let element = syn::Attribute
        {
          pound_token : input.parse()?,
          style : syn::AttrStyle::Outer,
          bracket_token : bracketed!( input2 in input ),
          // path : input2.call( syn::Path::parse_mod_style )?,
          // tokens : input2.parse()?,
          meta : input2.parse()?,
        };
        result.0.push( element );
      }
      Ok( result )
    }
  }

  impl quote::ToTokens
  for AttributesOuter
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      use crate::quote::TokenStreamExt;
      tokens.append_all( self.0.iter() );
    }
  }

  ///
  /// Attribute and ident.
  ///

  // qqq : example?

  pub type AttributedIdent = Pair< Many< AttributesInner >, syn::Ident >;

  impl From< syn::Ident > for AttributedIdent
  {
    fn from( src : syn::Ident ) -> Self
    {
      Self( Vec::< AttributesInner >::new().into(), src )
    }
  }

  impl From< AttributedIdent > for syn::Ident
  {
    fn from( src : AttributedIdent ) -> Self
    {
      src.1
    }
  }

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::protected as attr;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    equation,
    has_debug,
    is_standard,
    AttributesInner,
    AttributesOuter,
    AttributedIdent,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}

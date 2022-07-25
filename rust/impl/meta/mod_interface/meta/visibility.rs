/// Internal namespace.
pub( crate ) mod private
{
  use proc_macro_tools::prelude::*;
  use core::hash::{ Hash, Hasher };

  pub const VALID_VISIBILITY_LIST_STR : &str = "[ private, protected, orphan, exposed, prelude ]";

  ///
  /// Custom keywords
  ///

  pub mod kw
  {
    use super::*;
    // syn::custom_keyword!( private );
    syn::custom_keyword!( protected );
    syn::custom_keyword!( orphan );
    syn::custom_keyword!( exposed );
    syn::custom_keyword!( prelude );
  }

  ///
  /// Visibility constructor.
  ///

  pub trait VisibilityInterface
  {
    type Token : syn::token::Token + syn::parse::Parse;

    fn vis_make( token : Self::Token, restriction : Option< Restriction > ) -> Self;
    fn restriction( &self ) -> Option< &Restriction >;

  }

  ///
  /// Trait answering question can the visibility be used for non-standard module.
  ///

  pub trait ValidSubNamespace
  {
    fn valid_sub_namespace( &self ) -> bool { false }
  }

  /// Has kind.
  pub trait HasKind
  {

    /// Static function to get kind of the visibility.
    #[ allow( non_snake_case ) ]
    #[ allow( dead_code ) ]
    fn Kind() -> u32;

    /// Method to get kind of the visibility.
    #[ allow( dead_code ) ]
    fn kind( &self ) -> u32
    {
      Self::Kind()
    }

  }

  //

  macro_rules! Clause
  {

    ( $Name1:ident, $Kind:literal ) =>
    {

      #[ derive( Debug, PartialEq, Eq, Clone ) ]
      pub struct $Name1
      {
      }

      impl $Name1
      {
        #[ allow( dead_code ) ]
        pub fn new() -> Self
        {
          Self {}
        }
      }

      impl HasKind for $Name1
      {
        #[ allow( non_snake_case ) ]
        #[ allow( dead_code ) ]
        fn Kind() -> u32
        {
          $Kind
        }
      }

    }

  }

  //

  macro_rules! Vis
  {
    ( $Name0:ident, $Name1:ident, $Name2:ident, $Kind:literal ) =>
    {

      #[ derive( Debug, PartialEq, Eq, Clone ) ]
      pub struct $Name1
      {
        pub token : kw::$Name2,
        pub restriction : Option< Restriction >,
      }

      impl $Name1
      {
        #[ allow( dead_code ) ]
        pub fn new() -> Self
        {
          Self
          {
            token : kw::$Name2( proc_macro2::Span::call_site() ),
            restriction : None,
          }
        }
      }

      impl VisibilityInterface for $Name1
      {
        type Token = kw::$Name2;
        fn vis_make( token : Self::Token, restriction : Option< Restriction > ) -> Self
        {
          Self
          {
            token,
            restriction,
          }
        }
        fn restriction( &self ) -> Option< &Restriction >
        {
          self.restriction.as_ref()
        }
      }

      impl HasKind for $Name1
      {
        #[ allow( non_snake_case ) ]
        #[ allow( dead_code ) ]
        fn Kind() -> u32
        {
          $Kind
        }
      }

      impl quote::ToTokens for $Name1
      {
        fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
        {
          self.token.to_tokens( tokens );
        }
      }

      impl From< $Name1 > for Visibility
      {
        fn from( src : $Name1 ) -> Self
        {
          Self::$Name0( src )
        }
      }


    }
  }

  //

  macro_rules! HasKind
  {

    ( $Name1:path, $Kind:literal ) =>
    {

      impl HasKind for $Name1
      {
        #[ allow( non_snake_case ) ]
        #[ allow( dead_code ) ]
        fn Kind() -> u32
        {
          $Kind
        }
      }

    }

  }

  //

  macro_rules! impl_valid_sub_namespace
  {

    ( $Name1:path, $Val:literal ) =>
    {

      impl ValidSubNamespace for $Name1
      {
        fn valid_sub_namespace( &self ) -> bool
        {
          $Val
        }
      }

    }

  }

  // Vis!( Private, VisPrivate, private, 1 );
  Vis!( Protected, VisProtected, protected, 2 );
  Vis!( Orphan, VisOrphan, orphan, 3 );
  Vis!( Exposed, VisExposed, exposed, 4 );
  Vis!( Prelude, VisPrelude, prelude, 5 );
  HasKind!( syn::VisPublic, 6 );
  HasKind!( syn::VisRestricted, 7 );
  Clause!( ClauseImmediates, 11 );

  // impl_valid_sub_namespace!( VisPrivate, false );
  impl_valid_sub_namespace!( VisProtected, true );
  impl_valid_sub_namespace!( VisOrphan, true );
  impl_valid_sub_namespace!( VisExposed, true );
  impl_valid_sub_namespace!( VisPrelude, true );
  impl_valid_sub_namespace!( syn::VisPublic, false );
  impl_valid_sub_namespace!( syn::VisRestricted, false );

  ///
  /// Restriction, for example `pub( crate )`.
  ///

  #[ derive( Debug, PartialEq, Eq, Clone ) ]
  pub struct Restriction
  {
    paren_token : syn::token::Paren,
    in_token : Option< syn::token::In >,
    path : Box< syn::Path >,
  }

  ///
  /// Visibility of an element.
  ///

  #[ derive( Debug, PartialEq, Eq, Clone ) ]
  pub enum Visibility
  {
    //Private( VisPrivate ),
    Protected( VisProtected ),
    Orphan( VisOrphan ),
    Exposed( VisExposed ),
    Prelude( VisPrelude ),
    Public( syn::VisPublic ),
    // Crate( syn::VisCrate ),
    // Restricted( syn::VisRestricted ),
    Inherited,
  }

  impl Visibility
  {

    // fn parse_private( input : ParseStream< '_ > ) -> Result< Self >
    // {
    //   Ok( Visibility::Private( VisPrivate
    //   {
    //     token : input.parse()?,
    //     restriction : None,
    //   }))
    // }

    fn parse_protected( input : ParseStream< '_ > ) -> Result< Self >
    {
      Self::_parse_vis::< VisProtected >( input )
    }

    fn parse_orphan( input : ParseStream< '_ > ) -> Result< Self >
    {
      Self::_parse_vis::< VisOrphan >( input )
    }

    fn parse_exposed( input : ParseStream< '_ > ) -> Result< Self >
    {
      Self::_parse_vis::< VisExposed >( input )
    }

    fn parse_prelude( input : ParseStream< '_ > ) -> Result< Self >
    {
      Self::_parse_vis::< VisPrelude >( input )
    }

    fn parse_pub( input : ParseStream< '_ > ) -> Result< Self >
    {
      Ok( Visibility::Public( syn::VisPublic { pub_token : input.parse()? } ) )
    }

    fn _parse_vis< Vis >( input : ParseStream< '_ > ) -> Result< Self >
    where
      Vis : Into< Visibility > + VisibilityInterface,
    {
      use proc_macro_tools::syn::parse::discouraged::Speculative;
      use proc_macro_tools::syn::ext::IdentExt;
      let token = input.parse::< < Vis as VisibilityInterface >::Token >()?;

      if input.peek( syn::token::Paren )
      {
        let ahead = input.fork();

        let input2;
        let paren_token = syn::parenthesized!( input2 in ahead );
        if input2.peek( Token![ crate ] )
          || input2.peek( Token![ self ] )
          || input2.peek( Token![ super ] )
        {
          let path = input2.call( syn::Ident::parse_any )?;

          // Ensure there are no additional tokens within `input2`.
          // Without explicitly checking, we may misinterpret a tuple
          // field as a restricted visibility, causing a parse error.
          // e.g. `pub (crate::A, crate::B)` (Issue #720).
          if input2.is_empty()
          {
            input.advance_to( &ahead );

            let restriction = Restriction
            {
              paren_token,
              in_token : None,
              path : Box::new( syn::Path::from( path ) ),
            };

            return Ok( Vis::vis_make
            (
              token,
              Some( restriction ),
            ).into() );
          }
        }
        // else if input2.peek( Token![ in ] )
        // {
        //   let in_token : Token![ in ] = input2.parse()?;
        //   let path = input2.call( syn::Path::parse_mod_style )?;
        //   input.advance_to( &ahead );
        //   return Ok
        //   (
        //     Visibility::Restricted
        //     (
        //       syn::VisRestricted
        //       {
        //         pub_token,
        //         paren_token,
        //         in_token : Some( in_token ),
        //         path : Box::new( path ),
        //       }
        //     )
        //   );
        // }
      }

      return Ok( Vis::vis_make
      (
        token,
        None,
      ).into() );
    }

    // fn parse_in_crate( input : ParseStream< '_ > ) -> Result< Self >
    // {
    //   if input.peek2( Token![ :: ] )
    //   {
    //     Ok( Visibility::Inherited )
    //   }
    //   else
    //   {
    //     Ok( Visibility::Crate( VisInCrate
    //     {
    //       crate_token : input.parse()?,
    //     }))
    //   }
    // }

    /// Get kind.
    #[ allow( dead_code ) ]
    pub fn kind( &self ) -> u32
    {
      match self
      {
        // Visibility::Private( e ) => e.kind(),
        // Visibility::Crate( e ) => e.kind(),
        Visibility::Protected( e ) => e.kind(),
        Visibility::Orphan( e ) => e.kind(),
        Visibility::Exposed( e ) => e.kind(),
        Visibility::Prelude( e ) => e.kind(),
        Visibility::Public( e ) => e.kind(),
        // Visibility::Restricted( e ) => e.kind(),
        Visibility::Inherited => 9,
      }
    }

    /// Get restrictions.
    #[ allow( dead_code ) ]
    pub fn restriction( &self ) -> Option< &Restriction >
    {
      match self
      {
        // Visibility::Private( e ) => e.restriction(),
        // Visibility::Crate( e ) => e.restriction(),
        Visibility::Protected( e ) => e.restriction(),
        Visibility::Orphan( e ) => e.restriction(),
        Visibility::Exposed( e ) => e.restriction(),
        Visibility::Prelude( e ) => e.restriction(),
        Visibility::Public( _ ) => None,
        // Visibility::Restricted( e ) => e.restriction(),
        Visibility::Inherited => None,
      }
    }

  }

  impl syn::parse::Parse for Visibility
  {
    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      // Recognize an empty None-delimited group, as produced by a $:vis
      // matcher that matched no tokens.

      // if input.peek( syn::token::Group )
      // {
      //   let ahead = input.fork();
      //   let group = syn::group::parse_group( &ahead )?;
      //   if group.input2.is_empty()
      //   {
      //     input.advance_to( &ahead );
      //     return Ok( Visibility::Inherited );
      //   }
      // }

      match()
      {
        //_case if input.peek( kw::private ) => Self::parse_private( input ),
        _case if input.peek( kw::protected ) => Self::parse_protected( input ),
        _case if input.peek( kw::orphan ) => Self::parse_orphan( input ),
        _case if input.peek( kw::exposed ) => Self::parse_exposed( input ),
        _case if input.peek( kw::prelude ) => Self::parse_prelude( input ),
        _case if input.peek( Token![ pub ] ) => Self::parse_pub( input ),
        _default =>
        {
          Ok( Visibility::Inherited )
        },
      }

    }
  }

  impl quote::ToTokens for Visibility
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      match self
      {
        //Visibility::Private( e ) => e.to_tokens( tokens ),
        Visibility::Protected( e ) => e.to_tokens( tokens ),
        Visibility::Orphan( e ) => e.to_tokens( tokens ),
        Visibility::Exposed( e ) => e.to_tokens( tokens ),
        Visibility::Prelude( e ) => e.to_tokens( tokens ),
        Visibility::Public( e ) => e.to_tokens( tokens ),
        Visibility::Inherited => (),
      }
    }
  }

  impl Hash for Visibility
  {
    fn hash< H : Hasher >( &self, state : &mut H )
    {
      self.kind().hash( state )
    }
  }

  impl ValidSubNamespace for Visibility
  {
    fn valid_sub_namespace( &self ) -> bool
    {
      match self
      {
        //Visibility::Private( e ) => e.valid_sub_namespace(),
        Visibility::Protected( e ) => e.valid_sub_namespace(),
        Visibility::Orphan( e ) => e.valid_sub_namespace(),
        Visibility::Exposed( e ) => e.valid_sub_namespace(),
        Visibility::Prelude( e ) => e.valid_sub_namespace(),
        Visibility::Public( e ) => e.valid_sub_namespace(),
        Visibility::Inherited => false,
      }
    }
  }

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;

  pub use super::private::
  {
    kw,
    VALID_VISIBILITY_LIST_STR,
    ValidSubNamespace,
    HasKind,
    // VisPrivate,
    VisProtected,
    VisOrphan,
    VisExposed,
    VisPrelude,
    ClauseImmediates,
    Visibility,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}

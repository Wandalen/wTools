/// Internal namespace.
mod internal
{
  // use super::*;
  use proc_macro_tools::prelude::*;
  use core::hash::{ Hash, Hasher };

  ///
  /// Custom keywords
  ///

  pub mod kw
  {
    use super::*;
    syn::custom_keyword!( private );
    syn::custom_keyword!( protected );
    syn::custom_keyword!( orphan );
    syn::custom_keyword!( exposed );
    syn::custom_keyword!( prelude );
  }

  ///
  /// Trait answering question can the visibility be used for non-standard module.
  ///

  pub trait CanBeUsedForNonStandardModInterface
  {
    fn can_be_used_for_non_standard_mod( &self ) -> bool { false }
  }

  //

  macro_rules! Vis
  {

    ( $Name1 : tt, $Name2 : tt, $Kind : literal ) =>
    {

      #[ derive( Debug, PartialEq, Eq, Clone ) ]
      pub struct $Name1
      {
        pub token : kw::$Name2,
      }

      impl $Name1
      {
        #[ allow( dead_code ) ]
        pub fn new() -> Self
        {
          Self { token : kw::$Name2( proc_macro2::Span::call_site() ) }
        }
        #[ allow( non_snake_case ) ]
        #[ allow( dead_code ) ]
        pub fn Kind() -> u32
        {
          $Kind
        }
        #[ allow( dead_code ) ]
        pub fn kind( &self ) -> u32
        {
          Self::Kind()
        }
      }

      impl quote::ToTokens for $Name1
      {
        fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
        {
          self.token.to_tokens( tokens );
        }
      }

    }

  }

  //

  macro_rules! impl_can_be_non_standard
  {

    ( $Name1 : tt, $Val : literal ) =>
    {

      impl CanBeUsedForNonStandardModInterface for $Name1
      {
        fn can_be_used_for_non_standard_mod( &self ) -> bool
        {
          $Val
        }
      }

    }

  }

  Vis!( VisPrivate, private, 1 );
  Vis!( VisProtected, protected, 2 );
  Vis!( VisOrphan, orphan, 3 );
  Vis!( VisExposed, exposed, 4 );
  Vis!( VisPrelude, prelude, 5 );

  impl_can_be_non_standard!( VisPrivate, false );
  impl_can_be_non_standard!( VisProtected, true );
  impl_can_be_non_standard!( VisOrphan, true );
  impl_can_be_non_standard!( VisExposed, true );
  impl_can_be_non_standard!( VisPrelude, true );

  //

  #[ derive( Debug, PartialEq, Eq, Clone ) ]
  pub enum Visibility
  {
    Private( VisPrivate ),
    Protected( VisProtected ),
    Orphan( VisOrphan ),
    Exposed( VisExposed ),
    Prelude( VisPrelude ),
    Public( syn::VisPublic ),
    Crate( syn::VisCrate ),
    Restricted( syn::VisRestricted ),
    Inherited,
  }

  impl Visibility
  {

    fn parse_private( input : ParseStream< '_ > ) -> Result< Self >
    {
      Ok( Visibility::Private( VisPrivate
      {
        token : input.parse()?,
      }))
    }

    fn parse_protected( input : ParseStream< '_ > ) -> Result< Self >
    {
      Ok( Visibility::Protected( VisProtected
      {
        token : input.parse()?,
      }))
    }

    fn parse_orphan( input : ParseStream< '_ > ) -> Result< Self >
    {
      Ok( Visibility::Orphan( VisOrphan
      {
        token : input.parse()?,
      }))
    }

    fn parse_exposed( input : ParseStream< '_ > ) -> Result< Self >
    {
      Ok( Visibility::Exposed( VisExposed
      {
        token : input.parse()?,
      }))
    }

    fn parse_prelude( input : ParseStream< '_ > ) -> Result< Self >
    {
      Ok( Visibility::Prelude( VisPrelude
      {
        token : input.parse()?,
      }))
    }

    fn parse_pub( input : ParseStream< '_ > ) -> Result< Self >
    {
      let pub_token = input.parse::<Token![pub]>()?;

      if input.peek( syn::token::Paren )
      {
        use proc_macro_tools::syn::parse::discouraged::Speculative;
        use proc_macro_tools::syn::ext::IdentExt;
        let ahead = input.fork();

        let content;
        let paren_token = syn::parenthesized!( content in ahead );
        if content.peek( Token![ crate ] )
          || content.peek( Token![ self ] )
          || content.peek( Token![ super ] )
        {

          let path = content.call( syn::Ident::parse_any )?;

          // Ensure there are no additional tokens within `content`.
          // Without explicitly checking, we may misinterpret a tuple
          // field as a restricted visibility, causing a parse error.
          // e.g. `pub (crate::A, crate::B)` (Issue #720).
          if content.is_empty()
          {
            input.advance_to( &ahead );
            return Ok( Visibility::Restricted( syn::VisRestricted
            {
              pub_token,
              paren_token,
              in_token: None,
              path: Box::new( syn::Path::from( path ) ),
            }));
          }
        }
        else if content.peek( Token![ in ] )
        {
          let in_token : Token![ in ] = content.parse()?;
          let path = content.call( syn::Path::parse_mod_style )?;

          input.advance_to( &ahead );
          return Ok
          (
            Visibility::Restricted
            (
              syn::VisRestricted
              {
                pub_token,
                paren_token,
                in_token : Some( in_token ),
                path : Box::new( path ),
              }
            )
          );
        }
      }

      Ok( Visibility::Public( syn::VisPublic { pub_token } ) )
    }

    fn parse_crate( input : ParseStream< '_ > ) -> Result< Self >
    {
      if input.peek2( Token![ :: ] )
      {
        Ok( Visibility::Inherited )
      }
      else
      {
        Ok( Visibility::Crate( syn::VisCrate
        {
          crate_token : input.parse()?,
        }))
      }
    }

    // #[ allow( non_snake_case ) ]
    #[ allow( dead_code ) ]
    pub fn kind( &self ) -> u32
    {
      match self
      {
        Visibility::Private( e ) => e.kind(),
        Visibility::Protected( e ) => e.kind(),
        Visibility::Orphan( e ) => e.kind(),
        Visibility::Exposed( e ) => e.kind(),
        Visibility::Prelude( e ) => e.kind(),
        Visibility::Public( _ ) => 6,
        Visibility::Crate( _ ) => 7,
        Visibility::Restricted( _ ) => 8,
        Visibility::Inherited => 9,
        // _ => (),
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
      //   if group.content.is_empty()
      //   {
      //     input.advance_to( &ahead );
      //     return Ok( Visibility::Inherited );
      //   }
      // }

      if input.peek( kw::private )
      {
        Self::parse_private( input )
      }
      else if input.peek( kw::protected )
      {
        Self::parse_protected( input )
      }
      else if input.peek( kw::orphan )
      {
        Self::parse_orphan( input )
      }
      else if input.peek( kw::exposed )
      {
        Self::parse_exposed( input )
      }
      else if input.peek( kw::prelude )
      {
        Self::parse_prelude( input )
      }
      else if input.peek( Token![ pub ] )
      {
        Self::parse_pub( input )
      }
      else if input.peek( Token![ crate ] )
      {
        Self::parse_crate( input )
      }
      else
      {
        Ok( Visibility::Inherited )
      }

    }
  }

  impl quote::ToTokens for Visibility
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      match self
      {
        Visibility::Private( e ) => e.to_tokens( tokens ),
        Visibility::Protected( e ) => e.to_tokens( tokens ),
        Visibility::Orphan( e ) => e.to_tokens( tokens ),
        Visibility::Exposed( e ) => e.to_tokens( tokens ),
        Visibility::Prelude( e ) => e.to_tokens( tokens ),
        Visibility::Public( e ) => e.to_tokens( tokens ),
        Visibility::Crate( e ) => e.to_tokens( tokens ),
        Visibility::Restricted( e ) => e.to_tokens( tokens ),
        Visibility::Inherited => (),
        // _ => (),
      }
    }
  }

  impl Hash for Visibility
  {
    fn hash< H : Hasher >( &self, state : &mut H )
    {
      match self
      {
        Visibility::Private( _ ) => 1.hash( state ),
        Visibility::Protected( _ ) => 2.hash( state ),
        Visibility::Orphan( _ ) => 3.hash( state ),
        Visibility::Exposed( _ ) => 4.hash( state ),
        Visibility::Prelude( _ ) => 5.hash( state ),
        Visibility::Public( _ ) => 6.hash( state ),
        Visibility::Crate( _ ) => 7.hash( state ),
        Visibility::Restricted( _ ) => 8.hash( state ),
        Visibility::Inherited => 9.hash( state ),
        // _ => (),
      }
    }
  }

  impl CanBeUsedForNonStandardModInterface for Visibility
  {
    fn can_be_used_for_non_standard_mod( &self ) -> bool
    {
      match self
      {
        Visibility::Private( e ) => e.can_be_used_for_non_standard_mod(),
        Visibility::Protected( e ) => e.can_be_used_for_non_standard_mod(),
        Visibility::Orphan( e ) => e.can_be_used_for_non_standard_mod(),
        Visibility::Exposed( e ) => e.can_be_used_for_non_standard_mod(),
        Visibility::Prelude( e ) => e.can_be_used_for_non_standard_mod(),
        Visibility::Public( _ ) => false,
        Visibility::Crate( _ ) => false,
        Visibility::Restricted( _ ) => false,
        Visibility::Inherited => false,
        // _ => (),
      }
    }
  }

}

/// Own namespace of the module.
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

  pub use super::internal::
  {
    kw,
    CanBeUsedForNonStandardModInterface,
    VisPrivate,
    VisProtected,
    VisOrphan,
    VisExposed,
    VisPrelude,
    Visibility,
  };

}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
}

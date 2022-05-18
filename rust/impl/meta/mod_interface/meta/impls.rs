
// xxx : do same thing for other procedural macroses
#[ allow( unused_imports ) ]
use proc_macro_tools::prelude::*;
#[ allow( unused_imports ) ]
use proc_macro_tools::{ Result };

use proc_macro_tools::syn::
{
  ext::IdentExt,
  parse::discouraged::Speculative,
};
use core::hash::{ Hash, Hasher };

///
/// Custom keywords
///

mod kw
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

trait CanBeUsedForNonStandardModInterface
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

///
/// Attribute which is inner.
///

#[ derive( Debug, PartialEq, Eq, Clone ) ]
pub struct AttributeInner( pub syn::Attribute );

impl syn::parse::Parse for AttributeInner
{
  fn parse( input : ParseStream< '_ > ) -> Result< Self >
  {
    let input2;
    Ok( Self( syn::Attribute
    {
      pound_token : input.parse()?,
      style : syn::AttrStyle::Inner( input.parse()? ),
      bracket_token : bracketed!( input2 in input ),
      path : input2.call( syn::Path::parse_mod_style )?,
      tokens : input2.parse()?,
    }))
    // Ok( ( input.call( syn::Attribute::parse_inner )? ).into() )
  }
}

impl quote::ToTokens for AttributeInner
{
  fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
  {
    self.0.to_tokens( tokens );
  }
}

impl From< syn::Attribute > for AttributeInner
{
  fn from( src : syn::Attribute ) -> Self
  {
    Self( src )
  }
}

impl From< AttributeInner > for syn::Attribute
{
  fn from( src : AttributeInner ) -> Self
  {
    src.0
  }
}

///
/// Pair of syntax elements.
///

#[ derive( Debug, PartialEq, Eq, Clone ) ]
pub struct Pair< T1, T2 >
( pub T1, pub T2 )
where
  T1 : syn::parse::Parse + quote::ToTokens,
  T2 : syn::parse::Parse + quote::ToTokens,
;

impl< T1, T2 > syn::parse::Parse for Pair< T1, T2 >
where
  T1 : syn::parse::Parse + quote::ToTokens,
  T2 : syn::parse::Parse + quote::ToTokens,
{
  fn parse( input : ParseStream< '_ > ) -> Result< Self >
  {
    Ok( Self( input.parse()?, input.parse()? ) )
  }
}

impl< T1, T2 > quote::ToTokens for Pair< T1, T2 >
where
  T1 : syn::parse::Parse + quote::ToTokens,
  T2 : syn::parse::Parse + quote::ToTokens,
{
  fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
  {
    self.0.to_tokens( tokens );
    self.1.to_tokens( tokens );
  }
}

// xxx : publish module cotainer with good prelude

///
/// Pair of syntax elements.
///

#[ derive( Debug, PartialEq, Eq, Clone ) ]
pub struct Many< T > ( Vec< T > )
where
  T : quote::ToTokens,
;

impl< T > Many< T >
where
  T : quote::ToTokens,
{
  pub fn new() -> Self
  {
    Self( Vec::new() )
  }
}

impl< T > quote::ToTokens
for Many< T >
where
  T : quote::ToTokens,
{
  fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
  {
    use proc_macro_tools::quote::TokenStreamExt;
    tokens.append_all( self.0.iter() );
    // self.0.to_tokens( tokens );
  }
}

impl syn::parse::Parse
for Many< AttributeInner >
{
  fn parse( input : ParseStream< '_ > ) -> Result< Self >
  {
    let mut result = Self::new();
    while input.peek( Token![ # ] )
    {
      result.0.push( input.parse()? );
    }
    Ok( result )
  }
}

// xxx : macro?
impl< T > core::ops::Deref
for Many< T >
where
  T : quote::ToTokens,
{
  type Target = Vec< T >;
  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

// xxx : macro
impl< T > From< Vec< T > > for Many< T >
where
  T : quote::ToTokens,
{
  fn from( src : Vec< T > ) -> Self
  {
    Self( src )
  }
}

impl< T > From< Many< T > > for Vec< T >
where
  T : quote::ToTokens,
{
  fn from( src : Many< T > ) -> Self
  {
    src.0
  }
}

///
/// Attribute and ident.
///

pub type AttributedIdent = Pair< Many< AttributeInner >, syn::Ident >;

impl From< syn::Ident > for AttributedIdent
{
  fn from( src : syn::Ident ) -> Self
  {
    Self( Vec::new().into(), src )
  }
}

impl From< AttributedIdent > for syn::Ident
{
  fn from( src : AttributedIdent ) -> Self
  {
    src.1
  }
}

///
/// Record.
///

#[ derive( Debug, PartialEq, Eq, Clone ) ]
pub struct Record
{
  pub attrs : Vec< syn::Attribute >,
  pub vis : Visibility,
  pub mod_token : Option< syn::token::Mod >,
  pub elements : Many< AttributedIdent >,
  // pub ident : syn::Ident,
  // pub content : Option< ( syn::token::Brace, Vec< Record > ) >,
  pub semi : Option< syn::token::Semi >,
}

//

pub fn attrs_parse_inner_single( input : ParseStream< '_ > ) -> Result< syn::Attribute >
{
  let input2;
  Ok( syn::Attribute
  {
    pound_token : input.parse()?,
    style : syn::AttrStyle::Inner( input.parse()? ),
    bracket_token : bracketed!( input2 in input ),
    path : input2.call( syn::Path::parse_mod_style )?,
    tokens : input2.parse()?,
  })
}

//

pub fn attrs_parse_inner_as_much_as_possible( input : ParseStream< '_ >, attrs : &mut Vec< syn::Attribute > ) -> Result< () >
{
  while input.peek( Token![ # ] ) && input.peek2( Token![ ! ] )
  {
    attrs.push( input.call( attrs_parse_inner_single )? );
    // attrs.push( input.call( parsing::single_parse_inner )? );
  }
  Ok( () )
}

//

impl syn::parse::Parse for Record
{
  fn parse( input : ParseStream< '_ > ) -> Result< Self >
  {

    let mut attrs = input.call( syn::Attribute::parse_outer )?;
    let vis : Visibility = input.parse()?;

    // let mod_token : Token![ mod ] = input.parse()?;

    let mod_token : Option< Token![ mod ] > = input.parse()?;

//     if lookahead.peek( syn::token::Brace )
//     {
//       let input2;
//       let brace_token = syn::braced!( input2 in input );
//       // attrs_parse_inner_as_much_as_possible( &input2, &mut attrs )?;
//       // xxx : test with attributes
//
//       let mut elements = Vec::new();
//       while !input2.is_empty()
//       {
//         elements.push( input2.parse()? );
//       }
//
//       Ok( Record
//       {
//         attrs,
//         vis,
//         mod_token,
//         elements,
//         // ident,
//         // content : Some( ( brace_token, items ) ),
//         semi : None,
//       })
//     }

    let ident : syn::Ident = input.parse()?;
    let lookahead = input.lookahead1();
    if lookahead.peek( Token![ ; ] )
    {
      Ok( Record
      {
        attrs,
        vis,
        mod_token,
        elements : vec!( ident.into() ).into(),
        // content : None,
        semi : Some( input.parse()? ),
      })
    }
//     else if lookahead.peek( syn::token::Brace )
//     {
//       let input2;
//       let brace_token = syn::braced!( input2 in input );
//       attrs_parse_inner_as_much_as_possible( &input2, &mut attrs )?;
//
//       let mut items = Vec::new();
//       while !input2.is_empty()
//       {
//         items.push( input2.parse()? );
//       }
//
//       Ok( Record
//       {
//         attrs,
//         vis,
//         mod_token,
//         ident,
//         content : Some( ( brace_token, items ) ),
//         semi : None,
//       })
//     }
    else
    {
      Err( lookahead.error() )
    }

  }
}

//

impl quote::ToTokens for Record
{
  fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
  {
    use proc_macro_tools::quote::TokenStreamExt;
    tokens.append_all( &self.attrs );
    self.vis.to_tokens( tokens );
    self.mod_token.to_tokens( tokens );
    // self.ident.to_tokens( tokens );
    // self.content.to_tokens( tokens );
    self.elements.to_tokens( tokens ); // xxx : problem
    self.semi.to_tokens( tokens );
  }
}

  // pub attrs : Vec< syn::Attribute >,
  // pub vis : Visibility,
  // pub mod_token : Option< syn::token::Mod >,
  // pub elements : Many< AttributedIdent >,
  // pub semi : Option< syn::token::Semi >,

///
/// Module-specific item.
///

#[ derive( Debug ) ]
pub struct Records
(
  pub Vec< Record >,
);

// xxx
// impl From< Records > for Records
// {
//   fn from( src : Records ) -> Self
//   {
//     Self( src.0 )
//   }
// }

//

impl syn::parse::Parse for Records
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
  {
    let mut items = vec![];
    while !input.is_empty()
    {
      let item : Record = input.parse()?;
      items.push( item );
    }
    Ok( Self( items ) )
  }
}

//

impl quote::ToTokens for Records
{
  fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
  {
    use proc_macro_tools::quote::TokenStreamExt;
    tokens.append_all( &self.0 )
  }
}

///
/// Protocol of modularity unifying interface of a module.
///

pub fn mod_interface( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  use std::collections::HashMap; /* xxx : include into prelude of wtools */

  let records = syn::parse::< Records >( input )?;

  //let mut mods = vec![];
  let mut immediates : Vec< proc_macro2::TokenStream > = vec![];

  // use inspect_type::*;
  // inspect_type_of!( immediates );

  let mut fixes : HashMap< _ , Vec< proc_macro2::TokenStream > > = HashMap::new();
  fixes.insert( VisPrivate::Kind(), Vec::new() );
  fixes.insert( VisProtected::Kind(), Vec::new() );
  fixes.insert( VisOrphan::Kind(), Vec::new() );
  fixes.insert( VisExposed::Kind(), Vec::new() );
  fixes.insert( VisPrelude::Kind(), Vec::new() );

  let mut err = None;

  records.0.iter().for_each( | record |
  {
    record.elements.iter().for_each( | element |
    {
      //mods.push( record.ident.clone() );
//      let ident = &record.ident;
      let ident = &element.1;

      if record.mod_token.is_some()
      {
        immediates.push( qt!{ pub mod #ident; } );
        let fixes = fixes.get_mut( &record.vis.kind() ).unwrap();
        fixes.push( qt!{ pub use super::#ident; } );

        // xxx : test
        if !record.vis.can_be_used_for_non_standard_mod()
        {
          err = Some( syn_err!
          (
            record, "To include a non-standard module use either [ protected, orphan, exposed, prelude ] visibility:\n  {}",
            qt!{ #record }
          ));
        }
      }
    });

  });

  if let Some( _err ) = err
  {
    return Err( _err );
  }

  let _private_fix = fixes.get( &VisPrivate::Kind() ).unwrap();
  let protected_fix = fixes.get( &VisProtected::Kind() ).unwrap();
  let orphan_fix = fixes.get( &VisOrphan::Kind() ).unwrap();
  let exposed_fix = fixes.get( &VisExposed::Kind() ).unwrap();
  let prelude_fix = fixes.get( &VisPrelude::Kind() ).unwrap();

  let result = qt!
  {

    #( #immediates )*

    /// Protected namespace of the module.
    pub mod protected
    {
      #[ doc( inline ) ]
      pub use super::orphan::*;

      #( #protected_fix )*

    }

    #[ doc( inline ) ]
    pub use protected::*;

    /// Orphan namespace of the module.
    pub mod orphan
    {
      #[ doc( inline ) ]
      pub use super::exposed::*;

      #( #orphan_fix )*

    }

    /// Exposed namespace of the module.
    pub mod exposed
    {
      #[ doc( inline ) ]
      pub use super::prelude::*;

      #( #exposed_fix )*

    }

    /// Prelude to use: `use wtools::prelude::*`.
    pub mod prelude
    {

      #( #prelude_fix )*

    }

  };

  Ok( result )
}

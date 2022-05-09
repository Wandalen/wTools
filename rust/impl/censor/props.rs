
// pub use werror::*;
use std::collections::HashMap;

//

///
/// Parse properties.
///

pub trait PropsParseOptionsAdapter
{
  /// Parse from splits.
  fn parse_from_splits< I >( &self, mut _splits : I ) -> HashMap< Box< str >, Box< str > >
  where
    I : core::iter::Iterator,
    < I as Iterator >::Item : std::fmt::Display,
    < I as Iterator >::Item : AsRef< str >,
  {
    let result : HashMap< Box< str >, Box< str > > = HashMap::new();
    result
  }
}

///
/// Properties parsing options.
///

#[ derive( Debug, PartialEq ) ]
pub struct PropsParseOptions
{
  // result : HashMap< Box< str >, Box< str > >,
}

impl PropsParseOptions
{
  /// Create new parsing properties.
  pub fn new() -> Self
  {
    Self
    {
    }
  }
}

impl PropsParseOptionsAdapter for PropsParseOptions
{
}

//

///
/// Parse properties from splits.
///

pub fn parse_from_splits< I >( splits : I ) -> HashMap< Box< str >, Box< str > >
where
  < I as Iterator >::Item : std::fmt::Display,
  < I as Iterator >::Item : AsRef< str >,
  I : core::iter::Iterator,
{
  let options = PropsParseOptions::new();
  options.parse_from_splits( splits )
}

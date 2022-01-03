// pub use werror::*;
use std::collections::HashMap;

//

pub trait PropsParseOptionsAdapter
{
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

pub struct PropsParseOptions
{
  // result : HashMap< Box< str >, Box< str > >,
}

impl PropsParseOptions
{
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

pub fn parse_from_splits< I >( splits : I ) -> HashMap< Box< str >, Box< str > >
where
  < I as Iterator >::Item : std::fmt::Display,
  < I as Iterator >::Item : AsRef< str >,
  I : core::iter::Iterator,
{
  let options = PropsParseOptions::new();
  options.parse_from_splits( splits )
}

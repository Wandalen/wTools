
use super::*;

//

#[ test ]
fn basic()
{

  let item : syn::Item = syn::parse_quote!
  {
    pub struct Struct1< 'a, Context, Formed >
    {
      f1 : int32,
    }
  };

  let exp : syn::Item = syn::parse_quote!
  {
    pub struct Struct1FormerDefinitionTypes< 'a, Context, Formed >
    {
      f1 : int32,
      _phantom : core::marker::PhantomData< ( &'a(), Context, Formed ) >,
    }
  }

  // let got = item::phantom_add( item );
  // a_id!( got, exp );

}

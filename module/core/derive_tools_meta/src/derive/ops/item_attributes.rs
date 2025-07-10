use macro_tools::
{
  Result,
  syn,
};

///
/// Attributes of item.
///
#[ derive( Debug, Default ) ]
pub struct ItemAttributes
{
  /// An optional type representing the error type associated with the item, if specified in the attributes.
  pub error: Option< syn::Type >,
}

impl ItemAttributes
{
  ///
  /// Parse attributes.
  ///
  pub fn from_attrs<'a>( attrs : impl Iterator< Item = &'a syn::Attribute > ) -> Result< Self >
  where
    Self : Sized,
  {
    let mut result = Self::default();

    for attr in attrs
    {
      if attr.path().is_ident( "add" )
      {
        attr.parse_nested_meta( | meta |
        {
          if meta.path.is_ident( "error" ) 
          {
            let value = meta.value()?;
            let parsed : syn::Type = value.parse()?;
            result.error = Some( parsed );
         }
         Ok( () )
        })?;
      }
      else
      {
        // qqq : unknown attribute, but it is not an error, because it can be an attribute for other derive.
      }
    }

    Ok( result )
  }
}
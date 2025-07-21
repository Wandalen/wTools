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
  pub error_type: Option< syn::Type >,
  /// Full expr, if `MyError::DifferentVariants` is provided
  pub error_expr: Option< syn::Expr >,
}

impl ItemAttributes
{
  ///
  /// Parse attributes.
  ///
  pub fn from_attrs< 'a >( attrs : impl Iterator< Item = &'a syn::Attribute > ) -> Result< Self >
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
            let expr : syn::Expr = value.parse()?;
            result.error_expr = Some( expr.clone() );

            if result.error_type.is_none() {
              if let Some( ty ) = extract_type_from_expr( &expr ) 
              {
                result.error_type = Some( ty );
              }
            }
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

pub fn extract_type_from_expr( expr : &syn::Expr ) -> Option< syn::Type > 
{
    match expr 
    {
        syn::Expr::Path( expr_path ) => 
        {
            let segments = &expr_path.path.segments;

            if segments.is_empty() 
            {
                None
            }
            else 
            {
                let first = segments.first().cloned()?;
                let mut new_path = syn::Path 
                {
                  leading_colon : None,
                  segments : syn::punctuated::Punctuated::new(),
                };
                new_path.segments.push( first );

                Some( syn::Type::Path( syn::TypePath 
                {
                  qself: None,
                  path: new_path,
                }))
            } 
        },
        _ => None,
    }
}
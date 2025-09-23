use macro_tools :: { Result, syn };

use macro_tools :: { AttributePropertyOptionalSingletone };

///
/// Attributes of item.
///
#[ derive( Debug, Default ) ]
pub struct ItemAttributes 
{
  ///
  /// If true, the macro will not be applied.
  ///
  pub skip: AttributePropertyOptionalSingletone,
  ///
  /// If true, the macro will be applied.
  ///
  pub enabled: AttributePropertyOptionalSingletone,
  ///
  /// If true, print debug output.
  ///
  pub debug: AttributePropertyOptionalSingletone,
  ///
  /// If true, the macro will be applied.
  ///
  pub on: AttributePropertyOptionalSingletone,
}

impl ItemAttributes 
{
  ///
  /// Parse attributes.
  ///
  pub fn from_attrs< 'a >(attrs: impl Iterator< Item = &'a syn ::Attribute >) -> Result< Self >
  where
  Self: Sized,
  {
  let mut result = Self ::default();

  for attr in attrs 
  {
   if attr.path().is_ident("from") 
   {
  attr.parse_nested_meta(|meta| {
   if meta.path.is_ident("on") 
   {
  result.on = AttributePropertyOptionalSingletone ::from(true);
 } else  if meta.path.is_ident("debug") 
  {
  result.debug = AttributePropertyOptionalSingletone ::from(true);
 } else  if meta.path.is_ident("enabled") 
  {
  result.enabled = AttributePropertyOptionalSingletone ::from(true);
 } else  if meta.path.is_ident("skip") 
  {
  result.skip = AttributePropertyOptionalSingletone ::from(true);
 } else {
  // qqq: unknown attribute, but it is not an error, because it can be an attribute for other derive.
  // syn_err!( meta.path.span(), "Unknown attribute `#[ from( {} ) ]`", meta.path.to_token_stream() );
 }
   Ok(())
 })?;
 } else {
  // qqq: unknown attribute, but it is not an error, because it can be an attribute for other derive.
 }
 }

  Ok(result)
 }
}

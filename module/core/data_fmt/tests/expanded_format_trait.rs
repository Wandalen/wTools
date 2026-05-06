//! Tests: `ExpandedFormatter` via `Format` trait.

#[ cfg( feature = "enabled" ) ]
mod tests
{
  use data_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig, Format };

  #[ test ]
  fn test_expanded_postgres_style_via_format_trait()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();
    let output = Format::format( &ExpandedFormatter::new(), &view ).unwrap();
    assert!( output.contains( "-[ RECORD 1 ]" ) && output.contains( "Name | Alice" ) );
    assert!( output.contains( "Age  | 25" ), "shorter keys must be padded" );
  }

  #[ test ]
  fn test_expanded_property_style_via_format_trait()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Score".into() ] )
      .add_row( vec![ "Charlie".into(), "99".into() ] ).build_view();
    let formatter = ExpandedFormatter::with_config( ExpandedConfig::property_style() );
    let output = Format::format( &formatter, &view ).unwrap();
    assert!( output.contains( "Name:" ) && output.contains( "Charlie" ) );
  }
}

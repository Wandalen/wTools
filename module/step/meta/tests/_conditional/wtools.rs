
/// Mechanism to include tests only to terminal crate.
/// It exclude code in terminal module ( crate ), but include for aggregating module ( crate ).
#[ macro_export ]
macro_rules! only_for_terminal_module
{
  ( $( $Any : tt )* ) =>
  {
  }
}

/// Mechanism to include tests only to aggregating crate.
/// It exclude code in terminal module ( crate ), but include for aggregating module ( crate ).
#[ macro_export ]
macro_rules! only_for_aggregating_module
{
  ( $( $Any : tt )* ) =>
  {
    $( $Any )*
  }
}

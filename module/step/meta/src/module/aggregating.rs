
/// This macro allows including the passed tokens in an aggregating module. 
/// It does not restrict itself to any specific context and simply inserts 
/// the provided tokens, making it useful for any code location.
#[ macro_export ]
macro_rules! only_for_terminal_module
{
  ( $( $Any : tt )* ) =>
  {
  }
}

/// This macro allows including the passed tokens in an aggregating module. 
/// It does not restrict itself to any specific context and simply inserts 
/// the provided tokens, making it useful for any code location.
#[ macro_export ]
macro_rules! only_for_aggregating_module
{
  ( $( $Any : tt )* ) =>
  {
    $( $Any )*
  }
}

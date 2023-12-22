
/// This macro allows including the passed tokens in an aggregating module. 
/// It does not restrict itself to any specific context and simply inserts 
/// the provided tokens, making it useful for any code location.
/// 
/// This macro is designed for scenarios where certain code should only be available 
/// within the context of the terminal module. It serves as a marker for code that is 
/// exclusive to the terminal module and allows for better organization and control of 
/// functionality specific to this module.
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
/// 
/// This macro is versatile and can be used to include code in any module, 
/// providing a mechanism for aggregating functionality across different parts of the codebase. 
/// It allows for the inclusion of code without context-specific restrictions.
#[ macro_export ]
macro_rules! only_for_aggregating_module
{
  ( $( $Any : tt )* ) =>
  {
    $( $Any )*
  }
}

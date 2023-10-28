
#[ macro_export ]
macro_rules! only_for_terminal_module
{
( $( $Any : tt )* ) =>
  {
    $( $Any )*
  };
}

#[ macro_export ]
macro_rules! only_for_aggregating_module
{
  ( $( $Any : tt )* ) =>
  {
  }
}

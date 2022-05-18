
#[ macro_export ]
macro_rules! only_for_local_module
{
  ( $( $Any : tt )* ) =>
  {
    $( $Any )*
    ;
  }
}

#[ macro_export ]
macro_rules! only_for_wtools
{
  ( $( $Any : tt )* ) =>
  {
  }
}

// pub use only_for_local_module;
// pub use only_for_wtools;


#[ macro_export ]
macro_rules! only_for_local_module
{
  ( $( $Any : tt )* ) =>
  {
  }
}

#[ macro_export ]
macro_rules! only_for_wtools
{
  ( $( $Any : tt )* ) =>
  {
    $( $Any )*
    ;
  }
}

// pub use only_for_local_module;
// pub use only_for_wtools;

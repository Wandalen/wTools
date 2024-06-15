/// Internal namespace.
pub( crate ) mod private
{

  #[ doc( inline ) ]
  pub use proper_path_tools::
  {
    AbsolutePath,
    path::
    {
      canonicalize,
      unique_folder_name,
    },
  };

}

crate::mod_interface!
{
  protected use canonicalize;
  protected use unique_folder_name;
  protected use AbsolutePath;
}

/// Internal namespace.
pub( crate ) mod private
{

  #[ doc( inline ) ]
  pub use proper_path_tools::
  {
    AbsolutePath,
    Utf8Path,
    Utf8PathBuf,
    path::protected::*,
    // {
    //   canonicalize,
    //   unique_folder_name,
    // },
  };

}

crate::mod_interface!
{
  protected use canonicalize;
  protected use normalize;
  protected use unique_folder_name;
  exposed use AbsolutePath;
  exposed use Utf8PathBuf;
  exposed use Utf8Path;
}

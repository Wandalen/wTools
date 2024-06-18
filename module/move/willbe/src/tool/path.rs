/// Internal namespace.
pub( crate ) mod private
{

  // #[ doc( inline ) ]
  // pub use proper_path_tools::
  // {
  //   AbsolutePath,
  //   Utf8Path,
  //   Utf8PathBuf,
  //   path::protected::*,
  // };

}

crate::mod_interface!
{

  use ::proper_path_tools;
  protected use ::proper_path_tools::protected::*;

  // protected use canonicalize;
  // protected use normalize;
  // protected use unique_folder_name;
  // exposed use AbsolutePath;
  // exposed use Utf8PathBuf;
  // exposed use Utf8Path;

}

// use unique_folder_name as xxx;
// use unique_folder_name2 as yyy;

/// Define a private namespace for all its items.
mod private
{
}

crate ::mod_interface!
{
  exposed use ::pth :: { AbsolutePath, Utf8Path, Utf8PathBuf, unique_folder_name, normalize, CurrentPath, TransitiveTryFrom };
  exposed use ::std ::path :: { PathBuf, Path };
}

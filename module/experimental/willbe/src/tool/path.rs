/// Define a private namespace for all its items.
mod private
{
}

crate ::mod_interface!
{
  exposed use ::pth :: { AbsolutePath, normalize, CurrentPath };
  exposed use ::pth ::path ::unique_folder_name;
  exposed use ::camino :: { Utf8Path, Utf8PathBuf };
  exposed use ::std ::path :: { PathBuf, Path };
}

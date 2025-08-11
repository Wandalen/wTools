// module/core/pth/src/lib.rs
#![cfg_attr(feature = "no_std", no_std)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png")]
#![doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
)]
#![doc(html_root_url = "https://docs.rs/pth/latest/pth/")]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Path utilities" ) ]
#![allow(clippy::std_instead_of_alloc, clippy::std_instead_of_core)]

#[ cfg( feature = "enabled" ) ]
use ::mod_interface::mod_interface;

#[ cfg( feature = "no_std" ) ]
#[ macro_use ]
extern crate alloc;

// qqq : xxx : implement `pth::absolute::join` function or add option to `pth::path::join`
//       Desired Signature Idea 1: `pub fn join<T1, T2>(p1: T1, p2: T2) -> io::Result< AbsolutePath >` (extendable for more args or tuples)
//       Desired Signature Idea 2: `pub fn join<Paths: PathJoined>(paths: Paths, options: JoinOptions) -> io::Result< AbsolutePath >` where JoinOptions includes absolute handling.
//       Behavior:
//       1. Takes multiple path-like items (e.g., via tuple, slice, or multiple args).
//       2. Finds the rightmost item that represents an absolute path.
//       3. If an absolute path is found, it joins all path segments *from that absolute path onwards*.
//       4. If *no* absolute path is found, it joins *all* segments relative to the current working directory (implicitly using `CurrentPath` if needed).
//       5. The final joined path must be canonicalized and returned as an `AbsolutePath`.
//       6. Return an `io::Error` if input is invalid or joining/canonicalization fails.
//       Examples (assuming CurrentPath resolves relative paths):
//       - `pth::absolute::join("/abs/a", "rel/b")` -> `Ok(AbsolutePath::from("/abs/a/rel/b"))`
//       - `pth::absolute::join("rel/a", "/abs/b", "rel/c")` -> `Ok(AbsolutePath::from("/abs/b/rel/c"))`
//       - `pth::absolute::join("rel/a", "/abs/b", "/abs/c", "rel/d")` -> `Ok(AbsolutePath::from("/abs/c/rel/d"))`
//       - `pth::absolute::join("rel/a", "rel/b")` -> `Ok(AbsolutePath::from(current_dir.join("rel/a/rel/b")))`
//       - `pth::absolute::join("/abs/a/..", "b")` -> `Ok(AbsolutePath::from("/b"))`

/// Own namespace of the module. Contains items public within this layer, but not propagated.
mod private {}

#[ cfg( feature = "enabled" ) ]
mod_interface! {

  /// Basic functionality.
  layer path;

  /// `AsPath` trait.
  layer as_path;
  /// `TryIntoPath` trait.
  layer try_into_path;
  /// `TryIntoPath` trait.
  layer try_into_cow_path;

  /// Transitive `TryFrom` and `TryInto`.
  layer transitive;

  #[ cfg( feature = "path_utf8" ) ]
  own use ::camino::{ Utf8Path, Utf8PathBuf };

  // #[ cfg( not( feature = "no_std" ) ) ]
  // own use ::std::path::{ PathBuf, Path, Component };

  #[ cfg( not( feature = "no_std" ) ) ]
  own use ::std::path::*;

  #[ cfg( not( feature = "no_std" ) ) ]
  own use ::std::borrow::Cow;

}

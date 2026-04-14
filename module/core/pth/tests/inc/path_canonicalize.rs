#[ allow( unused_imports ) ]
use super :: *;
use std ::path ::PathBuf;
use the_module ::path;

/// Tests platform-specific path absoluteness behavior.
///
/// Path absoluteness is platform-dependent:
/// - **Windows**: "c: /src/" is relative (drive-relative), "/c/src/" is absolute (rooted)
/// - **Unix/Linux**: All paths starting with "/" are absolute, "c: /src/" treated as absolute
#[ test ]
fn assumptions()
{
  #[ cfg( target_os = "windows" ) ]
  {
    // Windows: "c: /src/" is relative (drive-relative path)
    assert!( !PathBuf ::from( "c: /src/" ).is_absolute() );

    // Windows: "/c/src/" is absolute (rooted at current drive)
    assert!( PathBuf ::from( "/c/src/" ).is_absolute() );

    // Windows: "/c: /src/" is absolute (rooted path)
    assert!( PathBuf ::from( "/c: /src/" ).is_absolute() );
  }

  #[ cfg( not( target_os = "windows" ) ) ]
  {
    // Unix/Linux: paths starting with "/" are always absolute
    assert!( PathBuf ::from( "/c/src/" ).is_absolute() );
    assert!( PathBuf ::from( "/c: /src/" ).is_absolute() );

    // Unix/Linux: "c: /src/" is NOT absolute - it's a relative path
    // with unusual characters (colon and space) in the name
    assert!( !PathBuf ::from( "c: /src/" ).is_absolute() );
  }
}

#[ test ]
fn basic()
{
  let got = path ::normalize_unchecked(PathBuf ::from("src"));
  let exp = PathBuf ::from("src");
  assert_eq!(got, exp);

  let got = path ::normalize_unchecked(PathBuf ::from("\\src"));
  let exp = PathBuf ::from("\\src");
  assert_eq!(got, exp);

  let got = path ::normalize_unchecked(PathBuf ::from("\\src\\"));
  let exp = PathBuf ::from("\\src\\");
  assert_eq!(got, exp);

  let got = path ::normalize_unchecked(PathBuf ::from("/src"));
  let exp = PathBuf ::from("/src");
  assert_eq!(got, exp);

  let got = path ::normalize_unchecked(PathBuf ::from("/src/"));
  let exp = PathBuf ::from("/src/");
  assert_eq!(got, exp);

  let got = path ::normalize_unchecked(PathBuf ::from("./src/"));
  let exp = PathBuf ::from("./src/");
  assert_eq!(got, exp);
}

/// Tests Windows-specific drive letter path normalization.
///
/// This test only runs on Windows as it tests Windows-specific path behavior.
#[ test ]
#[ cfg( target_os = "windows" ) ]
fn windows_drive_letter_normalization()
{
  // Windows drive letter with space should be normalized
  let got = path ::normalize_unchecked( PathBuf ::from( "c: /src/" ) );
  let exp = PathBuf ::from( "/c/src/" );
  assert_eq!( got, exp );
}

/// Tests Unix/Linux path normalization behavior.
///
/// On Unix systems, "c: /src/" is just a path with unusual characters,
/// not a Windows drive letter.
#[ test ]
#[ cfg( not( target_os = "windows" ) ) ]
fn unix_path_normalization()
{
  // On Unix, "c: /src/" is treated as "/src/" (the "c: " part is not special)
  // The normalize_unchecked function should handle this gracefully
  let got = path ::normalize_unchecked( PathBuf ::from( "c: /src/" ) );
  // On Unix this becomes "c: /src/" as-is (no drive letter transformation)
  let exp = PathBuf ::from( "c: /src/" );
  assert_eq!( got, exp );
}

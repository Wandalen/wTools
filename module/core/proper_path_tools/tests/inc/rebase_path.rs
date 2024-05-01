#[ allow( unused_imports ) ]
use super::*;
use std::path::PathBuf;

#[ test ]
fn test_rebase_without_old_path() 
{
  let file_path = "/home/user/documents/file.txt";
  let new_path = "/mnt/storage";
  let rebased_path = the_module::path::rebase( &file_path, &new_path, None ).unwrap();
  assert_eq!
  (
    rebased_path,
    PathBuf::from( "/mnt/storage/home/user/documents/file.txt" )
  );
}

#[ test ]
fn test_rebase_with_old_path() 
{
  let file_path = "/home/user/documents/file.txt";
  let new_path = "/mnt/storage";
  let old_path = "/home/user";
  let rebased_path = the_module::path::rebase( &file_path, &new_path, Some( &old_path ) ).unwrap();
  assert_eq!
  (
    rebased_path,
    PathBuf::from( "/mnt/storage/documents/file.txt" )
  );
}

#[ test ]
fn test_rebase_invalid_old_path() 
{
  let file_path = "/home/user/documents/file.txt";
  let new_path = "/mnt/storage";
  let old_path = "/tmp"; 
  let rebased_path = the_module::path::rebase( &file_path, &new_path, Some( &old_path ) ).unwrap();
  assert_eq!
  (
    rebased_path,
    PathBuf::from( "/mnt/storage/home/user/documents/file.txt" )
  );
}

#[ test ]
fn test_rebase_non_ascii_paths() 
{
  let file_path = "/home/пользователь/documents/файл.txt"; // Non-ASCII file path
  let new_path = "/mnt/存储"; // Non-ASCII new base path
  let rebased_path = the_module::path::rebase( &file_path, &new_path, None ).unwrap();
  assert_eq!
  (
    rebased_path,
    PathBuf::from( "/mnt/存储/home/пользователь/documents/файл.txt" )
  );
}
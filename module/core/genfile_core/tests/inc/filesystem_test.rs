/// Tests for `FileSystem` trait and implementations (FR13, FR14, FR15)
use super :: *;
use std ::path ::PathBuf;

//

#[ test ]
fn filesystem_trait_is_implementable()
{
  // FR13: FileSystem trait must be implementable
  struct TestFs;

  impl FileSystem for TestFs
  {
    fn read( &self, _path: &std ::path ::Path ) -> Result< String, Error >
    {
      Ok( "test content".to_string() )
    }

    fn write( &mut self, _path: &std ::path ::Path, _content: &str ) -> Result< (), Error >
    {
      Ok(())
    }

    fn exists( &self, _path: &std ::path ::Path ) -> bool
    {
      true
    }
  }

  let fs = TestFs;
  let result = fs.read( &PathBuf ::from( "test.txt" ) );
  assert!( result.is_ok() );
}

#[ test ]
fn memory_filesystem_reads_and_writes()
{
  // FR14: MemoryFileSystem must store files in HashMap
  let mut fs = MemoryFileSystem ::new();

  let path = PathBuf ::from( "test.txt" );
  let content = "Hello, World!";

  // Write should succeed
  let write_result = fs.write( &path, content );
  assert!( write_result.is_ok() );

  // Read should return written content
  let read_result = fs.read( &path );
  assert!( read_result.is_ok() );
  assert_eq!( read_result.unwrap(), content );
}

#[ test ]
fn memory_filesystem_exists_check()
{
  // FR14: MemoryFileSystem must implement exists()
  let mut fs = MemoryFileSystem ::new();

  let path = PathBuf ::from( "test.txt" );

  // Should not exist initially
  assert!( !fs.exists( &path ) );

  // Should exist after writing
  fs.write( &path, "content" ).unwrap();
  assert!( fs.exists( &path ) );
}

#[ test ]
fn memory_filesystem_read_nonexistent_returns_error()
{
  // FR14: Reading nonexistent file should return error
  let fs = MemoryFileSystem ::new();

  let result = fs.read( &PathBuf ::from( "nonexistent.txt" ) );
  assert!( result.is_err() );

  match result.unwrap_err()
  {
    Error ::Fs( _ ) => {},
    other => panic!( "Expected Error::Fs, got {other:?}" ),
  }
}

#[ test ]
fn real_filesystem_can_be_created()
{
  // FR15: RealFileSystem must be creatable
  let _fs = RealFileSystem ::new();
}

#[ test ]
fn filesystem_trait_has_read_method()
{
  // FR13: FileSystem must have read() method
  let fs = MemoryFileSystem ::new();

  // Type signature check - should return Result<String, Error>
  let _result: Result< String, Error > = fs.read( &PathBuf ::from( "test.txt" ) );
}

#[ test ]
fn filesystem_trait_has_write_method()
{
  // FR13: FileSystem must have write() method
  let mut fs = MemoryFileSystem ::new();

  // Type signature check - should accept path and content, return Result<(), Error>
  let _result: Result< (), Error > = fs.write( &PathBuf ::from( "test.txt" ), "content" );
}

#[ test ]
fn filesystem_trait_has_exists_method()
{
  // FR13: FileSystem must have exists() method
  let fs = MemoryFileSystem ::new();

  // Type signature check - should return bool
  let _exists: bool = fs.exists( &PathBuf ::from( "test.txt" ) );
}

#[ test ]
fn memory_filesystem_multiple_files()
{
  // FR14: MemoryFileSystem should handle multiple files
  let mut fs = MemoryFileSystem ::new();

  fs.write( &PathBuf ::from( "file1.txt" ), "content1" ).unwrap();
  fs.write( &PathBuf ::from( "file2.txt" ), "content2" ).unwrap();

  assert_eq!( fs.read( &PathBuf ::from( "file1.txt" ) ).unwrap(), "content1" );
  assert_eq!( fs.read( &PathBuf ::from( "file2.txt" ) ).unwrap(), "content2" );
}

#[ test ]
fn memory_filesystem_overwrite_existing_file()
{
  // FR14: Writing to existing path should overwrite
  let mut fs = MemoryFileSystem ::new();

  let path = PathBuf ::from( "test.txt" );
  fs.write( &path, "original" ).unwrap();
  fs.write( &path, "updated" ).unwrap();

  assert_eq!( fs.read( &path ).unwrap(), "updated" );
}

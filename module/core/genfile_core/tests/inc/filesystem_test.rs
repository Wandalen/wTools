/// Tests for `FileSystem` trait and implementations (docs/feature/010, docs/feature/011, docs/feature/012)
use super :: *;
use std ::path ::PathBuf;
use std ::time ::{ SystemTime, UNIX_EPOCH };

//

#[ test ]
fn filesystem_trait_is_implementable()
{
  // docs/feature/010: FileSystem trait must be implementable
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
  // docs/feature/011: MemoryFileSystem must store files in HashMap
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
  // docs/feature/011: MemoryFileSystem must implement exists()
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
  // docs/feature/011: Reading nonexistent file should return error
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
  // docs/feature/012: RealFileSystem must be creatable
  let _fs = RealFileSystem ::new();
}

#[ test ]
fn filesystem_trait_has_read_method()
{
  // docs/feature/010: FileSystem must have read() method
  let fs = MemoryFileSystem ::new();

  // Type signature check - should return Result<String, Error>
  let _result: Result< String, Error > = fs.read( &PathBuf ::from( "test.txt" ) );
}

#[ test ]
fn filesystem_trait_has_write_method()
{
  // docs/feature/010: FileSystem must have write() method
  let mut fs = MemoryFileSystem ::new();

  // Type signature check - should accept path and content, return Result<(), Error>
  let _result: Result< (), Error > = fs.write( &PathBuf ::from( "test.txt" ), "content" );
}

#[ test ]
fn filesystem_trait_has_exists_method()
{
  // docs/feature/010: FileSystem must have exists() method
  let fs = MemoryFileSystem ::new();

  // Type signature check - should return bool
  let _exists: bool = fs.exists( &PathBuf ::from( "test.txt" ) );
}

#[ test ]
fn memory_filesystem_multiple_files()
{
  // docs/feature/011: MemoryFileSystem should handle multiple files
  let mut fs = MemoryFileSystem ::new();

  fs.write( &PathBuf ::from( "file1.txt" ), "content1" ).unwrap();
  fs.write( &PathBuf ::from( "file2.txt" ), "content2" ).unwrap();

  assert_eq!( fs.read( &PathBuf ::from( "file1.txt" ) ).unwrap(), "content1" );
  assert_eq!( fs.read( &PathBuf ::from( "file2.txt" ) ).unwrap(), "content2" );
}

#[ test ]
fn memory_filesystem_overwrite_existing_file()
{
  // docs/feature/011: Writing to existing path should overwrite
  let mut fs = MemoryFileSystem ::new();

  let path = PathBuf ::from( "test.txt" );
  fs.write( &path, "original" ).unwrap();
  fs.write( &path, "updated" ).unwrap();

  assert_eq!( fs.read( &path ).unwrap(), "updated" );
}

#[ test ]
fn real_file_system_write_creates_parent_dirs_and_file()
{
  // docs/feature/011: RealFileSystem::write() must create parent directories before writing
  let ts = SystemTime ::now().duration_since( UNIX_EPOCH ).unwrap().subsec_nanos();
  let root = std ::env ::temp_dir().join( format!( "genfile_test_{ts}" ) );
  let nested = root.join( "a" ).join( "b" ).join( "output.txt" );

  let mut fs = RealFileSystem ::new();
  fs.write( &nested, "hello" ).expect( "write should create parents and file" );

  assert!( nested.exists(), "file should exist on disk after write" );
  assert!( fs.exists( &nested ) );

  std ::fs ::remove_dir_all( &root ).ok();
}

#[ test ]
fn real_file_system_read_returns_written_bytes()
{
  // docs/feature/011: RealFileSystem::read() must return byte-for-byte identical content
  let ts = SystemTime ::now().duration_since( UNIX_EPOCH ).unwrap().subsec_nanos();
  let path = std ::env ::temp_dir().join( format!( "genfile_rw_{ts}.txt" ) );

  let content = "line1\nline2\nspecial: <>&\"'\n";

  let mut fs = RealFileSystem ::new();
  fs.write( &path, content ).unwrap();
  let read_back = fs.read( &path ).unwrap();

  assert_eq!( read_back, content );

  std ::fs ::remove_file( &path ).ok();
}

#[ test ]
fn memory_file_system_create_directory_all_is_noop()
{
  // docs/feature/012: MemoryFileSystem stores only file paths — writing a nested path
  // must NOT create phantom directory entries accessible via read()
  // (MemoryFileSystem has no create_directory_all method; this no-op behavior is verified
  //  by confirming the parent path is not stored as a separate entry in the in-memory map)
  let mut fs = MemoryFileSystem ::new();

  let nested = PathBuf ::from( "a/b/c.txt" );
  fs.write( &nested, "data" ).unwrap();

  // The file itself is readable
  assert_eq!( fs.read( &nested ).unwrap(), "data" );

  // Parent path "a/b" is NOT a stored entry — read returns error
  let parent = PathBuf ::from( "a/b" );
  assert!( fs.read( &parent ).is_err(), "MemoryFileSystem must not create phantom directory entries" );
}

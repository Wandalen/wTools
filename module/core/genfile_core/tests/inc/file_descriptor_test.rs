/// Tests for `FileDescriptor` and `WriteMode` (FR10, FR11, FR12)
use super :: *;
use std ::path ::PathBuf;

//

#[ test ]
fn file_descriptor_holds_paths()
{
  // FR10: FileDescriptor must hold file_path and template_path
  let file_path = PathBuf ::from( "/output/test.txt" );
  let template_path = PathBuf ::from( "/templates/test.hbs" );
  let write_mode = WriteMode ::Rewrite;

  let descriptor = FileDescriptor
  {
    file_path: file_path.clone(),
    template_path: template_path.clone(),
    write_mode,
  };

  assert_eq!( descriptor.file_path, file_path );
  assert_eq!( descriptor.template_path, template_path );
}

#[ test ]
fn file_descriptor_holds_write_mode()
{
  // FR10: FileDescriptor must hold write_mode
  let descriptor = FileDescriptor
  {
    file_path: PathBuf ::from( "output.txt" ),
    template_path: PathBuf ::from( "template.hbs" ),
    write_mode: WriteMode ::Rewrite,
  };

  match descriptor.write_mode
  {
    WriteMode ::Rewrite => {},
    WriteMode ::TomlExtend => panic!( "Expected Rewrite mode" ),
  }
}

#[ test ]
fn write_mode_has_rewrite_variant()
{
  // FR11: WriteMode must have Rewrite variant
  let mode = WriteMode ::Rewrite;

  match mode
  {
    WriteMode ::Rewrite => {},
    WriteMode ::TomlExtend => panic!( "Expected Rewrite variant" ),
  }
}

#[ test ]
fn write_mode_has_toml_extend_variant()
{
  // FR11: WriteMode must have TomlExtend variant
  let mode = WriteMode ::TomlExtend;

  match mode
  {
    WriteMode ::TomlExtend => {},
    WriteMode ::Rewrite => panic!( "Expected TomlExtend variant" ),
  }
}

#[ test ]
fn write_mode_clone()
{
  // WriteMode should be cloneable
  let mode1 = WriteMode ::Rewrite;
  let mode2 = mode1.clone();

  match mode2
  {
    WriteMode ::Rewrite => {},
    WriteMode ::TomlExtend => panic!( "Clone failed" ),
  }
}

#[ test ]
fn file_descriptor_clone()
{
  // FileDescriptor should be cloneable
  let descriptor1 = FileDescriptor
  {
    file_path: PathBuf ::from( "test.txt" ),
    template_path: PathBuf ::from( "template.hbs" ),
    write_mode: WriteMode ::Rewrite,
  };

  let descriptor2 = descriptor1.clone();
  assert_eq!( descriptor1.file_path, descriptor2.file_path );
  assert_eq!( descriptor1.template_path, descriptor2.template_path );
}

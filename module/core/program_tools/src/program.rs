/// Internal namespace.
pub( crate ) mod private
{

  use former::Former;
  use std::
  {
    path::{ Path, PathBuf },
    // process::Command,
  };

  #[ derive( Debug, Default, Former ) ]
  pub struct SourceFile
  {
    file_path : PathBuf,
    data : GetData,
  }

  #[ derive( Debug, Default, Former ) ]
  pub struct Entry
  {
    source_file : SourceFile,
    typ : EntryType,
  }

  #[ derive( Debug, Default, Former ) ]
  pub struct CargoFile
  {
    file_path : PathBuf,
    data : GetData,
  }

  #[ derive( Debug, Default, Former ) ]
  // #[ debug ]
  pub struct Program
  {
    write_path : Option< PathBuf >,
    read_path : Option< PathBuf >,
    #[ subform_entry( name = entry ) ]
    entries : Vec< Entry >,
    #[ subform_entry( name = source ) ]
    sources : Vec< SourceFile >,
    cargo_file : Option< CargoFile >,
  }

  #[ derive( Debug, Default, Former ) ]
  pub struct ProgramPlan
  {
    // #[ embed ]
    program : Program,
    calls : Vec< ProgramCall >,
  }

  #[ derive( Debug ) ]
  pub enum GetData
  {
    FromStr( &'static str ),
    FromBin( &'static [ u8 ] ),
    FromFile( PathBuf ),
    FromString( String ),
  }

  impl Default for GetData
  {
    fn default() -> Self
    {
      GetData::FromStr( "" )
    }
  }

  #[ derive( Debug, Default ) ]
  pub struct ProgramCall
  {
    action : ProgramAction,
    current_path : Option< PathBuf >,
    args : Vec< String >,
    index_of_entry : i32,
  }

  #[ derive( Debug, Default ) ]
  pub enum ProgramAction
  {
    #[ default ]
    Run,
    Build,
    Test,
  }

  #[ derive( Debug, Default ) ]
  pub enum EntryType
  {
    #[ default ]
    Bin,
    Lib,
    Test,
  }

}

crate::mod_interface!
{
  // protected use run;
}

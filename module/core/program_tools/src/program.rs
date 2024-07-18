/// Internal namespace.
pub( crate ) mod private
{

  use former::Former;
  use std::
  {
    path::{ Path, PathBuf },
    // process::Command,
  };

  // xxx2 : get completed

  #[ derive( Debug, Default, Former ) ]
  // #[ debug ]
  pub struct Program
  {
    pub write_path : Option< PathBuf >,
    pub read_path : Option< PathBuf >,
    #[ subform_entry( name = entry ) ]
    pub entries : Vec< Entry >,
    #[ subform_entry( name = source ) ]
    pub sources : Vec< SourceFile >,
    pub cargo_file : Option< CargoFile >,
  }

  #[ derive( Debug, Default, Former ) ]
  pub struct Plan
  {
    #[ subform_scalar ]
    pub program : Program,
    pub calls : Vec< Call >,
  }

  #[ derive( Debug, Default ) ]
  pub struct Call
  {
    pub action : Action,
    pub current_path : Option< PathBuf >,
    pub args : Vec< String >,
    pub index_of_entry : i32,
  }

  #[ derive( Debug, Default ) ]
  pub enum Action
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

  #[ derive( Debug, Default, Former ) ]
  pub struct Entry
  {
    source_file : SourceFile,
    typ : EntryType,
  }

  #[ derive( Debug, Default, Former ) ]
  pub struct SourceFile
  {
    file_path : PathBuf,
    data : GetData,
  }

  #[ derive( Debug, Default, Former ) ]
  pub struct CargoFile
  {
    file_path : PathBuf,
    data : GetData,
  }

  #[ derive( Debug ) ]
  pub enum GetData
  {
    FromStr( &'static str ),
    FromBin( &'static [ u8 ] ),
    FromFile( PathBuf ),
    FromString( String ),
  }

  impl From< &'static str > for GetData
  {
    #[ inline ]
    fn from( src : &'static str ) -> Self
    {
      Self::FromStr( src )
    }
  }

  impl From< &'static [ u8 ] > for GetData
  {
    #[ inline ]
    fn from( src : &'static [ u8 ] ) -> Self
    {
      Self::FromBin( src )
    }
  }

  impl From< PathBuf > for GetData
  {
    #[ inline ]
    fn from( src : PathBuf ) -> Self
    {
      Self::FromFile( src )
    }
  }

  impl From< String > for GetData
  {
    #[ inline ]
    fn from( src : String ) -> Self
    {
      Self::FromString( src )
    }
  }

  impl Default for GetData
  {
    fn default() -> Self
    {
      GetData::FromStr( "" )
    }
  }

}

crate::mod_interface!
{

  exposed use
  {
    Program,
  };

  own use
  {
    Plan,
    Call,
    Action,
    EntryType,
    Entry,
    SourceFile,
    CargoFile,
    GetData,
  };

}

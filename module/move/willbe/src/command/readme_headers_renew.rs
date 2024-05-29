mod private
{
  use std::fmt::{Display, Formatter};
  use error_tools::{ Result, err };
  use crate::_path::AbsolutePath;
  use crate::action;
  use crate::action::{MainHeaderRenewReport, ModulesHeadersRenewReport};
  use crate::wtools::error::anyhow::Error;

  #[ derive( Debug, Default ) ]
  struct ReadmeHeadersRenewReport
  {
    main_header_renew_report : MainHeaderRenewReport,
    main_header_renew_error : Option< Error >,
    modules_headers_renew_report : ModulesHeadersRenewReport,
    modules_headers_renew_error : Option< Error >,
  }

  impl Display for ReadmeHeadersRenewReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      match ( &self.main_header_renew_error, &self.modules_headers_renew_error ) 
      { 
        ( Some( main ), Some( modules ) ) => 
        {
          writeln!
          ( 
            f, 
            "Main header renew report : {}\nError : {:?}\nModules headers renew report : {}\nError : {:?}",
            self.main_header_renew_report, main, self.modules_headers_renew_report, modules 
          )?;
        }
        ( Some( main ), None ) =>
        {
          writeln!
          (
            f,
            "Main header renew report : {}\nError : {:?}\nModules headers renew report : {}",
            self.main_header_renew_report, main, self.modules_headers_renew_report
          )?;
        }
        ( None, Some( modules) ) =>
        {
          writeln!
          (
            f,
            "Main header renew report : {}\nModules headers renew report : {}Error : {:?}\n",
            self.main_header_renew_report, self.modules_headers_renew_report, modules
          )?;
        }
        ( None, None ) =>
        {
          writeln!
          (
            f,
            "Main header renew report : {}\nModules headers renew report : {}",
            self.main_header_renew_report, self.modules_headers_renew_report
          )?;
        }
      }
      Ok( () )
    }
  }
  

  /// Aggregates two commands: `generate_modules_headers` & `generate_main_header`
  pub fn readme_headers_renew( ( _, _ ) : ( wca::Args, wca::Props ) ) -> Result< () >
  {
    let mut report = ReadmeHeadersRenewReport::default();
    let absolute_path = AbsolutePath::try_from( std::env::current_dir()? )?;
    let mut fail = false;
    
    match action::readme_header_renew( absolute_path.clone() ) 
    {
      Ok( r ) => 
      {
        report.main_header_renew_report = r;
      }
      Err( ( error, r) ) => 
      {
        fail = true;
        report.main_header_renew_report = r;
        report.main_header_renew_error = Some( Error::from( error ) );
      }
    };
    match action::readme_modules_headers_renew( absolute_path )
    {
      Ok( r ) =>
      {
        report.modules_headers_renew_report = r;
      }
      Err( ( error, r) ) =>
      {
        fail = true;
        report.modules_headers_renew_report = r;
        report.modules_headers_renew_error = Some( Error::from( error ) );
      }
    }
    
    if fail
    {
      eprintln!( "{report}" );
      Err( err!( "Something went wrong" ) )
    }
    else
    {
      Ok( () )
    }
  }
}

crate::mod_interface!
{
  /// Generate header's.
  orphan use readme_headers_renew;
}
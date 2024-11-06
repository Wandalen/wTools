mod private
{
  use crate::*;
  // use action;
  // use error::{ err };
  use std::fmt::{ Display, Formatter };

  #[ derive( Debug, Default ) ]
  struct ReadmeHeadersRenewReport
  {
    main_header_renew_report : action::MainHeaderRenewReport,
    main_header_renew_error : Option< action::MainHeaderRenewError >, // aaa : for Petro : typed error // aaa : done
    modules_headers_renew_report : action::ModulesHeadersRenewReport,
    modules_headers_renew_error : Option< action::ModulesHeadersRenewError >, // aaa : for Petro : typed error // aaa : done
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
            "Main header renew report : \
{}\nError : \n{:?}\nModules headers renew report : \n{}\nError : \n{:?}",
            self.main_header_renew_report, main, self.modules_headers_renew_report, modules
          )?;
        }
        ( Some( main ), None ) =>
        {
          writeln!
          (
            f,
            "Main header renew report : \
{}\nError : \n{:?}\nModules headers renew report : \n{}",
            self.main_header_renew_report, main, self.modules_headers_renew_report
          )?;
        }
        ( None, Some( modules) ) =>
        {
          writeln!
          (
            f,
            "Main header renew report : \
{}\nModules headers renew report : \n{}\nError : \n{:?}\n",
            self.main_header_renew_report, self.modules_headers_renew_report, modules
          )?;
        }
        ( None, None ) =>
        {
          writeln!
          (
            f,
            "Main header renew report : \n{}\n\nModules headers renew report : \n{}",
            self.main_header_renew_report, self.modules_headers_renew_report
          )?;
        }
      }
      Ok( () )
    }
  }

  /// Aggregates two commands: `generate_modules_headers` & `generate_main_header`
  pub fn readme_headers_renew() -> error::untyped::Result< () > // qqq : use typed error
  {
    let mut report = ReadmeHeadersRenewReport::default();
    // let absolute_path = AbsolutePath::try_from( std::env::current_dir()? )?;
    let crate_dir = CrateDir::transitive_try_from::< AbsolutePath >( CurrentPath )?;
    let mut fail = false;

    match crate::action::main_header::action( crate_dir.clone() )
    {
      Ok( r ) =>
      {
        report.main_header_renew_report = r;
      }
      Err( ( r, error ) ) =>
      {
        fail = true;
        report.main_header_renew_report = r;
        report.main_header_renew_error = Some( error );
      }
    };
    match action::readme_modules_headers_renew( crate_dir )
    {
      Ok( r ) =>
      {
        report.modules_headers_renew_report = r;
      }
      Err( ( r, error ) ) =>
      {
        fail = true;
        report.modules_headers_renew_report = r;
        report.modules_headers_renew_error = Some(  error );
      }
    }

    if fail
    {
      eprintln!( "{report}" );
      Err( error::untyped::format_err!( "Something went wrong" ) )
    }
    else
    {
      println!( "{report}" );
      Ok( () )
    }
  }
}

crate::mod_interface!
{
  /// Generate header's.
  orphan use readme_headers_renew;
}
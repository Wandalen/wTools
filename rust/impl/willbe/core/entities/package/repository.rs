/// Internal namespace.
pub( crate ) mod private
{
  use git2::*;
  use std::path::PathBuf;

  use wtools::{ BasicError, err };

  use crate::Package;

  /// Working with git
  pub struct PackageRepository
  {
    package : Package,
    pathspecs : Vec< String >,
    refspecs : Vec< String >,
    // remote : Option< Remote< 'a > >,
    repository : Repository,
  }

  impl core::fmt::Debug for PackageRepository
  {
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      f.debug_struct( "PackageRepository" )
      .field( "package", &self.package )
      .field( "pathspecs", &self.pathspecs )
      .field( "refspecs", &self.refspecs )
      // Repository is wrapper on raw poiner to git_repository from libgit2
      .field( "repository", &"" )
      .finish()
    }
  }

  impl TryFrom< PathBuf > for PackageRepository
  {
    type Error = BasicError;

    fn try_from( value : PathBuf ) -> Result< Self, Self::Error >
    {
      let package = Package::try_from( value )?;
      package.try_into()
    }
  }

  impl TryFrom< Package > for PackageRepository
  {
    type Error = BasicError;

    fn try_from( value : Package ) -> Result< Self, Self::Error >
    {
      // If Package is not a root of repository this fails
      let repository = Repository::open( value.path() )
      .map_err( | _ | err!( "Can not open the package repository" ) )?;

      Ok( Self
      {
        package : value,
        pathspecs : vec![],
        refspecs : vec![],
        // remote : None,
        repository
      })
    }
  }

  impl PackageRepository
  {
    /// Add path to staged for commit
    pub fn add( &mut self, path : impl Into< String > ) -> &mut Self
    {
      self.pathspecs.push( path.into() );
      self
    }

    /// Commit chenges
    pub fn commit< M >( &self, message : M ) -> Result< (), BasicError >
    where
      M : AsRef< str >
    {
      let mut index = self.repository.index()
      .map_err( | _ | err!( "Can not index the repository" ) )?;

      index.add_all( self.pathspecs.to_owned().into_iter(), git2::IndexAddOption::DEFAULT, None )
      .map_err( | _ | err!( "Can not add files to a commit" ) )?;
      index.write()
      .map_err( | _ | err!( "Can not write the commit" ) )?;

      let sig = self.repository.signature()
      .map_err( | _ | err!( "Can not read the repository signature" ) )?;
      let tree_id =
      {
        let mut index = self.repository.index().unwrap();
        index.write_tree().unwrap()
      };

      let parents = self.repository
      .head()
      .map( | x | x.target().unwrap() )
      .map( | x | self.repository.find_commit( x ).unwrap() );

      self.repository.commit
      (
        Some( "HEAD" ),
        &sig,
        &sig,
        message.as_ref(),
        &self.repository.find_tree( tree_id ).unwrap(),
        &parents.iter().collect::< Vec< _ > >()
      )
      .map_err( | _ | err!( "Commit failed" ) )?;

      Ok( () )
    }

    /// Adds refspec
    pub fn add_refspec( &mut self, refspec : impl Into< String > ) -> &mut Self
    {
      self.refspecs.push( refspec.into() );
      self
    }

    /// Gets remote by url
    pub fn remote_by_url( &self, url : impl AsRef< str > ) -> Result< Remote< '_ >, BasicError >
    {
      // It would be better to save remote inside PackageRepository but problems with lifetimes
      self.repository.remote_anonymous( url.as_ref() )
      .map_err( | _ | err!( "" ) )
    }

    /// Gets remote by name
    pub fn remote_by_name( &self, name : impl AsRef< str > ) -> Result< Remote< '_ >, BasicError >
    {
      // It would be better to save remote inside PackageRepository but problems with lifetimes
      self.repository.find_remote( name.as_ref() )
      .map_err( | _ | err!( "" ) )
    }

    /// Push chenges
    // refspecs example: &[ "refs/heads/master:refs/heads/master" ] - to push master branch to remote master branch
    pub fn push( &self, remote : &mut Remote< '_ > ) -> Result< (), BasicError >
    {
      remote.connect( Direction::Push )
      .map_err( | _ | err!( "Could not connect to remote repository to push" ) )?;

      remote.push( &self.refspecs, None )
      .map_err( | _ | err!( "Push failed" ) )?;

      Ok( () )
    }
  }
}

//

wtools::mod_interface!
{
  prelude use PackageRepository;
}

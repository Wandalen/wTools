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
    repository : Repository,
  }

  impl core::fmt::Debug for PackageRepository
  {
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      f.debug_struct( "PackageRepository" )
      .field( "package", &self.package )
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
        repository
      })
    }
  }

  impl PackageRepository
  {
    /// Commit chenges
    // pathspecs example: [ "*" ] - to commit all chenges. `git add * && git commit -m message`
    pub fn commit< T, I, M >( &self, pathspecs : I, message : M ) -> Result< (), BasicError >
    where
      T : IntoCString,
      I : IntoIterator< Item = T >,
      M : AsRef< str >
    {
      let mut index = self.repository.index()
      .map_err( | _ | err!( "Can not index the repository" ) )?;

      index.add_all( pathspecs.into_iter(), git2::IndexAddOption::DEFAULT, None )
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
      let tree = self.repository.find_tree( tree_id ).unwrap();
      let last_commit = self.repository
      .head()
      .map( | x | x.target().unwrap() )
      .map( | x | self.repository.find_commit( x ).unwrap() );
      let parents = last_commit
      .iter().collect::< Vec< _ > >();

      self.repository.commit
      (
        Some( "HEAD" ),
        &sig,
        &sig,
        message.as_ref(),
        &tree,
        &parents
      )
      .map_err( | _ | err!( "Commit failed" ) )?;

      Ok( () )
    }

    /// Push chenges
    // refspecs example: &[ "refs/heads/master:refs/heads/master" ] - to push master branch to remote master branch
    pub fn push< S >( &self, refspecs : &[ S ], remote_url : impl AsRef< str > ) -> Result< (), BasicError >
    where
      S : AsRef< str > + git2::IntoCString + Clone
    {
      // May be the url contains into package's metadata and we could use that instead of getting it as the parameter
      let mut remote = self.repository.remote_anonymous( remote_url.as_ref() )
      .map_err( | _ | err!( "Could not create remote connection" ) )?;
      remote.connect( Direction::Push )
      .map_err( | _ | err!( "Could not connect to remote repository to push" ) )?;

      remote.push( refspecs.as_ref(), None )
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

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
    repository : Repository, // Repository does not implement Debug
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
      let parent = self.repository.head().unwrap().target().unwrap();

      self.repository.commit
      (
        Some( "HEAD" ),
        &sig,
        &sig,
        message.as_ref(),
        &tree,
        &[ &self.repository.find_commit( parent ).unwrap() ]
      )
      .map_err( | _ | err!( "Commit failed" ) )?;
      Ok( () )
    }

    /// Push chenges
    pub fn push( &self, remote_url : impl AsRef< str > ) -> Result< (), BasicError >
    {
      // May be the url contains into package's metadata
      let mut remote = self.repository.remote_anonymous( remote_url.as_ref() ).unwrap();
      remote.connect( Direction::Push ).unwrap();

      // IDK what is this refs
      // TODO: improve this
      remote.push(&[ "refs/heads/master:refs/heads/master" ], None ).unwrap();

      Ok( () )
    }
  }
}

//

wtools::mod_interface!
{
  prelude use PackageRepository;
}
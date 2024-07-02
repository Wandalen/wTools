mod private
{
  use crate::*;
  use std::
  {
    fmt::Formatter,
  };
  use package::PackageName;
  use collection::{ HashMap, HashSet };

  // use workspace::WorkspacePackageRef< '_ >;
  // use Dependency;

  // aaa : poor description // aaa : removed

  /// A configuration struct for specifying optional filters when using the
  /// `filter` function. It allows users to provide custom filtering
  /// functions for packages and dependencies.
  #[ derive( Default ) ]
  pub struct FilterMapOptions
  {
    /// An optional package filtering function. If provided, this function is
    /// applied to each package, and only packages that satisfy the condition
    /// are included in the final result. If not provided, a default filter that
    /// accepts all packages is used.
    pub package_filter : Option< Box< dyn Fn( WorkspacePackageRef< '_ > ) -> bool > >,

    /// An optional dependency filtering function. If provided, this function
    /// is applied to each dependency of each package, and only dependencies
    /// that satisfy the condition are included in the final result. If not
    /// provided, a default filter that accepts all dependencies is used.
    pub dependency_filter : Option< Box< dyn Fn( WorkspacePackageRef< '_ >, DependencyRef< '_ > ) -> bool  > >,
  }

  impl std::fmt::Debug for FilterMapOptions
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      f
      .debug_struct( "FilterMapOptions" )
      .field( "package_filter", &"package_filter" )
      .field( "dependency_filter", &"dependency_filter" )
      .finish()
    }
  }

  /// Provides a means to filter both packages and dependencies of an existing package metadata set.
  ///
  /// # Arguments
  ///
  /// * `packages` - A slice of `PackageMetadata` instances presenting the original set of packages.
  ///
  /// * `options` - An instance of `FilterMapOptions` which includes a package filter
  ///   and a dependency filter, both optional. If these filters are not provided (`None`), then
  ///   all packages and their dependencies are accepted (`true`).
  ///
  /// # Returns
  ///
  /// This function returns a `HashMap` where :
  ///
  /// * The key is `PackageName`, referring to the name of each package.
  ///
  /// * The value is `HashSet< PackageName >`, representing a unique collection of names of its dependencies.
  ///
  /// # Filters
  ///
  /// * `package_filter`: When specified, it will be used to decide whether each incoming given
  ///   package should be included in the return. If this filter is not provided, all packages will be
  ///   included.
  ///
  /// * `dependency_filter`: When specified, it's used with each package and its dependencies to decide
  ///   which dependencies should be included in the return for that package. If not provided, all
  ///   dependencies for a package are included.

  // aaa : for Bohdan : for Petro : bad. don't use PackageMetadata directly, use its abstraction only!

  pub fn filter< 'a >
  (
    // packages : &[ WorkspacePackageRef< '_ > ],
    packages : impl Iterator< Item = WorkspacePackageRef< 'a > >,
    options : FilterMapOptions,
  )
  -> HashMap< PackageName, HashSet< PackageName > >
  {
    let FilterMapOptions { package_filter, dependency_filter } = options;
    let package_filter = package_filter.unwrap_or_else( || Box::new( | _ | true ) );
    let dependency_filter = dependency_filter.unwrap_or_else( || Box::new( | _, _ | true ) );
    packages
    // .iter()
    .filter( | &p | package_filter( p ) )
    .map
    (
      | package |
      (
        package.name().to_string().into(),
        package.dependencies()
        // .iter()
        .filter( | d | dependency_filter( package, *d ) )
        .map( | d | d.name().into() )
        .collect::< HashSet< _ > >()
      )
    )
    .collect()
  }
}

//

crate::mod_interface!
{

  protected use FilterMapOptions;
  protected use filter;

}

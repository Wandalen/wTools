/// Define a private namespace for all its items.
mod private
{

  /// Checks if the current execution environment is a Continuous Integration (CI) or Continuous Deployment (CD) pipeline.
  ///
  /// This function looks for environment variables that are commonly set by CI/CD systems to determine if it's running
  /// within such an environment. It supports detection for a variety of popular CI/CD platforms including GitHub Actions,
  /// GitLab CI, Travis CI, `CircleCI`, and Jenkins.
  ///
  /// # Returns
  /// - `true` if an environment variable indicating a CI/CD environment is found.
  /// - `false` otherwise.
  ///
  /// # Examples
  ///
  /// When running in a typical development environment (locally):
  /// ```no_run
  /// use process_tools::environment;
  /// assert_eq!( environment::is_cicd(), false );
  /// ```
  ///
  /// When running in a CI/CD environment, one of the specified environment variables would be set, and:
  /// ```no_run
  /// // This example cannot be run as a test since it depends on the environment
  /// // the code is executed in. However, in a CI environment, this would return true.
  /// use process_tools::environment;
  /// assert_eq!( environment::is_cicd(), true );
  /// ```

  #[ cfg( feature = "process_environment_is_cicd" ) ]
  #[ must_use ]
  pub fn is_cicd() -> bool
  {
    use std::env;
    let ci_vars =
    [
      "CI",             // Common in many CI systems
      "GITHUB_ACTIONS", // GitHub Actions
      "GITLAB_CI",      // GitLab CI
      "TRAVIS",         // Travis CI
      "CIRCLECI",       // CircleCI
      "JENKINS_URL",    // Jenkins
    ];

    ci_vars.iter().any( | &var | env::var( var ).is_ok() )
  }

}

crate::mod_interface!
{
  #[ cfg( feature = "process_environment_is_cicd" ) ]
  own use is_cicd;
}

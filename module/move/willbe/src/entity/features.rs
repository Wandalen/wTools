mod private
{
  use crate::*;
  use std::collections::{ BTreeSet, HashSet };
  use cargo_metadata::Package;
  use wtools::iter::Itertools;

  /// Generates a powerset of the features available in the given `package`,
  /// filtered according to specified inclusion and exclusion criteria,
  /// and limited by a specified maximum size (`power`).
  ///
  /// This function is useful for generating combinations of feature sets
  /// to test different feature configurations in a Rust package.
  ///
  /// # Arguments
  ///
  /// * `package` - A reference to the `Package` struct which contains the features.
  /// * `power` - The maximum size of each subset in the powerset. This limits the number of features in any given combination.
  /// * `exclude_features` - A slice of feature names to exclude from the powerset.
  /// * `include_features` - A slice of feature names to always include in every subset of the powerset.
  ///
  /// # Returns
  ///
  /// Returns a `HashSet<BTreeSet<String>>` where each `BTreeSet<String>` is a unique combination of feature names,
  /// taking into account the inclusion, exclusion, and size constraints.
  ///
  /// # Examples
  ///
  /// ```ignore
  /// // Assuming `package` is a valid `Package` instance with features.
  /// let power = 2;
  /// let exclude_features = vec![ "feature1".to_string() ];
  /// let include_features = vec![ "feature2".to_string() ];
  /// let feature_combinations = features_powerset( &package, power, &exclude_features, &include_features );
  /// // Use `feature_combinations` as needed.
  /// ```

  pub fn features_powerset
  (
    package : &Package,
    power : usize,
    exclude_features : &[ String ],
    include_features : &[ String ],
  )
    -> HashSet< BTreeSet< String > >
  {
    let mut features_powerset = HashSet::new();

    let filtered_features : Vec<_> = package
    .features
    .keys()
    .filter( | f | !exclude_features.contains( f ) )
    .cloned()
    .collect();

    for subset_size in 0..= std::cmp::min( filtered_features.len(), power )
    {
      for combination in filtered_features.iter().combinations( subset_size )
      {
        let mut subset : BTreeSet< String > = combination.into_iter().cloned().collect();
        subset.extend( include_features.iter().cloned() );
        features_powerset.insert( subset );
      }
    }

    features_powerset
  }
}

crate::mod_interface!
{
  /// Features
  protected use features_powerset;
}

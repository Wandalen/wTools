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
  /// * `include_features` - A slice of feature names to include in the powerset.
  /// * `enabled_features` - A slice of features names to always include in each subset of powerset.
  /// * `with_all_features` - If it's true - return powerset from one subset which contains all features.
  /// * `with_none_features` - If it's true - return powerset from one empty subset.
  ///
  /// # Returns
  ///
  /// Returns a `HashSet< BTreeSet< String > >` where each `BTreeSet< String >` is a unique combination of feature names,
  /// taking into account the inclusion, exclusion, and size constraints.
  ///
  /// # Examples
  ///
  /// ```ignore
  /// // Assuming `package` is a valid `Package` instance with features.
  /// let power = 2;
  /// let exclude_features = vec![ "feature1".to_string() ];
  /// let include_features = vec![ "feature2".to_string() ];
  /// let enable_features = vec![ "feature5".to_string() ];
  /// let feature_combinations = features_powerset( &package, power, &exclude_features, &include_features, enabled_features, false, false );
  /// // Use `feature_combinations` as needed.
  /// ```

  // aaa : for Petro : bad, don't use ignore with need
  // aaa : I have to ignore this test because the function accepts &Package as input, and to mock it requires a lot of lines

  pub fn features_powerset
  (
    package : &Package,
    power : usize,
    exclude_features : &[ String ],
    include_features : &[ String ],
    enabled_features : &[ String ],
    with_all_features : bool,
    with_none_features : bool,
    variants_cap : usize, // qqq максимальна кількість варіантів
  )
    -> HashSet< BTreeSet< String > >
  {
    let mut features_powerset = HashSet::new();

    let filtered_features : BTreeSet< _ > = package
    .features
    .keys()
    .filter( | f | !exclude_features.contains( f ) && ( include_features.contains( f ) || include_features.is_empty() ) )
    .cloned()
    .collect();

    for subset_size in 0..= std::cmp::min( filtered_features.len(), power )
    {
      for combination in filtered_features.iter().combinations( subset_size )
      {
        let subset : BTreeSet< String > = combination.into_iter().cloned().collect();
        if subset.is_empty() || subset == filtered_features
        {
          continue
        }
        // subset.extend( enabled_features.iter().cloned() );
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

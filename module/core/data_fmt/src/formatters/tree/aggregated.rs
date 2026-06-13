//! Aggregation-aware tree rendering for `TreeFormatter`
//!
//! Renders a tree where each directory node shows a rolled-up aggregate value
//! (e.g., total file count, total size, total line count) together with its
//! percentage of the grand total.

use crate::TreeNode;

/// Output buffer capacity hint
const INITIAL_CAPACITY : usize = 1024;

impl super::TreeFormatter
{
  /// Format a tree with aggregated roll-up values per directory
  ///
  /// Each leaf calls `aggregate_fn` to produce a value `V`. Directory nodes
  /// show the sum of all descendants. `convert_to_f64` converts `V` to a
  /// float for percentage calculation.
  ///
  /// # Examples
  ///
  /// ```
  /// use data_fmt::{ TreeBuilder, TreeFormatter };
  ///
  /// struct FileInfo { lines : u64, lang : String }
  ///
  /// let tree : data_fmt::TreeNode< FileInfo > = TreeBuilder::new( "root" ).build();
  /// let formatter = TreeFormatter::new();
  /// let output = formatter.format_with_aggregation(
  ///   &tree,
  ///   1000u64, // grand total
  ///   | f | f.lines,
  ///   | v | v as f64,
  ///   | f, total, pct | format!( "({} lines, {:.1}%, {})", f.lines, pct, f.lang ),
  ///   | _name, total, pct | format!( "({} lines, {:.1}%)", total, pct ),
  /// );
  /// ```
  pub fn format_with_aggregation< T, V, A, F, D, C >(
    &self,
    tree : &TreeNode< T >,
    grand_total : V,
    aggregate_fn : A,
    convert_to_f64 : C,
    render_file : F,
    render_directory : D,
  ) -> String
  where
    V : Copy + std::ops::Add< Output = V > + Default + std::iter::Sum,
    A : Fn( &T ) -> V,
    C : Fn( V ) -> f64,
    F : Fn( &T, V, f64 ) -> String,
    D : Fn( &str, V, f64 ) -> String,
  {
    let mut output = String::with_capacity( INITIAL_CAPACITY );
    self.format_node_with_aggregation(
      tree,
      &mut output,
      "",
      true,
      0,
      grand_total,
      true,
      &aggregate_fn,
      &convert_to_f64,
      &render_file,
      &render_directory,
    );
    output
  }

  /// Calculate aggregated total for a node and all descendants
  fn calculate_aggregate< T, V, A >( node : &TreeNode< T >, aggregate_fn : &A ) -> V
  where
    V : Copy + std::ops::Add< Output = V > + Default + std::iter::Sum,
    A : Fn( &T ) -> V,
  {
    let direct = node.data.as_ref().map_or( V::default(), aggregate_fn );
    let children_total : V = node.children.iter()
      .map( | c | Self::calculate_aggregate( c, aggregate_fn ) )
      .sum();
    direct + children_total
  }

  /// Format node with aggregation support (recursive)
  // Aggregation traversal requires node, buffer, prefix, depth, grand_total, is_root,
  // plus three typed callbacks (aggregate_fn, convert_to_f64, render_file/directory);
  // a context struct would add an allocation on every recursive call.
  #[ allow( clippy::too_many_arguments ) ]
  fn format_node_with_aggregation< T, V, A, C, F, D >(
    &self,
    node : &TreeNode< T >,
    output : &mut String,
    prefix : &str,
    is_last : bool,
    depth : usize,
    grand_total : V,
    is_root : bool,
    aggregate_fn : &A,
    convert_to_f64 : &C,
    render_file : &F,
    render_directory : &D,
  )
  where
    V : Copy + std::ops::Add< Output = V > + Default + std::iter::Sum,
    A : Fn( &T ) -> V,
    C : Fn( V ) -> f64,
    F : Fn( &T, V, f64 ) -> String,
    D : Fn( &str, V, f64 ) -> String,
  {
    // Check max depth
    if let Some( max_depth ) = self.config.max_depth
    {
      if depth >= max_depth
      {
        return;
      }
    }

    // For directory nodes (no data), show directory name with aggregated total
    if node.data.is_none() && !is_root
    {
      let node_total = Self::calculate_aggregate( node, aggregate_fn );
      let percentage = convert_to_f64( node_total ) / convert_to_f64( grand_total ) * 100.0;

      output.push_str( &format!(
        "{}/ {}",
        node.name,
        render_directory( &node.name, node_total, percentage )
      ) );
      output.push( '\n' );
    }

    // Calculate child prefix
    let child_prefix = if is_root
    {
      String::new()
    }
    else if is_last
    {
      format!( "{prefix}    " )
    }
    else
    {
      format!( "{prefix}│   " )
    };

    // Display file data if present
    if let Some( ref file_data ) = node.data
    {
      let value = aggregate_fn( file_data );
      let percentage = convert_to_f64( value ) / convert_to_f64( grand_total ) * 100.0;

      let symbol = if is_last { "└──" } else { "├──" };
      output.push_str( &format!(
        "{}{} {} {}",
        prefix,
        symbol,
        node.name,
        render_file( file_data, value, percentage )
      ) );
      output.push( '\n' );
    }

    // Recursively format children
    for ( idx, child ) in node.children.iter().enumerate()
    {
      let is_last_child = idx == node.children.len() - 1;
      self.format_node_with_aggregation(
        child,
        output,
        &child_prefix,
        is_last_child,
        depth + 1,
        grand_total,
        false,
        aggregate_fn,
        convert_to_f64,
        render_file,
        render_directory,
      );
    }
  }

}

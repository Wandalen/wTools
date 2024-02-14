/// Internal namespace.
pub( crate ) mod private
{
  use crate::{ Package, PackageMetadata };
  use iter_tools::prelude::*;

  /// Represent with which order strategy to iterate over packages
  #[ derive( Debug, Clone, Copy ) ]
  pub enum OrderStrategy
  {
    /// Alphabetical by package name
    Alphabetical,
    /// Based on their dependencies
    Topological,
    /// Shuffle packages
    Random,
  }

  /// This trait defines a method to sort packages by selected order strategy
  pub trait Ordered : Iterator
  where
    Vec< Package > : FromIterator< < Self as Iterator >::Item >
  {
    /// Collect all iterator elements into a sorted vector
    fn ordered( self, order : OrderStrategy ) -> Vec< Package >
    where
      Self : Sized
    {
      let v : Vec< Package > = self.collect();

      match order
      {
        OrderStrategy::Alphabetical => alphabetical( v ),
        OrderStrategy::Topological => toposort( v ),
        OrderStrategy::Random => shuffle( v ),
      }
    }

    /// Returns iterator over sorted Packages
    fn ordered_iter( self, order : OrderStrategy ) -> Box< dyn Iterator< Item = Package > >
    where
      Self : Sized
    {
      Box::new( self.ordered( order ).into_iter() )
    }
  }

  fn alphabetical( packages : Vec< Package > ) -> Vec< Package >
  {
    packages.iter().cloned()
    .filter_map( | p |
    {
      PackageMetadata::try_from( p ).ok()
    })

    .sorted_by_key( | meta | meta.name().to_owned() )

    .map( | meta | meta.as_package().to_owned() )
    .collect_vec()
  }

  fn toposort( packages : Vec< Package > ) -> Vec< Package >
  {
    use petgraph::Graph;
    use cargo_metadata::DependencyKind;
    use std::collections::HashMap;

    let ( deps, package_map ) = packages.iter()
    .filter_map( | p | PackageMetadata::try_from( p.to_owned() ).ok() )
    .fold( ( Graph::new(), HashMap::new() ), | ( mut deps, mut packages ), meta |
    {
      packages.insert( meta.name().to_owned(), meta.as_package().to_owned() );

      let root_node = if let Some( node ) = deps.node_indices().find( | i | deps[ *i ] == meta.name().to_owned() )
      { node }
      else
      { deps.add_node( meta.name().to_owned() ) };

      for dep in &meta.all().dependencies
      {
        if dep.path.is_some() && dep.kind != DependencyKind::Development
        {
          let dep_node = if let Some( node ) = deps.node_indices().find( | i | deps[ *i ] == dep.name )
          { node }
          else
          { deps.add_node( dep.name.to_owned() ) };

          deps.add_edge( root_node, dep_node, () );
        }
      }

      ( deps, packages )
    });

    let sorted = petgraph::algo::toposort( &deps, None ).unwrap();
    sorted.iter()
    .rev()
    .map( | &dep_idx | deps.node_weight( dep_idx ).unwrap().to_owned() )
    .filter_map( | name | package_map.get( &name ) )
    .cloned().collect::< Vec< Package > >()
  }

  fn shuffle( mut packages : Vec< Package > ) -> Vec< Package >
  {
    use rand::seq::SliceRandom;

    let mut rng = rand::thread_rng();
    packages.shuffle( &mut rng );

    packages
  }


  impl< T : ?Sized > Ordered for T
  where T : Iterator, Vec< Package >: FromIterator< < T as Iterator >::Item > {}
}

//

crate::mod_interface!
{
  prelude use OrderStrategy;
  prelude use Ordered;
}

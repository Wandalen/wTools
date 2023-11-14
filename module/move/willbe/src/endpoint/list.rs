/// Internal namespace.
mod private
{
  use std::collections::HashMap;
  use crate::tools::
  {
    manifest::Manifest,
  };
  use std::fmt::Formatter;
  use cargo_metadata::{ DependencyKind, MetadataCommand };
  use petgraph::{ algo::toposort, algo::has_path_connecting, Graph };
  use std::path::PathBuf;
  use std::str::FromStr;
  use crate::wtools::error::{ for_app::Error, err };

  #[ derive( Debug, Default, Copy, Clone ) ]
  pub enum ListFormat
  {
    #[ default ]
    Tree,
    Topological,
  }

  impl FromStr for ListFormat
  {
    type Err = Error;

    fn from_str( s : &str ) -> Result< Self, Self::Err >
    {
      let value = match s
      {
        "tree" => ListFormat::Tree,
        "toposort" => ListFormat::Topological,
        e => return Err( err!( "Unknown format '{}'. Available values: [tree, toposort]", e ))
      };

      Ok( value )
    }
  }

  #[ derive( Debug, Default, Copy, Clone ) ]
  pub enum ListFilter
  {
    #[ default ]
    Nothing,
    Local,
  }

  impl FromStr for ListFilter
  {
    type Err = Error;

    fn from_str( s : &str ) -> Result< Self, Self::Err >
    {
      let value = match s
      {
        "nothing" => ListFilter::Nothing,
        "local" => ListFilter::Local,
        e => return Err( err!( "Unknown filter '{}'. Available values: [nothing, local]", e ) )
      };

      Ok( value )
    }
  }

  #[ derive( Debug, Default, Clone ) ]
  pub enum ListReport
  {
    Tree
    {
      graph : Graph< String, String >,
      names : Vec< petgraph::stable_graph::NodeIndex >,
    },
    List( Vec< String > ),
    #[ default ]
    Empty
  }

  /// Wrapper to redirect output from `ptree` graph to `fmt::Write`
  struct Io2FmtWrite< 'a, 'b >
  {
    f : &'a mut Formatter< 'b >,
  }

  impl std::io::Write for Io2FmtWrite< '_, '_ >
  {
    fn write( &mut self, buf : &[ u8 ] ) -> std::io::Result< usize >
    {
      let size = buf.len();

      self.f.write_str( std::str::from_utf8( buf ).unwrap() ).unwrap();

      Ok( size )
    }

    fn flush( &mut self ) -> std::io::Result< () >
    {
      Ok( () )
    }
  }

  impl std::fmt::Display for ListReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      match self
      {
        ListReport::Tree { graph, names } => for n in names
        {
          ptree::graph::write_graph_with(&graph, *n, Io2FmtWrite { f }, &ptree::PrintConfig::from_env() ).unwrap();
        },
        ListReport::List ( list ) => for ( i ,e ) in list.iter().enumerate() { writeln!( f, "{i}) {e}" )? },
        _ => {},
      }

      Ok( () )
    }
  }

  ///
  /// List workspace packages.
  ///

  pub fn list( path_to_workspace : PathBuf, root_crate : &str, format : ListFormat, filter : ListFilter ) -> Result< ListReport, ( ListReport, Error ) >
  {
    let mut report = ListReport::default();

    let mut manifest = Manifest::new();
    let manifest_path = manifest.manifest_path_from_str( &path_to_workspace ).map_err( | e | ( report.clone(), e.into() ) )?;
    let metadata = MetadataCommand::new()
    .manifest_path( &manifest_path )
    .no_deps()
    .exec()
    .map_err( | e | ( report.clone(), e.into() ) )?;

    let packages_map = metadata.packages.iter().map( | p | ( p.name.clone(), p ) ).collect::< HashMap< _, _ > >();

    let mut graph = Graph::new();
    for ( _, package ) in packages_map
    {
      let root_node = if let Some( node ) = graph.node_indices().find( | i | graph[ *i ] == &package.name )
      {
        node
      }
      else
      {
        graph.add_node( &package.name )
      };

      for dep in &package.dependencies
      {
        if match filter
        {
          ListFilter::Nothing => dep.kind != DependencyKind::Development,
          ListFilter::Local => dep.path.is_some() && dep.kind != DependencyKind::Development,
        }
        {
          let dep_node = if let Some( node ) = graph.node_indices().find( | i | graph[ *i ] == &dep.name )
          {
            node
          }
          else
          {
            graph.add_node( &dep.name )
          };

          graph.add_edge( root_node, dep_node, &package.name );
        }
      }
    }
    let sorted = toposort( &graph, None ).expect( "Failed to process toposort for packages" );

    match format
    {
      ListFormat::Tree if root_crate.is_empty() =>
      {
        let mut names = vec![ sorted[ 0 ] ];
        for node in sorted.iter().skip( 1 )
        {
          if names.iter().all( | name | !has_path_connecting( &graph, *name, *node, None ) ) && !names.contains( node )
          {
            names.push( *node );
          }
        }
        report = ListReport::Tree { graph : graph.map( | _, &n | String::from( n ), | _, &e | String::from( e ) ), names };
      },
      ListFormat::Tree =>
      {
        let names = sorted
        .iter()
        .filter_map( | idx | if graph.node_weight( *idx ).unwrap() == &root_crate { Some( *idx ) } else { None } )
        .collect::< Vec< _ > >();

        report = ListReport::Tree { graph : graph.map( | _, &n | String::from( n ), | _, &e | String::from( e ) ), names };
      }
      ListFormat::Topological =>
      {
        let names = sorted
        .iter()
        .rev()
        .map( | dep_idx | graph.node_weight( *dep_idx ).unwrap().to_string() )
        .collect::< Vec< String > >();

        report = ListReport::List( names );
      },
    }

    Ok( report )
  }
}

//

crate::mod_interface!
{
  protected( crate ) use ListFormat;
  protected( crate ) use ListFilter;
  /// List packages in workspace.
  prelude use list;
}

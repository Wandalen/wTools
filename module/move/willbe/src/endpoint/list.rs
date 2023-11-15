/// Internal namespace.
mod private
{
  use std::collections::HashMap;
  use crate::tools::
  {
    manifest::Manifest,
  };
  use std::fmt::Formatter;
  use petgraph::{ algo::toposort, algo::has_path_connecting, Graph };
  use std::path::PathBuf;
  use std::str::FromStr;
  use crate::package::functions::graph_build;
  use crate::wtools::error::{ for_app::Error, err };
  use cargo_metadata::MetadataCommand;

  /// Args for `list` endpoint.
  #[ derive( Debug, Default, Copy, Clone ) ]
  pub enum ListFormat
  {
    /// Tree like format.
    #[ default ]
    Tree,
    /// Topologically sorted list.
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

  /// Args for `list` endpoint.
  #[ derive( Debug, Default, Copy, Clone ) ]
  pub enum ListFilter
  {
    /// With all packages.
    #[ default ]
    Nothing,
    /// With local only packages.
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

  /// Output of the `list` endpoint
  #[ derive( Debug, Default, Clone ) ]
  pub enum ListReport
  {
    /// With tree format.
    Tree
    {
      /// Dependencies graph.
      graph : Graph< String, String >,
      /// Packages indexes to display.
      names : Vec< petgraph::stable_graph::NodeIndex >,
    },
    /// With topologically sorted list.
    List( Vec< String > ),
    /// Nothing to show.
    #[ default ]
    Empty
  }

  /// Wrapper to redirect output from `ptree` graph to `fmt::Write`
  pub( crate ) struct Io2FmtWrite< 'a, W >
  {
    pub f : &'a mut W,
  }

  impl< W : std::fmt::Write > std::io::Write for Io2FmtWrite< '_, W >
  {
    fn write( &mut self, buf : &[ u8 ] ) -> std::io::Result< usize >
    {
      use std::io::ErrorKind;

      let size = buf.len();

      self.f.write_str
      (
        std::str::from_utf8( buf )
        .map_err( | _ | std::io::Error::new( ErrorKind::InvalidData, "Allow only valid UTF-8 string" ) )?
      )
      .map_err( | e | std::io::Error::new( ErrorKind::Other, e ) )?;

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
    let graph = graph_build( &packages_map, matches!( filter, ListFilter::Local ) );
    let sorted = toposort( &graph, None ).map_err( | e | ( report.clone() , err!( "Failed to process toposort for packages: {:?}", e ) ) )?;

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

mod tests
{
  #[ test ]
  fn io2fmt_write()
  {
    use super::private::Io2FmtWrite;

    // Arrange
    fn accepts_io_write< W : std::io::Write >( mut w : W ) -> std::io::Result< () >
    {
      w.write( b"Hello, world!" )?;

      Ok( () )
    }

    let mut string = String::new();

    // Act
    accepts_io_write( Io2FmtWrite { f : &mut string } ).unwrap();

    // Assert
    assert_eq!( "Hello, world!", &string );
  }
}

//

crate::mod_interface!
{
  /// Argument for `list` endpoint. Sets the output format.
  orphan use ListFormat;
  /// Argument for `list` endpoint. Sets filter(local or all) packages should be in the output.
  orphan use ListFilter;
  /// Contains output of the endpoint.
  orphan use ListReport;
  /// List packages in workspace.
  orphan use list;
}

mod private
{
  use std::fmt::Write;
  use crate::CrateDir;
  use std::fmt::Formatter;

  /// Struct for formatting and printing tree-like structures.
  /// It contains symbols used for visualizing the tree and information about the tree nodes.
  #[ derive( Debug, Clone, Eq, PartialEq ) ]
  pub struct TreePrinter
  {
    /// Symbols used for visualizing the tree.
    symbols : Symbols,
    /// Information about the tree nodes.
    pub info : ListNodeReport,
  }

  impl TreePrinter 
  {
    /// Creates a new instance of `TreePrinter` with the provided node information.
    ///
    /// # Parameters
    ///
    /// - `info`: A reference to a `ListNodeReport` object containing information about the tree nodes.
    ///
    /// # Returns
    ///
    /// A new instance of `TreePrinter`.
    pub fn new(info : &ListNodeReport) -> Self 
    {
      TreePrinter
      {
        symbols : Symbols::default(),
        info : info.clone(),
      }
    }

    /// Displays the name, version, path, and dependencies of a package with appropriate indentation and spacing.
    ///
    /// # Arguments
    ///
    /// * `spacer` - A string used for indentation.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the formatted string or a `std::fmt::Error` if formatting fails.
    pub fn display_with_spacer( &self, spacer : &str ) -> Result< String, std::fmt::Error >
    {
      let mut f = String::new();

      write!( f, "{}", self.info.name )?;
      if let Some( version ) = &self.info.version { write!( f, " {version}" )? }
      if let Some( crate_dir ) = &self.info.crate_dir { write!( f, " {}", crate_dir )? }
      if self.info.duplicate { write!( f, "(*)" )? }
      write!( f, "\n" )?;

      let mut new_spacer = format!( "{spacer}{}  ", if self.info.normal_dependencies.len() < 2 { " " } else { self.symbols.down } );

      let mut normal_dependencies_iter = self.info.normal_dependencies.iter();
      let last = normal_dependencies_iter.next_back();

      for dep in normal_dependencies_iter
      {
        write!( f, "{spacer}{}{} {}", self.symbols.tee, self.symbols.right, Self::display_with_spacer( &TreePrinter::new( dep ), &new_spacer )? )?;
      }
      if let Some( last ) = last
      {
        new_spacer = format!( "{spacer}   " );
        write!( f, "{spacer}{}{} {}", self.symbols.ell, self.symbols.right, Self::display_with_spacer( &TreePrinter::new( last ), &new_spacer )? )?;
      }
      if !self.info.dev_dependencies.is_empty()
      {
        let mut dev_dependencies_iter = self.info.dev_dependencies.iter();
        let last = dev_dependencies_iter.next_back();
        write!( f, "{spacer}[dev-dependencies]\n" )?;
        for dep in dev_dependencies_iter
        {
          write!( f, "{spacer}{}{} {}", self.symbols.tee, self.symbols.right, Self::display_with_spacer( &TreePrinter::new( dep ), &new_spacer )? )?;
        }
        // unwrap - safe because `is_empty` check
        write!( f, "{spacer}{}{} {}", self.symbols.ell, self.symbols.right, Self::display_with_spacer( &TreePrinter::new( last.unwrap() ), &new_spacer )? )?;
      }
      if !self.info.build_dependencies.is_empty()
      {
        let mut build_dependencies_iter = self.info.build_dependencies.iter();
        let last = build_dependencies_iter.next_back();
        write!( f, "{spacer}[build-dependencies]\n" )?;
        for dep in build_dependencies_iter
        {
          write!( f, "{spacer}{}{} {}", self.symbols.tee, self.symbols.right, Self::display_with_spacer( &TreePrinter::new( dep ), &new_spacer )? )?;
        }
        // unwrap - safe because `is_empty` check
        write!( f, "{spacer}{}{} {}", self.symbols.ell, self.symbols.right, Self::display_with_spacer( &TreePrinter::new( last.unwrap() ), &new_spacer )? )?;
      }

      Ok( f )
    }
  }

  impl std::fmt::Display for TreePrinter
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      write!( f, "{}", self.display_with_spacer( "" )? )?;

      Ok( () )
    }
  }

  #[ derive( Debug, Clone, Eq, PartialEq ) ]
  struct Symbols
  {
    down : &'static str,
    tee : &'static str,
    ell : &'static str,
    right : &'static str,
  }

  impl Default for Symbols
  {
    fn default() -> Self {
      Self 
      { 
        down : "│",
        tee  : "├",
        ell  : "└",
        right : "─", 
      }
    }
  }

  /// Represents a node in a dependency graph.
  /// It holds essential information about the project dependencies. It is also capable
  /// of holding any nested dependencies in a recursive manner, allowing the modeling
  /// of complex dependency structures.
  #[ derive( Debug, Clone, Eq, PartialEq ) ]
  pub struct ListNodeReport
  {
    /// This could be the name of the library or crate.
    pub name : String,
    /// Ihe version of the crate.
    pub version : Option< String >,
    /// The path to the node's source files in the local filesystem. This is
    /// optional as not all nodes may have a local presence (e.g., nodes representing remote crates).
    pub crate_dir : Option< CrateDir >,
    /// This field is a flag indicating whether the Node is a duplicate or not.
    pub duplicate : bool,
    /// A list that stores normal dependencies.
    /// Each element in the list is also of the same 'ListNodeReport' type to allow
    /// storage of nested dependencies.
    pub normal_dependencies : Vec< ListNodeReport >,
    /// A list that stores dev dependencies(dependencies required for tests or examples).
    /// Each element in the list is also of the same 'ListNodeReport' type to allow
    /// storage of nested dependencies.
    pub dev_dependencies : Vec< ListNodeReport >,
    /// A list that stores build dependencies.
    /// Each element in the list is also of the same 'ListNodeReport' type to allow
    /// storage of nested dependencies.
    pub build_dependencies : Vec< ListNodeReport >,
  }
}

crate::mod_interface!
{
  orphan use TreePrinter;
  orphan use ListNodeReport;
}
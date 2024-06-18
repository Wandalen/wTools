mod private
{
	use std::fmt::Write;

	pub struct TreePrinter
	{
		pub symbols : Symbols,
		pub info : Info,
		pub dependencies : Dependencies,
	}

	impl TreePrinter 
	{
		const fn get_default_symbols() -> Symbols 
		{
			Symbols 
			{
				down : "│",
				tee  : "├",
				ell  : "└",
				right : "─",
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
		fn display_with_spacer( &self, spacer : &str ) -> Result< String, std::fmt::Error >
		{
			let mut f = String::new();

			write!( f, "{}", self.name )?;
			if let Some( version ) = &self.version { write!( f, " {version}" )? }
			if let Some( crate_dir ) = &self.crate_dir { write!( f, " {}", crate_dir )? }
			if self.duplicate { write!( f, "(*)" )? }
			write!( f, "\n" )?;

			let mut new_spacer = format!( "{spacer}{}  ", if self.normal_dependencies.len() < 2 { " " } else { UTF8_SYMBOLS.down } );

			let mut normal_dependencies_iter = self.normal_dependencies.iter();
			let last = normal_dependencies_iter.next_back();

			for dep in normal_dependencies_iter
			{
				write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.tee, UTF8_SYMBOLS.right, dep.display_with_spacer( &new_spacer )? )?;
			}
			if let Some( last ) = last
			{
				new_spacer = format!( "{spacer}   " );
				write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.ell, UTF8_SYMBOLS.right, last.display_with_spacer( &new_spacer )? )?;
			}
			if !self.dev_dependencies.is_empty()
			{
				let mut dev_dependencies_iter = self.dev_dependencies.iter();
				let last = dev_dependencies_iter.next_back();
				write!( f, "{spacer}[dev-dependencies]\n" )?;
				for dep in dev_dependencies_iter
				{
					write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.tee, UTF8_SYMBOLS.right, dep.display_with_spacer( &new_spacer )? )?;
				}
				// unwrap - safe because `is_empty` check
				write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.ell, UTF8_SYMBOLS.right, last.unwrap().display_with_spacer( &new_spacer )? )?;
			}
			if !self.build_dependencies.is_empty()
			{
				let mut build_dependencies_iter = self.build_dependencies.iter();
				let last = build_dependencies_iter.next_back();
				write!( f, "{spacer}[build-dependencies]\n" )?;
				for dep in build_dependencies_iter
				{
					write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.tee, UTF8_SYMBOLS.right, dep.display_with_spacer( &new_spacer )? )?;
				}
				// unwrap - safe because `is_empty` check
				write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.ell, UTF8_SYMBOLS.right, last.unwrap().display_with_spacer( &new_spacer )? )?;
			}

			Ok( f )
		}
	}

	struct Symbols
	{
		down : &'static str,
		tee : &'static str,
		ell : &'static str,
		right : &'static str,
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

	impl std::fmt::Display for ListNodeReport
	{
		fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
		{
			write!( f, "{}", self.display_with_spacer( "" )? )?;

			Ok( () )
		}
	}
}

crate::mod_interface!
{
	orphan use TreePrinter;
	orphan use ListNodeReport;
}
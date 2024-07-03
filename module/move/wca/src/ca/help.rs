pub( crate ) mod private
{
  use crate::*;
  use ca::
  {
    Command,
    Routine,
    Type,
    formatter::private::
    {
      HelpFormat,
      md_generator
    },
    tool::table::format_table,
  };

  use iter_tools::Itertools;
  use std::rc::Rc;
  use error::untyped::format_err;
  use former::Former;

  // qqq : for Bohdan : it should transparent mechanist which patch list of commands, not a stand-alone mechanism

  #[ derive( Debug, Default, Copy, Clone, PartialEq, Eq ) ]
  pub enum LevelOfDetail
  {
    #[ default ]
    None,
    Simple,
    Detailed,
  }

  /// Container for arguments passed to a help generator function.
  #[ derive( Debug, Former ) ]
  pub struct HelpGeneratorOptions< 'a >
  {
    /// Prefix that will be shown before command name
    #[ former( default = String::new() ) ]
    pub command_prefix : String,
    /// Show help for the specified commands
    pub for_commands : Vec< &'a Command >,
    /// Reresents how much information to display for the subjects
    ///
    /// - `None` - nothing
    /// - `Simple` - <subjects>
    /// - `Detailed` - each subject with information about it. E.g. `<String>`
    pub subject_detailing : LevelOfDetail,
    /// Reresents how much information to display for the properties
    ///
    /// - `None` - nothing
    /// - `Simple` - <properties>
    /// - `Detailed` - each property with information about it. E.g. `<property_name:String>`
    pub property_detailing : LevelOfDetail,
    /// Reresents how much information to display for the properties
    ///
    /// - `None` - nothing
    /// - `Simple` - short hint
    /// - `Detailed` - long hint
    pub description_detailing : LevelOfDetail,
    /// If enabled - shows complete description of subjects and properties
    pub with_footer : bool,
    /// Order of property and commands.
    pub order : Order,
  }

  // qqq : for Barsik : make possible to change properties order
  pub( crate ) fn generate_help_content( dictionary : &Dictionary, o : HelpGeneratorOptions< '_ > ) -> String
  {
    struct Row
    {
      name : String,
      args : String,
      hint : String,
      footer : String,
    }
    let for_single_command = | command : &Command |
    {
      let name = &command.phrase;
      let hint = match o.description_detailing
      {
        LevelOfDetail::None => "",
        _ if command.hint.is_empty() && command.long_hint.is_empty() => "",
        LevelOfDetail::Simple if !command.hint.is_empty() => command.hint.as_str(),
        LevelOfDetail::Detailed if !command.long_hint.is_empty() => command.long_hint.as_str(),
        _ if !command.long_hint.is_empty() => command.long_hint.as_str(),
        _ if !command.hint.is_empty() => command.hint.as_str(),
        _ => unreachable!(),
      };
      let subjects = match o.subject_detailing
      {
        LevelOfDetail::None => "".into(),
        _ if command.subjects.is_empty() => "".into(),
        LevelOfDetail::Simple => "< subjects >".into(),
        LevelOfDetail::Detailed => command.subjects.iter().map( | v | format!( "< {}{:?} >", if v.optional { "?" } else { "" }, v.kind ) ).collect::< Vec< _ > >().join( " " ),
      };
      let properties = match o.property_detailing
      {
        LevelOfDetail::None => "".into(),
        _ if command.subjects.is_empty() => "".into(),
        LevelOfDetail::Simple => "< properties >".into(),
        LevelOfDetail::Detailed => command.properties( dictionary.order ).iter().map( |( n, v )| format!( "< {}:{}{:?} >", if v.optional { "?" } else { "" }, n, v.kind ) ).collect::< Vec< _ > >().join( " " ),
      };

      let footer = if o.with_footer
      {
        let full_subjects = command.subjects.iter().map( | subj | format!( "- {} [{}{:?}]", subj.hint, if subj.optional { "?" } else { "" }, subj.kind ) ).join( "\n\t" );
        let full_properties = format_table( command.properties( dictionary.order ).into_iter().map( | ( name, value ) | [ name.clone(), format!( "- {} [{}{:?}]", value.hint, if value.optional { "?" } else { "" }, value.kind ) ] ) ).unwrap().replace( '\n', "\n\t" );

        format!
        (
          "{}{}",
          if command.subjects.is_empty() { "".to_string() } else { format!( "\nSubjects:\n\t{}", &full_subjects ) },
          if command.properties.is_empty() { "".to_string() } else { format!( "\nProperties:\n\t{}",&full_properties ) }
        )
      } else { "".into() };

      Row
      {
        name : format!( "{}{name}", o.command_prefix ),
        args : format!( "{subjects}{}{properties}", if !subjects.is_empty() || !properties.is_empty() { " " } else { "" } ),
        hint : format!( "{}{hint}", if hint.is_empty() { "" } else { "- " } ),
        footer,
      }
    };
    if o.for_commands.len() == 1 || !o.for_commands.is_empty() && !o.with_footer
    {
      o.for_commands.into_iter().map( | command |
      {
        let row = for_single_command( command );
        format!
        (
          "{}{}{}",
          format_table([[ row.name, row.args, row.hint ]]).unwrap(),
          if row.footer.is_empty() { "" } else { "\n" },
          row.footer
        )
      })
      .join( "\n" )
    }
    else
    {
      let rows = dictionary.commands()
      .into_iter()
      .map( |( _, cmd )| cmd )
      .map( for_single_command )
      .map( | row | [ row.name, row.args, row.hint ] );
      format_table( rows ).unwrap()
    }
  }

  /// Available help commands variants
  #[ derive( Debug, Hash, Eq, PartialEq, Ord, PartialOrd ) ]
  pub enum HelpVariants
  {
    /// Make all available variants
    All,
    /// Help for whole program. E.g. `.help`
    General,
    /// Detailed help for one command as subject in help command. E.g. `.help command_name`
    SubjectCommand,
    /// Detailed help for one command as separate help command. E.g. `.help.command_name`
    DotCommand,
  }

  impl HelpVariants
  {
    /// Generates help commands
    pub fn generate( &self, helper : &HelpGeneratorFn, dictionary : &mut Dictionary, order : Order )
    {
      match self
      {
        HelpVariants::All =>
        {
          self.general_help( helper, dictionary, order );
          self.subject_command_help( helper, dictionary );
          // self.dot_command_help( helper, dictionary );
        },
        HelpVariants::General => self.general_help( helper, dictionary, order ),
        HelpVariants::SubjectCommand => self.subject_command_help( helper, dictionary ),
        _ => unimplemented!()
        // HelpVariants::DotCommand => self.dot_command_help( helper, dictionary ),
      }
    }

    // .help
    fn general_help( &self, helper : &HelpGeneratorFn, dictionary : &mut Dictionary, order : Order )
    {
      let phrase = "help".to_string();

      let grammar = dictionary.clone();
      let generator = helper.clone();

      let moved_phrase = phrase.clone();
      let routine = move | o : VerifiedCommand |
      {
        let subject_help = grammar.command( &moved_phrase );
        match &subject_help
        {
          Some( Command { routine: Routine::WithoutContext( help ), .. } )
          if !o.args.0.is_empty() => help( o )?,
          _ =>
          {
            let format_prop : String = o.props.get_owned( "format" ).unwrap_or_default();
            let format = match format_prop.as_str()
            {
              "md" | "markdown" => HelpFormat::Markdown,
              _ => HelpFormat::Another,
            };
            if format == HelpFormat::Markdown
            {
              println!( "Help command\n{text}", text = md_generator( &grammar, order ) );
            }
            else
            {
              let options = HelpGeneratorOptions::former()
              .command_prefix( "." )
              .description_detailing( LevelOfDetail::Simple )
              .subject_detailing( LevelOfDetail::Simple )
              .property_detailing( LevelOfDetail::Simple );
              println!
              (
                "Help command\n\n{text}",
                text = generator.exec
                (
                  &grammar,
                  options.form()
                )
              );
            }
          }
        }

        Ok::< _, error_tools::untyped::Error >( () )
      };
      let help = Command::former()
      .hint( "prints information about existing commands" )
      .property( "format" )
        .hint( "help generates in format witch you write" )
        .kind( Type::String )
        .optional( true )
        .end()
      .phrase( &phrase )
      .routine( routine )
      .form();

      dictionary.register( help );
    }

    // .help command_name
    fn subject_command_help( &self, helper : &HelpGeneratorFn, dictionary : &mut Dictionary )
    {
      let phrase = "help".to_string();

      let grammar = dictionary.clone();
      let generator = helper.clone();

      let moved_phrase = phrase.clone();
      let routine = move | o : VerifiedCommand |
      {
        let full_help = grammar.command( &moved_phrase );
        match &full_help
        {
          Some( Command { routine: Routine::WithoutContext( help ), .. } )
          if o.args.0.is_empty() => help( o )?,
          _ =>
          {
            let command = o.args.get_owned::< String >( 0 ).unwrap();
            let cmd = grammar.commands
            .get( &command )
            .ok_or_else( || format_err!( "Can not found help for command `{command}`" ) )?;

            let args = HelpGeneratorOptions::former()
            .command_prefix( "." )
            .for_commands([ cmd ])
            .description_detailing( LevelOfDetail::Detailed )
            .subject_detailing( LevelOfDetail::Simple )
            .property_detailing( LevelOfDetail::Simple )
            .with_footer( true );

            let text = generator.exec( &grammar, args.form() );

            println!( "Help command\n\n{text}" );
          }
        };

        Ok::< _, error_tools::untyped::Error >( () )
      };

      let help = Command::former()
      .hint( "prints full information about a specified command" )
      .subject().hint( "command name" ).kind( Type::String ).optional( true ).end()
      .property( "format" ).hint( "help generates in format witch you write" ).kind( Type::String ).optional( true ).end()
      .phrase( &phrase )
      .routine( routine )
      .form();

      dictionary.register( help );
    }

    // .help.command_name
    // fn dot_command_help( &self, helper : &HelpGeneratorFn, grammar : &mut Dictionary )
    // {
    //   // generate commands names
    //   let commands : Vec< _ > = grammar.commands.iter().map( |( name, cmd )| ( format!( "help.{name}" ), cmd.clone() ) ).collect();
    //
    //   // generate Commands grammar
    //   let grammar_helps = commands
    //   .iter()
    //   .map( |( help_name, _ )| Command::former().hint( "prints full information about a specified command" ).phrase( help_name ).form() )
    //   .collect::< Vec< _ > >();
    //
    //   // add commands to Verifier
    //   for cmd in grammar_helps
    //   {
    //     let command_variants = grammar.commands.entry( cmd.phrase.to_owned() ).or_insert_with( Vec::new );
    //     command_variants.push( cmd );
    //   }
    //
    //   // generate Commands routines
    //   let executable = commands
    //   .into_iter()
    //   .fold( vec![], | mut acc, ( help_name, cmds ) |
    //   {
    //     let generator = helper.clone();
    //     // TODO: Will be static
    //     let grammar = grammar.clone();
    //
    //     let routine = Routine::new( move | _ |
    //     {
    //       let text = cmds.iter()
    //       .map
    //       (
    //         | cmd | generator.exec( &grammar, Some( cmd ) )
    //       )
    //       .join( "\n\n" );
    //
    //       println!( "Help for command\n\n{text}" );
    //
    //       Ok( () )
    //     });
    //     acc.push(( help_name, routine ));
    //
    //     acc
    //   });
    //
    //   // add commands to ExecutorConverter
    //   for ( phrase, routine ) in executable
    //   {
    //     executor.routines.insert( phrase, routine );
    //   }
    // }
  }

  type HelpFunctionFn = Rc< dyn Fn( &Dictionary, HelpGeneratorOptions< '_ > ) -> String >;

  /// Container for function that generates help string for any command
  ///
  /// ```
  /// # use wca::ca::help::{ HelpGeneratorOptions, HelpGeneratorFn };
  /// use wca::{ Command, Dictionary };
  ///
  /// fn my_help_generator( dictionary : &Dictionary, args : HelpGeneratorOptions< '_ > ) -> String
  /// {
  ///   format!( "Help content based on grammar and command" )
  /// }
  ///
  /// let help_fn = HelpGeneratorFn::new( my_help_generator );
  /// # let grammar = &Dictionary::former().form();
  ///
  /// help_fn.exec( grammar, HelpGeneratorOptions::former().form() );
  /// // or
  /// # let cmd = Command::former().form();
  /// help_fn.exec( grammar, HelpGeneratorOptions::former().for_commands( [ &cmd ] ).form() );
  /// ```
  #[ derive( Clone ) ]
  pub struct HelpGeneratorFn( HelpFunctionFn );

  impl Default for HelpGeneratorFn
  {
    fn default() -> Self
    {
      Self( Rc::new( generate_help_content ) )
    }
  }

  impl HelpGeneratorFn
  {
    /// Wrap a help function
    pub fn new< HelpFunction >( func : HelpFunction ) -> Self
    where
      HelpFunction : Fn( &Dictionary, HelpGeneratorOptions< '_ > ) -> String + 'static
    {
        Self( Rc::new( func ) )
    }
  }

  impl HelpGeneratorFn
  {
    /// Executes the function to generate help content
    pub fn exec( &self, dictionary : &Dictionary, args : HelpGeneratorOptions< '_ > ) -> String
    {
      self.0( dictionary, args )
    }
  }

  impl std::fmt::Debug for HelpGeneratorFn
  {
    fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      f.write_str( "HelpGenerator" )
    }
  }
}

//

crate::mod_interface!
{
  own use HelpGeneratorFn;
  own use HelpGeneratorOptions;
  prelude use HelpVariants;
}

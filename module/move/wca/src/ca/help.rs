pub( crate ) mod private
{
  use crate::*;
  use ca::
  {
    Command,
    Routine, Type, formatter::private::{ HelpFormat, md_generator },
  };

  use wtools::Itertools;
  use std::rc::Rc;
  use error_tools::for_app::anyhow;
  use former::Former;

  // qqq : for Bohdan : it should transparent mechanist which patch list of commands, not a stand-alone mechanism

  /// Generate `dot` command
  pub fn dot_command( generator : &HelpGeneratorFn, dictionary : &mut Dictionary )
  {
    let generator = generator.clone();
    let grammar = dictionary.clone();
    let routine = move | props : Props |
    {
      let prefix : String = props.get_owned( "command_prefix" ).unwrap();

      let generator_args = HelpGeneratorArgs::former()
      .command_prefix( prefix )
      .form();

      println!( "{}", generator.exec( &grammar, generator_args ) );
    };

    let cmd = Command::former()
    .hint( "prints all available commands" )
    .phrase( "" )
    .property( "command_prefix" ).kind( Type::String ).end()
    .routine( routine )
    .form();

    dictionary.register( cmd );
  }

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
  pub struct HelpGeneratorArgs< 'a >
  {
    #[ default( String::new() ) ]
    pub command_prefix : String,
    pub for_command : Option< &'a Command >,
    pub subject_detailing : LevelOfDetail,
    pub property_detailing : LevelOfDetail,
    pub description_detailing : LevelOfDetail,
    pub with_footer : bool,
  }

  fn generate_help_content( dictionary : &Dictionary, args : HelpGeneratorArgs< '_ > ) -> String
  {
    let for_single_command = | command : &Command |
    {
      let name = &command.phrase;
      let hint = match args.description_detailing
      {
        LevelOfDetail::None => "",
        _ if command.hint.is_empty() && command.long_hint.is_empty() => "",
        LevelOfDetail::Simple if !command.hint.is_empty() => command.hint.as_str(),
        LevelOfDetail::Detailed if !command.long_hint.is_empty() => command.long_hint.as_str(),
        _ if !command.long_hint.is_empty() => command.long_hint.as_str(),
        _ if !command.hint.is_empty() => command.hint.as_str(),
        _ => unreachable!(),
      };
      let subjects = match args.subject_detailing
      {
        LevelOfDetail::None => "".into(),
        _ if command.subjects.is_empty() => "".into(),
        LevelOfDetail::Simple => "<subjects>".into(),
        LevelOfDetail::Detailed => command.subjects.iter().map( | v | format!( "<{}{:?}>", if v.optional { "?" } else { "" }, v.kind ) ).collect::< Vec< _ > >().join( " " ),
      };
      let properties = match args.property_detailing
      {
        LevelOfDetail::None => "".into(),
        _ if command.subjects.is_empty() => "".into(),
        LevelOfDetail::Simple => "<properties>".into(),
        LevelOfDetail::Detailed => command.properties.iter().map( |( n, v )| format!( "<{n}:{}{:?}>", if v.optional { "?" } else { "" }, v.kind ) ).collect::< Vec< _ > >().join( " " ),
      };

      let footer = if args.with_footer
      {
        let full_subjects = command.subjects.iter().map( | subj | format!( "- {} [{}{:?}]", subj.hint, if subj.optional { "?" } else { "" }, subj.kind ) ).join( "\n\t" );
        let full_properties = command.properties.iter().sorted_by_key( |( name, _ )| *name ).map( |( name, value )| format!( "{name} - {} [{}{:?}]", value.hint, if value.optional { "?" } else { "" }, value.kind ) ).join( "\n\t" );
        format!
        (
          "{}{}",
          if command.subjects.is_empty() { "".to_string() } else { format!( "\nSubjects:\n\t{}", &full_subjects ) },
          if command.properties.is_empty() { "".to_string() } else { format!( "\nProperties:\n\t{}",&full_properties ) }
        )
      } else { "".into() };

      format!
      (
        "{}{name}{}{subjects}{}{properties}{}{hint}{}{footer}",
        args.command_prefix,
        if !subjects.is_empty() || !properties.is_empty() { " " } else { "" },
        if properties.is_empty() { "" } else { " " },
        if hint.is_empty() { "" } else { " - " },
        if footer.is_empty() { "" } else { "\n" },
      )
    };
    if let Some( command ) = args.for_command
    {
      for_single_command( command )
    }
    else
    {
      dictionary.commands
      .iter()
      .sorted_by_key( |( name, _ )| *name )
      .map( |( _, cmd )| cmd )
      .map( for_single_command )
      .fold( String::new(), | acc, cmd |
      {
        format!( "{acc}{}{cmd}", if acc.is_empty() { "" } else { "\n" } )
      })
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
    pub fn generate( &self, helper : &HelpGeneratorFn, dictionary : &mut Dictionary )
    {
      match self
      {
        HelpVariants::All =>
        {
          self.general_help( helper, dictionary );
          self.subject_command_help( helper, dictionary );
          // self.dot_command_help( helper, dictionary );
        },
        HelpVariants::General => self.general_help( helper, dictionary ),
        HelpVariants::SubjectCommand => self.subject_command_help( helper, dictionary ),
        _ => unimplemented!()
        // HelpVariants::DotCommand => self.dot_command_help( helper, dictionary ),
      }
    }

    // .help
    fn general_help( &self, helper : &HelpGeneratorFn, dictionary : &mut Dictionary )
    {
      let phrase = "help".to_string();

      let grammar = dictionary.clone();
      let generator = helper.clone();

      let moved_phrase = phrase.clone();
      let routine = move | args : Args, props : Props |
      {
        let subject_help = grammar.command( &moved_phrase );
        match &subject_help
        {
          Some( Command { routine: Routine::WithoutContext( help ), .. } )
          if !args.is_empty() => help(( args, props ))?,
          _ =>
          {
            let format_prop : String = props.get_owned( "format" ).unwrap_or_default();
            let format = match format_prop.as_str()
            {
              "md" | "markdown" => HelpFormat::Markdown,
              _ => HelpFormat::Another,
            };
            if format == HelpFormat::Markdown
            {
              println!( "Help command\n{text}", text = md_generator( &grammar ) );
            }
            else
            {
              println!
              (
                "Help command\n\n{text}",
                text = generator.exec
                (
                  &grammar,
                  HelpGeneratorArgs::former()
                  .description_detailing( LevelOfDetail::Simple )
                  .subject_detailing( LevelOfDetail::Simple )
                  .property_detailing( LevelOfDetail::Simple )
                  .form()
                )
              );
            }
          }
        }

        Ok::< _, error_tools::for_app::Error >( () )
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
      let routine = move | args : Args, props : Props |
      {
        let full_help = grammar.command( &moved_phrase );
        match &full_help
        {
          Some( Command { routine: Routine::WithoutContext( help ), .. } )
          if args.is_empty() => help(( args, props ))?,
          _ =>
          {
            let command = args.get_owned::< String >( 0 ).unwrap();
            let cmd = grammar.commands.get( &command ).ok_or_else( || anyhow!( "Can not found help for command `{command}`" ) )?;

            let args = HelpGeneratorArgs::former()
            .for_command( cmd )
            .description_detailing( LevelOfDetail::Detailed )
            .subject_detailing( LevelOfDetail::Simple )
            .property_detailing( LevelOfDetail::Simple )
            .with_footer( true )
            .form();
            let text = generator.exec( &grammar, args );

            println!( "Help command\n\n{text}" );
          }
        };

        Ok::< _, error_tools::for_app::Error >( () )
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

  type HelpFunctionFn = Rc< dyn Fn( &Dictionary, HelpGeneratorArgs< '_ > ) -> String >;

  /// Container for function that generates help string for any command
  ///
  /// ```
  /// # use wca::ca::help::{ HelpGeneratorArgs, HelpGeneratorFn };
  /// use wca::{ Command, Dictionary };
  ///
  /// fn my_help_generator( grammar : &Dictionary, command : Option< &Command > ) -> String
  /// {
  ///   format!( "Help content based on grammar and command" )
  /// }
  ///
  /// let help_fn = HelpGeneratorFn::new( my_help_generator );
  /// # let grammar = &Dictionary::former().form();
  ///
  /// help_fn.exec( grammar, HelpGeneratorArgs::former().form() );
  /// // or
  /// # let cmd = Command::former().form();
  /// help_fn.exec( grammar, HelpGeneratorArgs::former().for_command( &cmd ).form() );
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
      HelpFunction : Fn( &Dictionary, HelpGeneratorArgs< '_ > ) -> String + 'static
    {
        Self( Rc::new( func ) )
    }
  }

  impl HelpGeneratorFn
  {
    /// Executes the function to generate help content
    pub fn exec( &self, dictionary : &Dictionary, args : HelpGeneratorArgs< '_ > ) -> String
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
  protected use HelpGeneratorFn;
  protected use HelpGeneratorArgs;
  protected use dot_command;
  prelude use HelpVariants;
}

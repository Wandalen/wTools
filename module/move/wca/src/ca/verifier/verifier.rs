pub( crate ) mod private
{
  use crate::*;

  use ca::grammar::command::ValueDescription;
  // use former::Former;
  use std::collections::HashMap;
  use indexmap::IndexMap;
  use wtools::{ error, error::Result, err };
  use ca::help::private::{ HelpGeneratorOptions, LevelOfDetail, generate_help_content };

  /// Converts a `ParsedCommand` to a `VerifiedCommand` by performing validation and type casting on values.
  ///
  /// ```
  /// # use wca::{ Command, Type, Verifier, Dictionary, ParsedCommand };
  /// # use std::collections::HashMap;
  /// # fn main() -> Result< (), Box< dyn std::error::Error > >
  /// # {
  /// # let verifier = Verifier;
  /// let dictionary = Dictionary::former()
  /// .command( Command::former().phrase( "command" ).form() )
  /// .form();
  ///
  /// let raw_command = ParsedCommand
  /// {
  ///   name: "command".to_string(),
  ///   subjects: vec![],
  ///   properties: HashMap::new(),
  /// };
  ///
  /// let grammar_command = verifier.to_command( &dictionary, raw_command )?;
  /// # Ok( () )
  /// # }
  /// ```
  #[ derive( Debug, Clone ) ]
  pub struct Verifier;

  impl Verifier
  {
    /// Converts raw program to grammatically correct
    ///
    /// Converts all namespaces into it with `to_namespace` method.
    pub fn to_program
    (
      &self,
      dictionary : &Dictionary,
      raw_program : Program< ParsedCommand >
    )
    -> Result< Program< VerifiedCommand > >
    {
      let commands = raw_program.commands
      .into_iter()
      .map( | n | self.to_command( dictionary, n ) )
      .collect::< Result< Vec< VerifiedCommand > > >()?;

      Ok( Program { commands } )
    }

    #[ cfg( feature = "on_unknown_suggest" ) ]
    fn suggest_command< 'a >( dictionary : &'a Dictionary, user_input: &str ) -> Option< &'a str >
    {
      use textdistance::{ Algorithm, JaroWinkler };
      let jaro = JaroWinkler::default();
      let sim = dictionary
      .commands
      .iter()
      .map( |( name, c )| ( jaro.for_str( name.as_str(), user_input ).nsim(), c ) )
      .max_by( |( s1, _ ), ( s2, _ )| s1.total_cmp( s2 ) );
      if let Some(( sim, variant )) = sim
      {
        if sim > 0.0
        {
          let phrase = &variant.phrase;
          return Some( phrase );
        }
      }

      None
    }

    fn get_count_from_properties
    (
      properties : &IndexMap< String, ValueDescription >,
      properties_aliases : &HashMap< String, String >,
      raw_properties : &HashMap< String, String >
    ) -> usize
    {
      raw_properties.iter()
        .filter( |( k, _ )| !( properties.contains_key( *k ) || properties_aliases.get( *k ).map_or( false, | key | properties.contains_key( key ) ) ) )
        .count()
    }

    fn is_valid_command_variant( subjects_count : usize, raw_count : usize, possible_count : usize ) -> bool
    {
      raw_count + possible_count <= subjects_count
    }

    fn check_command< 'a >( variant : &'a Command, raw_command : &ParsedCommand ) -> Option< &'a Command >
    {
      let Command { subjects, properties, properties_aliases, .. } = variant;
      let raw_subjects_count = raw_command.subjects.len();
      let expected_subjects_count = subjects.len();
      if raw_subjects_count > expected_subjects_count { return None; }

      let possible_subjects_count = Self::get_count_from_properties( properties, properties_aliases, &raw_command.properties );
      if Self::is_valid_command_variant( expected_subjects_count, raw_subjects_count, possible_subjects_count ) { Some( variant ) } else { None }
    }

    // qqq : for Barsik :
    // Problem with separating properties and subjects:
    // if we pass to wca a command that has an incorrectly named property, it defines this property as part of an subject.
    // You can simulate this problem by running the code from https://github.com/Wandalen/wTools/blob/alpha/module/move/wca/examples/wca_trivial.rs in this form `cargo r .echo propertyf:123`
    // where the console shows that the subject is `propertyf:123` and the property is empty.
    //
    // I would like to get an error in this case.
    //
    // A real example of the problem can be seen in the `.test` command in willbe where if you don't specify the option and make a mistake in the name of the properties when running it,
    // the option will be an incorrectly written property that will produce an error with unobvious output.
    // log:
    // kosli@kos-msi-creator MINGW64 /c/pro/rust/lib/wTools/module/move/willbe (alpha)
    // $ RUST_BACKTRACE=1 cargo run .test enabled_features:enabled power:1 dry:0
    // warning: usage of an `unsafe` block
    //   --> module\move\wca\src\ca\executor\context.rs:88:7
    //    |
    // 88 |       unsafe{ self.inner.as_ptr().as_ref()?.get() }
    //    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //    |
    //    = note: requested on the command line with `-W unsafe-code`
    //
    // warning: usage of an `unsafe` block
    //   --> module\move\wca\src\ca\executor\context.rs:94:7
    //    |
    // 94 |       unsafe { self.inner.as_ptr().as_mut()?.get_mut() }
    //    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //
    // warning: method `deep_clone` is never used
    //    --> module\move\wca\src\ca\executor\context.rs:120:21
    //     |
    // 70  |   impl Context
    //     |   ------------ method in this implementation
    // ...
    // 120 |     pub( crate ) fn deep_clone( &self ) -> Self
    //     |                     ^^^^^^^^^^
    //     |
    //     = note: `#[warn(dead_code)]` on by default
    //
    // warning: `wca` (lib) generated 3 warnings
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
    //      Running `C:\pro\rust\lib\wTools\target\debug\will.exe .test 'enabled_features:enabled' 'power:1' 'dry:0'`
    // Error: Execution failed. The system cannot find the file specified. (os error 2)
    //
    // Stack backtrace:
    //    0: std::backtrace_rs::backtrace::dbghelp64::trace
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f/library\std\src\..\..\backtrace\src\backtrace\dbghelp64.rs:99
    //    1: std::backtrace_rs::backtrace::trace_unsynchronized
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f/library\std\src\..\..\backtrace\src\backtrace\mod.rs:66
    //    2: std::backtrace::Backtrace::create
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f/library\std\src\backtrace.rs:331
    //    3: std::backtrace::Backtrace::capture
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f/library\std\src\backtrace.rs:296
    //    4: anyhow::error::impl$1::from<std::io::error::Error>
    //              at C:\Users\kosli\.cargo\registry\src\index.crates.io-6f17d22bba15001f\anyhow-1.0.81\src\error.rs:565
    //    5: core::result::impl$27::from_residual<tuple$<>,std::io::error::Error,anyhow::Error>
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\core\src\result.rs:1964
    //    6: willbe::command::test::private::test
    //              at .\src\command\test.rs:50
    //    7: core::ops::function::Fn::call<enum2$<core::result::Result<tuple$<>,anyhow::Error> > (*)(wca::ca::executor::routine::private::Args,wca::ca::executor::routine::private::Props),tuple$<wca::ca::executor::routine::private::Args,wca::ca::executor::routine::priv
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\core\src\ops\function.rs:79
    //    8: wca::ca::executor::routine::private::impl$8::from::closure$0<enum2$<core::result::Result<tuple$<>,anyhow::Error> > (*)(wca::ca::executor::routine::private::Args,wca::ca::executor::routine::private::Props),enum2$<core::result::Result<tuple$<>,anyhow::Error
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\executor\routine.rs:218
    //    9: alloc::boxed::impl$49::call<tuple$<tuple$<wca::ca::executor::routine::private::Args,wca::ca::executor::routine::private::Props> >,dyn$<core::ops::function::Fn<tuple$<tuple$<wca::ca::executor::routine::private::Args,wca::ca::executor::routine::private::Pro
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\alloc\src\boxed.rs:2029
    //   10: wca::ca::executor::routine::private::impl$13::from::closure$0<tuple$<wca::ca::executor::routine::private::Args,wca::ca::executor::routine::private::Props>,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\executor\routine.rs:275
    //   11: alloc::boxed::impl$49::call<tuple$<tuple$<wca::ca::executor::routine::private::Args,wca::ca::executor::routine::private::Props> >,dyn$<core::ops::function::Fn<tuple$<tuple$<wca::ca::executor::routine::private::Args,wca::ca::executor::routine::private::Pro
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\alloc\src\boxed.rs:2029
    //   12: wca::ca::executor::routine::private::impl$18::from::closure$0
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\executor\routine.rs:335
    //   13: wca::ca::executor::runtime::private::_exec_command
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\executor\runtime.rs:92
    //   14: wca::ca::executor::runtime::private::impl$0::do::closure$1
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\executor\runtime.rs:80
    //   15: enum2$<core::result::Result<ref$<wca::ca::verifier::command::private::VerifiedCommand>,anyhow::Error> >::and_then<ref$<wca::ca::verifier::command::private::VerifiedCommand>,anyhow::Error,tuple$<>,wca::ca::executor::runtime::private::impl$0::do::closure_en
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\core\src\result.rs:1321
    //   16: wca::ca::executor::runtime::private::Runtime::do
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\executor\runtime.rs:73
    //   17: wca::ca::executor::executor::private::Executor::sequential_execution_loop
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\executor\executor.rs:60
    //   18: wca::ca::executor::executor::private::Executor::program
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\executor\executor.rs:37
    //   19: wca::ca::aggregator::private::CommandsAggregator::perform<ref$<str$> >
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\aggregator.rs:276
    //   20: willbe::private::run
    //              at .\src\lib.rs:42
    //   21: will::main
    //              at .\src\bin\will.rs:14
    //   22: core::ops::function::FnOnce::call_once<enum2$<core::result::Result<tuple$<>,anyhow::Error> > (*)(),tuple$<> >
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\core\src\ops\function.rs:250
    //   23: std::sys_common::backtrace::__rust_begin_short_backtrace<enum2$<core::result::Result<tuple$<>,anyhow::Error> > (*)(),enum2$<core::result::Result<tuple$<>,anyhow::Error> > >
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\std\src\sys_common\backtrace.rs:155
    //   24: std::rt::lang_start::closure$0<enum2$<core::result::Result<tuple$<>,anyhow::Error> > >
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\std\src\rt.rs:166
    //   25: std::rt::lang_start_internal
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f/library\std\src\rt.rs:148
    //   26: std::rt::lang_start<enum2$<core::result::Result<tuple$<>,anyhow::Error> > >
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\std\src\rt.rs:165
    //   27: main
    //   28: invoke_main
    //              at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
    //   29: __scrt_common_main_seh
    //              at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
    //   30: BaseThreadInitThunk
    //   31: RtlUserThreadStart
    //
    // Stack backtrace:
    //    0: std::backtrace_rs::backtrace::dbghelp64::trace
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f/library\std\src\..\..\backtrace\src\backtrace\dbghelp64.rs:99
    //    1: std::backtrace_rs::backtrace::trace_unsynchronized
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f/library\std\src\..\..\backtrace\src\backtrace\mod.rs:66
    //    2: std::backtrace::Backtrace::create
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f/library\std\src\backtrace.rs:331
    //    3: std::backtrace::Backtrace::capture
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f/library\std\src\backtrace.rs:296
    //    4: anyhow::Error::msg<alloc::string::String>
    //              at C:\Users\kosli\.cargo\registry\src\index.crates.io-6f17d22bba15001f\anyhow-1.0.81\src\error.rs:83
    //    5: anyhow::__private::format_err
    //              at C:\Users\kosli\.cargo\registry\src\index.crates.io-6f17d22bba15001f\anyhow-1.0.81\src\lib.rs:691
    //    6: wca::ca::executor::routine::private::impl$28::into_result::closure$0<anyhow::Error>
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\executor\routine.rs:450
    //    7: enum2$<core::result::Result<tuple$<>,anyhow::Error> >::map_err<tuple$<>,anyhow::Error,anyhow::Error,wca::ca::executor::routine::private::impl$28::into_result::closure_env$0<anyhow::Error> >
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\core\src\result.rs:829
    //    8: wca::ca::executor::routine::private::impl$28::into_result<anyhow::Error>
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\executor\routine.rs:450
    //    9: wca::ca::executor::routine::private::impl$13::from::closure$0<tuple$<wca::ca::executor::routine::private::Args,wca::ca::executor::routine::private::Props>,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\executor\routine.rs:275
    //   10: alloc::boxed::impl$49::call<tuple$<tuple$<wca::ca::executor::routine::private::Args,wca::ca::executor::routine::private::Props> >,dyn$<core::ops::function::Fn<tuple$<tuple$<wca::ca::executor::routine::private::Args,wca::ca::executor::routine::private::Pro
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\alloc\src\boxed.rs:2029
    //   11: wca::ca::executor::routine::private::impl$18::from::closure$0
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\executor\routine.rs:335
    //   12: wca::ca::executor::runtime::private::_exec_command
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\executor\runtime.rs:92
    //   13: wca::ca::executor::runtime::private::impl$0::do::closure$1
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\executor\runtime.rs:80
    //   14: enum2$<core::result::Result<ref$<wca::ca::verifier::command::private::VerifiedCommand>,anyhow::Error> >::and_then<ref$<wca::ca::verifier::command::private::VerifiedCommand>,anyhow::Error,tuple$<>,wca::ca::executor::runtime::private::impl$0::do::closure_en
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\core\src\result.rs:1321
    //   15: wca::ca::executor::runtime::private::Runtime::do
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\executor\runtime.rs:73
    //   16: wca::ca::executor::executor::private::Executor::sequential_execution_loop
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\executor\executor.rs:60
    //   17: wca::ca::executor::executor::private::Executor::program
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\executor\executor.rs:37
    //   18: wca::ca::aggregator::private::CommandsAggregator::perform<ref$<str$> >
    //              at C:\pro\rust\lib\wTools\module\move\wca\src\ca\aggregator.rs:276
    //   19: willbe::private::run
    //              at .\src\lib.rs:42
    //   20: will::main
    //              at .\src\bin\will.rs:14
    //   21: core::ops::function::FnOnce::call_once<enum2$<core::result::Result<tuple$<>,anyhow::Error> > (*)(),tuple$<> >
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\core\src\ops\function.rs:250
    //   22: std::sys_common::backtrace::__rust_begin_short_backtrace<enum2$<core::result::Result<tuple$<>,anyhow::Error> > (*)(),enum2$<core::result::Result<tuple$<>,anyhow::Error> > >
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\std\src\sys_common\backtrace.rs:155
    //   23: std::rt::lang_start::closure$0<enum2$<core::result::Result<tuple$<>,anyhow::Error> > >
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\std\src\rt.rs:166
    //   24: std::rt::lang_start_internal
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f/library\std\src\rt.rs:148
    //   25: std::rt::lang_start<enum2$<core::result::Result<tuple$<>,anyhow::Error> > >
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\std\src\rt.rs:165
    //   26: main
    //   27: invoke_main
    //              at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
    //   28: __scrt_common_main_seh
    //              at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
    //   29: BaseThreadInitThunk
    //   30: RtlUserThreadStart
    //
    // Stack backtrace:
    //    0: std::backtrace_rs::backtrace::dbghelp64::trace
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f/library\std\src\..\..\backtrace\src\backtrace\dbghelp64.rs:99
    //    1: std::backtrace_rs::backtrace::trace_unsynchronized
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f/library\std\src\..\..\backtrace\src\backtrace\mod.rs:66
    //    2: std::backtrace::Backtrace::create
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f/library\std\src\backtrace.rs:331
    //    3: std::backtrace::Backtrace::capture
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f/library\std\src\backtrace.rs:296
    //    4: anyhow::error::impl$1::from<enum2$<wca::ca::aggregator::private::Error> >
    //              at C:\Users\kosli\.cargo\registry\src\index.crates.io-6f17d22bba15001f\anyhow-1.0.81\src\error.rs:565
    //    5: core::result::impl$27::from_residual<tuple$<>,enum2$<wca::ca::aggregator::private::Error>,anyhow::Error>
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\core\src\result.rs:1964
    //    6: willbe::private::run
    //              at .\src\lib.rs:42
    //    7: will::main
    //              at .\src\bin\will.rs:14
    //    8: core::ops::function::FnOnce::call_once<enum2$<core::result::Result<tuple$<>,anyhow::Error> > (*)(),tuple$<> >
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\core\src\ops\function.rs:250
    //    9: std::sys_common::backtrace::__rust_begin_short_backtrace<enum2$<core::result::Result<tuple$<>,anyhow::Error> > (*)(),enum2$<core::result::Result<tuple$<>,anyhow::Error> > >
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\std\src\sys_common\backtrace.rs:155
    //   10: std::rt::lang_start::closure$0<enum2$<core::result::Result<tuple$<>,anyhow::Error> > >
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\std\src\rt.rs:166
    //   11: std::rt::lang_start_internal
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f/library\std\src\rt.rs:148
    //   12: std::rt::lang_start<enum2$<core::result::Result<tuple$<>,anyhow::Error> > >
    //              at /rustc/3c85e56249b0b1942339a6a989a971bf6f1c9e0f\library\std\src\rt.rs:165
    //   13: main
    //   14: invoke_main
    //              at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
    //   15: __scrt_common_main_seh
    //              at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
    //   16: BaseThreadInitThunk
    //   17: RtlUserThreadStart
    // error: process didn't exit successfully: `C:\pro\rust\lib\wTools\target\debug\will.exe .test 'enabled_features:enabled' 'power:1' 'dry:0'` (exit code: 1)

    fn extract_subjects( command : &Command, raw_command : &ParsedCommand, used_properties : &[ &String ] ) -> Result< Vec< Value > >
    {
      let mut subjects = vec![];

      let all_subjects = raw_command
      .subjects.clone().into_iter()
      .chain
      (
        raw_command.properties.iter()
        .filter( |( key, _ )| !used_properties.contains( key ) )
        .map( |( key, value )| format!( "{key}:{value}" ) )
      )
      .collect::< Vec< _ > >();
      let mut rc_subjects_iter = all_subjects.iter();
      let mut current = rc_subjects_iter.next();

      for ValueDescription { kind, optional, .. } in &command.subjects
      {
        let value = match current.and_then( | v | kind.try_cast( v.clone() ).ok() )
        {
          Some( v ) => v,
          None if *optional => continue,
          _ => return Err( err!( "Missing not optional subject" ) ),
        };
        subjects.push( value );
        current = rc_subjects_iter.next();
      }
      if let Some( value ) = current { return Err( err!( "Can not identify a subject: `{}`", value ) ) }

      Ok( subjects )
    }

    fn extract_properties( command: &Command, raw_command : HashMap< String, String > ) -> Result< HashMap< String, Value > >
    {
      raw_command.into_iter()
      .filter_map
      (
        |( key, value )|
        // try to find a key
        if command.properties.contains_key( &key ) { Some( key ) }
        else if let Some( original_key ) = command.properties_aliases.get( &key ) { Some( original_key.clone() ) }
        else { None }
        // give a description. unwrap is safe because previous checks
        .map( | key | ( command.properties.get( &key ).unwrap(), key, value ) )
      )
      .map
      (
        |( value_description, key, value )|
        value_description.kind.try_cast( value ).map( | v | ( key.clone(), v ) )
      )
      .collect::< Result< HashMap< _, _ > > >()
    }

    fn group_properties_and_their_aliases< 'a, Ks >( aliases : &'a HashMap< String, String >, used_keys :  Ks ) -> Vec< &String >
    where
      Ks : Iterator< Item = &'a String >
    {
      let reverse_aliases =
      {
        let mut map = HashMap::< &String, Vec< &String > >::new();
        for ( property, alias ) in aliases
        {
          map.entry( alias ).or_default().push( property );
        }
        map
      };

      used_keys.flat_map( | key |
      {
        reverse_aliases.get( key ).into_iter().flatten().map( | k | *k ).chain( Some( key ) )
      })
      .collect::< Vec< _ > >()
    }

    /// Converts raw command to grammatically correct
    ///
    /// Make sure that this command is described in the grammar and matches it(command itself and all it options too).
    pub fn to_command( &self, dictionary : &Dictionary, raw_command : ParsedCommand ) -> Result< VerifiedCommand >
    {
      if raw_command.name.ends_with( '.' ) | raw_command.name.ends_with( ".?" )
      {
        return Ok( VerifiedCommand
        {
          phrase : raw_command.name,
          internal_command : true,
          args : Args( vec![] ),
          props : Props( HashMap::new() ),
        });
      }
      let command = dictionary.command( &raw_command.name )
      .ok_or_else::< error::for_app::Error, _ >
      (
        ||
        {
          #[ cfg( feature = "on_unknown_suggest" ) ]
          if let Some( phrase ) = Self::suggest_command( dictionary, &raw_command.name )
          { return err!( "Command not found. Maybe you mean `.{}`?", phrase ) }
          err!( "Command not found. Please use `.` command to see the list of available commands." )
        }
      )?;

      let Some( cmd ) = Self::check_command( command, &raw_command ) else
      {
        error::for_app::bail!
        (
          "`{}` command with specified subjects not found. Command info: `{}`",
          &raw_command.name,
          generate_help_content( dictionary, HelpGeneratorOptions::former().for_commands([ dictionary.command( &raw_command.name ).unwrap() ]).command_prefix( "." ).subject_detailing( LevelOfDetail::Detailed ).form() ).strip_suffix( "  " ).unwrap()
        );
      };

      let properties = Self::extract_properties( cmd, raw_command.properties.clone() )?;
      let used_properties_with_their_aliases = Self::group_properties_and_their_aliases( &cmd.properties_aliases, properties.keys() );
      let subjects = Self::extract_subjects( cmd, &raw_command, &used_properties_with_their_aliases )?;

      Ok( VerifiedCommand
      {
        phrase : cmd.phrase.to_owned(),
        internal_command : false,
        args : Args( subjects ),
        props : Props( properties ),
      })
    }
  }
}

//

crate::mod_interface!
{
  exposed use Verifier;
}

//! Demonstrates the two main capabilities of `cli_fmt`:
//! output processing (head/tail/width filtering) and
//! structured CLI help text rendering.

fn main()
{
  output_processing();
  help_template();
}

/// Output processing: filter and truncate CLI command output.
fn output_processing()
{
  use cli_fmt::output::*;

  // Simulate captured process output
  let stdout = "line 1: compiling crate with all features enabled\n\
                line 2: running integration tests across workspace\n\
                line 3: test result ok — 42 passed, 0 failed\n\
                line 4: finished release build with optimizations\n\
                line 5: artifacts written to target/release directory";
  let stderr = "warning: unused variable `cfg` in src/lib.rs:42";

  // Configure: keep first 3 lines, truncate at 30 visible columns
  let config = OutputConfig::default()
    .with_head( 3 )
    .with_width( 30 );

  let result = process_output( stdout, stderr, &config );

  println!( "=== Output Processing ===" );
  println!( "{}", result.content );
  println!( "Lines omitted: {}", result.lines_omitted );
  println!( "Width truncated: {}", result.width_truncated );
  println!();

  // Stream filtering: show only stderr
  let stderr_only = OutputConfig::default()
    .with_stream_filter( StreamFilter::Stderr );

  let err_result = process_output( stdout, stderr, &stderr_only );
  println!( "=== Stderr Only ===" );
  println!( "{}", err_result.content );
}

/// Help template: render structured CLI help text.
fn help_template()
{
  use cli_fmt::help::*;

  let data = CliHelpData
  {
    binary : "mytool".into(),
    tagline : "A sample CLI tool for demonstration purposes.".into(),
    groups : vec!
    [
      CommandGroup
      {
        name : "File operations".into(),
        entries : vec!
        [
          CommandEntry { name : "read".into(),   desc : "Read a file and print contents".into() },
          CommandEntry { name : "write".into(),  desc : "Write text to a file".into() },
          CommandEntry { name : "delete".into(), desc : "Remove a file from disk".into() },
        ],
      },
      CommandGroup
      {
        name : "Info".into(),
        entries : vec!
        [
          CommandEntry { name : "version".into(), desc : "Print version information".into() },
          CommandEntry { name : "status".into(),  desc : "Show current project status".into() },
        ],
      },
    ],
    options : vec!
    [
      OptionEntry { name : "--verbose".into(), desc : "Enable verbose output".into() },
      OptionEntry { name : "--format".into(),  desc : "Output format (text|json)".into() },
    ],
    examples : vec!
    [
      ExampleEntry { invocation : "mytool read config.toml".into(), desc : Some( "read a config file".into() ) },
      ExampleEntry { invocation : "mytool write --format json out.json".into(), desc : None },
    ],
  };

  // Force colors off so the example prints clean text in any terminal
  let style = CliHelpStyle
  {
    tty_detect : false,
    .. CliHelpStyle::default()
  };

  let tpl = CliHelpTemplate::new( style, data );

  println!( "=== Help Template ===" );
  print!( "{}", tpl.render() );
}

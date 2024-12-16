<!-- {{# generate.module_header{} #}} -->

# Module :: wca
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_wca_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_wca_push.yml) [![docs.rs](https://img.shields.io/docsrs/wca?color=e3e8f0&logo=docs.rs)](https://docs.rs/wca) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fmove%2Fwca%2Fexamples%2Fwca_trivial.rs,RUN_POSTFIX=--example%20wca_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

The tool to make CLI ( commands user interface ). It is able to aggregate external binary applications, as well as functions, which are written in your language.

## Sample

<!-- {{# generate.module{} #}} -->

```rust
#[ cfg( not( feature = "no_std" ) ) ]
{
    use wca::{ VerifiedCommand, Type };

    fn main()
    {

      let ca = wca::CommandsAggregator::former()
      .command( "echo" )
        .hint( "prints all subjects and properties" )
        .subject().hint( "Subject" ).kind( Type::String ).optional( true ).end()
        .property( "property" ).hint( "simple property" ).kind( Type::String ).optional( true ).end()
        .routine( | o : VerifiedCommand | { println!( "= Args\n{:?}\n\n= Properties\n{:?}\n", o.args, o.props ) } )
        .end()
      .command( "error" )
        .hint( "prints all subjects and properties" )
        .subject().hint( "Error message" ).kind( Type::String ).optional( true ).end()
        .routine( | o : VerifiedCommand | { println!( "Returns an error" ); Err( format!( "{}", o.args.get_owned::< String >( 0 ).unwrap_or_default() ) ) } )
        .end()
      .command( "exit" )
        .hint( "just exit" )
        .routine( || { println!( "exit" ); std::process::exit( 0 ) } )
        .end()
      .perform();

      let args: Vec< String > = std::env::args().skip( 1 ).collect();
      ca.perform( args ).unwrap();

    }
}
```

### To add to your project

```sh
cargo add wca
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/wca_trivial
cargo run
```

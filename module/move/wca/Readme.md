<!-- {{# generate.module_header{} #}} -->

# Module :: wca
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) |[![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleWcaPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleWcaPush.yml)[![docs.rs](https://img.shields.io/docsrs/wca?color=e3e8f0&logo=docs.rs)](https://docs.rs/wca)[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fwca_trivial%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20wca_trivial/https://github.com/Wandalen/wTools)
<!--{ generate.module_header.end }-->

The tool to make CLI ( commands user interface ). It is able to aggregate external binary applications, as well as functions, which are written in your language.

## Sample

<!-- {{# generate.module{} #}} -->

```rust
#[ cfg( not( feature = "no_std" ) ) ]
{
    use wca::{ Args, Context, Type };

    fn main()
    {

      let ca = wca::CommandsAggregator::former()
      .command( "echo" )
        .hint( "prints all subjects and properties" )
        .subject().hint( "Subject" ).kind( Type::String ).optional( true ).end()
        .property( "property" ).hint( "simple property" ).kind( Type::String ).optional( true ).end()
        .routine( | args : Args, props | { println!( "= Args\n{args:?}\n\n= Properties\n{props:?}\n" ) } )
        .end()
      .command( "inc" )
        .hint( "This command increments a state number each time it is called consecutively. (E.g. `.inc .inc`)" )
        .routine( | ctx : Context | { let i : &mut i32 = ctx.get_or_default(); println!( "i = {i}" ); *i += 1; } )
        .end()
      .command( "error" )
        .hint( "prints all subjects and properties" )
        .subject().hint( "Error message" ).kind( Type::String ).optional( true ).end()
        .routine( | args : Args | { println!( "Returns an error" ); Err( format!( "{}", args.get_owned::< String >( 0 ).unwrap_or_default() ) ) } )
        .end()
      .command( "exit" )
        .hint( "just exit" )
        .routine( || { println!( "exit" ); std::process::exit( 0 ) } )
        .end()
      .perform();

      let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
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


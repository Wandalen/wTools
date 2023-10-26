<!-- {{# generate.module_header{} #}} -->

# Module :: woptions
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModulewOptionsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModulewOptionsPush.yml) [![docs.rs](https://img.shields.io/docsrs/woptions?color=e3e8f0&logo=docs.rs)](https://docs.rs/woptions) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fwoptions_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20woptions_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Mechanism to define map of options for a function and its defaults laconically.

### Basic use-case

<!-- {{# generate.module_sample{} #}} -->

```rust
mod splitter
{
  use former::Former;

  #[ derive( PartialOrd ) ]
  #[ derive( Former, PartialEq, Debug ) ]
  #[ perform( fn perform( self ) -> Box< ( dyn std::iter::Iterator< Item = &'a str > + 'a ) > ) ]
  pub struct Options< 'a >
  {
    pub src : &'a str,
    pub delimeter : &'a str,
    #[ default( true ) ]
    pub left : bool,
  }

  pub trait OptionsAdapter< 'a >
  {
    fn src( &self ) -> &'a str;
    fn delimeter( &self ) -> &'a str;
    fn left( &self ) -> &bool;
    #[ inline ]
    fn perform( self ) -> Box< ( dyn std::iter::Iterator< Item = &'a str > + 'a ) >
    where
      Self : Sized,
    {
      if *self.left()
      {
        Box::new( self.src().split( self.delimeter() ) )
      }
      else
      {
        Box::new( self.src().rsplit( self.delimeter() ) )
      }
    }
  }

  impl< 'a > OptionsAdapter< 'a > for Options< 'a >
  {
    #[ inline ]
    fn src( &self ) -> &'a str
    {
      &self.src
    }
    #[ inline ]
    fn delimeter( &self ) -> &'a str
    {
      &self.delimeter
    }
    #[ inline ]
    fn left( &self ) -> &bool
    {
      &self.left
    }
  }

  #[ inline ]
  pub fn former< 'a >() -> OptionsFormer< 'a >
  {
    Options::< 'a >::former()
  }

}

#[ inline ]
fn splitter< 'a >() -> splitter::OptionsFormer< 'a >
{
  splitter::former::< 'a >()
}

//

fn main()
{
  /* form options */
  let from_former = splitter().src( "abc" ).delimeter( "b" ).form();
  let from_options = splitter::Options
  {
    src : "abc",
    delimeter : "b",
    left : true,
  };
  assert_eq!( from_former, from_options );
}
```
<!-- xxx --> <!-- aaa : done -->

### To add to your project

```sh
cargo add woptions
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/woptions_trivial
cargo run
```

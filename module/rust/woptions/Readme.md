# Module :: woptions [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml) [![stable](https://img.shields.io/badge/stability-stable-brightgreen.svg)](https://github.com/emersion/stability-badges#stable)

Mechanism to define map of options for a function and its defaults laconically.

### Sample

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
cd sample/rust/woptions_trivial
cargo run
```

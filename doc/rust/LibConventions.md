# Conventions of writing libraries

How to write library.

### Content

- [Module structure](#module-structure)
- [Convention for writing main library files](#convention-for-writing-main-library-files)
- [Writing library code with macro `mod_interface`](#writing-library-code-with-macro-mod_interface)
- [Conventions for writing common tests for several modules](#conventions-for-writing-common-tests-for-several-modules)
- [Conventions for writing samples](#conventions-for-writing-samples)

### Module structure

File tree of library should have :

- file `Cargo.toml`
- file `Readme.md`
- common license file `License`
- library files in directory `./rust/impl/[module_name]`. If module name starts with `w` it can be skipped
  - for module `for_each` it will be `./rust/impl/for_each`
  - for module `wtools` it will be `./rust/impl/tools`
- main library file should be named `[module_name]_lib.rs`
  - for module `for_each` it will be `for_each_lib.rs`
  - for module `wtools` it will be `wtools_lib.rs`
- test files in directory `./rust/test/[module_name]`. If module name starts with `w` it can be skipped
  - for module `for_each` it will be `./rust/test/for_each`
  - for module `wtools` it will be `./rust/test/tools`
- main test file should be named `[module_name]_test.rs`
  - for module `for_each` it will be `for_each_test.rs`
  - for module `wtools` it will be `wtools_test.rs`
- sample in directory `./sample/rust/[sample_name]`. Convention for naming of samples see below

File tree for module `tools` should looks like :

```
tools
  ├── Cargo.toml
  ├── License
  ├── Readme.md
  ├── rust
  │     ├── impl
  │     │     └── tools
  │     │           ├── tools_lib.rs
  │     │          ...
  │     │
  │     └── test
  │           └── tools
  │                 ├── tools_test.rs
  │                ...
  └── sample
        └── rust
              └── tools_trivial_sample
                           ├── Cargo.toml
                          ...
```

### Convention for writing main library files

Main library file should be named `[module_name]_lib.rs`.

Main library file consists of next parts :

- common library headers ( __required__ )
- crate description with included library file `Readme.md` ( __required__ )
- module `dependencies` ( __optional__ )
- submodules of library ( __optional__ )
- module `private` ( __optional__ )
- module `protected` ( __optional__ )
- module `orphan` ( __optional__ )
- module `exposed` ( __required__ )
- module `prelude` ( __required__ )

Full template of main library file ( in comments `/* ... */` described content of the parts of the main library file, it should be removed from the real library file ) :

```rust
/* common library headers */
#![ warn( rust_2018_idioms ) ] /* warn if public library interfaces has no useful rust idioms like lifetime annotations */
#![ warn( missing_debug_implementations ) ] /* warn if public library structure has no debug implementation */
#![ warn( missing_docs ) ] /* warn if public library interfaces has no documentation */

/* crate  description */
/* it generates first line of library documentation after name */
//!
//! Library description.
//!

/* documentation from library file `Readme.md` */
/* it includes documentation from file `Readme.md` at the root of the library, if file has examples, then its will be tested with rustdoc */
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/* dependencies module of library */
/* optional module that reexports library dependencies */
/// Dependencies.
pub mod dependencies
{
  /* reexported crate dependencies */
  /* example :
  pub use ::[dependency_name]::*;
  */
}

/* module of library */
/* optional module, describes module namespace */
/// Description of library namespace.
pub mod some_module_name;

/* private module of library */
/* optional module that consists of interfaces which intended for only library use */
/// Private namespace of the module.
pub mod private
{
  /* private library exports */
}

/* protected module of library */
/* optional module that consists of interfaces which intended for library use and can be reexported in external scope */
/// Own namespace of the module.
pub mod protected
{
  /* protected library exports */
}

/* example of public export */
pub use protected::*;

/* orphan module of library */
/* optional module that consists of interfaces which intended for library use and can be reexported in external scope */
/// Orphan namespace of the module.
pub mod orphan
{
  /* orphan library exports */
}

/* exposed module of library */
/* module that consists of common public interfaces */
/// Exposed namespace of the module.
pub mod exposed
{
  /* exposed library exports */
}

/* prelude module of library */
/* module that consists of public interfaces which intended for only public use */
/// Prelude to use: `use [library_name]::prelude::*`.
pub mod prelude
{
  /* prelude library exports */
}
```

### Writing library code with macro `mod_interface`

To automatically generate modules and its content the macro `mod_interface` can be used.

Macro generates modules and its submodules with code written in module files.

Example of generating module `exposed` with submodule `inner` and routine `inner_is`.

Part of file tree of library `tools` :

```
├── impl
│     └── tools
│           ├── tools_lib.rs
│           └── inner.rs
```

Content of file `tools_lib.rs` :

```rust
use meta_tools::mod_interface;

mod_interface!
{
  exposed mod inner;
}
```

Content of file `inner.rs` :

```rust
pub fn inner_is() -> bool
{
  true
}
```

Simplest test file that tests generated library modules :

```rust
use tools::exposed::*;

#[ test ]
fn inner_is()
{
  assert_eq!( inner::inner_is(), true );
}
```

### Conventions for writing common tests for several modules

Some tests can be used by several modules. The reason of it is reexports or making of alias of module.

Such tests should :

- contain main test file for each module
- main test file of module should be named `[module_name]_test.rs`
- contain common module`inc` reused from main module test files
- import module using alias `TheModule`

Part of file tree of files for library `wtools` and its alias `tools` :

```
├── test
│     └── tools
│           ├── tools_test.rs
│           ├── wtools_test.rs
│           ├── inc.rs
│           └── inc
│                ├── common1.rs
│                └── common2.rs
```

Content of file `tools_test.rs` :

```rust
use tools as TheModule;
mod inc;
```

Content of file `wtools_test.rs` :

```rust
use wtools as TheModule;
mod inc;
```

Content of file `inc.rs` :

```rust
use super::TheModule as TheModule;
mod common1;
mod common2;
```

Inside modules `common*` import library using code

```rust
use super::TheModule as TheModule;
```

### Conventions for writing samples

Each sample should be a binary module that use parent library as main dependency.

Common structure of the module is default file tree generated by utility `cargo`. You able to change default structure as you need.

A file tree for sample `tools_trivial_sample`

```
tools_trivial_sample
  ├── Cargo.toml
  └── src
       └── main.rs
```

The sample should be named used pattern :

```
[name_of_lib][content/description]_sample
```

The sample `tools_trivial_sample` says that it is sample of library `tools`, it simplest ( trivial ) sample.

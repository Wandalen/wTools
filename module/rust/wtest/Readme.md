# module::wtest

Tools for writing tests and runnint tests.

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/test_trivial
cargo run
```

### To add to your project

```
cargo add wtest --dev
```

### Sample

``` rust

use wtest_basic::*;

//

fn _pass1()
{
  assert_eq!( true, true );
}

//

fn _pass2()
{
  assert_eq!( 1, 1 );
}

//

test_suite!
{
  pass1,
  pass2,
}

```

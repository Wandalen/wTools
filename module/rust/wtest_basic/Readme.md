# module::wtest_basic

Tools for writing tests. The most basic things.

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/test_basic_trivial
cargo run
```

### To add to your project

```
cargo add wtest_basic --dev
```

### Sample

``` rust

use wtest_basic::test_suite;

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

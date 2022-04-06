# module::wtest_basic

Tools for writing and running tests. The most basic things.

### Sample

```rust
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

### To add to your project

```sh
cargo add wtest_basic --dev
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/test_basic_trivial
cargo run
```

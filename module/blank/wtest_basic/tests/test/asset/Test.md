
### Sample

```rust
use wtest_basic::*;

//

tests_impls!
{

  fn pass1()
  {
    assert_eq!( true, true );
  }

  //

  fn pass2()
  {
    assert_eq!( 1, 1 );
  }

}

//

tests_index!
{
  pass1,
  pass2,
}

```

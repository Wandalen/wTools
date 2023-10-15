
use diagnostics_tools::prelude::*;

//

#[ test ]
#[ should_panic ]
fn a_id_panic_test()
{
  a_id!( 1, 2 );
  /*
    print :
    ...

thread 'a_id_panic_test' panicked at 'assertion failed: `(left == right)`

Diff < left / right > :
<1
>2
...
  */
}

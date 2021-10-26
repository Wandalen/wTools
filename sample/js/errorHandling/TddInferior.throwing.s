
let _ = require( '../..' );

function routine( bool )
{
  if( bool )
  {
    console.log( 'Do one thing' );
    _.assert( 0, 'not tested' );
    /* throws error "not tested" */
  }
  else
  {
    console.log( 'Do another thing' );
    _.assert( 0, 'not tested' );
    /* throws error "not tested" */
  }
}

routine( true );

/*
In the example above routine has both branches implemented, but developer marked them as "not tested".
The first time entering the branch developer will become aware of that and will pay attention to test
the branch of the routine manually.
It allows postponing the moment of manual testing.
This practice should be used with caution as test-driven development practice is superior in most cases.
*/

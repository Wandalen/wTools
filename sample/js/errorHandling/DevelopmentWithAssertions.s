
let _ = require( '../..' );

function routine( bool )
{
  if( bool )
  {
    console.log( 'Do something' );
  }
  else
  {
    _.assert( 0, 'not implemented' );
    /* throws error "not implemented" */
  }
}

routine( true );

/*
Second branch of the example above has implemented the first branch, but the second has unconditional
assertion throwing error "not implemented".
It will allow to postpone design of the branch and to be sure no silent bug introduced.
*/

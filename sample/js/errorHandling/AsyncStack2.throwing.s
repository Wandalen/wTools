
let _ = require( 'wTools' );
_.include( 'wFiles' );
_.include( 'wConsequence' );

program();

function program()
{
  let consequence = new _.Consequence();
  consequence.then( function asyncCallback( arg )
  {
    if( arg >= 0 )
    return arg-1;
    else
    throw _.error.err( 'Example' );
  } );
  consequence.take( -1 );
}

/*
  In case of consequences native async stack is wrong.
  Synthetic async stack is has proper information for each callback of consequence.
*/

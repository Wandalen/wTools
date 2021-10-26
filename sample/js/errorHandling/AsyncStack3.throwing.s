
let _ = require( '../..' );
_.include( 'wFiles' );
_.include( 'wConsequence' );

program();

function program()
{

  var con = _.Consequence();

  con.then( function callback1( arg )
  {
    console.log( 'sourcePath::callback1 ' + _.procedure.activeProcedure._sourcePath );
    return 'callback1';
  } )

  con.then( function callback2( arg )
  {
    console.log( 'sourcePath::callback2 ' + _.procedure.activeProcedure._sourcePath );
    throw _.error.err( 'callback2' );
    return 'callback2';
  } )

  _.time.out( 100, function timeOut1()
  {
    console.log( 'sourcePath::timeout ' + _.procedure.activeProcedure._sourcePath );
    con.take( 'timeout1' );
  } );

  console.log( 'sourcePath::program ' + _.procedure.activeProcedure._sourcePath );
}

/*
  In case of consequences native async stack is wrong.
  Synthetic async stack is has proper information for each callback of consequence.
*/

function arrayFromArguments()
{
  console.log( arguments );
  /* log : [Arguments] { '0': 'hello', '1': 'world' } */
  console.log( arguments instanceof Array );
  /* log : false */

  let argumentsArr = Array.from( arguments );
  console.log( argumentsArr );
  /* log : [ 'hello', 'world' ] */
  console.log( argumentsArr instanceof Array );
  /* log : true */
}
arrayFromArguments( 'hello', 'world' );


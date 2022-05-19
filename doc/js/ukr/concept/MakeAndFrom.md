# Конвенція <code>make</code> і <code>from</code>
Конвенція іменування рутин <code>make</code> і <code>from</code>` забезпечує уніфікацію коду та впроваджує єдині правила для створення рутин-конструкторів.

### Список рутин, що використовують дану конвенцію
* `make`
  * `_.arrayMake`
  * `_.arrayMakeUndefined`
  * `_.argumentsArray.make`
  * `_.unrollMake`
  * `_.bufferMake`
  * `_.longDescriptor.make`
  * `_.longMake`
  * `_.longMakeEmpty`
  * `_.longMakeZeroed`
  * `_.longMakeFilling`
  * `_.longMakeUndefined`
  * `_._longMakeOfLength`
  * `_.bufferMakeUndefined`
  * `_.containerAdapter.make`
  * `_.entity.make`
  * `_.entity.makeEmpty`
  * `_.entity.makeUndefined`
  * `_.escape.make`

* `from`
  * `_.locationFromStackFrame`
  * `_.set.from`
  * `_.set.s.from`
  * `_.arrayFrom`
  * `_.arrayFromCoercing`
  * `_.arrayFromStr`
  * `_.event.nameValueFrom`
  * `_.scalarFrom`
  * `_.scalarFromOrNull`
  * `_.cinterval.fromLeft`
  * `_.cinterval.fromRight`
  * `_.cinterval.fromSingle`
  * `_.bigInt.from`
  * `_.bigInt.s.from`
  * `_.property.mapperFromFilter`
  * `_.property.mapperFrom`
  * `_.property.filterFrom`
  * `_.strFrom`
  * `_.time.from`
  * `_.unrollFrom`
  * `_.unrollsFrom`
  * `_.unrollFromMaybe`
  * `_.containerAdapter.from`
  * `_.entity.assign2FieldFromContainer`
  * `_.bool.from`
  * `_.bool.fromMaybe`
  * `_.bool.fromForce`
  * `_.bufferFromArrayOfArray`
  * `_.bufferFrom`
  * `_.bufferRawFromTyped`
  * `_.bufferRawFrom`
  * `_.bufferBytesFrom`
  * `_.bufferBytesFromNode`
  * `_.bufferNodeFrom`
  * `_.longFrom`
  * `_.longFromCoercing`
  * `_.longFromRange`
  * `_.longFromProgressionArithmetic`
  * `_.longFromRangeWithStep`
  * `_.longFromRangeWithNumberOfSteps`
  * `_.number.from`
  * `_.number.s.from`
  * `_.number.fromStr`
  * `_.number.fromStrMaybe`
  * `_.number.s.fromNumber`
  * `_.number.s.fromInt`
  * `_.arrayFrom`
  * `_.arrayFromLongDescriptor`
  * `_.arrayFromCoercing`
  * `_.arrayFromCoercingLongDescriptor`

## Використання існуючих рутин конструкторів

Приклад - використання рутини <code>_.arrayMake</code>
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>var srcArray = [ 'a', 'b', 'c' ];

var gotArray = _.arrayMake( srcArray );
console.log( gotArray ); /* log : [ 'a', 'b', 'c' ] */
console.log( gotArray === srcArray ); /* log : false */</code></pre>
</details>

Приклад - використання рутини <code>_.arrayFrom</code>.
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>var srcArray = [ 'a', 'b', 'c' ];

var gotArray = _.arrayFrom( srcArray );
console.log( gotArray ); /* log : [ 'a', 'b', 'c' ] */
console.log( gotArray === srcArray ); /* log : true */</code></pre>
</details>

Приклад - використання рутини <code>_.escape.make</code>.
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>
let src = _.escape.make( 1 );
let got = _.escape.make( src );

console.log( got === src ); /* log : false */</code></pre>
</details>

Приклад - використання рутини <code>_.escape.from</code>.
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>
let src = _.escape.make( 1 );
let got = _.escape.from( src );

console.log( got === src ); /* log : true */</code></pre>
</details>

## Створення кастомних рутин конструкторів

Приклад - реалізація рутини <code>arrayMake</code>
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>function arrayMake( src )
{
  if( src === null || src === undefined )
  return new Array();

  if( _.number.is( src ) )
  return new Array( src );

  if( src.length === 1 )
  return [ src[ 0 ] ];
  else
  return Array.apply( Array, src );
}</code></pre>
</details>

Приклад - реалізація рутини <code>arrayFrom</code>
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>let arrayMake = require( './4_implementArrayMake.s' );

function arrayFrom( src )
{
  if( _.arrayIs( src ) )
  return src;
  return arrayMake( src );
}</code></pre>
</details>


## Використання кастомних рутин конструкторів

Приклад - використання рутини <code>arrayMake</code>
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>let arrayMake = require( './4_implementArrayMake.s' );

let src = [ 1, 2, 3 ];
let got = arrayMake( src );

console.log( got ); /* log : [ 1, 2, 3 ] */
console.log( got === src ); /* log : false */
</code></pre>
</details>

Приклад - використання рутини <code>arrayFrom</code>
<details>
  <summary>
    <b>Код</b>
  </summary>

<pre><code>let arrayFrom = require( './5_implementArrayFrom.s' );

let src = [ 1, 2, 3 ];
let got = arrayFrom( src );

console.log( got ); /* log : [ 1, 2, 3 ] */
console.log( got === src ); /* log : true */
</code></pre>
</details>

[Повернутись до змісту](../README.md#Концепції)
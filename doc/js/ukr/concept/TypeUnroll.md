# Тип <code>unroll</code>

Тип даних <code>unroll</code> - особливий вид масиву, здатний розгортатись в іншому масиві при виконанні операції над останнім.

При виконанні над `unroll-масивом` операцій, призначених для звичайного масиву( `arrayPrepend`, `arrayAppend`, `arrayRemove`, `arrayReplace`, `arrayFlatten` та будь-яких із Array.prototype ), він веде себе як звичайний масив і не змінює свого типу.

```js
// creating unroll array
var unroll = _.unroll.make([ 2, 3, 4 ]);

var result = _.arrayAppend( unroll, 5 );
/* returns : [ 2, 3, 4, 5 ] */
console.log( _.unroll.is( result ) );
/* log : true */

unroll.push( 'str' );
console.log( _.unroll.is( unroll ) );
/* log : true */
```

Для створення  `unroll-масива` використовується рутина `make` неймспесу `unroll`. Після виконання рутини `arrayAppend` i метода `push`, що працюють зі звичайними масивами, `unroll-масив` `unroll` зберіг свій тип.

Рутини призначені для роботи з `unroll-масивами` можна застосовувати до звичайних масивів, це не змінює їх тип.

```js
var arr = [ 0, 1, 2, 3 ];

var result = _.unrollPrepend( arr, 4 );
/* returns : [ 4, 0, 1, 2, 3 ] */
console.log( _.unroll.is( result ) );
/* log : false */
```

Рутина `unrollPrepend` додала елемент до масиву `arr`. Масив `arr` зберіг тип `Array`.

При використанні рутин, в назві яких є префікс `unroll`, вміст `unroll-масивів` буде розгорнено. В результаті розгортання `unroll-масивів`, кожен елемент `unroll-масиву` стане елементом масиву в якому знаходився `unroll-масив`, а сам `unroll-масив` перестане бути елементом свого контейнера.

```js
var unroll1 = _.unroll.make([ 7, [ 2 ] ]);
var unroll2 = _.unroll.make([ 0, 1, 'str' ]);

var result = _.unrollAppend( unroll1, unroll2 );
/* returns : [ 7, [ 2 ], 0, 1, 'str' ] */
console.log( _.unroll.is( result ) );
/* log : true */
```

Елементи двох `unroll-масивів` `unroll1` i `unroll2` стали елементами єдиного `unroll-масива` `result`. При цьому було розгорнуто `unroll-масив` `unroll2`, елементи якого було додано до `unroll1`.

```js
var unroll1 = _.unroll.make([ '5' ]);
var unroll2 = _.unroll.make([ 's', 3, [ 4 ] ]);

// creating an unroll-array from a given array
var result = _.unroll.from([ 1, 2, unroll1, unroll2 ]);
/* returns : [ 1, 2, '5', 's', 3, [ 4 ] ] */
console.log( _.unroll.is( result ) );
/* log : true */
```

При створенні нового `unroll-масива` вкладені `unroll-масиви` розгорнулись. Новий `unroll-масив` `result` містить елементи `unroll-масивів` `unroll1` i `unroll2`.

```js
var unroll1 = _.unroll.make([ '5' ]);
var unroll2 = _.unroll.make([ 'str', [ 3 ] ]);

// unrolling elements of the given array that are unroll-array
var result = _.unroll.normalize([ 0, 7, unroll1, [ unroll2, unroll1 ] ]);
/* returns : [ 0, 7, '5', [ 'str', [ 3 ],  '5' ] ] */
console.log( _.unroll.is( result ) );
/* log : false */
```

Рутина `unrollNormalize` дозволяє нормалізувати ( розгорнути ) вкладені `unroll-масиви` зберігаючи тип контейнера. Нормалізований масив `result` зберіг тип `Array`.

При виконанні операцій над багатовимірним `unroll-масивом`, який містить інші `unroll-масиви`, буде розгорнено тільки перший рівень кожного елементу цього масиву, якщо він є `unroll-масивом`.

```js
var unroll1 = _.unroll.make([ 1, 2 ]);
var unroll2 = _.unroll.make([ 3, 4 ]);
unroll1[ 2 ] = unroll2;

console.log( unroll1 );
/* log : [ 1, 2, [ 3, 4 ] ] */
console.log( unroll2 );
/* log : [ 3, 4 ] */

var result = _.unroll.normalize( [ unroll1, unroll2 ] );
console.log( result );
/* log :
[
  1, 2, [ 3, 4 ],
  3, 4
]
*/
```

Елементи `unroll-масивів` `unroll1` і `unroll2` перестали бути елмементами власного контейнера. Одночасно з цим вкладений `unroll-масив` `unroll2` залишився нерозгорнутим.

### Підсумок

- `Unroll-масиви`, які є елементами інших `unroll-масивів` та звичайних масивів - здатні розгортатись в них;
- При виконанні над вкладеним `unroll-масивом` операції для `unroll-масивів`, такий масив розгортається;
- При виконанні над `unroll-масивом` операції для звичайного масиву, він поводиться як звичайний масив;
- При виконанні операції для `unroll-масивів` над багатовимірним `unroll-масивом`, буде розгорнено тільки перший рівень кожного елементу цього масиву, якщо він є `unroll-масивом`.

[Повернутись до змісту](../README.md#Концепції)


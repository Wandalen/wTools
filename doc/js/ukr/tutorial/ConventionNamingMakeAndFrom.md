# Суфікси <code>Make</code> i <code>From</code> в найменуванні рутин 

Для позначення рутин-конструкторів використовуються рутини в назві яких є суфікси <code>Make</code> i <code>From</code>.

### Порівняння <code>Make</code> i <code>From</code> рутин

Рутини з суфіксами `Make` i `From` є конструкторами. Тип, що повертається вказується перед суфіксом. Наприклад, `arrayMake` i `arrayFrom` повертають масиви, а `unrollMake` i `unrollFrom` повертають контейнери [типу `unroll`]( '../concept/TypeUnroll.md' ).

```js
var array = _.arrayMake( 2 );
_.arrayIs( array ); // returns true

var unroll = _.unrollFrom( 2 );
_.unrollIs( unroll ); // returns true
```

Після суфіксів `Make` i `From` може йти уточнення вмісту створеного інстансу. Наприклад, для `Make` рутин закінчення `Undefined` говорить що елементи створеного інстансу матимуть значення `undefined`, а закінчення `Empty` свідчить про те, що створений інстанс буде порожній. Для `From` рутин закінчення говорять про тип якого буде створено бажаний - `Coercing` створює бажаний тип з іншого, а `Constructing` - створює подібний.

```js
_.arrayMakeUndefined( [ 1, 2, 3 ] ); // returns [ undefined, undefined, undefined ]

_.arrayFromCoercing( { a : 2, b : 'abc' } ); // returns [ [ 'a', 2 ], [ 'b', 'abc' ] ]
```

Відмінність між `Make` i `From` рутинами полягає в тому, що `Make` рутини завжди створюють новий інстанс незалежно від того, аргумент якого типу передано. `From` рутини зберігають існуючий інстанс, якщо бажаний тип і тип переданого аргументу співпадають.

```js
var srcArray = [ 1, 2, 3 ];

var gotArray1 = _.arrayMake( srcArray );
console.log( gotArray1 ); // log [ 1, 2, 3 ]
console.log( gotArray1 === srcArray ); // log false

var gotArray2 = _.arrayFrom( srcArray );
console.log( gotArray2 ); // log [ 1, 2, 3 ]
console.log( gotArray2 === srcArray ); // log true

/* */

var srcU8x = new U8x( [ 1, 2, 3 ] );

var gotArray3 = _.arrayMake( srcU8x );
console.log( gotArray3 ); // log [ 1, 2, 3 ]
console.log( gotArray3 === srcU8x ); // log false

var gotArray4 = _.arrayFrom( srcU8x );
console.log( gotArray4 ); // log [ 1, 2, 3 ]
console.log( gotArray4 === srcU8x ); // log false
```

Вказані особливості рутин з суфіксами `Make` i `From` відносяться також до рутин неймспесів. Так, виклик рутини `make` неймспейсу `containerAdapter` створить новий інстанс `containerAdapter`, a рутина `from` збереже існуючий інстанс.

```js
var container = _.containerAdapter.make( [ 1, 2, 3 ] );

var gotContainer1 = _.containerAdapter.make( container );
console.log( gotContainer1 === container ); // log false 

var gotContainer2 = _.containerAdapter.from( container );
console.log( gotContainer2 === container ); // log true
```

### Підсумок

- Рутини з суфіксами `Make` i `From`, а також рутини `make` i `from` неймспесів, це рутини-конструктори. 
- Дані рутини повертають інстанси тип яких позначається на початку найменування ( або неймспесом ).
- `Make` рутини завжди створюють новий інстанс незалжно від типу переданого аргументу.
- `From` рутини зберігають оригінальний інстанс, якщо бажаний тип і тип переданого аргументу зберігаються.

[Повернутись до змісту](../README.md#Концепції)


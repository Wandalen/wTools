## Тип <code>Long</code>

Комплексні типи даних, що містять множину елементів котрі ідентифікуються за індексом.

### Перелік масивоподібних типів

`Array` - тип даних, структура якого складається з послідовності елементів, котрі ідентифікуються за індексом ( індексами ).

```js
let array1 = [ 1, 2, 'str' ];
let array2 = new Array([ 1, 2, 'str' ]);
console.log( _.arrayIs( array1 ) );
/* log : true */
console.log( _.arrayIs( array2 ) );
/* log : true */
```

`Unroll` - особливий масивоподібний тип даних, що здатний розгортатись в іншому масиві, анролі чи елементі типу `ArgumentsArray`.

```js
let unroll = _.unrollMake([ 1, 2, 'str' ]);
console.log( _.unrollIs( unroll ) );
/* log : true */
```

`ArgumentsArray` - незалежний масивоподібний тип даних, що позначає вхідні дані рутини, як і тип `Unroll` може розгортатись в інших масивах та анролах.

```js
let argumentsArray = _.argumentsArray.make([ 1, 2, 'str' ]);
console.log( _.argumentsArray.is( argumentsArray ) );
/* log : true */
```

`Long` - збірний тип, що збирає в собі всі інші типи, властивостями котрих є  довжина і елементи. Включає: `Array`, `Unroll`, `ArgumentsArray`, `BufferTyped`.

```js
let array = [ 1, 2, 'str' ];
let unroll = _.unrollMake([ 1, 2, 'str' ]);
let argumentsArray = _.argumentsArray.from([ 1, 2, 'str' ]);
let bufferTyped = new U8x( 10 );
console.log( _.longIs( array ) );
/* log : true */
console.log( _.longIs( unroll ) );
/* log : true */
console.log( _.longIs( argumentsArray ) );
/* log : true */
console.log( _.longIs( bufferTyped ) );
/* log : true */
```

### Перелік буферів

`BufferRaw` - послідовність сирих двійкових даних для представлення іншими буферами. Двійкові дані в буферi `BufferRaw` зберігаються в байтах. Тип `BufferRaw` включає буфери: `ArrayBuffer`, `SharedArrayBuffer`.

```js
let arrayBuffer = new ArrayBuffer( 10 );
console.log( _.bufferRawIs( arrayBuffer ) );
/* log : true */

let sharedArrayBuffer = new SharedArrayBuffer( 10 ) );
console.log( _.bufferRawIs( sharedArrayBuffer ) );
/* log : true */
```

`BufferView` - буфер для представлення даних буферу `BufferRaw`, який не містить елементів. Може змінювати представлення динамічно - для того щоб по-різному інтерпретувати одні й ті ж дані достатньо використати інший метод доступу.

```js
let bufferView = new DataView( new ArrayBuffer( 20 ) );
console.log( _.bufferViewIs( bufferView ) );
/* log : true */
```

`BufferNode` - буфер інтерпретатора `NodeJS`. Подібно буферу `BufferView` не має визначених елементів та використовується для динамічного представлення двійкових даних. Одночасно з цим має властивості `BufferTyped` i `BufferRaw`.

```js
let bufferNode1 = Buffer.alloc( 20 );
console.log( _.bufferNodeIs( bufferNode1 ) );
/* log : true */

let bufferNode2 = Buffer.from( [ 1, 2 ] );
console.log( _.bufferNodeIs( bufferNode2 ) );
/* log : true */
```

`BufferTyped` - збірний тип даних, що збирає в собі сукупність типів для представлення послідовності двійкових даних у обраному числовому форматі - у вигляді беззнакових чисел, чисел зі знаком, та чисел з плаваючою комою. Включає наступні типи: `U32x`, `U16x`, `U8x`, `U8ClampedArrayx`, `Ux`, `I32x`, `I16x`, `I8x`, `Ix`, `F64x`, `F32x`, `Fx`.

`BufferBytes` - альтернативна назва для типізованого буферу `U8x`. Конструктора на має.

`BufferAny` - збірний тип, що збирає в собі всі типи `Buffer*` включно із типами, які не є підтипами `Long`. Включає: `BufferNode`, `BufferRaw`, `BufferView`, `BufferTyped`.

```js
let bufferRaw = new ArrayBuffer( 20 );
console.log( _.bufferAnyIs( bufferRaw ) );
/* log : true */

let bufferView = new DataView( bufferRaw );
console.log( _.bufferAnyIs( bufferView ) );
/* log : true */

let bufferNode = Buffer.alloc( 100 );
console.log( _.bufferAnyIs( bufferNode ) );
/* log : true */

let bufferTyped = new U8x( bufferRaw );
console.log( _.bufferAnyIs( bufferTyped ) );
/* log : true */
```

### Типізовані буфери

`U32x` - представлення буферу `BufferRaw` у вигляді послідовності 4-байтних беззнакових чисел.

`U16x` - представлення буферу `BufferRaw` у вигляді послідовності 2-байтних беззнакових чисел.

`U8x` - представлення буферу `BufferRaw` у вигляді послідовності 1-байтних беззнакових чисел.

`U8ClampedX` - представлення буферу `BufferRaw` у вигляді послідовності 1-байтних беззнакових чисел. Значення передані в буфер обмежені значеннями від 0 до 255.

`Ux` - альтернативний тип даних для `U32x`.

`I32x` - представлення буферу `BufferRaw` у вигляді послідовності 4-байтних чисел зі знаком.

`I16x` - представлення буферу `BufferRaw` у вигляді послідовності 2-байтних чисел зі знаком.

`I8x` - представлення буферу `BufferRaw` у вигляді послідовності 1-байтних чисел зі знаком.

`Ix` - альтернативний тип даних для `І32x`.

`F64x` - представлення буферу `BufferRaw` у вигляді послідовності 8-байтних чисел з плаваючою комою.

`F32x` - представлення буферу `BufferRaw` у вигляді послідовності 4-байтних чисел з плаваючою комою.

`Fx` - альтернативний тип даних для `F32x`.

```js
let u32x = new U32x([ 1, 2, 100 ]);
console.log( _.bufferTypedIs( u32x ) );
/* log : true */

let f32x = new Fx([ 1, 2, 34 ]);
console.log( _.bufferTypedIs( Fx ) );
/* log : true */
```

При застосуванні указаних конструкторів створюються інстанси типу `BufferTyped`.

<!-- aaa : переписати з нуля | Dmytro : переписано -->

## Дескриптор типу <code>Long</code>

Дескриптор, що визначає тип контейнеру при створенні лонгу, якщо тип не вказано явно.

Якщо рутина при створенні нового лонгу не знає якого типу повинен бути вихідний контейнер, то створений лонг матиме тип дескриптора `longDescriptor`.
Рутини не знають про тип призначення якщо :

- замість інстанса лонга ( або конструктора ) передано `null`;
- інстанс має тип `ArgumentsArray`.

### Створення `Long`-а за замовчуванням

```js
var long = _.longMake( null );
/* returns : [] */
_.arrayIs( long );
/* returns : true */
```

Створений лонг контейнер `long` має тип `Array`. Це дефолтний тип лонгів.

```js
var argumentsArray = _.argumentsArray.make([ 1, 2, 3 ]);
var newLong = _.longMake( argumentsArray, 2 );
/* returns : [ 1, 2 ] */
_.argumentsArray.is( newLong );
/* returns : false */
_.arrayIs( newLong );
/* returns : true */
```

Рутина `longMake` перетворила лонг `argumentsArray` до типу `Array`.

### Створення `Long`-a з указаним типом

Для того, щоб змінити тип дескриптору на інший використовується спеціальне поле `withDefaultLong`.

```js
var newLong = _.withDefaultLong.Unroll.longMake( null );
/* returns : [] */
_.unrollIs( newLong );
/* returns : true */
```

Новий лонг має тип `Unroll`. Це було вказано в дескрипторі.

```js
var _ = _.withDefaultLong.U8x;
var argumentsArray = _.argumentsArray.make([ 1, 2, 3 ]);
_.argumentsArray.is( argumentsArray );
/* returns : true */
var newLong = _.longMake( argumentsArray, 2 );
/* returns : Uint8Array [ 1, 2 ] */
_.BufferTypedIs( newLong );
/* returns : true */
```

Перший рядок позначає шорткат, що включає дескриптор `U8x`.

Як видно з приведеного прикладу контейнер `argumentsArray` має тип `ArgumentsArray`. Таким чином, указання дескриптора не впливає на поведінку рутин, що створюють інстанси визначеного типу.

Після створення нового лонга `newLong` його тип - `U8x` позначений в шорткаті.

### Перелік дескрипторів

Доступні дескриптори зберігаються в мапі `_.LongDescriptors`. До них належать:

- `Array` - дескриптор за замовчуванням, створює масиви.
- `Unroll` - створює `unroll`-масиви.
- `ArgumentsArray` - створює контейнер типу `ArgumentsArray`.
- `U8x` - створює типізований буфер `Uint8Array`.
- `U16x` - створює типізований буфер `Uint16Array`.
- `U32x` - створює типізований буфер `Uint32Array`.
- `Ux` - створює типізований буфер `Uint32Array`.
- `I8x` - створює типізований буфер `Int8Array`.
- `I16x` - створює типізований буфер `Int16Array`.
- `I32x` - створює типізований буфер `Int32Array`.
- `Ix` - створює типізований буфер `Int32Array`.
- `F32x` - створює типізований буфер `Float32Array`.
- `F64x` - створює типізований буфер `Float64Array`.
- `Fx` - створює типізований буфер `Float32Array`.

### Підсумок

- Рутини створюють новий лонг з типом визначеним в `longDescriptor`, якщо рутина не може визначити тип вихідного контейнера.
- Рутини створюють новий лонг з типом визначеним в `longDescriptor`, якщо рутина приймає аргументом контейнер типу `ArgumentsArray`.
- Повний виклик з позначенням дескриптору виглядає: `wTools.withDefaultLong.[ name of descriptor ].[ routine ]( arguments )`.

[Повернутись до змісту](../README.md#Концепції)


#  Мануал початківця wTools

## Зміст
- [Задачі та цілі модульного тестування](#unit-testing)
- [Репозиторій](#repository)
- [Клонування репозиторію](#repository-cloning)
- [Встановлення пакету](#package-installation)
- [Способи запуска тестів](#way-of-test-running)
- [Кількість вихідної інформації](#verbosity)
- [Підказка по параметрам скрипта](#getting-parameters-help)
- [Сценарії тестера](#tester-scenarios)
- [Опції тестера](#tester-options)
- [Структурні одиниці тестування](#testing-structure-units)
- [Розташуванння окремих тест рутин та їх реалізацій](#test-routines-location)
- [Анатомія модульного теста](#unit-testing-anatomy)

<a name="unit-testing"/>

## Задачі та цілі модульного тестування

Модульні тести (юніт тести), пишуться для того, щоб протестувати чи працює данний модуль(юніт). Сам юніт тест - це код - який тестує юніти (частини) кода: функції, модулі або класси. Юніт тести - найбільш прості для написання та найбільш прості для розуміння. Суть юніт тестування полягає в тому, щоб подати щось на вход юніта і перевірити результат на виході. (Наприклад на вход подаєм параметри функції, а на виході отримаемо значення). Юніт тести дають впевненність, що программа працює як задумано. Такі тести можно запускати багатократно. Успішне виконання тестів покаже розробнику, що його зміни нічого не зламали. Тест, що провалився дозволе виявити, що в коді зроблені зміни, які змінюють або ломають його поведінку. Дослідження помилки, яку видає тест, що провалився та порівняння очікуваного результата з отриманим, дасть можливисть зрозуміти, де виникла помилка.
Усі залежності виключаються, а це значить, що ми надаємо підроблені залежності (фейки) для модуля. Крім того необхідно писати код таким чином, щоб це дозволяло тестувати юніти окремо.

<a name="repository"/>

## Репозиторій

Репозіторій розташовано за адресою  [https://github.com/Wandalen/wTools](https://github.com/Wandalen/wTools)

<a name="repository-cloning"/>

## Клонування репозиторію

Для клонування репозіторію використовувати команду `git clone https://github.com/Wandalen/wTools.git`

<a name="package-installation"/>

## Встановлення пакету

Для встановлення пакету перейдіть в сколоновану теку проекту та виконайте
`npm install`
Для встановлення пакету для тестування глобально використайте
`npm install -g wTesting`

<a name="way-of-test-running"/>

## Способи запуска тестів

[//]: # ( xxx : не правильно, треба розбити на 4-ри пункти )

 Існують три способа запуска тестів з наступними командами

|N п/п      | Спосіб запуску                                              | Команда                                              |
|:----------|:------------------------------------------------------------|:-----------------------------------------------------|
|[1](#1)    | Безпосередньо через npm                                     | `npm test`                                           |
|[2.1](#2.1)| Альтернативний спосіб запуску                               | `wtest .run proto/**`                                      |
|[2.2](#2.2)| Альтернативний спосіб запуску використовуючи файл тест сюіта| `wtest .run proto/**/wtools/abase/layer1.test/Map.test.s` |
|[3](#3)    | Використовуючи файл тест сюіта                              | `node proto/wtools/abase/layer1.test/Map.test.s`  | 

Экран терміналу вищовказаних команд в операційній системі Linux буде виглядати наступним образом:

<a name="1"/>

Запуск теста безпосередньо через npm. Команда `npm test`.

![Результат команди 1](./img/1.png)

Як можно побачити на скріншоті - було зроблено 13 помилок. Проскролив вікно консолі можно побачити - директорію з якої були виконані тест сюіти, та ті з них які не було включено до тестування, а також кількість пройдених тестів та помилок для кожного тест сюіта - окремо.

![Окремий тест](./img/5.png)

<a name="2.1"/>

Результат команди `wtest .run proto/**`

![Результат команди 2.1](./img/2.1.png) Це - альтернативний спосіб запуску тестування. Команда для запуска `wtest .run proto/**`. Спосіб [1](#1) та [2.1](#2.1) виконує усі наявні тест сюіти

<a name="2.2"/>

Результат команди 2.2 див.визначення [Тест сюіт](#test-suit). В же час можно виконувати окремі тест сюіти за допомогою [2.2](#2.2) та [3](#3)

![Результат команди 2.2](./img/2.2.png)  

<a name="3"/>

Результат команди `node proto/wtools/abase/layer1.test/Map.test.s`, яка дозволяє виконати тестування, використовуючи файл тест сюіта див.визначення [Тест сюіт](#test-suit)

![Результат команди 3](./img/3.png)

На цьому скріншоті зображені вихідні параметри тест сюіта включаючи ім'я та статистику.

![node staging](./img/6.png)

<a name="verbosity"/>

## Кількість вихідної інформації

Кількість інформації, яка буде видаватися скриптом, можно регулювати наступним чином:
використовуючи параметр `verbosity`, який регулює багатослівність.

| Параметр      | Кількість отриманої інформації |
|:--------------|:-------------------------------|
| verbosity:0   | Жодної інформації              |
| verbosity:1   | Один рядок                     |
| verbosity:2   | Багато рядків                  |
| verbosity:9   | Максимум інформації            |

Наприклад: `wtest .run proto/** verbosity:2`

<a name="getting-parameters-help"/>

## Підказка по параметрам скрипта

Для отримання підказки по опціям пакету для тестування використовуйте
`wtest scenario:help`

<a name="tester-scenarios"/>

## Сценарії тестера

| Сценарії      | Виконувана дія                            |
|:--------------|:------------------------------------------|
| test          | Запустити тести, сценарій за замовчуванням|
| help          | Отримати допомогу                         |
| options.list  | Перелік наявних опцій                     |
| scenarios.list| Перелік наявних сценаріїв                 |
| suits.list    | Перелік наявних сютів                     |

<a name="tester-options"/>

## Опції тестера

В наявності наступні опції:

| Опція             | Виконувана дія                                                        |
|:------------------|:----------------------------------------------------------------------|
| scenario          | Ім'я сценарію для запуску                                             |
| sanitareTime      | Затримка перед запуском наступного тест сюіта                         |
| usingBeep         | дзвінок після закінчення тесту                                        |
| routine           | Iм'я рутини для виконання                                             |
| fails             | Максимальне число хибних тестів перед тим, як закінчити               |
| silencing         | Дозволяє ловити вихід консолі, який виникає під час тестового запуску |
| testRoutineTimeOut| Лімітує час,який кожна рутина має працювати                           |
| concurrent        | Виконання тест сюітів параллельно з іншими тест сюітами               |
| verbosity         | Кількість вихідної інформації                                         |

<a name="testing-structure-units"/>

## Структурні одиниці тестування

[//]: # ( xxx : трохи не точно )

<a name="test-suit"/>

> `Тест сюіт` (тест комлект,тестовий набір) - це набір тест рутин, та тестових данних, необхідних для максимально повного тестування окремих частин задачі.

> `Тест рутина`( функція, метод ) - це набір тест кейсів, що виконуються послідовно одна за одною та поєднанні тим, що відносяться до одного модуля, що тестується або функціональності. Рутина завершує своє виконання на першій викинутій помилці тож тест кейси, які йдуть пізніше можуть лишитися не виконаними, проте це не вплине на результат тестування - він буде `failed`.

> `Тест кейс` - це одна або декілька перевірок поєднаних із супровідним кодом для виявлення несправності лише одного аспекту об'єкту, що тестується. Тест кейси описуються в тест рутині кодом та мають текстовий опис, що пояснює обставини тестування та очікуваний результат в даному тест кейсі.

> `Тест перевірка` - це найменша структурна одиниця тестування. Тест кейс складається із одної або декількох перевірок. Достатньо лише одного перевірки із результатом `failed` щоб увесь об'єкт, що тестується вважався таким, що не пройшов його.

| Структурна одиниця      | Кількість в пакеті wTools|
|:------------------------|:-------------------------|
| Тест сюіт               | 14                       |
| Тест рутина             | 184                      |
| Тест кейс               | 1332                     |
| Тест перевірка          | 2296                     |

<a name="test-routines-location"/>

## Розташуванння окремих тест рутин та їх реалізацій

Окремі тест рутини розташовані в директорії `wTools/proto/wtools/abase`

Вони мають вигляд (на прикладі Тест рутини mapsAreIdentical):

```javascript
let Self =
{

  /* options of the test suit */
  name : 'MapTest',
  silencing : 1,

  /* map of test routines available in the test suit */
  tests :
  {

    mapsAreIdentical : mapsAreIdentical,

  }

}
```

Тест рутина `mapsAreIdentical` має таку реалізацію:

```javascript

function mapsAreIdentical( test )
{

  test.description = 'same values';
  var got = _.mapsAreIdentical( { a : 7, b : 13 }, { a : 7, b : 13 } );
  var expected = true;
  test.identical( got, expected );

  test.description = 'not the same values in'
  var got = _.mapsAreIdentical( { 'a' : 7, 'b' : 13 }, { 'a' : 7, 'b': 14 } );
  var expected = false;
  test.identical( got, expected );

  test.description = 'different number of keys, more in the first argument'
  var got = _.mapsAreIdentical( { 'a' : 7, 'b' : 13, 'с' : 15 }, { 'a' : 7, 'b' : 13 } );
  var expected = false;
  test.identical( got, expected );

  test.description = 'different number of keys, more in the second argument'
  var got = _.mapsAreIdentical( { 'a' : 7, 'b' : 13 }, { 'a' : 7, 'b' : 13, 'с' : 15 } );
  var expected = false;
  test.identical( got, expected );

  /* special cases */

  test.description = 'empty maps, standrard'
  var got = _.mapsAreIdentical( {}, {} );
  var expected = true;
  test.identical( got, expected );

  test.description = 'empty maps, pure'
  var got = _.mapsAreIdentical( Object.create( null ), Object.create( null ) );
  var expected = true;
  test.identical( got, expected );

  test.description = 'empty maps, one standard another pure'
  var got = _.mapsAreIdentical( {}, Object.create( null ) );
  var expected = true;
  test.identical( got, expected );

  /* bad arguments */

  if( !Config.debug )
  return;

  test.description = 'no arguments';
  test.shouldThrowError( function()
  {
    _.mapsAreIdentical();
  });

  test.description = 'not object-like arguments';
  test.shouldThrowError( function()
  {
    _.mapsAreIdentical( [ 'a', 7, 'b', 13 ], [ 'a', 7, 'b', 14 ] );
  });
  test.shouldThrowError( function()
  {
    _.mapsAreIdentical( 'a','b' );
  });
  test.shouldThrowError( function()
  {
    _.mapsAreIdentical( 1,3 );
  });
  test.shouldThrowError( function()
  {
    _.mapsAreIdentical( null,null );
  });
  test.shouldThrowError( function()
  {
    _.mapsAreIdentical( undefined,undefined );
  });

  test.description = 'too few arguments';
  test.shouldThrowError( function()
  {
    _.mapsAreIdentical( {} );
  });

  test.description = 'too much arguments';
  test.shouldThrowError( function()
  {
    _.mapsAreIdentical( {}, {}, 'redundant argument' );
  });

}

```
Тест рутина `mapsAreIdentical` прокиває ( тестує ) рутину із аналогічною назвою `mapsAreIdentical` та такою реалізацією:

```javascript

function mapsAreIdentical( src1,src2 )
{

  _.assert( arguments.length === 2 );

  if( Object.keys( src1 ).length !== Object.keys( src2 ).length )
  return fale;

  for( var s in src1 )
  {
    if( src1[ s ] !== src2[ s ] )
  return false;
  }

  return true;
}

```

<a name="unit-testing-anatomy"/>

## Анатомія модульного теста

Кожний модульний тест має наступну структуру :

1. Налаштування теста

2. Виклик тестуємого метода

3. Твердження

Для прикладу розглянемо слідуючий модульний тест arraySetBut - він розташований в файлі `wTools/proto/wtools/abase/layer1.test/Array.test.s` :

```javascript
  test.description = 'first argument has single extra element, second argument has single extra element either';
  var a = [ 1, 2, 3, 4, 15 ];
  var b = [ 1, 2, 3, 4, 5 ];
  var got = _.arraySetBut( a, b );
  var expected = [ 15 ];
  test.identical( got, expected );
  test.shouldBe( got !=== a );
  test.shouldBe( got !=== b );
```

В налаштуваннях він має :

1. Короткий опис - `  test.description = 'first argument has single extra element, second argument has single extra element either';`

2. Значення параметрів :
```javascript
  var a = [ 1, 2, 3, 4, 15 ];
  var b = [ 1, 2, 3, 4, 5 ];
``` 

Виклик тестуємого метода :

```javascript
  var got = _.arraySetBut( a, b );
```

Твердження : 

```javascript
  test.identical( got, expected );
```

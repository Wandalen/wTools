# Концепції

<details>
  <summary><a href="./concept/RoutineInput.md#Масив-аргументів">
    Масив аргументів
  </a></summary>
    Це сукупність параметрів рутини, котрі сформовано в псевдомасив <code>arguments</code>.
</details>

<details>
  <summary><a href="./concept/RoutineInput.md#Мапа-опцій">
    Мапа опцій
  </a></summary>
    Це контейнер, що призначений для передачі даних в рутину та управління її поведінкою.
</details>

<details>
  <summary><a href="./concept/Vectorization.md#векторизація">
    Векторизація
  </a></summary>
    Векторизація - це процес перетворення алгоритму від операцій над одним значенням за раз, до операцій над набором
    значень(<a href="./concept/Vector.md#вектор">вектором</a>) за один раз.
</details>

<details>
  <summary><a href="./concept/Vectorization.md#автоматична-векторизація">
    Автоматична векторизація
  </a></summary>
  Векторизацію можливо виконати за допомогою рутини <a href="./tutorial/Vectorize.md">vectorize</a>.
</details>

<details>
  <summary><a href="./concept/Vectorization.md#ручна-векторизація">
    Ручна векторизація
  </a></summary>
  Операції над даними можуть бути складними і не піддаватись автоматичній векторизації або робити її недоцільною.
  В такому разі можливо векторизувати рутини вручну.
</details>

<details>
  <summary><a href="./concept/Vector.md">
    Вектор
  </a></summary>
    Це контейнер, що призначений для передачі даних в рутину, містить впорядкований скінченний набір елементів (даних).
</details>

<details>
  <summary><a href="./concept/Evaluator.md">
    Евалуатор
  </a></summary>
    Це рутина або пара рутин, які дозволяють довільним чином перетворити елементи масиву перед їх порівнянням. Якщо отримані значення співпадають евалуатор повертає <code>true</code>, інакше - <code>false</code>. Евалуатор, в якості колбека, передається в рутини для роботи із [Long](./concept/Long.md) типами.
</details>

<details>
  <summary><a href="./concept/Equalizer.md">
    Еквалайзер
  </a></summary>
    Це функція, котра порівнюючи два елементи масиву повертає <code>true</code> у випадку, якщо умова порівняння виконується, інакше - <code>false</code>. Еквалайзер, в якості колбека, передається в рутини для роботи із [Long](./concept/Long.md) типами.
</details>

<details>
  <summary><a href="./concept/TypePrimitive.md">
    Тип <code>primitive</code>
  </a></summary>
    В <code>JavaScript</code> усі типи даних, що є цілісними і не мають змінюваних елементів називають примітивними.
</details>

<details>
  <summary><a href="./concept/TypePrimitive.md#тип-string">
    Тип string
  </a></summary>
    Примітивний тип даних для представлення текстових даних.
</details>

<details>
  <summary><a href="./concept/Long.md">
    Тип <code>Long</code>
  </a></summary>
    Комплексні типи даних, що містять множину елементів котрі ідентифікуються за індексом.
</details>

<details>
  <summary><a href="./concept/Long.md#Дескриптор-типу-Long">
    Дескриптор типу <code>Long</code>
  </a></summary>
    Дескриптор, що визначає тип контейнеру при створенні лонгу, якщо тип не вказано явно.
</details>

<details>
  <summary><a href="./concept/TypeUnroll.md">
    Тип <code>unroll</code>
  </a></summary>
    Тип даних <code>unroll</code> - особливий вид масиву, здатний розгортатись в іншому масиві при виконанні операції над останнім.
</details>

<details>
  <summary><a href="./concept/Buffer.md">
    Буферні типи
  </a></summary>
    Сутності, для зберігання та обробки бінарних даних.
</details>

<details>
  <summary><a href="./concept/BufferRaw.md">
    Тип <code>BufferRaw</code>
  </a></summary>
    Стандартний нетипізований буфер, буфер необроблених даних, що призначений не для доступу до даних через цей об'єкт напряму, а для використання іншими буферами, як контейнера даних.
</details>

<details>
  <summary><a href="./concept/BufferTyped.md">
    Тип <code>BufferTyped</code>
  </a></summary>
    Стандартні типізовані буфери.
</details>

<details>
  <summary><a href="./concept/BufferView.md">
    Тип <code>BufferView</code>
  </a></summary>
    Cтандартний нетипізований буфер для динамічного типізування даних `BufferRaw`.
</details>

<details>
  <summary><a href="./concept/BufferNode.md">
    Тип <code>BufferNode</code>
  </a></summary>
    Нестандартна реалізація <code>NodeJS</code> нетипізованого буфера з вбудованою можливістю доступу до нього.
</details>

<details>
  <summary><a href="./concept/TypeInterval.md">
    Тип <code>Interval</code>
  </a></summary>
    <code>Interval</code> - збірний тип даних - пара чисел, що задає послідовність.
</details>

<details>
  <summary><a href="./concept/TypeInterval.md#Тип-Ointerval">
    Тип <code>Ointerval</code>
  </a></summary>
    <code>Ointerval</code> ( від <code>open interval</code> ) - перший елемент ренжа позначає індекс елемента з якого починається відлік елементів, а другий - на індекс останнього елемента відліку та не включає його.
</details>

<details>
  <summary><a href="./concept/TypeInterval.md#Тип-Cinterval">
    Тип <code>Cinterval</code>
  </a></summary>
    <code>Cinterval</code> ( від <code>closed interval</code> ) - перший елемент ренжа позначає індекс елемента з якого починається відлік елементів, а другий - на індекс останнього елемента відліку включаючи його.
</details>

<details>
  <summary><a href="./concept/TypeInterval.md#Тип-Linterval">
    Тип <code>Linterval</code>
  </a></summary>
    <code>Linterval</code> ( від <code>length interval</code> ) - перший елемент ренжа позначає індекс елемента з якого починається відлік, а другий позначає довжину - кількість елементів, що потрбно перебрати.
</details>

<details>
  <summary><a href="./concept/Stream.md">
    Тип <code>Stream</code>
  </a></summary>
    <code>Stream</code> - абстрактний інтерфейс або його реалізація для роботи з даними розбиваючи їх на послідовність шматків, що йдуть одна за одною та обробляючи так в обробниках подій даного інтерфейсу.
</details>

<details>
  <summary><a href="./concept/MakeAndFrom.md">
    Конвенція <code>make</code> і <code>from</code>
  </a></summary>
    Конвенція іменування рутин <code>make</code> і <code>from</code>` забезпечує уніфікацію коду та впроваджує єдині правила для створення рутин-конструкторів.
</details>

<details>
  <summary><a href="./concept/Escape.md">
    Ескейпінг
  </a></summary>
    Ескейпінг - процес маскування певного значення. Реалізується рутинами і засобами, щоб вирішити проблему обмеженності значень.
</details>

## Туторіали

<!--
<details>
  <summary><a href="./tutorial/Abstract.md">
    Загальна інформація
  </a></summary>
    Загальна інформація про модуль Tools.
</details>

<details>
  <summary><a href="./tutorial/Installation.md">
    Встановлення
  </a></summary>
    Процедура встановлення модуль Tools.
</details>
-->

<details>
  <summary><a href="./tutorial/ConventionNamingInArray.md">
    Конвенції рутин _.array*: найменування рутин
  </a></summary>
    Принципи найменування рутин сімейства <code>Array</code>.
</details>

<details>
  <summary><a href="./tutorial/ConventionCallbacksInArray.md">
    Конвенції рутин _.array*: callbacks
  </a></summary>
    Як використовувати <code>callback</code>-и в рутинах сімейства <code>Array</code>.
</details>

<details>
  <summary><a href="./tutorial/ConventionDstArgInArray.md">
    Конвенції рутин _.array*: аргумент dst
  </a></summary>
    Про що свідчить використання аргументів <code>dst</code> і <code>src</code>.
</details>

<details>
  <summary><a href="./tutorial/ConventionDstNull.md">
    Конвенції: dst=null
  </a></summary>
    Для чого передається <code>null</code> в параметр <code>dst</code>.
</details>

<details>
  <summary><a href="./tutorial/ConventionDefaultsMapInRoutine.md">
    Конвенції: мапа дефолтних значень
  </a></summary>
    Як зберігаються та змінюються налаштування в рутинах.
</details>

<details>
  <summary><a href="./tutorial/ConventionNamingMakeAndFrom.md">
    Суфікси <code>Make</code> i <code>From</code> в найменуванні рутин
  </a></summary>
    Для позначення рутин-конструкторів використовуються рутини в назві яких є суфікси <code>Make</code> i <code>From</code>.
</details>

<details>
  <summary><a href="./tutorial/TimeMeasurement.md">
    Як заміряти час
  </a></summary>
    Вимірювання часу виконання алгоритмів для вибору оптимального рішення.
</details>

<details>
  <summary><a href="./tutorial/Timer.md">
    Таймер
  </a></summary>
    Все про таймери.
</details>

<details>
  <summary><a href="./tutorial/EmbeddedModules.md">
    Вбудовані модулі
  </a></summary>
    Вбудовані модулі та відстрочене виконання коду, що потребує недоступного в даний момент модуля за допомогою механізму подій.
</details>

<details>
  <summary><a href="./tutorial/RoutineUnite.md">
    Рутина routine.unite
  </a></summary>
    Автоматичне об'єднання рутини підготовки даних і рутини для їх обробки.
</details>

<details>
  <summary><a href="./tutorial/Vectorize.md">
    Векторизація рутин
  </a></summary>
    Використання рутини <code>vectorize</code> для векторизації скалярних рутин.
</details>

<details>
  <summary><a href="./tutorial/VectorizeOptions.md">
    Опції векторизації
  </a></summary>
    Як використовувати опції рутини <code>vectorize</code> при створенні векторизованої рутини.
</details>

<details>
  <summary><a href="./tutorial/Ofication.md">
    Офікація рутини
  </a></summary>
    Офікація рутини - перетворення рутини, котра очікує масив аргументів в таку рутину, яка очікує мапу опцій <code>о</code>.
</details>

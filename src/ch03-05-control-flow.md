## Przepływ sterowania

Wykonanie jakiegoś kodu w zależności od tego, czy warunek jest spełniony, albo wykonywania go wielokrotnie dopóki warunek jest spełniony, to podstawowe możliwości większości języków programowania. Najpopularniejsze konstrukcje, które pozwalają sterować przebiegiem wykonania kodu Rusta to wyrażenia `if` oraz pętle.

### Wyrażenia `if`

Wyrażenie `if` pozwala na rozgałęzienie kodu w zależności od spełnienia warunków. Podajemy warunek i nakazujemy: “Jeśli ten warunek jest spełniony, uruchom ten blok kodu. Jeśli warunek nie jest spełniony, nie uruchamiaj tego bloku kodu.”

Stwórzmy nowy projekt o nazwie *branches* (z ang. rozgałęzienia) w katalogu *projects* by zgłębić wyrażanie `if`. W pliku *src/main.rs* wpiszmy:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Wszystkie wyrażenia `if` rozpoczynają się słowem kluczowym `if`, po którym następuje warunek. W tym przypadku, warunek sprawdza czy zmienna `number` ma wartość mniejszą od 5.
Blok kodu umieszczony w nawiasach klamrowych bezpośrednio po warunku zostanie wykonany tylko wtedy, gdy warunek będzie spełniony, tj. będzie miał wartość logiczną `true`.
Bloki kodu powiązane z warunkami w wyrażenie `if` nazywane są czasami *odnogami*, podobnie jak to było w przypadku wyrażenia `match`, o którym wspomnieliśmy w sekcji [“Porównywanie Odpowiedzi z Sekretnym Numerem”][comparing-the-guess-to-the-secret-number]<!-- ignore --> rozdziału 2.

Opcjonalnie, możemy również dodać wyrażenie `else`, co zresztą uczyniliśmy, aby dodać alternatywny blok kodu, wykonywany gdy warunek nie zajdzie (da `false`). Jeśli nie ma wyrażenia `else`, a warunek da `false`, program po prostu pominie blok `if` i przejdzie do następnego fragmentu kodu.

Spróbujmy uruchomić ten kod. Powinniśmy zobaczyć:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

Zmieńmy `number` tak, by jego wartość nie spełniała warunku `false` i zobaczmy co się stanie:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

Uruchommy program ponownie i zobaczmy co wypisze:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

Warto też zauważyć, że warunek w tym kodzie *musi* dawać wartość typu `bool`, bo inaczej kod się nie skompiluje. Na przykład, spróbujmy uruchomić następujący kod:

<span class="filename">Plik: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

Tutaj wartością warunku w `if` jest liczba `3`, co prowadzi do błędu kompilacji:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

Błąd wskazuje, że Rust oczekiwał `bool`a, a dostał liczbę. W przeciwieństwie do języków takich jak Ruby i JavaScript, Rust nie próbuje automatycznie konwertować typów nie-Boolean na Boolean.
Jako warunek w `if` należy zawsze podać jawne wyrażenie dające wartość logiczną.
Jeśli na przykład chcemy, aby blok kodu `if` uruchamiał się tylko wtedy, gdy liczba nie jest równa `0`, to możemy zmienić wyrażenie `if` tak:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

Ten program wypisze `number wynosi coś innego niż zero`.

#### Obsługa wielu warunków za pomocą `else if`

Można użyć wielu warunków łącząc `if` i `else` w wyrażenie `else if`. Na przykład:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

Ten program może podążyć czterema różnymi ścieżkami. Po jego uruchomieniu powinniśmy zobaczyć:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

Kiedy ten program wykonuje się, sprawdza każde wyrażenie `if` po kolei i wykonuje pierwszy blok kodu, dla którego zachodzi warunek. Proszę zauważyć, że mimo iż 6 jest podzielne przez 2, nie widzimy wyjścia `liczba jest podzielna przez 2`, ani nie widzimy tekstu `liczba nie jest podzielna przez 4, 3, ani 2` z bloku `else`.
Dzieje się tak dlatego, że Rust wykonuje blok tylko dla pierwszego warunku dającego `true`, a gdy już go znajdzie, to dalszych warunków nawet nie sprawdza.

Użycie zbyt wielu wyrażeń `else if` może zagmatwać kod. Więc jeśli jest ich więcej niż jedno, to warto rozważyć refaktoryzację kodu. Dla takich przypadków, w Ruście przewidziano potężną konstrukcję `match`, która jest opisana w rozdziale 6.

<!-- #### Using `if` in a `let` Statement -->
#### `if` w składni `let`

Ponieważ `if` jest wyrażeniem, to możemy go użyć po prawej stronie instrukcji `let`, aby przypisać jego wynik do zmiennej, co pokazano na Listingu 3-2.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

<span class="caption">Listing 3-2: Przypisanie wyniku wyrażenia `if` do zmiennej</span>

Zmiennej `number` zostanie nadana wartość będąca wynikiem wyrażenia `if`. Uruchommy ten kod, aby zobaczyć co się stanie:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

Należy pamiętać, że wartościami bloków kodu są ostatnie wyrażenie w nich zawarte, a liczby same w sobie są również wyrażeniami. W tym przypadku, wartość całego wyrażenia `if` zależy od tego, który blok kodu zostanie wykonany.
Oznacza to, że wartości, które potencjalnie mogą być wynikami poszczególnych odnóg `if` muszą być tego samego typu. W Listingu 3-2, wynikami zarówno ramienia `if` jak i ramienia `else` były liczby całkowite `i32`. Jeśli typy są niezgodne, jak w poniższym przykładzie, to otrzymamy błąd:

<span class="filename">Plik: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

Próba skompilowania tego kodu skończy się błędem. Odnogi `if` i `else` mają niezgodne typy wartości, zaś Rust dokładnie wskazuje, gdzie w programie należy szukać problemu:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

Wyrażenie w bloku `if` daje liczbę całkowitą, a wyrażenie w bloku `else` łańcuch znaków. To nie zadziała, ponieważ zmienne muszą mieć jeden typ, a Rust musi już w czasie kompilacji definitywnie wiedzieć jakiego typu jest zmienna `number`. Znajomość tego typu pozwala kompilatorowi sprawdzić, czy jest on poprawny we wszystkich miejscach użycia `number`. Rust nie byłby w stanie tego zrobić, gdyby typ `number` był określany dopiero w czasie wykonywania; kompilator byłby bardziej skomplikowany i dawałby mniej gwarancji na kod, gdyby musiał śledzić wiele hipotetycznych typów dla każdej zmiennej.

<!-- ### Repetition with Loops -->
### Powtarzanie Za Pomocą Pętli

Często przydaje się wykonanie bloku kodu więcej niż raz. Do tego zadania Rust udostępnia kilka *pętli*, które wykonują kod wewnątrz ciała pętli do końca, po czym natychmiast wykonują go ponownie. Aby poeksperymentować z pętlami, stwórzmy nowy projekt o nazwie *loops*.

Rust ma trzy rodzaje pętli: `loop`, `while`, i `for`. Wypróbujmy każdy z nich.

<!-- #### Repeating Code with `loop` -->
#### Powtarzanie Wykonania Za Pomocą `loop`

Słowo kluczowe `loop` nakazuje Rustowi wykonywać blok kodu w koło, bez końca, lub do momentu, w którym wyraźnie nakażemy mu przestać.

By to zilustrować, zmieńmy zawartość pliku *src/main.rs* w katalogu *loops* na następującą:

<span class="filename">Plik: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

Gdy uruchomimy ten program, zobaczymy `znowu!` wypisywane w koło, bez końca, dopóki ręcznie nie zatrzymamy programu. W większości terminali można to zrobić za pomocą skrótu klawiszowego <span class="keystroke">ctrl-c</span>. Spróbujmy:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
znowu!
znowu!
znowu!
znowu!
^Cznowu!
```

Symbol `^C` pokazuje gdzie wcisnęliśmy <span class="keystroke">ctrl-c</span>.
Słowo `znowu!` może zostać wypisane lub nie po `^C`, zależnie od tego, która linia programu była wykonywana w momencie gdy odebrał on sygnał przerwania.

Szczęśliwie, Rust zapewnia również sposób na przerwanie pętli za pomocą kodu. Można umieścić słowo kluczowe `break` wewnątrz pętli, aby powiedzieć programowi, gdzie ma przerwać jej wykonywanie.
Proszę sobie przypomnieć, że już to zrobiliśmy w grze-zgadywance w sekcji [“Quitting After a Correct Guess”][quitting-after-a-correct-guess]<!-- ignore --> rozdziału 2, aby zakończyć program, gdy użytkownik wygrał grę, zgadując poprawną liczbę.

W grze-zgadywance użyliśmy również `continue`, które w pętli nakazuje programowi pominąć kod pozostały do wykonania w bieżącej iteracji pętli i rozpocząć następną iterację.

<!-- #### Returning Values from Loops -->
#### Zwracanie Wartości z Pętli

One of the uses of a `loop` is to retry an operation you know might fail, such
as checking whether a thread has completed its job. You might also need to pass
the result of that operation out of the loop to the rest of your code. To do
this, you can add the value you want returned after the `break` expression you
use to stop the loop; that value will be returned out of the loop so you can
use it, as shown here:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

Before the loop, we declare a variable named `counter` and initialize it to
`0`. Then we declare a variable named `result` to hold the value returned from
the loop. On every iteration of the loop, we add `1` to the `counter` variable,
and then check whether the `counter` is equal to `10`. When it is, we use the
`break` keyword with the value `counter * 2`. After the loop, we use a
semicolon to end the statement that assigns the value to `result`. Finally, we
print the value in `result`, which in this case is `20`.

#### Loop Labels to Disambiguate Between Multiple Loops

If you have loops within loops, `break` and `continue` apply to the innermost
loop at that point. You can optionally specify a *loop label* on a loop that
you can then use with `break` or `continue` to specify that those keywords
apply to the labeled loop instead of the innermost loop. Loop labels must begin
with a single quote. Here’s an example with two nested loops:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

The outer loop has the label `'counting_up`, and it will count up from 0 to 2.
The inner loop without a label counts down from 10 to 9. The first `break` that
doesn’t specify a label will exit the inner loop only. The `break
'counting_up;` statement will exit the outer loop. This code prints:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

#### Conditional Loops with `while`

A program will often need to evaluate a condition within a loop. While the
condition is `true`, the loop runs. When the condition ceases to be `true`, the
program calls `break`, stopping the loop. It’s possible to implement behavior
like this using a combination of `loop`, `if`, `else`, and `break`; you could
try that now in a program, if you’d like. However, this pattern is so common
that Rust has a built-in language construct for it, called a `while` loop. In
Listing 3-3, we use `while` to loop the program three times, counting down each
time, and then, after the loop, print a message and exit.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

<span class="caption">Listing 3-3: Using a `while` loop to run code while a
condition holds true</span>

This construct eliminates a lot of nesting that would be necessary if you used
`loop`, `if`, `else`, and `break`, and it’s clearer. While a condition
evaluates to `true`, the code runs; otherwise, it exits the loop.

#### Looping Through a Collection with `for`

You can choose to use the `while` construct to loop over the elements of a
collection, such as an array. For example, the loop in Listing 3-4 prints each
element in the array `a`.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

<span class="caption">Listing 3-4: Looping through each element of a collection
using a `while` loop</span>

Here, the code counts up through the elements in the array. It starts at index
`0`, and then loops until it reaches the final index in the array (that is,
when `index < 5` is no longer `true`). Running this code will print every
element in the array:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

All five array values appear in the terminal, as expected. Even though `index`
will reach a value of `5` at some point, the loop stops executing before trying
to fetch a sixth value from the array.

However, this approach is error prone; we could cause the program to panic if
the index value or test condition is incorrect. For example, if you changed the
definition of the `a` array to have four elements but forgot to update the
condition to `while index < 4`, the code would panic. It’s also slow, because
the compiler adds runtime code to perform the conditional check of whether the
index is within the bounds of the array on every iteration through the loop.

As a more concise alternative, you can use a `for` loop and execute some code
for each item in a collection. A `for` loop looks like the code in Listing 3-5.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

<span class="caption">Listing 3-5: Looping through each element of a collection
using a `for` loop</span>

When we run this code, we’ll see the same output as in Listing 3-4. More
importantly, we’ve now increased the safety of the code and eliminated the
chance of bugs that might result from going beyond the end of the array or not
going far enough and missing some items.

Using the `for` loop, you wouldn’t need to remember to change any other code if
you changed the number of values in the array, as you would with the method
used in Listing 3-4.

The safety and conciseness of `for` loops make them the most commonly used loop
construct in Rust. Even in situations in which you want to run some code a
certain number of times, as in the countdown example that used a `while` loop
in Listing 3-3, most Rustaceans would use a `for` loop. The way to do that
would be to use a `Range`, provided by the standard library, which generates
all numbers in sequence starting from one number and ending before another
number.

Here’s what the countdown would look like using a `for` loop and another method
we’ve not yet talked about, `rev`, to reverse the range:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

This code is a bit nicer, isn’t it?

## Summary

You made it! This was a sizable chapter: you learned about variables, scalar
and compound data types, functions, comments, `if` expressions, and loops! To
practice with the concepts discussed in this chapter, try building programs to
do the following:

* Convert temperatures between Fahrenheit and Celsius.
* Generate the *n*th Fibonacci number.
* Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
  taking advantage of the repetition in the song.

When you’re ready to move on, we’ll talk about a concept in Rust that *doesn’t*
commonly exist in other programming languages: ownership.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#porwnywanie-odpowiedzi-gracza-z-sekretnym-numerem
[quitting-after-a-correct-guess]:
ch02-00-guessing-game-tutorial.md#wychodzenie-z-programu-po-poprawnym-odgadniciu-liczby

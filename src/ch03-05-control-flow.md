## Przepływ sterowania

Wykonanie jakiegoś kodu w zależności od tego, czy warunek jest spełniony, albo wykonywania go wielokrotnie dopóki warunek jest spełniony, to podstawowe możliwości większości języków programowania. Najpopularniejsze konstrukcje, które pozwalają sterować przebiegiem wykonania kodu Rusta to wyrażenia `if` oraz pętle.

### Wyrażenia `if`

Wyrażenie `if` pozwala na rozgałęzienie kodu w zależności od spełnienia warunków. Podajemy warunek i nakazujemy: „Jeśli ten warunek jest spełniony, uruchom ten blok kodu. Jeśli warunek nie jest spełniony, nie uruchamiaj tego bloku kodu.”

Stwórzmy nowy projekt o nazwie *branches* (z ang. rozgałęzienia) w katalogu *projects* by zgłębić wyrażanie `if`. W pliku *src/main.rs* wpiszmy:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Wszystkie wyrażenia `if` rozpoczynają się słowem kluczowym `if`, po którym następuje warunek. W tym przypadku, warunek sprawdza czy zmienna `number` ma wartość mniejszą od 5.
Blok kodu umieszczony w nawiasach klamrowych bezpośrednio po warunku zostanie wykonany tylko wtedy, gdy warunek będzie spełniony, tj. będzie miał wartość logiczną `true`.
Bloki kodu powiązane z warunkami w wyrażenie `if` nazywane są czasami *odnogami*, podobnie jak to było w przypadku wyrażenia `match`, o którym wspomnieliśmy w sekcji [„Porównywanie Odpowiedzi z Sekretnym Numerem”][comparing-the-guess-to-the-secret-number]<!-- ignore --> rozdziału 2.

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

Ponieważ `if` jest wyrażeniem, to możemy go użyć po prawej stronie instrukcji `let`, aby przypisać jego wynik do zmiennej, co pokazano na listingu 3-2.

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
Oznacza to, że wartości, które potencjalnie mogą być wynikami poszczególnych odnóg `if` muszą być tego samego typu. Na listingu 3-2, wynikami zarówno ramienia `if` jak i ramienia `else` były liczby całkowite `i32`. Jeśli typy są niezgodne, jak w poniższym przykładzie, to otrzymamy błąd:

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
Proszę sobie przypomnieć, że już to zrobiliśmy w grze-zgadywance w sekcji [„Quitting After a Correct Guess”][quitting-after-a-correct-guess]<!-- ignore --> rozdziału 2, aby zakończyć program, gdy użytkownik wygrał grę, zgadując poprawną liczbę.

W grze-zgadywance użyliśmy również `continue`, które, użyte w pętli, nakazuje programowi pominąć kod pozostały do wykonania w bieżącej iteracji i rozpocząć następną iterację.

<!-- #### Returning Values from Loops -->
#### Zwracanie Wartości z Pętli

Jednym z zastosowań pętli `loop` jest ponawianie prób operacji, która może się nie udać, jak na przykład sprawdzenie czy wątek zakończył swoją pracę. Może zajść również potrzeba przekazania wyniku tej operacji poza pętlę, do reszty kodu. Aby to zrobić, można ten wynik podać bezpośrednio po wyrażeniu `break` zatrzymującym pętle. Zostanie on zwrócony na zewnątrz pętli i będzie można go tam użyć, na przykład tak:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

Przed pętlą deklarujemy zmienną o nazwie `counter` i inicjujemy ją wartością `0`. Następnie deklarujemy zmienną o nazwie `result`, która będzie przechowywać wartość zwracaną z pętli.
W każdej iteracji pętli dodajemy `1` do zmiennej `counter` i, następnie, sprawdzamy czy `counter` jest równy `10`. Jeśli jest, to używamy słowa kluczowego `break` z wartością `counter * 2`.
Za pętlą umieściliśmy średnik kończący instrukcję przypisującą tą wartość do `result`.
Na koniec wypisujemy wartość `result`, która w tym przypadku wynosi `20`.

<!-- #### Loop Labels to Disambiguate Between Multiple Loops -->
#### Etykiety Rozróżniające Pętle

Gdy zagnieżdżamy pętle w pętlach, `break` i `continue` odnoszą się do najbardziej wewnętrznej pętli, w której się znajdują.
Można opcjonalnie określić *etykietę dla pętli*, której można następnie użyć z `break` lub `continue`, by wskazać, że odnoszą się one do wskazanej pętli zamiast najbardziej zagnieżdżonej.
Etykiety pętli zaczynają się znakiem pojedynczego cytatu. Oto przykład:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

Pętla zewnętrzna ma etykietę `counting_up` i liczy w górę od 0 do 2.
Wewnętrzna pętla bez etykiety odlicza w dół od 10 do 9. Pierwszy `break`, bez etykiety, zakończy tylko wewnętrzną pętlę. Instrukcja `break 'counting_up;` wyjdzie z pętli zewnętrznej. Ten kod drukuje:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

<!-- Conditional Loops with `while` -->
#### Pętla Warunkowa `while`

Program często potrzebuje sprawdzić warunek wewnątrz pętli.
Pętla działa tak długa jak warunek daje `true`. Gdy warunek przestaje dawać `true`, program wywołuje `break`, zatrzymując pętlę. Można zaimplementować takie zachowania za pomocą kombinacji `loop`, `if`, `else` i `break`; zachęcam do spróbowania. Jednak jest ono na tyle powszechne, że Rust ma dla niego wbudowaną konstrukcję językową, zwaną pętlą `while`. Program z listingu 3-3 wykonuje trzy iteracje za pomocą pętli `while`, odliczając za każdym razem, a następnie, po zakończeniu pętli, drukuje komunikat i wychodzi.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

<span class="caption">Listing 3-3: Użycie pętli `while` by wykonywać kod dopóki zachodzi warunek</span>

Taka konstrukcja eliminuje wiele zagnieżdżeń, które byłyby konieczne w przypadku użycia `loop`, `if`, `else` i `break`, równocześnie poprawiając czytelność.
Pętla się wykonuje tak długo jak warunek daje `true`.

<!-- #### Looping Through a Collection with `for` -->
#### Przechodzenie Po Kolekcji Za Pomocą `for`

Do przejścia po kolekcji, takiej jak tablica, można użyć pętli `while`. Na przykład, pętla na listingu 3-4 wypisuje każdy element tablicy `a`.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

<span class="caption">Listing 3-4: Przechodzenie po każdym elemencie kolekcji za pomocą pętli `while`</span>

Ten kod liczy w górę po elementach tablicy. Rozpoczyna od indeksu `0` i wykonuje pętle aż do osiągnięcia ostatniego indeksu tablicy, tj. do momentu gdy `index < 5` przestaje dawać `true`.
Uruchomienie tego kodu spowoduje wydrukowanie każdego elementu tablicy:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

Wszystkie pięć wartości zawartych w tablicy pojawia się w terminalu, zgodnie z oczekiwaniami. Nawet jeśli `index` osiągnie w pewnym momencie wartość `5`, pętla zatrzymuje się przed próbą pobrania z tablicy szóstej wartości.

Jednakże to rozwiązanie jest podatne na błędy; program może spanikować, jeśli indeks lub warunek będzie nieprawidłowy. Na przykład, jeśli skócimy tablicę `a` do czterech elementów, zapominając przy tym zaktualizować warunek na `while index < 4`, kod spanikuje.
To rozwiązanie może być również powolne, ponieważ kompilator może dodać kod sprawdzający, w każdej iteracji pętli, czy indeks znajduje się w granicach tablicy.

Zwięźlejszą alternatywą jest pętla `for` wykonująca jakiś kod dla każdego elementu w kolekcji. Listing 3-5 pokazuje przykład jej użycia.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

<span class="caption">Listing 3-5: Przechodzenie po każdym elemencie kolekcji za pomocą pętli `for`</span>

Ten kod daje takie same wyjście jak ten na listingu 3-4. Co ważne, zwiększyliśmy bezpieczeństwo kodu i wyeliminowaliśmy zagrożenie wystąpienie błędów związanych z przekroczeniem końca tablicy albo niedojściem do niego i w konsekwencji nieosiągnięciem niektórych elementów.

W przeciwieństwie do metody użytej na listingu 3-4, korzystając z pętli `for` nie musimy martwić się poprawianiem innego kodu gdy zmieniamy liczbę elementów w tablicy.

Bezpieczeństwo i zwięzłość pętli `for` sprawiają, że jest ona najczęściej wykorzystywaną pętlą w Rust. Nawet gdy istnieje potrzeba wykonania jakiegoś kod określoną liczbę razy, jak w przykładzie odliczania, który na listingu 3-3 używał pętli `while`, większość rustowców użyłaby pętli `for` wraz z zawartym w bibliotece standardowej `Range` (z ang. zakres), który generuje wszystkie liczby w kolejności, zaczynając od jednej liczby i kończąc przed inną liczbą.

Oto jak wyglądałoby odliczanie przy użyciu pętli `for` i metody `rev` (o której jeszcze nie mówiliśmy) odwracającej zakres:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

Ten kod jest nieco ładniejszy, nieprawdaż?

<!-- ## Summary -->
## Podsumowanie

Udało się! To był długi rozdział: poznaliśmy zmienne, skalarne i złożone typy danych, funkcje, komentarze, wyrażenia `if` i pętle! Aby przećwiczyć pojęcia omawiane w tym rozdziale, spróbuj zbudować programy wykonujące następujące czynności:

* Przeliczanie temperatur pomiędzy stopniami Celsjusza i Fahrenheita.
* Generowanie *n*-tej liczby ciągu Fibonacciego.
* Drukowanie tekstu kolędy "The Twelve Days of Christmas" z wykorzystaniem powtórzeń w piosence.

Kiedy będziesz gotowy aby przejść dalej, porozmawiamy o koncepcji Rusta, która *nie* jest powszechna w innych językach programowania, o własności.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#porównywanie-odpowiedzi-z-sekretnym-numerem
[quitting-after-a-correct-guess]:
ch02-00-guessing-game-tutorial.md#wychodzenie-z-programu-po-poprawnym-odgadnięciu

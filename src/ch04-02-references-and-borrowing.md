## Referencje i pożyczanie

Z rozwiązaniem z krotką zastosowanym na listingu 4-5 związana jest taka niedogodność,
że musieliśmy zwrócić `String` do funkcji wywołującej, by ta mogła dalej z niego korzystać
(po wywołaniu `calculate_length`). Było tak dlatego że wspomniany `String` był przenoszony do `calculate_length`.

Funkcję `calculate_length` można by zdefiniować w następujący sposób, w którym jako parametr przekazywana jest jedynie referencja do obiektu i dzięki temu nie jest on przez `calculate_length` przejmowany na własność:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:all}}
```
Po pierwsze proszę zauważyć, że krotki nie są nam już dalej potrzebne. Nie ma ich ani przy deklaracji zmiennej, ani w typie zwracanym przez funkcję `calculate_length`. Po drugie, proszę zwrócić uwagę, że przekazujemy do tej funkcji `&s1` oraz, że typ jej parametru został zmieniony z `String` na `&String`.

Te ampersandy oznaczają *referencje* (ang. references) i pozwalają na odnoszenie się (referowanie) do wartości bez przejmowania jej na własność.
Jest to zilustrowane na Rysunku 4-5.

<img alt="&String s wskazuje na String s1" src="img/trpl04-05.svg" class="center" />

<span class="caption">Rysunek 4-5: `&String s` wskazuje na `String s1`</span>

> Uwaga: Przeciwieństwem referowania za pomocą `&` jest *dereferowanie*,
> które realizowane jest za pomocą operatora dereferencji, `*`.
> W rozdziale 8 przedstawiono przykładowe zastosowania operatora dereferencji,
> zaś więcej szczegółów na jego temat można znaleźć w rozdziale 15.

Przyjrzyjmy się nieco bliżej temu wywołaniu funkcji:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:here}}
```
Składnia `&s1` tworzy referencję, która co prawda *referuje* do `s1`, ale
nie posiada go na własność.
Skoro zaś go nie posiada, to wskazywana przez nią wartość nie zostanie
usunięta wraz z końcem zakresu życia samej referencji.

Sygnatura funkcji także używa `&` do wskazania, że `s` jest referencją.
Poniżej dodano kilka komentarzy z wyjaśnieniami:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-08-reference-with-annotations/src/main.rs:here}}
```
Zakres, w którym zmienna `s` jest dostępna, jest taki sam jak zakres każdego innego parametru funkcji.
Jednakże, ponieważ `s` nie posiada tego, na co wskazuje, to nie jest to kasowane gdy `s` wyjdzie poza swój zakres.
W przeciwieństwie do argumentów przekazywanych przez wartość, te przekazywane przez referencje nie są funkcji dawane na własność.
Dlatego też funkcja nie musi więcej zwracać ich za pomocą `return`, by je oddać.

Przekazywanie referencji jako parametrów funkcji nazywamy *pożyczaniem* (ang. borrowing).
I jak w prawdziwym życiu, jeśli ktoś coś posiada, możemy to od niego pożyczyć.
W końcu jednak musimy mu to także oddać.

Co więc się stanie gdy spróbujemy zmodyfikować coś, co pożyczyliśmy?
Wypróbujmy kod z listingu 4-6. Uwaga: on nie zadziała!

<span class="filename">Plik: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-06/src/main.rs}}
```

<span class="caption">Listing 4-6: Próba modyfikacji pożyczonej wartości</span>

Otrzymamy następujący błąd:

```text
{{#include ../listings/ch04-understanding-ownership/listing-04-06/output.txt}}
```

Tak jak zmienne, referencje są domyślnie niemutowalne.
Nie możemy zmieniać czegoś do czego mamy referencję.

### Mutowalne Referencje

Możemy wyeliminować błąd z kodu z listingu 4-6 wprowadzając drobną poprawkę:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-09-fixes-listing-04-06/src/main.rs}}
```

Po pierwsze, zmieniliśmy `s` by było `mut`. Następnie, utworzyliśmy mutowalną referencję za pomocą `&mut s` i ją przyjęliśmy za pomocą `some_string: &mut String`.

Jednakże mutowalne referencję posiadają jedno spore ograniczenie: w danym zakresie można mieć tylko jedną mutowalną referencję do konkretnych danych. Ten kod nie skompiluje się:

<span class="filename">Plik: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/src/main.rs:here}}
```

Otrzymamy następujący błąd:

```text
{{#include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/output.txt}}
```

To ograniczenie pozwala na mutowalność jedynie w bardzo kontrolowany sposób.
Może ono być kłopotliwe dla początkujących rustowców, gdyż większość innych języków nie nakłada podobnych ograniczeń.

Korzyścią z tego ograniczenia jest to, że Rust może zapobiec tzw. wyścigom do danych (ang. data races) i to już na etapie kompilacji. Wyścig do danych podobny jest do wyścigu (ang. race condition) i ma miejsce, gdy zachodzą następujące trzy warunki:

* W tym samym czasie współistnieją dwa lub więcej wskaźniki umożliwiające dostęp do tych samych danych.
* Przynajmniej jeden z tych wskaźników jest używany do zapisu danych.
* Nie ma żadnego mechanizmu synchronizacji dostępu do danych.

Wyścigi danych powodują niezdefiniowane zachowania i mogą być trudne do zdiagnozowania, wyśledzenia w czasie wykonywania programu i naprawienia; Tymczasem Rust całkowicie im zapobiega, nie kompilując nawet kodu który je zawiera!

Oczywiście zawsze możemy użyć nawiasów klamrowych do stworzenia nowego zakresu, pozwalając na
wiele mutowalnych referencji, ale nie *równocześnie*:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-11-muts-in-separate-scopes/src/main.rs:here}}
```

Podobne ograniczenie dotyczy mieszania mutowalnych i niemutowalnych referencji. Następujący kod nie skompiluje się:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/src/main.rs:here}}
```

Kompilator wyświetli następujący komunikat błędu:

```text
{{#include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/output.txt}}
```

Whew! We *also* cannot have a mutable reference while we have an immutable one.
Users of an immutable reference don’t expect the values to suddenly change out
from under them! However, multiple immutable references are okay because no one
who is just reading the data has the ability to affect anyone else’s reading of
the data.

Note that a reference’s scope starts from where it is introduced and continues
through the last time that reference is used. For instance, this code will
compile because the last usage of the immutable references occurs before the
mutable reference is introduced:

```rust,edition2018
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-13-reference-scope-ends/src/main.rs:here}}
```

The scopes of the immutable references `r1` and `r2` end after the `println!`
where they are last used, which is before the mutable reference `r3` is
created. These scopes don’t overlap, so this code is allowed.

Even though borrowing errors may be frustrating at times, remember that it’s the
Rust compiler pointing out a potential bug early (at compile time rather than
at runtime) and showing you exactly where the problem is. Then you don’t have
to track down why your data isn’t what you thought it was.

### Dangling References

In languages with pointers, it’s easy to erroneously create a *dangling
pointer*, a pointer that references a location in memory that may have been
given to someone else, by freeing some memory while preserving a pointer to
that memory. In Rust, by contrast, the compiler guarantees that references will
never be dangling references: if you have a reference to some data, the
compiler will ensure that the data will not go out of scope before the
reference to the data does.

Let’s try to create a dangling reference, which Rust will prevent with a
compile-time error:

<span class="filename">Plik: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/src/main.rs}}
```

Here’s the error:

```text
{{#include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/output.txt}}
```

This error message refers to a feature we haven’t covered yet: lifetimes. We’ll
discuss lifetimes in detail in Chapter 10. But, if you disregard the parts
about lifetimes, the message does contain the key to why this code is a problem:

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from.
```

Let’s take a closer look at exactly what’s happening at each stage of our
`dangle` code:

<span class="filename">Plik: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-15-dangling-reference-annotated/src/main.rs:here}}
```

Because `s` is created inside `dangle`, when the code of `dangle` is finished,
`s` will be deallocated. But we tried to return a reference to it. That means
this reference would be pointing to an invalid `String`. That’s no good! Rust
won’t let us do this.

The solution here is to return the `String` directly:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-16-no-dangle/src/main.rs:here}}
```

This works without any problems. Ownership is moved out, and nothing is
deallocated.

### The Rules of References

Let’s recap what we’ve discussed about references:

* At any given time, you can have *either* one mutable reference *or* any
  number of immutable references.
* References must always be valid.

Next, we’ll look at a different kind of reference: slices.

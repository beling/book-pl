<!-- ## Bringing Paths Into Scope with the `use` Keyword -->
## Włączanie Ścieżek do Zasięgu za Pomocą Słowa Kluczowego `use`

Konieczność ciągłego wypisywania ścieżek, by wywołać funkcję może być uciążliwa.
Na listingu 7-7, niezależnie od tego, czy wybraliśmy bezwzględną czy względną ścieżkę do funkcji `add_to_waitlist`, za każdym razem, wywołując ją, musieliśmy napisać także `front_of_house` i `hosting`.
Na szczęście istnieje sposób na uproszczenie tego procesu: możemy raz utworzyć skrót do ścieżki za pomocą słowa kluczowego `use` i używać go wielokrotnie w obrębie zasięgu.

Na listingu 7-11 włączamy moduł `crate::front_of_house::hosting` w zasięg funkcji `eat_at_restaurant`, więc musimy podać jedynie `hosting::add_to_waitlist`, aby wywołać funkcję `add_to_waitlist` z `eat_at_restaurant`.

<span class="filename">Plik: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

<span class="caption">Listing 7-11: Włączanie modułu w zasięg za pomocą `use`</span>

Dodanie do zasięgu `use` i ścieżki jest podobne do tworzenia dowiązania symbolicznego w systemie plików.
Dodanie `use crate::front_of_house::hosting` w korzeniu skrzyni sprawia, że `hosting` staje się poprawną nazwą w tym zasięgu, tak jakby moduł `hosting` był zdefiniowany w korzeniu skrzyni.
Ścieżki wprowadzone w zasięg za pomocą `use` podlegają takim samym zasadą prywatność, jak wszystkie inne ścieżki.

Warto podkreślić, że `use` tworzy skrót tylko w zasięgu, w którym występuje.
Na listingu 7-12 przeniesiono funkcję `eat_at_restaurant` do nowego modułu podrzędnego o nazwie `customer`, który tworzy zasięg odrębny od tego, w którym użyto `use`. Dlatego ciało funkcji nie skompiluje się:

<span class="filename">Plik: src/lib.rs</span>

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

<span class="caption">Listing 7-12: `use` obowiązuje jedynie w zasięgu, w którym się znajduje</span>

Błąd kompilatora pokazuje, że skrót nie ma zastosowania w obrębie modułu `customer`:

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

Proszę zauważyć, że pojawiło się również ostrzeżenie, że `use` nie jest używany w swoim zasięgu!
Aby rozwiązać ten problem, należy przenieść `use` do modułu `customer`, lub z modułu `customer` odwołać się do skrótu w module nadrzędnym za pomocą `super::hosting`.

<!-- ### Creating Idiomatic `use` Paths -->
### Tworzenie Idiomatycznych Ścieżek `use`

Patrząc na listing 7-11, można się zastanawiać, dlaczego zdefiniowaliśmy `use crate::front_of_house::hosting`, a następnie w `eat_at_restaurant` wywołaliśmy `hosting::add_to_waitlist`, zamiast od razu podać w `use` całą ścieżkę do `add_to_waitlist`, tak jak na listingu 7-13.

<span class="filename">Plik: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

<span class="caption">Listing 7-13: Nieidiomatyczne włączenie w zasięg funkcji `add_to_waitlist` za pomocą `use`</span>

Działanie kodu na obu listingach, 7-11 i 7-13, jest takie samo.
Jednakże tylko listing 7-11 pokazuje idiomatyczne wykorzystanie `use` do włączenia funkcji w zasięg.
Włączenie do zasięgu jej modułu nadrzędnego sprawia, że wywołując tę funkcję musimy podać nazwę jej modułu.
To zaś jasno mówi, iż funkcja ta nie jest zdefiniowana lokalnie, a jednocześnie ogranicza konieczność podawania pełnej ścieżki.
Dla odmiany kod na listingu 7-13 jest niejasny co do miejsca, w którym zdefiniowano `add_to_waitlist`.

Z drugiej strony, gdy za pomocą `use` wskazujemy struktury, enumy i inne elementy, idiomatycznie jest podać pełną ścieżkę.
Listing 7-14 pokazuje idiomatyczny sposób włączania w zasięg skrzyni binarnej struktury `HashMap` z biblioteki standardowej.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

<span class="caption">Listing 7-14: Idiomatyczne włączenie `HashMap` w zasięg</span>

Za tym idiomem nie stoi żaden mocny argument: jest to po prostu przyjęta konwencja, zaś ludzie przyzwyczaili się do czytania i pisania zgodnego z nią kodu.

Jednakże nie możemy podążyć za tą konwencją, gdy za pomocą `use` chcemy wprowadzić w zasięg dwa elementy o tej samej nazwie.
Rust nam na to nie pozwoli.
Listing 7-15 pokazuje, jak włączyć w zasięg i odwoływać się do dwóch typów `Result`, które mają tę samą nazwę, ale pochodzą z różnych modułów.

<span class="filename">Plik: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

<span class="caption">Listing 7-15: Wprowadzenie w ten sam zasięg dwóch typów o tej samej nazwie wymaga określania ich przy użyciu ich modułów nadrzędnych.</span>

Jak widać, używanie modułów nadrzędnych pozwala rozróżnić dwa typy `Result`.
Gdybyśmy zamiast tego napisali `use std::fmt::Result` i `use std::io::Result`, mielibyśmy w tym samym zasięgu dwa różne typy `Result` i Rust nie wiedziałby, który z nich mamy na myśli, gdy piszemy `Result`.

<!-- ### Providing New Names with the `as` Keyword -->
### Nadawanie Nowych Nazw Za Pomocą Słowa Kluczowego `as`

Istnieje też inne rozwiązanie problemu wprowadzania dwóch typów o tej samej nazwie w ten sam zasięg za pomocą `use`: po ścieżce możemy podać `as` i nową nazwę lokalną, *alias* dla typu. Listing 7-16 pokazuje kod równoważny temu z listingu 7-15, ale wykorzystujący zmianę nazwy jednego z dwóch typów `Result` przy użyciu `as`.

<span class="filename">Plik: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

<span class="caption">Listing 7-16: Zmiana nazwy typu za pomocą słowa kluczowego `as`, gdy jest on włączany w zasięg</span>

W drugiej deklaracji `use` typowi `std::io::Result` nadajemy nową nazwę `IoResult`, niekolidującą z nazwą `Result` z `std::fmt`, którą również włączamy w ten sam zasięg.
Kod pokazany na obu listingach, 7-15 i 7-16, uważany jest za idiomatyczny.
Więc w takim przypadku wybór zależy jedynie od naszych osobistych preferencji!

<!-- ### Re-exporting Names with `pub use` -->
### Re-eksportowanie Nazw Za Pomocą `pub use`

Kiedy za pomocą słowa kluczowego `use` włączamy nazwę w zasięg, to w nowym zasięgu jest ona prywatna.
Aby kodowi wywołującemu nasz kod umożliwić odwołanie się do tej nazwy tak, jakby była zdefiniowana w jego zasięgu, możemy połączyć `pub` i `use`. 
Technika ta nazywana jest *reeksportem*, ponieważ wprowadzamy element w zasięg, ale również udostępniamy ten element innym, by mogli go włączyć w swój zasięg.

Listing 7-17 pokazuje kod z listingu 7-11 z zmienionym `use` w module głównym na `pub use`.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

<span class="caption">Listing 7-17: Wykorzystanie `pub use` by udostępnić nazwę do użycia przez dowolny kod z nowego zasięgu</span>

Przed zmianą, zewnętrzny kod, by wywołać funkcję `add_to_waitlist`, musiałby użyć ścieżki
`restaurant::front_of_house::hosting::add_to_waitlist()`.
Po zmianie, gdy `pub use` reeksportował moduł `hosting` z modułu głównego, zewnętrzny kod może w zamian użyć ścieżki `restaurant::hosting::add_to_waitlist()`.

Reeksportowanie jest przydatne, gdy wewnętrzna struktura twojego kodu różni się od tego, jak wywołujący go programiści postrzegają jego domenę.
Na przykład w naszej metaforze restauracyjnej, ludzie prowadzący restaurację dzielą ją na „front of house“ i „back of house“.
Ale klienci odwiedzający restaurację prawdopodobnie nie będą myśleć o częściach restauracji w ten sam sposób.
Dzięki `pub use`, możemy napisać nasz kod korzystając z innej struktury, od tej, którą ujawnimy.
Czynimy to, by nasza biblioteka była dobrze zorganizowana zarówno dla programistów pracujących nad nią, jak i tych ją wywołujących.
Przyjrzymy się innemu przykładowi `pub use` i temu, jak wpływa on na dokumentację skrzyni w sekcji [„Eksportowanie Wygodnego Publicznego API Za Pomocą `pub use`“][ch14-pub-use]<!-- ignore --> rozdziału 14.

<!-- ### Using External Packages -->
### Używanie Pakietów Zewnętrznych

W rozdziale 2 zaprogramowaliśmy grę w zgadywanie, która wykorzystywała zewnętrzny pakiet o nazwie `rand` do uzyskiwania liczb losowych. Aby użyć `rand` w naszym projekcie, dodaliśmy następującą linię do *Cargo.toml*:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Plik: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

Dodanie `rand` jako zależności w *Cargo.toml* powoduje, że Cargo pobiera pakiet `rand` wraz ze wszystkimi jego zależnościami z [crates.io](https://crates.io/) i udostępnia `rand` naszemu projektowi.

Następnie, aby włączyć definicje z `rand` w zasięg naszego pakietu, dodaliśmy linię `use` ze ścieżką rozpoczynającą się od nazwy skrzyni, czyli `rand`, i wymieniliśmy elementy, które chcemy włączyć w zasięg.
Przypomnijmy, że w sekcji [„Generowanie Losowej Liczby“][rand]<!-- ignore --> rozdziału 2, włączyliśmy w zasięg cechę `Rng` i wywołaliśmy funkcję `rand::thread_rng`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Członkowie społeczności Rusta udostępnili na stronie [crates.io](https://crates.io/) wiele pakietów, a użycie dowolnego z nich we własnym pakiecie wymaga wykonania tych samych kroków: dodania go do pliku *Cargo.toml* i włączenia w zasięg jego wybranych elementów za pomocą `use`.

Proszę zauważyć, że standardowa biblioteka `std` również jest skrzynią, która jest zewnętrzna względem naszego pakietu.
Ponieważ standardowa biblioteka jest dostarczana z językiem Rust, nie musimy dodawać `std` do *Cargo.toml*.
Musimy jednak odwoływać się do niej za pomocą `use`, aby wprowadzić jej elementy w zasięg naszego pakietu.
Na przykład, możemy skorzystać z `HashMap` za pomocą następującej linii:

```rust
use std::collections::HashMap;
```

Jest to ścieżka bezwzględna rozpoczynająca się od `std`, czyli nazwy skrzyni biblioteki standardowej.

<!-- ### Using Nested Paths to Clean Up Large `use` Lists -->
### Porządkowania Długich List `use` za Pomocą Zagnieżdżonych Ścieżek

Gdy używamy wielu elementów zdefiniowanych w tej samej skrzyni lub w tym samym module, wymienienie każdego elementu w osobnej linii pochłania sporo miejsca. Na przykład, te dwie deklaracje `use`, które mieliśmy w grze zgadywance na listingu 2-4, włączają w zasięg elementy z `std`:

<span class="filename">Plik: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

Zamiast nich, możemy użyć zagnieżdżonych ścieżek, aby włączyć te same elementy w jednym wierszu.
Robimy to, podając wspólną część ścieżki, po której następują dwa dwukropki, a następnie nawiasy klamrowe obejmujące listę różniących się fragmentów ścieżek, co pokazano na listingu 7-18.

<span class="filename">Plik: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

<span class="caption">Listing 7-18: Podanie ścieżki zagnieżdżonej w celu włączenia w zasięg wielu elementów z tym samym prefiksem</span>

W większych programach, wprowadzenie wielu elementów tej samej skrzyni lub modułu w zasięg przy użyciu zagnieżdżonych ścieżek może znacznie zmniejszyć liczbę napisanych linii `use`!

Ścieżki można zagnieżdżać na dowolnym poziomie, co jest przydatne przy łączeniu dwóch deklaracji `use` dzielących podścieżkę.
Na przykład, listing 7-19 pokazuje dwie instrukcje `use`: pierwsza włącza w zasięg `std::io`, zaś druga `std::io::Write`.

<span class="filename">Plik: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

<span class="caption">Listing 7-19: Dwie deklaracje `use`, z których jedna włącza podścieżkę drugiej</span>

Częścią wspólną tych dwóch ścieżek jest `std::io`, co daje całą pierwszą ścieżkę.
Aby zawrzeć te dwie ścieżki w jednej deklaracji `use`, można użyć `self` w zagnieżdżonej ścieżce, tak jak pokazano na listingu 7-20.

<span class="filename">Plik: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

<span class="caption">Listing 7-20: Zawarcie ścieżek z listingu 7-19 w jednej deklaracji `use`</span>

Ta linia włącza `std::io` i `std::io::Write` w zasięg.

<!-- ### The Glob Operator -->
### Operator Glob

Jeśli chcemy włączyć w zasięg *wszystkie* elementy publiczne zdefiniowane w ścieżce, możemy podać tę ścieżkę, a za nią `*` zwaną operatorem glob:

```rust
use std::collections::*;
```

Ta deklaracja `use` wprowadza do bieżącego zasięgu wszystkie publiczne elementy zdefiniowane w `std::collections`.
Operatora glob należy używać z rozwagą!
Glob może utrudnić ustalenie, które nazwy są w zasięgu i gdzie zostały zdefiniowane.

Operator globy jest często używany podczas testowania, aby wprowadzić wszystko, co jest testowane, do modułu `tests`;
o czym będziemy mówić w sekcji ["How to Write Tests"][writing-tests]<!-- ignore --> rozdziału 11.
Operator glob jest też czasami używany jako część wzorca prelude, opisanego w [dokumentacji biblioteki standardowej](../std/prelude/index.html#other-preludes)<!-- ignore -->.

[ch14-pub-use]: ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use
[rand]: ch02-00-guessing-game-tutorial.html#generowanie-losowej-liczby
[writing-tests]: ch11-01-writing-tests.html#how-to-write-tests

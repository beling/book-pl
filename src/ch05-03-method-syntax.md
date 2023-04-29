## Składnia metod

*Metody* są podobne do funkcji: też deklarujemy je słowem kluczowym `fn`,
po którym umieszczamy nazwę, a czasem parametry i typ zwracanej wartości.
Tak jak funkcje, zawierają jakieś polecenia wykonywane przy wywołaniu. 
Metody jednak różnią się od funkcji tym, że definiujemy je wewnątrz struktur
(lub enumeracji czy obiektów-cech, które omówimy odpowiednio w rozdziałach [6][enums]<!-- ignore --> i [17][trait-objects]<!-- ignore -->), a ich pierwszym parametrem jest zawsze `self` reprezentujące instancję
struktury, na której metoda jest wywołana.

### Definiowanie metod

Zmieńmy funkcję `area` przyjmującą jako parametr instancję struktury `Rectangle`,
w taki sposób aby od teraz `area` było metodą zdefiniowaną na strukturze `Rectangle`.
To przedstawia listing 5-13.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

<span class="caption">Listing 5-13: Definicja metody `area` w strukturze `Rectangle`</span>

Aby zdefiniować funkcję w kontekście `Rectangle` otwieramy blok `impl` (czyli implementacji) dla `Rectangle`.
Wszystko wewnątrz tego bloku będzie związane z typem `Rectangle`.
Następnie przenosimy funkcję `area` do nawiasów klamrowych
należących do bloku `impl` i jako pierwszy parametr funkcji dodajemy
`self` w jej sygnaturze i wszędzie w jej wnętrzu. W funkcji `main`,
zamiast jak poprzednio wywołania funkcji `area`, a jako argument
podawania jej `rect1`, możemy użyć *składni metody* aby wywołać metodę `area`
na instancji struktury `Rectangle`.
Składnia metody pojawia się po nazwie instancji: dodajemy kropkę, a po niej nazwę samej metody, oraz
nawiasy i argumenty jeśli są wymagane.

W sygnaturze funkcji `area` używamy `&self` zamiast `rectangle: &Rectangle`.
The `&self` is actually short for `self: &Self`. Within an `impl` block, the
type `Self` is an alias for the type that the `impl` block is for. Methods must
have a parameter named `self` of type `Self` for their first parameter, so Rust
lets you abbreviate this with only the name `self` in the first parameter spot.
Note that we still need to use the `&` in front of the `self` shorthand to
indicate that this method borrows the `Self` instance, just as we did in
`rectangle: &Rectangle`.
Metody mogą wejść w posiadanie `self`, pożyczyć `self` niemutowalnie, lub pożyczyć `self` mutowalnie, 
tak jakby to był jakikolwiek inny parametr.

W tym wypadku używamy `&self` z tego samego powodu, co `&Rectangle`
w wersji z funkcją. Nie chcemy ani zostać właścicielem struktury, 
ani do niej pisać, a jedynie z niej czytać.
Jeśli chcielibyśmy zmienić dane instancji w trakcie wywoływania
metody użylibyśmy `&mut self` jako pierwszego parametru. 
Tworzenie metody która wchodzi w posiadanie instancji przy użyciu `self`
jest dość rzadkie; tej techniki używamy głównie jedynie, kiedy metoda przeobraża `self`
w coś innego i chcesz zabronić wywołującemu metody wykorzystanie oryginalnej instancji
po jej transformacji.

Podstawowym celem używania metod zamiast funkcji, oprócz używania składni metody oraz braku wymogu podawania typu `self` w każdej sygnaturze, jest organizacja.  
Umieściliśmy wszystko, co związane jest z instancją danego typu w jednym bloku `impl`.
Dzięki temu oszczędzamy przyszłym użytkownikom kodu szukania zachowań struktury `Rectangle` po różnych zakątkach naszej biblioteki.

Uwaga: Możemy nadać metodzie taką samą nazwę, jak ma jedno z pól struktury.
Na przykład w `Rectangle` możemy zdefiniować metodę `width`:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

Tu zdecydowaliśmy aby metoda `width` zwracała `true` jeśli wartość w polu `width` instancji jest większa niż `0` oraz `false` gdy jest równa `0`: możemy użyć pola wewnątrz metody o tej samej nazwie w dowolnym celu.
W `main`, Rust wie, że mamy na myśli metodę `width`, gdy bezpośrednio za `rect1.width` są nawiasy,
oraz, że mamy na myśli pole `width`, gdy nawiasów nie ma.

Często, ale nie zawsze, gdy nadajemy metodzie taką samą nazwę jak polu, to chcemy aby ograniczyła ona swoje działanie jedynie do zwrócenia wartość pola. Metody takie nazywane są *getterami* (ang. *getters*). Rust, w przeciwieństwie to niektórych innych języków, nie implementuje getterów automatycznie dla pól struktury. Gettery są przydatne, ponieważ można uczynić pole prywatnym, zaś metodę publiczną, i w ten sposób umożliwić dostęp tylko do odczytu do tego pola, w publicznym API typu. Publiczne i prywatne modyfikatory dostępu do pól i metod omówimy w [rozdziale 7][public]<!-- ignore -->.

> ### Co z operatorem `->`?
>
> W C i C++ istnieją dwa różne operatory używane do wywoływania metod:
> symbolu `.` używamy do bezpośredniego wywoływania metody na obiekcie, zaś
> symbolu `->`, jeśli metodę wywołujemy na wskaźniku do obiektu, który najpierw
> musimy poddać dereferencji. Innymi słowy, jeśli `object` jest wskaźnikiem,
> `object->something()` jest podobne do `(*object).something()`.
>
> Rust nie ma operatora równoważnego z `->`; a za to w Ruście spotkasz
> funkcję *automatycznej referencji i dereferencji*. Wywoływanie metod jest jednym z niewielu miejsc,
> w których Rust wykorzystuje tę funkcję.
>
> Działa to następująco: kiedy wywołujesz metodę kodem `object.something()`, Rust
> automatycznie dodaje `&`, `&mut`, lub `*` aby `object`
> pasował do sygnatury metody. Inaczej mówiąc, oba te przykłady są równoważne:
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> Pierwszy wygląda o wiele bardziej przejrzyście. Zastosowana w niej jest automatyczna referencja,
> ponieważ metoda ma wyraźnie oznaczonego odbierającego (*ang. receiver*) - typ `self`.
> Mając informacje o odbierającym oraz o nazwie metody Rust jest w stanie jednoznacznie
> stwierdzić, czy metoda odczytuje (`&self`), zmienia (`&mut self`) lub konsumuje (`self`).
> Wymaganie przez Rusta oznaczenia pożyczania w odbierającym metody jest
> w dużej części dlaczego mechanizm posiadania jest tak ergonomiczny w używaniu.

### Metody z wieloma parametrami

Poćwiczymy używanie metod implementując drugą metodę na strukturze `Rectangle`.
Tym razem chcemy, żeby instancja struktury `Rectangle` przyjęła inną instancję `Rectangle`,
i zwróciła: wartość `true`, jeśli ta inna instancja `Rectangle`
całkowicie mieści się w instancji `self`; a jeśli nie, wartość `false`.
Innymi słowy, zakładając, że wcześniej zdefiniowaliśmy metodę `can_hold`, chcemy
być w stanie napisać program przedstawiony w listingu 5-14.

<span class="filename">Plik: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

<span class="caption">Listing 5-14: Użycie jeszcze nieistniejącej metody `can_hold`</span>

Skoro wymiary `rect2` są mniejsze od wymiarów `rect1`, a `rect3` jest szerszy
od `rect1`, spodziewamy się następującego wyniku wykonania powyższej funkcji `main`.

```text
Czy rect2 zmieści się wewnątrz rect1? true
Czy rect3 zmieści się wewnątrz rect1? false
```

Chcemy zdefiniować metodę, więc umieścimy ją w bloku `impl Rectangle`.
Metodę nazwiemy `can_hold`, i przyjmie ona jako parametr niemutowalne wypożyczenie
innej instancji `Rectangle`. Aby dowiedzieć się jaki dokładnie typ powinien
znajdować się w parametrze, spójrzmy na kod wywołujący metodę:
`rect1.can_hold(&rect2)`, przekazuje `&rect2` będące niezmiennym wypożyczeniem
`rect2`, czyli pewnej instancji `Rectangle`. Zależy nam jedynie
na odczytaniu wartości zawartych w `rect2` (do ich zmieniania wymagane byłoby mutowalne wypożyczenie),
a chcemy, żeby `main` pozostało właścicielem `rect2`, abyśmy mogli ponownie wykorzystać `rect2`
po wywołaniu metody `can_hold`. Wartość zwracana przez `can_hold` będzie typem
Boolean, a sama implementacja sprawdzi czy wysokość i szerokość instancji
`self` są większe niż odpowiednio wysokość i szerokość innej instancji `Rectangle`.
Dodanie nowej metody `can_hold` do bloku `impl` z
listingu 5-13 pokazane jest w listingu 5-15.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

<span class="caption">Listing 5-15: Implementacja metody `can_hold` na instancji
`Rectangle`, która przyjmuje inną instancję `Rectangle` jako parametr</span>

Po uruchomieniu tego kodu funkcją `main` z listingu 5-14, naszym oczom ukaże się oczekiwany przez nas tekst.
Metody mogą przyjmować wiele parametrów, które dodać możemy do ich sygnatur po parametrze `self`.
Te parametry niczym nie różnią się od parametrów funkcji.

### Funkcje powiązane

Wszystkie funkcje zdefiniowane w bloku `impl` nazywamy *funkcjami powiązanymi* (*ang. associated functions*).
Można definiować funkcje powiązane, które *nie* mają parametru `self` (więc nie są metodami), bo nie potrzebują do działania instancji typu. Już mialiśmy okazję używać funkcji powiązanej. Była nią `String::from` zdefiniowana dla typu `String`.

Funkcje powiązane są często wykorzystywane do zdefiniowania konstruktorów zwracających nową
instancję pewnej struktury. Zwykle nazywa sie je `new`, ale `new` nie jest specjalną nazwą wbudowaną w język.
Na przykład, możemy stworzyć funkcję powiązaną `square`, która przyjmie tylko jeden wymiar jako parametr i przypisze go zarówno do wysokości i szerokości, umożliwiając stworzenie kwadratowego `Rectangle` bez podawania dwa razy tej samej wartości:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

Słowa kluczowe `Self` w typie zwracanym i w ciele funkcji są aliasami do typu, który pojawia się po słowie kluczowym `impl`, czyli w tym przypadku do `Rectangle`.

Aby wywołać funkcję powiązaną używamy składni `::` po nazwie struktury, np.
`let sq = Rectangle::square(3)`. Ta funkcja znajduje się w przestrzeni nazw
struktury: składnia `::` używana jest zarówno w kontekście funkcji powiązanych,
ale i też w przestrzeniach nazw modułów. Moduły omówimy w [rozdziale 7][modules]<!-- ignore -->.

### Wiele bloków `impl`

Każda struktura może mieć wiele bloków `impl`. Dla przykładu, kod w listingu 5-15 jest równoważny z kodem w listingu 5-16 zawierającym każdą metodę w innym bloku `impl`.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

<span class="caption">Listing 5-16: Wersja kodu z listingu 5-15 z wieloma blokami `impl`</span>

W tym przypadku nie ma powodu, by metody były od siebie odseparowane w różne bloki `impl`,
ale jest to poprawna składnia. Przypadek przydatnego wykorzystania wielu bloków `impl`
omówimy w rozdziale 10, w którym omówimy typy uniwersalne i cechy.

## Podsumowanie

Dzięki strukturom możesz tworzyć własne typy, potrzebne do rozwiązania problemów w Twojej domenie. 
Kiedy używasz struktury grupujesz powiązane elementy danych, a każdemu elementowi nadajesz nazwę, co sprawia, że twój kod staje się przejrzysty. 
W blokach `impl` można zdefiniować funkcje powiązane z naszym typem, których szczególnym rodzajem są metody pozwalające określić zachowanie instancji struktury.

Ale struktury to nie jest jedyny sposób na tworzenie własnych typów:
poznamy kolejną funkcjonalność Rusta - enumeracje, czyli kolejne niezbędne narzędzie w twojej kolekcji.

[enums]: ch06-00-enums.html
[trait-objects]: ch17-02-trait-objects.md
[public]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#eksponowanie-Ścieżek-za-pomocą-słowa-kluczowego-pub
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html


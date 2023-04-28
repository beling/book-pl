<!-- ## Data Types -->
## Typy danych

Każda wartość w Ruście ma pewien *typ danych*, dzięki czemu Rust wie, z jakim
rodzajem danych ma do czynienia i jak z nimi pracować. Przyjrzymy się bliżej
dwóm grupom typów danych: skalarnym i złożonym.

Pamiętaj, że Rust jest językiem statycznie typowanym (*statically typed*),
co oznacza, że podczas kompilacji musi znać typy danych wszystkich zmiennych
obecnych w kodzie. Zazwyczaj kompilator może wywnioskować typ danych, którego
chcemy użyć na podstawie użytej wartość i sposobu jej wykorzystywania.
W przypadku gdy wiele typów danych spełnia dane założenia, przykładowo gdy
w rozdziale 2 w sekcji ["Porównywanie odpowiedzi gracza z sekretnym numerem"][comparing-the-guess-to-the-secret-number]<!-- ignore -->
konwertowaliśmy `String` do typu numerycznego wykorzystując funkcję `parse`
musimy dodać adnotację typu danych:

```rust
let guess: u32 = "42".parse().expect("To nie liczba!");
```

Jeżeli w powyższym kodzie nie dodalibyśmy adnotacji typu danych `: u32`, Rust wyświetliłby następujący komunikat o błędzie, mówiący o tym, że kompilator potrzebuje więcej informacji, aby określić, jakiego typu danych chcemy użyć:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

Można napotkać różne zapisy poszczególnych typów danych.

<!-- ### Scalar Types -->
### Typy skalarne

Typ *skalarny* reprezentuje pojedynczą wartość. Rust posiada 4 główne,
skalarne typy danych: całkowity (ang. integer), zmiennoprzecinkowy (ang. floating-point numbers), logiczny (ang. Boolean) i znakowy (ang. characters). Możesz kojarzyć je z innych języków programowania. Zobaczmy jak działają w Ruście.

<!-- #### Integer Types -->
#### Typy całkowite

*Liczba całkowita* to liczba nieposiadająca części ułamkowej.
Jeden z typów całkowitych, `u32`, wykorzystywaliśmy w rozdziale 2. Ten typ danych określa, że wartość, do której się odnosi, jest liczbą całkowitą bez znaku (typy całkowite ze znakiem zaczynają się od `i` zamiast `u`), która zajmuje 32 bity pamięci. Tabela 3-1 pokazuje typy całkowite wbudowane w Rusta.
Każdy z wariantów w kolumnach Ze znakiem i Bez znaku (na przykład `i16`) może zostać użyty do zadeklarowania typu danych liczby całkowitej.

<span class="caption">Tabela 3-1: Typy całkowite w Ruście</span>

| Rozmiar   | Ze znakiem | Bez znaku |
| --------- | ---------- | --------- |
| 8-bitów   | i8         | u8        |
| 16-bitów  | i16        | u16       |
| 32-bity   | i32        | u32       |
| 64-bity   | i64        | u64       |
| 128-bitów | i128       | u128      |
| arch      | isize      | usize     |

Każdy z wariantów może posiadać znak lub nie, a także ma określony rozmiar.
Nazwy *Ze znakiem* i *Bez znaku* odnoszą się do tego, czy dana liczba może być ujemna, czy tylko dodatnia -- inaczej mówiąc, czy liczba musi posiadać znak (ze znakiem), czy też nie, gdyż wiadomo, że zawsze będzie dodatnia (bez znaku).
Można to porównać do zapisywania liczb na kartce, gdy znak ma znaczenie, zapisujemy go -- odpowiednio plus lub minus przed liczbą, ale gdy liczba jest dodatnia i w danym kontekście nie jest to konieczne, pomijamy znak. Liczby całkowite ze znakiem przechowywane są z pomocą [Kodu uzupełnień do dwóch][twos-complement]<!-- ignore -->
(jeżeli nie jesteś pewien, co to oznacza, możesz poszukać
informacji w internecie; wyjaśnienie jest poza zakresem materiału zawartego w tej książce).

Każdy wariant ze znakiem może przechowywać liczby od -(2<sup>n - 1</sup>) do
2<sup>n - 1</sup> - 1 włącznie, gdzie *n* to liczba bitów, które wykorzystuje
dany wariant. Tak więc `i8` może przechowywać liczby od -(2<sup>7</sup>) do
2<sup>7</sup> - 1, co daje zakres od -128 do 127. Warianty bez znaku mogą
przechowywać liczby od 0 do 2<sup>n</sup> - 1, więc `u8` może przechowywać
liczby od 0 do 2<sup>8</sup> - 1, co daje zakres od 0 do 255.

Dodatkowo rozmiar typów `isize` oraz `usize` zależy od architektury komputera, na którym uruchamiasz swój program: 64 bity na komputerze o 64-bitowej architekturze i 32 bity na komputerze o 32-bitowej architekturze.

Możesz zapisywać literały liczb całkowitych w każdej z form uwzględnionych w Tabeli 3-2. Zauważ, że wszystkie literały mogące oznaczać różne typy numeryczne, pozwalają na użycie przyrostka by wskazać typ, np. `57u8`.
Literały numeryczne dopuszczają też wizualny separator `_` poprawiający czytelności, np. `1_000` oznacza tą samą wartość co `1000`.

<span class="caption">Tabela 3-2: Literały liczb całkowitych w Ruście</span>

| Literały liczbowe | Przykład      |
| ----------------- | ------------- |
| Dziesiętny        | `98_222`      |
| Szesnastkowy      | `0xff`        |
| Ósemkowy          | `0o77`        |
| Binarny           | `0b1111_0000` |
| Bajt (tylko `u8`) | `b'A'`        |

W takim razie skąd masz wiedzieć, którego typu całkowitego użyć? Jeżeli nie masz pewności, to zazwyczaj dobrze jest zacząć od typów domyślnych wykorzystywanych przez Rusta. Dla liczb całkowitych to `i32`. Z typów `isize` i `usize`
korzystamy głównie przy indeksowaniu różnego rodzaju kolekcji danych.

<!-- ##### Integer Overflow -->
> ##### Przekroczenie zakresu liczb całkowitych
>
> Załóżmy, że mamy zmienną typy `u8`, która może przechowywać wartości
> między 0 i 255. Jeżeli spróbujemy przypisać tej zmiennej wartość nie
> mieszczącą się w podanym zakresie, np. 256, nastąpi przekroczenie zakresu
> liczb całkowitych. Rust posiada kilka ciekawych zasad dotyczących takiej
> sytuacji. Kiedy program kompilowany jest w trybie debugowania, Rust dołącza 
> do niego mechanizmy powodujące jego "spanikowanie" (*panic*)
> w momencie wystąpienia przekroczenia zakresu liczb całkowitych.
> Rust wykorzystuje termin 
> "panikowania" programu wtedy, gdy program kończy działaniem zwracając błąd;
> panikowanie szczegółowiej omówimy w sekcji [„Nieodwracalne błędy z `panic!`”][unrecoverable-errors-with-panic]<!-- ignore -->
> w rozdziale 9.
>
> Kiedy kompilujemy program w trybie produkcyjnym z włączoną flagą `--release`,
> Rust *nie* dołącza do programu mechanizmów wykrywających przekroczenia 
> zakresu liczb całkowitych, które spowodują spanikowanie programu. Zamiast tego 
> w przypadku wystąpienia przekroczenia zakresu, Rust wykona operację nazywaną
> *zawinięciem uzupełnia do dwóch*. Krótko mówiąc, wartości większe niż 
> maksymalna dla danego typu danych zostaną "zawinięte w koło" do mniejszych wartości, 
> odpowiednich dla danego typu danych. Na przykład w przypadku `u8`, 256 zostanie zamienione 
> na 0, 257 na 1 itd. Program nie spanikuje, ale zmiennym zostaną przypisane 
> inne wartości niż byś tego oczekiwał. Poleganie na zawinięciu uzupełnia do 
> dwóch jest uważane za błąd.
>
> Przepełnienia można obsłużyć jawnie. W tym celu można skorzystać z następujących rodzin
> metod, zapewnionych prymitywnym typom liczbowym przez bibliotekę standardową:
>
> * zawijanie we wszystkich trybach kompilacji za pomocą metod `wrapping_*`, takich jak `wrapping_add`;
> * zwracanie wartość `None` jeśli wystąpiło przepełnienie za pomocą metod `checked_*`;
> * zwracanie wartość liczbowej wraz z wartością logiczną (boolean) wskazującą, czy wystąpiło przepełnienie za pomocą metod `overflowing_*`;
> * nasycenie do minimalnych lub maksymalnych wartości za pomocą metod `saturating_*`.

<!-- #### Floating-Point Types -->
#### Typy zmiennoprzecinkowe

Rust posiada też dwa prymitywne typy danych dla *liczb zmiennoprzecinkowych*, czyli liczb posiadających część ułamkową. Typy zmiennoprzecinkowe w Ruście to: `f32` i `f64`, o rozmiarach, odpowiednio, 32 i 64 bity. Domyślnie Rust
wykorzystuje `f64`, gdyż nowoczesne procesory wykonują operacje na tym typie niemal tak szybko, jak na `f32`, a jest on bardziej precyzyjny.

Oto przykład pokazujący liczby zmiennoprzecinkowe w akcji:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

Liczby zmiennoprzecinkowe są reprezentowane zgodnie ze standardem IEEE-754.
Typ `f32` to liczba zmiennoprzecinkowa zapisana w wyżej wymienionym standardzie z pojedynczą precyzją, a `f64` -- z podwójną.

<!-- #### Numeric Operations -->
#### Operacje arytmetyczne

Rust wspiera podstawowe operacje arytmetyczne na wszystkich numerycznych typach danych: dodawanie, odejmowanie, mnożenie, dzielenia i resztę z dzielenia.
Dzielenie na typach liczb całkowitych odrzuca resztę z dzielenia, zaokrąglając w kierunku zera.
Poniższy kod przedstawia przykładowe użycie każdej z wymienionych operacji w połączeniu z instrukcją `let`:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

Każde z wyrażeń w tych instrukcjach korzysta z operatora matematycznego i jest wyliczane do pojedynczej wartości, która następnie jest przypisywana do zmiennej. Listę wszystkich operatorów obsługiwanych
przez Rusta znajdziesz w [Dodatku B][appendix_b]<!-- ignore -->.

<!-- #### The Boolean Type -->
#### Typ logiczny (Boolean)

W Ruście, podobnie jak w wielu innych językach programowania, typ Boolean może
przyjąć jedną z dwóch wartości: `true` lub `false`. Boolean ma wielkość jednego bajta.
Typ logiczny w Ruście jest deklarowany z pomocą `bool`. Na przykład:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

Jednym z głównych zastosowań typu Boolean są wyrażenia logiczne, takie jak `if`.
Działanie wyrażenia `if` w Ruście omówimy w sekcji `Kontrola przepływu`.

<!-- #### The Character Type -->
#### Typ znakowy

Do tej pory pracowaliśmy tylko z liczbami, ale Rust wspiera też litery. Najprostszym typ znakowym jest `char`. Oto przykłady jego deklaracji:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

Proszę zauważyć, że literały typu `char` są zapisywane z użyciem pojedynczego cudzysłowia, w przeciwieństwie do literałów łańcuchowych, które korzystają z podwójnego cudzysłowia.
Typ `char` w Ruście ma wielkość czterech bajtów i reprezentuje Skalarną Wartość Unikod, co oznacza, że można w nim przedstawić dużo więcej niż tylko znaki ASCII.
Litery akcentowane; chińskie, japońskie i koreańskie symbole; emoji; pola o zerowej długości to wszystko poprawne wartości dla typu `char` w Ruście.
Skalarne Wartości Unikod mieszczą się w zakresach od `U+0000` do `U+D7FF` i od `U+E000` do `U+10FFFF` włącznie. Jednak „znak” nie jest naprawdę ideą w Unikodzie, więc twój intuicyjny sposób postrzegania tego, czym jest „znak” może nie
być zgodny z tym, czym w rzeczywistości jest `char` w Ruście. Szczegółowo omówimy ten temat w ["Ciągach znaków"][strings]<!-- ignore --> w rozdziale 8.

<!-- ### Compound Types -->
### Typy złożone

*Typy złożone* mogą grupować wiele wartości w jeden typ danych. Rust posiada
dwa prymitywne typy złożone: krotki i tablice.

<!-- #### The Tuple Type -->
#### Krotki

Krotka pozwala na zgrupowanie pewnej liczby wartości o różnych typach danych w jeden złożony typ danych.
Krotka ma stałą długość. Po zadeklarowaniu nie może się powiększyć ani pomniejszyć.

Aby stworzyć krotkę, zapisujemy w nawiasie okrągłym listę wartości oddzielonych przecinkami. Każda pozycja w krotce ma pewien typ danych, przy czym wszystkie wartości nie muszą mieć tego samego typu danych. W tym przykładzie dodaliśmy
opcjonalne adnotacje typów danych:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

Zmienna `tup` odnosi się do całej krotki, gdyż krotka jest traktowana jak jeden złożony element. Aby uzyskać dostęp do wartości, które składają się na krotkę, możemy skorzystać z dopasowywania do wzorca i rozdzielić wartość krotki,
tak jak poniżej:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

Powyższy program najpierw tworzy krotkę i przypisuje ją do zmiennej `tup`.
Następnie korzysta ze wzorca w połączeniu z instrukcją `let`, aby przetransformować `tup` w trzy niezależne zmienne `x`, `y`, i `z`.
Tę operację nazywamy *destrukturyzacją*, gdyż rozdziela pojedynczą krotkę na trzy części. Na końcu, program wypisuje wartość zmiennej `y`, czyli `6.4`.

Możemy też uzyskać bezpośredni dostęp do elementu krotki, wykorzystując znak kropki (`.`) oraz indeks wartości, do której chcemy uzyskać dostęp. Na przykład:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

Powyższy program tworzy krotkę `x`, a następnie uzyskuje dostęp do jej elementów wykorzystując ich indeksy. Podobnie, jak w większości języków programowania pierwszy indeks w krotce ma wartość 0.

The tuple without any values has a special name, *unit*. This value and its corresponding type are both written `()` and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.

<!-- #### The Array Type -->
#### Tablice

Innym sposobem na stworzenie kolekcji wartości jest użycie *tablicy*.
W przeciwieństwie do krotki każdy element tablicy musi być tego samego typu. Tablice w Ruście różnią się od tablic znanych z paru innych języków programowania tym, że mają stały rozmiar. Raz zadeklarowane nie mogą zwiększyć
ani zmniejszyć swojego rozmiaru.

W Ruście, aby umieścić wartości w tablicy, zapisujemy je jako lista rozdzieloną przecinkami, wewnątrz nawiasów kwadratowych:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Tablice są przydatne, gdy chcesz umieścić dane na stosie, a nie na stercie (Stos i stertę omówimy w [rozdziale 4][stack-and-heap]<!-- ignore -->) lub gdy chcesz mieć pewność, że ilość elementów nigdy się nie zmieni. Jednak tablica nie jest tak elastyczna, jak typ wektorowy. Wektor jest podobnym typem kolekcji, dostarczanym przez bibliotekę standardową, ale *może* zwiększać i zmniejszać swój rozmiar.
Jeżeli nie jesteś pewien, czy użyć wektora, czy tablicy, prawdopodobnie powinieneś użyć wektora. [Rozdział 8][vectors]<!-- ignore --> szczegółowo opisuje wektory i ich działanie.

Jednak tablice są bardziej przydatne, gdy wiadomo, że liczba elementów nie zmieni się.
Przykładowo gdy w programie chcemy używać nazw miesięcy, lepiej przechowywać je w tablicy niż w wektorze, ponieważ wiemy, że potrzebujemy dokładnie 12 elementów:

```rust
let months = ["Styczeń", "Luty", "Marzec", "Kwiecień", "Maj", "Czerwiec", "Lipiec",
              "Sierpień", "Wrzesień", "Październik", "Listopad", "Grudzień"];
```

Typ tablicy zapisujemy używając nawiasów kwadratowych, wewnątrz których umieszczamy typ każdego z elementów, po nim średnik, a następnie liczbę elementów w tablicy, tak jak poniżej:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Powyżej `i32` to typ każdego elementu. Po średniku liczba `5` oznacza, że w tej tablicy znajdzie się pięć elementów.

By stworzyć tablicę mającą te same wartości dla każdego elementu, można podać tę wartość, po niej średnik i liczbę 
elementów, całość obejmując nawiasami kwadratowymi:

```rust
let a = [3; 5];
```

Tablica `a` będzie zawierać `5` elementów, a każdy z nich początkowo przyjmie wartość `3`.
Taki sam rezultat osiągnąłby taki zapis: `let a = [3, 3, 3, 3, 3];`, ale ten pierwszy jest krótszy.

<!-- ##### Accessing Array Elements -->
##### Uzyskiwanie Dostępu do Elementów Tablicy

Tablica to obszar pamięci ulokowany na stosie. Możesz uzyskać dostęp do elementów tablicy, korzystając z indeksowania, tak jak poniżej:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

W tym przykładzie, zmienna o nazwie `first` otrzyma wartość `1`, ponieważ taka wartość znajduje się w tablicy na miejscu o indeksie `[0]`. Zmienna o nazwie `second` otrzyma wartość `2` od pozycji w tablicy o indeksie `[1]`.

<!-- ##### Invalid Array Element Access -->
##### Próba Uzyskania Dostępu do Niepoprawnego Elementu Tablicy

Co się stanie, gdy spróbujemy uzyskać dostęp do elementu, który jest poza tablicą? Zmienimy wcześniejszy przykład na poniższy kod, który pobiera indeks tablicy od użytkownika, używając kodu podobnego do tego z gry zgadywanki z rozdziału 2:

<span class="filename">Plik: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

Nie wystąpiły żadne błędy w trakcie kompilacji. Po uruchomieniu za pomocą `cargo run` i podaniu 0, 1, 2, 3, lub 4, program wypisuje wartość z tablicy o podanym indeksie. Jeśli jednak w zamian zostanie podana liczba niebędąca
poprawnym indeksem tej tablicy, jak np. 10, pojawi się następujący komunikat:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Uruchomienie programu poskutkowało błędem *wykonania* w momencie użycia niepoprawnej wartości dla operacji indeksowania. Program zakończył działanie w tym momencie
z komunikatem o błędzie i nie wykonał końcowego `println!`. Przy próbie dostępu do elementu z wykorzystaniem indeksowania, Rust sprawdza, czy podany indeks jest mniejszy
niż długość tablicy. Jeżeli ten indeks jest większy lub równy długości tablicy, program spanikuje. To sprawdzenie musi się odbyć w czasie wykonywania, szczególnie w tym przypadku, w którym kompilator nie może wiedzieć, jaką wartość wprowadzi użytkownik uruchamiający kod.

Oto pierwszy przykład zasad bezpieczeństwa Rusta w akcji. W wielu niskopoziomowych językach programowania tego rodzaju test nie jest wykonywany, a skorzystanie z niepoprawnego indeksu może skutkować uzyskaniem dostępu do
niewłaściwego bloku pamięci. Rust chroni przed takimi błędami. Zamiast pozwolić na uzyskanie dostępu do pamięci i kontynuację działania, zamyka program. Obsługę błędów w Ruście dokładniej omówimy w rozdziale 9. Tam też pokażemy jak pisać czytelny i bezpieczny kod, który nigdy nie panikuje i nie dopuszcza do nieprawidłowych dostępów do pamięci.

[comparing-the-guess-to-the-secret-number]:ch02-00-guessing-game-tutorial.html#porwnywanie-odpowiedzi-gracza-z-sekretnym-numerem
[twos-complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[control-flow]: ch03-05-control-flow.html#control-flow
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[stack-and-heap]: ch04-01-what-is-ownership.html#the-stack-and-the-heap
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[appendix_b]: appendix-02-operators.md

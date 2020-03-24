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

Jeżeli w powyższym kodzie nie dodalibyśmy adnotacji typu danych, Rust
wyświeliłby następujący komunikat o błędzie, mówiący o tym, że kompilator
potrzebuje więcej informacji, aby określić, jakiego typu danych chcemy użyć:

```text
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

Spotkasz się z odpowiednimi zapisami dla poszczególnych typów danych.

### Typy skalarne

Typ *skalarny* reprezentuje pojedynczą wartość. Rust posiada 4 główne,
skalarne typy danych: całkowity, zmiennoprzecinkowy, logiczny (Boolean)
i znakowy. Możesz kojarzyć je z innych języków programowania. Zobaczmy
jak działają w Ruście.

#### Typy całkowite

*Liczba całkowita* to liczba nieposiadająca części ułamkowej.
Wykorzystywaliśmy jeden z typów całkowitych — `u32` — w rozdziale 2. Ten typ
danych określa, że wartość, do której się odnosi, jest liczbą całkowitą bez
znaku (typy całkowite ze znakiem zaczynają się od `i` zamiast `u`), która
zajmuje 32 bity pamięci. Tabela 3-1 pokazuje typy całkowite wbudowane w Rusta.
Każdy z wariantów w kolumnach Ze znakiem i Bez znaku (na przykład `i16`) może
zostać użyty do zadeklarowania typu danych liczby całkowitej.

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
Nazwy *Ze znakiem* i *Bez znaku* odnoszą się do tego, czy dana liczba może
być ujemna, czy tylko dodatnia — inaczej mówiąc, czy liczba musi posiadać znak
(Ze znakiem), czy też nie, gdyż wiadomo, że zawsze będzie dodatnia (Bez znaku).
Można to porównać do zapisywania liczb na kartce, gdy znak ma znaczenie,
zapisujemy go — odpowiednio plus lub minus przed liczbą, ale gdy liczba jest
dodatnia i w danym kontekście nie jest to konieczne, pomijamy znak. Liczby
całkowite ze znakiem przechowywane są z pomocą *[Kodu uzupełnień do dwóch](https://en.wikipedia.org/wiki/Two%27s_complement)
(jeżeli nie jesteś pewien/pewna, co to oznacza, możesz poszukać
informacji w internecie; wyjaśnienie jest poza zakresem materiału zawartego
w tej książce).

Każdy wariant ze znakiem może przechowywać liczby od -(2<sup>n - 1</sup>) do
2<sup>n - 1</sup> - 1 włącznie, gdzie *n* to liczba bitów, które wykorzystuje
dany wariant. Tak więc `i8` może przechowywać liczby od -(2<sup>7</sup>) do
2<sup>7</sup> - 1, co daje zakres od -128 do 127. Warianty bez znaku mogą
przechowywać liczby od 0 do 2<sup>n</sup> - 1, więc `u8` może przechowywać
liczby od 0 do 2<sup>8</sup> - 1, co daje zakres od 0 do 255.

Dodatkowo typ `isize` oraz `usize` dopasowują swój rozmiar do architektury
komputera, na którym uruchamiasz swój program: 64 bity na komputerze
o 64-bitowej architekturze i 32 bity na komputerze o 32-bitowej architekturze.

Możesz zapisywać literały liczb całkowitych w każdej z form uwzględnionych
w Tabeli 3-2. Zauważ, że wszystkie literały, poza bajtem, pozwalają na użycie
przyrostka, np. `57u8` i wizualnego separatora `_`, np. `1_000`.

<span class="caption">Tabela 3-2: Literały liczb całkowitych w Ruście</span>

| Literały liczbowe | Przykład      |
| ----------------- | ------------- |
| Dziesiętny        | `98_222`      |
| Szesnastkowy      | `0xff`        |
| Ósemkowy          | `0o77`        |
| Binarny           | `0b1111_0000` |
| Bajt (tylko `u8`) | `b'A'`        |

W takim razie skąd masz wiedzieć, którego typu całkowitego użyć? Jeżeli nie
jesteś pewien/pewna, typy domyślnie wykorzystywane przez Rusta są w większości
przypadków dobrym wyborem, dla liczb całkowitych to `i32`; ogólnie ten typ
jest najszybszy, nawet na 64-bitowych systemach. Z typów `isize` i `usize`
skorzystasz głównie przy indeksowaniu różnego rodzaju kolekcji danych.

> ##### Przekroczenie zakresu liczb całkowitych
>
> Załóż, że masz zmienną typy `u8`, która może przechowywać wartości
> między 0 i 255. Jeżeli spróbujesz przypisać tej zmiennej wartość nie
> mieszczącą się w podanym zakresie, np. 256, nastąpi przekroczenie zakresu
> liczb całkowitych. Rust posiada kilka ciekawych zasad dotyczących takiego
> zachowania. Kiedy kompilujesz program w trybie debugowania, Rust dołącza 
> do programu mechanizmy wykrywające przekroczenia zakresu liczb całkowitych,
> które w przypadku wystąpienia przekroczenia zakresu liczb całkowitych 
> zmuszą twój program do spanikowania (*panic*). Rust wykorzystuje termin 
> "panikowania" programu wtedy, gdy program kończy działaniem zwracając błąd;
> panikowanie szczegółowiej omówimy w sekcji [“Nieodwracalne błędy z `panic!`”][unrecoverable-errors-with-panic]<!-- ignore -->
> w rozdziale 9.
>
> Kiedy kompilujesz program w trybie produkcyjnym z włączoną flagą `--release`,
> Rust *nie* dołącza do programu mechanizmów wykrywających przekroczenia 
> zakresu liczb całkowitych, które spowodują spanikowanie programu. Zamiast tego 
> w przypadku wystąpienia przekroczenia zakresu, Rust wykona operację nazywaną
> *zwinięciem uzupełnia do dwóch*. Krótko mówiąc, wartości większe niż 
> maksymalna dla danego typu danych zostaną "zwinięte" do najmniejszych wartości 
> dla danego typu danych. Na przykład w przypadku `u8`, 256 zostanie zamienione 
> na 0, 257 na 1 itd. Program nie spanikuje, ale zmiennym zostaną przypisane 
> inne wartości niż byś tego oczekiwał. Poleganie na zwinięciu uzupełnia do 
> dwóch jest uważane za błąd. Jeżeli chcesz jawnie zwijać, może skorzystać 
> z typu danych z biblioteki standardowej o nazwie [`Wrapping`][wrapping].

#### Typy zmiennoprzecinkowe

Rust posiada też dwa prymitywne typy danych dla *liczb zmiennoprzecinkowych*,
czyli liczb posiadających część ułamkową. Typy zmiennoprzecinkowe w Ruście
to: `f32` i `f64`, czyli o rozmiarach odpowiednio 32 i 64 bitów. Domyślnie Rust
wykorzystuje `f64`, gdyż nowoczesne procesory wykonują operacje na tym typie
niemal tak szybko, jak na `f32`, a jest on bardziej precyzyjny.

Oto przykład pokazujący liczby zmiennoprzecinkowe w akcji:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

Liczby zmiennoprzecinkowe są reprezentowane zgodnie ze standardem IEEE-754.
Typ `f32` to liczba zmiennoprzecinkowa zapisana w wyżej wymienionym standardzie
z pojedynczą precyzją, a `f64` — z podwójną.

#### Operacje arytmetyczne

Rust wspiera podstawowe operacje arytmetyczne na wszystkich, numerycznych
typach danych: dodawanie, odejmowanie, mnożenie, dzielenia i resztę z dzielenia.
Poniższy kod przedstawia przykładowe użycie każdej z wymienionych operacji
w połączeniu z instrukcją `let`:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

Każde z wyrażeń w tych instrukcjach korzysta z operatora matematycznego
i jest konwertowane (ewaluowane) do pojedynczej wartości, która następnie
jest przypisywana do zmiennej. Listę wszystkich operatorów obsługiwanych
przez Rusta znajdziesz w Dodatku B.

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

#### Typ znakowy

Do tej pory pracowaliśmy tylko z liczbami, ale Rust wspiera też litery. Typ
danych `char` jest najprostszym typem znakowym zaimplementowanym w Ruście.
Poniższy kod pokazuje jeden ze sposobów jego użycia. (Zauważ, że typ `char`
jest zapisywany z użyciem pojedynczego cudzysłowia, w przeciwieństwie
do ciągów znaków, które korzystają z podwójnego cudzysłowia)

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

Typ `char` w Ruście ma wielkość czterech bajtów i reprezentuje Skalarną Wartość Unikod,
co oznacza, że można w nim przedstawić dużo więcej niż tylko znaki ASCII.
Litery akcentowane; Chińskie, japońskie i koreańskie symbole; emoji;
pola o zerowej długości to wszystko poprawne wartości dla typu `char` w Ruście.
Skalarne Wartości Unikod mieszczą się w zakresach od `U+0000` do `U+D7FF` i od `U+E000` do
`U+10FFFF` włącznie. Jednak “znak” nie jest na prawdę ideą w Unikodzie,
więc twój intuicyjny sposób postrzegania tego, czym jest “znak” może nie
być zgodny z tym, czym w rzeczywistości jest `char` w Ruście. Szczegółowo
omówimy ten temat w ["Ciągach znaków"][strings]<!-- ignore --> w rozdziale 8.

### Typy złożone

*Typy złożone* mogą grupować wiele wartości w jeden typ danych. Rust posiada
dwa prymitywne typy złożone: krotki i tablice.

#### Krotka

Krotka pozwala na zgrupowanie pewnej liczby wartości o różnych typach danych
w jeden złożony typ danych.

Aby stworzyć krotkę, zapisujemy w nawiasie okrągłym, listę wartości oddzielonych
przecinkami. Każda pozycja w krotce ma pewien typ danych, przy czym wszystkie
wartości nie muszą mieć tego samego typu danych. W tym przykładzie dodaliśmy
opcjonalne adnotacje typów danych:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

Zmienna `tup` odnosi się do całej krotki, gdyż krotka jest traktowana jak
jeden złożony element. Aby uzyskać dostęp do wartości, które składają się na
krotkę, możemy skorzystać z dopasowywania do wzorca i rozdzielić wartość krotki,
tak jak poniżej:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

Powyższy program najpierw tworzy krotkę i przypisuje ją do zmiennej `tup`.
Następnie korzysta ze wzorca w połączeniu z instrukcją `let`, aby
przetransformować `tup` w trzy niezależne zmienne `x`, `y`, i `z`.
Tę operację nazywamy *destrukcją*, gdyż rozdziela pojedynczą krotkę na trzy
części. Na końcu, program wypisuje wartość zmiennej `y`, czyli `6.4`.

Oprócz rozkładania krotki z użyciem dopasowywania do wzorca możemy uzyskać
bezpośredni dostęp do elementu krotki, wykorzystując znak kropki (`.`) oraz
indeks wartości, do której chcemy uzyskać dostęp.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

Powyższy program tworzy krotkę `x`, a następnie — po jednej zmiennej dla każdego
elementu zawartego w krotce, wykorzystując przy tym ich indeksy. Podobnie, jak
w większości języków programowania pierwszy indeks w krotce ma wartość 0.

#### Typ tablicowy

Innnym sposobem na stworzenie kolekcji wartości jest użycie *tablicy*.
W przeciwieństwie do krotki każdy element tablicy musi mieć ten sam typ
danych. Tablice w Ruście różnią się, od tablic znanych z paru innych języków
programowania, tym że mają stały rozmiar; raz zadeklarowane nie mogą zwiększyć,
ani zmniejszyć swojego rozmiaru.

W Ruście, aby umieścić wartości w tablicy, zapisujemy je jako lista rozdzieloną
przecinkami, wewnątrz nawiasów kwadratowych:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Tablice są przydatne, gdy chcesz umieścić dane na stosie, a nie na stercie
(Stos i stertę omówimy w rozdziale 4) lub gdy chcesz mieć pewność, że ilość
elementów nigdy się nie zmieni. Jednak tablica nie jest tak elastyczna,
jak typ wektorowy. Wektor jest podobnym typem kolekcji, dostarczanym przez
bibliotekę standardową, ale *może* zwiększać i zmniejszać swój rozmiar.
Jeżeli nie jesteś pewien/pewna czy użyć wektora, czy tablicy, prawdopodobnie
powinieneś/powinnaś użyć wektora. Rozdział 8 szczegółowo opisuje wektory i ich działanie.

Przykładowa sytuacja, w której lepiej użyć tablicy niz wektora ma miejsce,
gdy nasz program potrzebuje znać nazwy wszystkich miesięcy. Jest bardzo mało
prawdopodobne, by trzeba było dodać lub usunąć miesiąc, więc możemy
skorzystać z tablicy, ponieważ wiemy, że zawsze będzie zawierać 12 pozycji.

```rust
let months = ["Styczeń", "Luty", "Marzec", "Kwiecień", "Maj", "Czerwiec", "Lipiec",
              "Sierpień", "Wrzesień", "Październik", "Listopad", "Grudzień"];
```

Typ tablicy zapisać możesz używając nawiasów kwadratowych, wewnątrz których
umieszczone zostaną typy każdego elementu, po nim średnik, a następnie ilość elementów
w tablicy, tak jak poniżej:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Powyżej `i32` to typ każdego elementu. Po średniku liczba `5` oznacza, że w tej
tablicy znajdzie się pięć elementów.

Zapisywanie typu tablicy w ten sposób może wyglądać podobnie do składni alternatywnej
składni inicjalizacji tablicy: jeśli chcesz stworzyć tablicę mającą te same wartości
dla każdego elementu, możesz zdefiniować tą wartość, a po niej średnik oraz ilość 
elementów - to wszystko w nawiasach kwadratowych, jak pokazano poniżej:

```rust
let a = [3; 5];
```

Tablica `a` będzie zawierać `5` elementów, a każdy z nich początkowo przyjmie wartość `3`.
Taki sam rezultat osiągnąłby taki zapis: `let a = [3, 3, 3, 3, 3];`, ale ten pierwszy jest krótszy.

##### Uzyskiwanie dostępu do elementów tablicy

Tablica to obszar pamięci ulokowany na stosie. Możesz uzyskać dostęp
do elementów tablicy, korzystając z indeksowania, tak jak poniżej:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

W tym przykładzie, zmienna o nazwie `first` otrzyma wartość `1`, ponieważ taka
wartość znajduje się w tablicy na miejscu o indeksie `[0]`. Zmienna o nazwie
`second` otrzyma wartość `2` od pozycji w tablicy o indeksie `[1]`.

##### Próba uzyskania dostępu do niepoprawnego elementu tablicy

Co się stanie, jeśli spróbujesz uzyskać dostęp do elementu, który jest poza
tablicą? Powiedzmy, że zmienisz wcześniejszy przykład na poniższy kod, który
poprawnie skompiluje się, ale próba uruchomienia, zakończy się błędem:

<span class="filename">Plik: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

Rezultatem uruchomienia tego kodu przy pomocy `cargo run` będzie:

```text
{{#include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/output.txt}}
```

Nie wystąpiły żadne błędy w trakcie kompilacji, ale uruchomienie programu
poskutkowało błędem *uruchomieniowym* i nie zakończyło się sukcesem. Kiedy
próbujesz uzyskać dostęp do elementu wykorzystając indeksowanie, Rust sprawdza,
czy indeks, który zapisałeś, jest mniejszy niż długość tablicy. Jeżeli ten
indeks jest większy lub równy długości tablicy, Rust spanikuje.

Oto pierwszy przykład zasad bezpieczeństwa Rusta w akcji. W wielu
niskopoziomowych językach programowania tego rodzaju test nie jest wykonywany,
a skorzystanie z niepoprawnego indeksu, może skutkować uzyskaniem dostępu do
niewłaściwego bloku pamięci. Rust chroni cię przed takimi błędami. Zamiast
pozwolić ci na uzyskanie dostępu do pamięci i kontynuację działania, zamyka
program. W rozdziale 9 szczegółowiej omówiono obługę błędów w Ruście.

[comparing-the-guess-to-the-secret-number]:ch02-00-guessing-game-tutorial.html#porwnywanie-odpowiedzi-gracza-z-sekretnym-numerem
[control-flow]: ch03-05-control-flow.html#control-flow
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[wrapping]: ../std/num/struct.Wrapping.html

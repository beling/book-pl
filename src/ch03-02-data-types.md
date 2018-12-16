## Typy danych

Każda wartość w Ruście ma pewien *typ danych*, dzięki czemu Rust wie, z jakim
rodzajem danych ma do czynienia i jak z nimi pracować. Przyjrzymy się bliżej
dwóm grupom typów danych: skalarnym i złożonym.

Pamiętaj, że Rust jest językiem statycznie typowanym (*statically typed*),
co oznacza, że podczas kompilacji musi znać typy danych wszystkich zmiennych
obecnych w kodzie. Zazwyczaj kompilator może wywnioskować typ danych, którego
chcemy użyć na podstawie użytej wartość i sposobu jej wykorzystywania.
W przypadku gdy wiele typów danych spełnia dane założenia, przykładowo gdy
w Rozdziale 2 w sekcji "Porównywanie odpowiedzi gracza z sekretnym numerem"
konwertowaliśmy `String` do typu numerycznego wykorzystując funkcję `parse`
musimy dodać adnotację typu danych:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

Jeżeli w powyższym kodzie nie dodalibyśmy adnotacji typu danych, Rust
wyświeliłby następujący komunikat o błędzie, mówiący o tym, że kompilator
potrzebuje więcej informacji, aby określić, jakiego typu danych chcemy użyć:

```text
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^
  |         |
  |         cannot infer type for `_`
  |         consider giving `guess` a type
```

Spotkasz się z odpowiednimi zapisami dla poszczególnych typów danych.

### Typy skalarne

Typ *skalarny* reprezentuje pojedynczą wartość. Rust posiada 4 główne,
skalarne typy danych: całkowity, zmiennoprzecinkowy, logiczny (Boolean)
i znakowy. Możesz kojarzyć je z innych języków programowania. Zobaczmy
jak działają w Ruście.

#### Typy całkowite

*Liczba całkowita* to liczba nieposiadająca części ułamkowej.
Wykorzystywaliśmy jeden z typów całkowitych — `u32` — w Rozdziale 2. Ten typ
danych określa, że wartość, do której się odnosi, jest liczbą całkowitą bez
znaku (typy całkowite ze znakiem zaczynają się od `i` zamiast `u`), która
zajmuje 32 bity pamięci. Tabela 3-1 pokazuje typy całkowite wbudowane w Rusta.
Każdy z wariantów w kolumnach Ze znakiem i Bez znaku (na przykład `i16`) może
zostać użyty do zadeklarowania typu danych liczby całkowitej.

<span class="caption">Tabela 3-1: Typy całkowite w Ruście</span>

| Rozmiar  | Ze znakiem | Bez znaku |
| -------- | ---------- | --------- |
| 8-bitów  | i8         | u8        |
| 16-bitów | i16        | u16       |
| 32-bity  | i32        | u32       |
| 64-bity  | i64        | u64       |
| arch     | isize      | usize     |

Każdy z wariantów może posiadać znak lub nie, a także ma określony rozmiar.
Nazwy *Ze znakiem* i *Bez znaku* odnoszą się do tego, czy dana liczba może
być ujemna, czy tylko dodatnia — inaczej mówiąc, czy liczba musi posiadać znak
(Ze znakiem), czy też nie, gdyż wiadomo, że zawsze będzie dodatnia (Bez znaku).
Można to porównać do zapisywania liczb na kartce, gdy znak ma znaczenie,
zapisujemy go — odpowiednio plus lub minus przed liczbą, ale gdy liczba jest
dodatnia i w danym kontekście nie jest to konieczne, pomijamy znak. Liczby
całkowite ze znakiem przechowywane są z pomocą dwóch uzupełniających się
reprezentacji (jeżeli nie jesteś pewien, co to oznacza, możesz poszukać
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
jesteś pewien, typy domyślnie wykorzystywane przez Rusta są w większości
przypadków dobrym wyborem, dla liczb całkowitych to `i32`; ogólnie ten typ
jest najszybszy, nawet na 64-bitowych systemach. Z typów `isize` i `usize`
skorzystasz głównie przy indeksowaniu różnego rodzaju kolekcji danych.

#### Typy zmiennoprzecinkowe

Rust posiada też dwa prymitywne typy danych dla *liczb zmiennoprzecinkowych*,
czyli liczb posiadających część ułamkową. Typy zmiennoprzecinkowe w Ruście
to: `f32` i `f64`, czyli o rozmiarach odpowiednio 32 i 64 bitów. Domyślnie Rust
wykorzystuje `f64`, gdyż nowoczesne procesory wykonują operacje na tym typie
niemal tak szybko, jak na `f32`, a jest on bardziej precyzyjny.

Oto przykład pokazujący liczby zmiennoprzecinkowe w akcji:

<span class="filename">Plik: src/main.rs</span>

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
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
fn main() {
    // dodawanie
    let sum = 5 + 10;

    // odejmowanie
    let difference = 95.5 - 4.3;

    // mnożenie
    let product = 4 * 30;

    // dzielenie
    let quotient = 56.7 / 32.2;

    // reszta z dzielenia
    let remainder = 43 % 5;
}
```

Każde z wyrażeń w tych instrukcjach korzysta z operatora matematycznego
i jest konwertowane (ewaluowane) do pojedynczej wartości, która następnie
jest przypisywana do zmiennej. Listę wszystkich operatorów obsługiwanych
przez Rusta znajdziesz w Dodatku B.

#### Typ logiczny (Boolean)

W Ruście, podobnie jak w wielu innych językach programowania, typ Boolean może
przyjąć jedną z dwóch wartości: `true` lub `false`. Typ logiczny w Ruście jest
deklarowany z pomocą `bool`. Na przykład:

<span class="filename">Plik: src/main.rs</span>

```rust
fn main() {
    let t = true;

    let f: bool = false; // z jawną adnotacją typu danych
}
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
fn main() {
   let c = 'z';
   let z = 'ℤ';
   let heart_eyed_cat = '😻';
}
```

Typ `char` w Ruście reprezentuje Skalarną Wartość Unikod, co oznacza, że
można w nim przedstawić dużo więcej niż tylko znaki ASCII. Litery akcentowane;
Chińskie, japońskie i koreańskie symbole; emoji; pola o zerowej długości
to wszystko poprawne wartości dla typu `char` w Ruście. Skalarne Wartości
Unikod mieszczą się w zakresach od `U+0000` do `U+D7FF` i od `U+E000` do
`U+10FFFF` włącznie. Jednak “znak” nie jest na prawdę ideą w Unikodzie,
więc twój intuicyjny sposób postrzegania tego, czym jest “znak” może nie
być zgodny z tym, czym w rzeczywistości jest `char` w Ruście. Szczegółowo
omówimy ten temat w "Ciągach znaków" w Rozdziale 8.

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
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

Zmienna `tup` odnosi się do całej krotki, gdyż krotka jest traktowana jak
jeden złożony element. Aby uzyskać dostęp do wartości, które składają się na
krotkę, możemy skorzystać z dopasowywania do wzorca i rozdzielić wartość krotki,
tak jak poniżej:

<span class="filename">Plik: src/main.rs</span>

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
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
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
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
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Tablice są przydatne, gdy chcesz umieścić dane na stosie, a nie na stercie
(Stos i stertę omówimy w Rozdziale 4) lub gdy chcesz mieć pewność, że ilość
elementów nigdy się nie zmieni. Jednak tablica nie jest tak elastyczna,
jak typ wektorowy. Wektor jest podobnym typem kolekcji, dostarczanym przez
bibliotekę standardową, ale *może* zwiększać i zmniejszać swój rozmiar.
Jeżeli nie jesteś pewien czy użyć wektora, czy tablicy, prawdopodobnie
powinieneś użyć wektora. Rozdział 8 szczegółowo opisuje wektory i ich działanie.

Przykładowa sytuacja, w której lepiej użyć tablicy niz wektora ma miejsce,
gdy nasz program potrzebuje znać nazwy wszystkich miesięcy. Jest bardzo mało
prawdopodobne, by trzeba było dodać lub usunąć miesiąc, więc możemy
skorzystać z tablicy, ponieważ wiemy, że zawsze będzie zawierać 12 pozycji.

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

##### Uzyskiwanie dostępu do elementów Tablicy

Tablica to obszar pamięci ulokowany na stosie. Możesz uzyskać dostęp
do elementów tablicy, korzystając z indeksowania, tak jak poniżej:

<span class="filename">Plik: src/main.rs</span>

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

W tym przykładzie, zmienna o nazwie `first` otrzyma wartość `1`, ponieważ taka
wartość znajduje się w tablicy na miejscu o indeksie `[0]`. Zmienna o nazwie
`second` otrzyma wartość `2` od pozycji w tablicy o indeksie `[1]`.

##### Próba uzyskania dostępu do niepoprawnego elementu Tablicy

Co się stanie, jeśli spróbujesz uzyskać dostęp do elementu, który jest poza
tablicą? Powiedzmy, że zmienisz wcześniejszy przykład na poniższy kod, który
poprawnie skompiluje się, ale próba uruchomienia, zakończy się błędem:

<span class="filename">Plik: src/main.rs</span>

```rust,ignore
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
```

Rezultatem uruchomienia tego kodu przy pomocy `cargo run` będzie:

```text
$ cargo run
   Compiling arrays v0.1.0 (file:///projects/arrays)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/arrays`
thread '<main>' panicked at 'index out of bounds: the len is 5 but the index is
 10', src/main.rs:6
note: Run with `RUST_BACKTRACE=1` for a backtrace.
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
program. W Rozdziale 9 szczegółowiej omówiono obługę błędów w Ruście.

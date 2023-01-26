## Wycinek

Kolejnym typem danych, który nie przejmuje własności jest *wycinek* (ang. *slice*).
Wycinki pozwalają na odniesienie się do wybranej ciągłej sekwencji elementów w kolekcji, bez konieczności odnoszenia się do całej kolekcji.

Rozważmy mały problem programistyczny: napisać funkcję, która pobiera łańcuch znaków zawierający słowa rozdzielone spacjami i zwraca pierwsze słowo, które się w nim znajdzie. Jeśli funkcja nie znajdzie znaku spacji w łańcuchu, należy założyć, że cały łańcuch stanowi jedno słowo i zwrócić go w całości.

Pomyślmy nad sygnaturą tej funkcji (na razie bez użycia wycinków, by zrozumieć problem, który one rozwiązują):

```rust,ignore
fn first_word(s: &String) -> ?
```

Funkcja `first_word` przyjmuje parametr typu `&String`, co jest w porządku, bo funkcja ta nie potrzebuje tego łańcucha na własność. Ale jakiego typu wynik powinna zwrócić? Naprawdę brakuje nam sposobu na mówienie o *części* łańcucha. Możemy jednak zwrócić indeks końca słowa wskazanego przez spację. Próbujmy tego dokonać na Listingu 4-7.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:here}}
```

<span class="caption">Listing 4-7: Funkcja `first_word` która zwraca indeks bajta w parametrze typu `String`</span>

By przejść przez `String` element po elemencie i spróbować znaleźć spacje, konwertujemy nasz `String` na tablicę bajtów używając metody `as_bytes`.

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:as_bytes}}
```

Następnie tworzymy iterator po tablicy bajtów za pomocą metody `iter`:

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:iter}}
```

Iteratory omówimy szczegółowo w [rozdziale 13][ch13]<!-- ignore -->.
Na razie wiedzmy, że `iter` jest metodą, która daje nam każdy element w kolekcji, zaś `enumerate` opakowuje wynik `iter` i daje każdy element jako część krotki.
Pierwszy element tej krotki to indeks, a drugi element to referencja do elementu z kolekcji.
Użycie `enumerate` jest wygodniejsze od samodzielnego obliczanie indeksu.

Metoda `enumerate` daje krotkę, którą destrukturyzujemy za pomocą wzorca.
Wzorce omówimy szczegółowo w [rozdziale 6][ch6]<!-- ignore -->.
W pętli `for` używamy wzorca dopasowanego do krotki, w którym `i` jest dopasowane do zawartego w niej indeksu, zaś `&item` do bajtu.
Ponieważ z `.iter().enumerate()` otrzymujemy referencję do elementu (bajtu), we wzorcu używamy `&`.

Wewnątrz pętli `for` szukamy bajtu będącego spacją poprzez porównywanie do reprezentującego go literału. Gdy znajdziemy spację, zwracamy jej pozycję.
W przeciwnym razie zwracamy długość łańcucha otrzymaną za pomocą `s.len()`.

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:inside_for}}
```

Mamy teraz wprawdzie sposób na znalezienie indeksu końca pierwszego słowa, ale nie jest on wolny od wad.
Zwracamy osobną liczbę typu `usize`, która ma jednak znaczenie jedynie w kontekście `&String`.
Innymi słowy, ponieważ jest to wartość niezależna od naszego `String`a, to nie ma gwarancji, że w przyszłości zachowa ona ważność. Rozważmy program z Listingu 4-8, który używa funkcji `first_word` z Listingu 4-7.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-08/src/main.rs:here}}
```

<span class="caption">Listing 4-8: Zachowanie wyniku zwróconego przez funkcję
`first_word` i zmiana wartości `String`a</span>

Ten program kompiluje się bez błędów i zrobiłby to również, gdybyśmy użyli zmiennej `word` po wywołaniu `s.clear()`.
Ponieważ `word` nie jest w ogóle związany ze stanem `s`, `word` nadal zawiera wartość `5`.
Moglibyśmy spróbować użyć tej wartości `5` aby wyodrębnić pierwsze słowo z `s`, ale byłby to błąd, ponieważ zawartość `s` zmieniła się od czasu gdy zapisaliśmy `5` w `word`.

Martwienie się o to, że indeks w `word` przestanie być zsynchronizowany z danymi w `s` jest uciążliwe i podatne na błędy!
Zarządzanie tymi indeksami byłoby jeszcze bardziej uciążliwe, gdybyśmy chcieli napisać funkcję `second_word` (z ang. drugie słowo). Jej sygnatura musiałaby wyglądać tak:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

Teraz śledzimy indeks początkowy *i* końcowy.
Jest więc więcej wartości, które zostały obliczone na podstawie danych w określonym stanie, ale nie są w ogóle związane z tym stanem.
Mamy trzy niepowiązane zmienne wymagające synchronizacji.

Na szczęście Rust ma rozwiązanie tego problemu: wycinki łańcuchów (ang. string slices).

### Wycinki Łańcuchów

*Wycinek łańcucha* (ang. *string slice*) jest referencją do części `String`a i wygląda tak:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-17-slice/src/main.rs:here}}
```

Zamiast referencji do całego `String`a, `hello` jest referencją do jego części, opisanej fragmentem `[0..5]`.
Wycinek tworzymy podając zakres w nawiasach `[indeks_początkowy..indeks_końcowy]` i obejmuje on indeksy od `indeks_początkowy` **włącznie** do  `indeks_końcowy` **wyłącznie**.
Wewnętrznie, struktura danych wycinka przechowuje indeks jego początku i jego długość, która jest równa `indeks_końcowy` minus `indeks_początkowy`.
Więc w przypadku `let world = &s[6..11];`, `world` jest wycinkiem składającym się ze wskaźnik do bajtu o indeksie 6 w `s` i z długości `5`.

Rysunek 4-6 pokazuje to w formie diagramu.

<img alt="Three tables: a table representing the stack data of s, which points
to the byte at index 0 in a table of the string data &quot;hello world&quot; on
the heap. The third table rep-resents the stack data of the slice world, which
has a length value of 5 and points to byte 6 of the heap data table."
src="img/trpl04-06.svg" class="center" style="width: 50%;" />

<span class="caption">Rysunek 4-6: Wycinek łańcucha wskazujący część `String`a</span>

Jeżeli indeksem początkowym jest `0` to można je opcjonalnie pominąć.
Innymi słowy, dwa wycinki podane poniżej są takie same:

```rust
let s = String::from("witaj");

let slice = &s[0..2];
let slice = &s[..2];
```

Analogicznie, jeśli wycinek zawiera ostatni bajt `String`a, to można zrezygnować z podania indeksu końcowego.
Dwa wycinki podane poniżej także są sobie równoważne:

```rust
let s = String::from("witaj");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

Można również pominąć oba indeksy, aby uzyskać wycinek obejmujący cały łańcuch.
Następujące wycinki także są sobie równoważne:

```rust
let s = String::from("witaj");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

> Uwaga: Indeksy zakresu wycinka łańcucha muszą znajdować się na granicach znaków UTF-8.
> Próba utworzenia wycinka w środku wielobajtowego znaku spowoduje zakończenie programu z błędem.
> We wprowadzeniu do wycinków łańcuchów zawartym w niniejszym rozdziale
> ograniczamy się jedynie do znaków ASCII,
> zaś bardziej szczegółowe omówienie obsługi UTF-8 znajduje się w sekcji
> ["Storing UTF-8 Encoded Text with Strings"][strings]<!-- ignoruj -->rozdziału 8.

Mając na uwadze powyższe informacje, przepiszmy `first_word` tak, aby zwracał wycinek.
Typ oznaczający "wycinek łańcucha" zapisujemy jako `&str`:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-18-first-word-slice/src/main.rs:here}}
```

Indeks końca słowa otrzymujemy w taki sam sposób, jak na Listingu 4-7, czyli odnajdując pierwsze wystąpienie spacji.
Gdy znajdziemy spację, zwracamy wycinek łańcucha używając początku łańcucha i indeksu spacji jako odpowiednio indeksu początkowego i końcowego.

Teraz, gdy wywołujemy `first_word`, otrzymujemy w wyniku pojedynczą wartość, związaną z danymi wejściowymi. Wartość ta składa się z referencji do punktu początkowego wycinka i liczby elementów w wycinku.

Zwrócenie wycinka zadziałałoby również w przypadku funkcji `second_word`:

```rust,ignore
fn second_word(s: &String) -> &str {
```

Mamy teraz proste API, które jest znacznie odporniejsze na błędy, ponieważ kompilator zapewni, że referencje do `String`a pozostaną ważne.
Proszę przypomnieć sobie błąd w programie z Listingu 4-8, kiedy uzyskaliśmy indeks do końca pierwszego słowa, ale potem wyczyściliśmy łańcuch i nasz indeks utracił ważność. Tamten kod był logicznie niepoprawny, ale jego błędy początkowo się nie ujawniały. Problemy ujawniłyby się dopiero gdybyśmy spróbowali użyć indeksu pierwszego słowa z wyczyszczonym łańcuchem. Wycinki zapobiegają podobnym błędom i dają nam znać, że mamy problem z naszym kodem znacznie wcześniej. Funkcja `first_word` używająca wycinków obnaża wspomniane błędy już podczas kompilacji:

<span class="filename">Plik: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/src/main.rs:here}}
```

Oto błąd kompilatora:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/output.txt}}
```

Proszę sobie przypomnieć, że jedna z reguł pożyczania mówi, że jeśli mamy do czegoś niemutowalną referencję, to nie możemy równocześnie mieć do tego referencji mutowalnej. Ponieważ `clear` skraca `String`a, to musi przyjąć go poprzez mutowalną referencję.
Makro `println!` używa referencji do `word` po wywołaniu `clear`, więc niemutowalna referencja musi być nadal aktywna.
Rust nie pozwala, aby mutowalna referencja w `clear` i niemutowalna referencja w `word` istniały w tym samym czasie i dlatego kompilacja kończy się niepowodzeniem. Rust nie tylko uczynił nasze API łatwiejszym w użyciu, ale także sprawił, że cała klasa błędów zostanie wykryta już w czasie kompilacji!

<!-- Old heading. Do not remove or links may break. -->
<a id="string-literals-are-slices"></a>

#### Literały Łańcuchowe jako Wycinki

Przypomnijmy, że mówiliśmy o literałach łańcuchowych przechowywanych wewnątrz binarki. Zaś teraz, mając wiedzę o wycinkach, możemy właściwie zrozumieć literały łańcuchowe:

```rust
let s = "Witaj, świecie!";
```

Typem `s` jest tutaj `&str`: jest to wycinek wskazujący na konkretny punkt w binarce.
Dlatego też literały łańcuchowe nie są modyfikowalne; `&str` jest referencją niemutowalną.

#### Wycinki Łańcuchów jako Parametry

Wiedza, że wycinki można uzyskać zarówno z literałów jak i wartości `String` prowadzi do jeszcze jednego ulepszenia `first_word`, którego można dokonać w jego sygnaturze:

```rust,ignore
fn first_word(s: &String) -> &str {
```

Jednak bardziej doświadczony Rustowiec dokonałby kolejnej zmiany i w zamian napisałby sygnaturę pokazaną na Listingu 4-9, która może być używana zarówno z parametrem typu `&String`, jak i `&str`.

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:here}}
```

<span class="caption">Listing 4-9: Ulepszenie funkcji `first_word` poprzez użycie wycinka łańcucha jako typu parametru `s`</span>

Jeśli mamy wycinek łańcucha, to możemy go przekazać bezpośrednio.
Jeśli mamy `String`a, to możemy przekazać wycinek tego `String`a lub referencję do tego `String`a.
Ta elastyczność wykorzystuje *deref coercions*, własność, którą omówimy w sekcji ["Implicit Deref Coercions with Functions and Methods"][deref-coercions]<!--ignore--> rozdziału 15.

Zdefiniowanie funkcji przyjmującej wycinek łańcucha zamiast referencji do `String`a czyni nasze API bardziej ogólnym i użytecznym bez utraty jakiejkolwiek funkcjonalności:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:usage}}
```

### Inne Wycinki

Wycinki łańcuchów są oczywiście specyficzne dla łańcuchów.
Istnieje jednak bardziej ogólny typ wycinka.
Rozważmy tablicę:

```rust
let a = [1, 2, 3, 4, 5];
```

Tak samo jak możemy chcieć odwołać się do części łańcucha, możemy też chcieć odwołać się do części tablicy.
Robimy to w ten sposób:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

Ten wycinek ma typ `&[i32]`. Działa w taki sam sposób, jak wycinki łańcuchów, przechowując referencję do pierwszego elementu i długość.
Używamy tego typu wycinków do wszelkiego rodzaju pozostałych kolekcji.
Kolekcje te omówimy szczegółowo, gdy będziemy rozmawiać o wektorach w rozdziale 8.

## Podsumowanie

Koncepcje własności, pożyczania i wycinków zapewniają bezpieczeństwo pamięci w programach Rusta w czasie kompilacji.
Język Rust daje taką samą kontrolę nad wykorzystaniem pamięci jak inne języki programowania systemowego.
Ale posiadanie właściciela danych automatycznie zwalniającego je gdy wychodzi poza zasięg oznacza,
że nie ma potrzeby pisania i debugowania dodatkowego kodu, aby uzyskać tę kontrolę.

Własność wpływa na to, jak działa wiele innych części Rusta.
Będziemy więc mówić o tych koncepcjach dalej przez resztę książki.
Przejdźmy teraz do rozdziału 5, który omawia grupowanie danych w strukturach (`struct`).

[ch13]: ch13-02-iterators.html
[ch6]: ch06-02-match.html#patterns-that-bind-to-values
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[deref-coercions]: ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods

## Przykładowy program wykorzystujący struktury

Aby pokazać, kiedy warto skorzystać ze struktur, napiszmy program, który
policzy pole prostokąta. Zaczniemy od pojedynczych zmiennych, potem zrefaktorujemy
program tak, aby używał struktur.

Stwórzmy projekt aplikacji binarnej przy użyciu Cargo. Nazwijmy go *prostokąty*.
Jako wejście przyjmie on szerokość i wysokość danego prostokąta i wyliczy
jego pole. Listing 5-8 pokazuje krótki program obrazujący jeden ze sposobów,
w jaki możemy to wykonać.

<span class="filename">Plik: src/main.rs</span>

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "Pole prostokąta wynosi {} pikseli kwadratowych."
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    height * width
}
```

<span class="caption">Listing 5-8: Obliczanie pola prostokąta o szerokości i wysokości
podanych jako osobne argumenty</span>

Uruchommy program komendą `cargo run`:

```text
Pole prostokąta wynosi 1500 pikseli kwadratowych.
```

Pomimo że program z Listingu 5-8 wygląda dobrze i poprawnie oblicza pole prostokąta
wywołując funkcję `area` podając oba wymiary, to możemy napisać go lepiej.
Szerokość i wysokość są blisko ze sobą spokrewnione, bo razem opisują pewien prostokąt.

Problem w tym kodzie widnieje w sygnaturze funkcji `area`:

```rust,ignore
fn area(width: u32, height: u32) -> u32 {
```

Funkcja `area` ma wyliczyć pole jakiegoś prostokąta, ale przecież
funkcja którą my napisaliśmy ma dwa parametry.
Parametry są ze sobą powiązane, ale ta zależność nie widnieje nigdzie w naszym
programie. Łatwiej byłoby ten kod zrozumieć i nim się posługiwać,
jeśli szerokość i wysokość byłyby ze sobą zgrupowane.
Już omówiliśmy jeden ze sposobów, w jaki można to wykonać w sekcji 
[“Krotka”][the-tuple-type]<!-- ignore --> rozdziału 3, czyli poprzez
wykorzystanie krotek.

### Refaktoryzacja z krotkami

Listing 5-9 pokazuje jeszcze jedną wersję programu wykorzystującego krotki.

<span class="filename">Plik: src/main.rs</span>

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "Pole prostokąta wynosi {} pikseli kwadratowych."
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

<span class="caption">Listing 5-9: Określenie szerokości i wysokości prostokąta
przy użyciu krotki</span>

Ten program jest, w pewnych aspektach, lepszy.
Krotki dodają odrobinę organizacji,
oraz pozwalają nam podać funkcji tylko jeden argument.
Z drugiej zaś strony ta wersja jest mniej czytelna:
elementy krotki nie mają nazw, a nasze obliczenia stały się enigmatyczne, 
bo dany wymiar prostokąta reprezentowany jest przez indeks elementu krotki.

Ewentualne pomylenie wymiarów prostokąta nie ma znaczenia przy obliczaniu jego pola,
ale sytuacja by się zmieniła gdybyśmy chcieli na przykład narysować go na ekranie.
Musielibyśmy zapamiętać, że szerokość znajduje się w elemencie krotki o indeksie 
`0`, a wysokość pod indeksem `1`.
Jeśli ktoś inny pracowałby nad tym kodem musiałby rozgryźć to samemu,
a także to zapamiętać. Nie byłoby zaskakujące omyłkowe pomieszanie tych dwóch wartości,
wynikające z braku zawarcia znaczenia danych w naszym kodzie.

### Refaktoryzacja ze strukturami: ukazywanie znaczenia

Struktur używamy, aby przekazać znaczenie poprzez etykietowanie danych.
Możemy przekształcić używaną przez nas krotkę nazywając zarówno
całość jak i pojedyncze jej części, tak jak w Listingu 5-10.

<span class="filename">Plik: src/main.rs</span>

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "Pole prostokąta wynosi {} pikseli kwadratowych."
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

<span class="caption">Listing 5-10: Definicja struktury `Rectangle`.

Powyżej zdefiniowaliśmy strukturę i nazwaliśmy ją `Rectangle`.
Wewnątrz nawiasów klamrowych zdefiniowaliśmy pola `width` i `height`,
oba mające typ `u32`.
Następnie w funkcji `main` stworzyliśmy konkretną instancję struktury `Rectangle`,
gdzie szerokość wynosi 30 jednostek i wysokość 50 jednostek.

Nasza funkcja `area` przyjmuje teraz jeden parametr,
który nazwaliśmy `rectangle`, którego typ to niezmienne zapożyczenie
instancji struktury `Rectangle`.
Jak wspomnieliśmy w Rozdziale 4, chcemy jedynie pożyczyć strukturę bez
przenoszenia jej własności. Takim sposobem `main` pozostaje właścicielem i może dalej
używać `rect1`, i dlatego używamy `&` w sygnaturze funkcji podczas jej wywołania.

Funkcja `area` dostaje się do pól `width` i `height`
instancji struktury `Rectangle`.
Znaczenie sygnatury funkcji `area` jest teraz jednakowe jak nasze zamiary:
obliczenie pola danego prostokąta `Rectangle` poprzez wykorzystanie jego
szerokości i wysokości. Bez niejasności przedstawiamy relację między
szerokością a wysokością i przypisujemy logiczne nazwy wartościom
zamiast indeksowania krotek wartościami `0` oraz `1`.
 
To wygrana dla przejrzystości.

### Dodawanie przydatnej funkcjonalności dzięki cechom derywowanym

Miło byłoby móc wyświetlić instancję struktury `Rectangle` w trakcie
debugowania naszego programu i zobaczyć wartość każdego pola.
Listing 5-11 próbuje użyć makra `println!`,
którego używaliśmy w poprzednich rozdziałach.
To jednakowoż nie zadziała.

<span class="filename">Plik: src/main.rs</span>

```rust,ignore,does_not_compile
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 to {}", rect1);
}
```

<span class="caption">Listing 5-11: Próba wyświetlenia instancji `Rectangle` </span>

Podczas próby uruchomienia tego kodu wyświetlany jest błąd z poniższym komunikatem:

```text
error[E0277]: the trait bound `Rectangle: std::fmt::Display` is not satisfied
```

Makro `println!` może formatować na wiele sposobów, a domyślnie para nawiasów klamrowych
daje `println!` znać, że chcemy wykorzystać formatowanie `Display` (ang. wyświetlenie).
Jest to tekst przeznaczony dla docelowego użytkownika.
Widziane przez nas wcześniej prymitywne typy implementują `Display` automatycznie,
bo przecież jest tylko jeden sposób wyświetlenia użytkownikowi symbolu `1` czy 
jakiegokolwiek innego prymitywnego typu.
Ale kiedy w grę wchodzą struktury, sposób w jaki `println!` powinno formatować
tekst jest mniej oczywiste, bo wyświetlać strukturę można na wiele sposobów:
z przecinkami, czy bez?
Chcesz wyświetlić nawiasy klamrowe?
Czy każde pole powinno być wyświetlone?
Przez tę wieloznaczność Rust nie zakłada z góry co jest dla nas najlepsze, 
więc z tego powodu struktury nie implementują automatycznie cechy `Display`.

Jeśli będziemy czytać dalej znajdziemy taką przydatną informację:

```text
`Rectangle` cannot be formatted with the default formatter; try using
`:?` instead if you are using a format string
```

Jesteśmy poinformowani, że podana przez nas struktura nie może być użyta
z domyślnym formaterem i zasugerowane jest nam użycie specyfikatora formatowania `:?`.
To tak też zróbmy! Wywołanie makra `println!` teraz będzie wyglądać następująco:
`println!("rect1 to {:?}", rect1);`. 
Wprowadzenie specyfikatora `:?` wewnątrz pary nawiasów klamrowych
przekazuje `println!`, że chcemy użyć formatu wyjścia o nazwie `Debug`.
Cecha Debug pozwala nam wypisać strukturę w sposób użyteczny dla deweloperów,
pozwalając nam na obejrzenie jej wartości podczas debugowania kodu.

Uruchommy kod z tymi zmianami. A niech to! Nadal pojawia się komunikat o błędzie:

```text
error[E0277]: the trait bound `Rectangle: std::fmt::Debug` is not satisfied
```

Ale kompilator ponownie daje nam pomocny komunikat:

```text
`Rectangle` cannot be formatted using `:?`; if it is defined in your
crate, add `#[derive(Debug)]` or manually implement it
```

Powyższy komunikat informuje nas, że cecha `Debug` nie jest zaimplementowana dla
struktury `Rectangle` i zaleca nam dodanie adnotacji. Rust *doprawdy* zawiera
funkcjonalność pozwalającą wyświetlić informacje pomocne w debugowaniu, ale
wymaga od nas, abyśmy ręcznie i wyraźnie zaznaczyli naszą decyzję
o dodaniu tej funkcjonalności do naszej struktury.
W tym celu dodajemy adnotację `#[derive(Debug)]` przed samą definicją
struktury, jak w Listingu 5-12.

<span class="filename">Plik: src/main.rs</span>

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 to {}", rect1);
}
```

<span class="caption">Listing 5-12: Dodanie adnotacji nadającą cechę `Debug`
i wyświetlanie instancji `Rectangle` formatowaniem przeznaczonym do celów debugowania</span> 

Teraz kiedy uruchomimy program nie wyskoczy nam żaden błąd, a naszym oczom
ukaże się poniższy tekst:

```text
rect1 to Rectangle { width: 30, height: 50 }
```

No nieźle! Nie jest to może najpiękniejsza reprezentacja, ale spełnia swoje zadanie i
pokazuje wartości wszystkich pól tej instancji,
co zdecydowanie by pomogło gdybyśmy polowali na bugi.
Kiedy w grę wchodzą większe struktury miło byłoby też mieć troszkę czytelniejszy wydruk;
w takich sytuacjach możemy użyć `{:#?}` zamiast `{:?}` w makrze `println!`.
Użycie stylu `{:#?}` w tym przypadku wyglądało będzie tak:

```text
rect1 to Rectangle {
    width: 30,
    height: 50
}
```

Rust oddaje nam do użytku cały szereg cech, które możemy nadać za pomocą adnotacji `derive`,
która dostarcza przydatne funkcjonalności zadeklarowanym przez nas typom.
Te cechy i ich zachowania opisane są w Załączniku C. Jak dodawać takim
cechom własne implementacje oraz także jak tworzyć własne cechy omówimy w Rozdziale 10.

Nasza funkcja `area` jest dość specyficzna: oblicza pola jedynie prostokątów.
Skoro i tak nie zadziała ona z żadnym innym typem, przydatne 
byłoby bliższe połączenie poleceń zawartych w tej funkcji z naszą strukturą `Rectangle`.

Kontynuacja tej refaktoryzacji zmieni funkcję `area` w metodę `area`, którą
zdefiniujemy w naszym typie *Rectangle*.

[the-tuple-type]: ch03-02-data-types.html#krotka

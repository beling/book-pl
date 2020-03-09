## Przykładowy program wykorzystujący struktury

Aby zrozumieć dlaczego chcielibyśmy używać struktury napiszmy program, który
policzy pole prostokąta. Zaczniemy od jednej zmiennej, potem zrefaktorujemy
program, tak aby używał struktur.

Stwórzmy projekt aplikacji binarnej przy użyciu Cargo. Nazwijmy go *prostokaty*.
Jako wejście przyjmie szerokość i wysokość danego prostokąta i wyliczy
jego pole. Listing 5-8 pokazuje krótki program obrazujący jeden ze sposobów,
w jaki możemy to wykonać.

<span class="filename">Plik: src/main.rs</span>

```rust
fn main() {
    let szerokosc1 = 30;
    let wysokosc1 = 50;

    println!(
        "Pole prostokąta wynosi {} pikseli kwadratowych."
        pole(szerokosc1, wysokosc1)
    );
}

fn pole(wysokosc: u32, szerokosc: u32) -> u32 {
    wysokosc * szerokosc
}
```

<span class="caption">Listing 5-8: Kalkulacja pola prostokąta określonego
poprzez oddzielne zmienne wymiarów: szerokości i wysokości</span>

Uruchommy program komendą `cargo run`:

```text
Pole prostokąta wynosi 1500 pikseli kwadratowych.
```

Mimo że w Listingu 5-8 wszystko wygląda OK, a więc wylicza pole prostokąta
wywołując funkcję `pole` z oboma wymiarami, to jednak da się to napisać lepiej.
Szerokość i wysokość są blisko ze sobą spokrewnione, bo razem opisują pewien prostokąt.

Problem w tym kodzie widnieje w sygnaturze funkcji `pole`:

```rust,ignore
fn pole(wysokosc: u32, szerokosc: u32) -> u32 {
```

Funkcja `pole` ma wyliczyć pole jakiegoś prostokąta, ale przecież
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
        pole(rect1)
    );
}

fn pole(wymiary: (u32, u32)) -> u32 {
    wymiary.0 * wymiary.1
}
```

<span class="caption">Listing 5-9: Określenie szerokości i wysokości prostokąta
przy użyciu krotki</span>

Ten program jest, w pewnych aspektach, lepszy.
Krotki dodają odrobinę organizacji,
oraz pozwalają nam podać funkcji tylko jeden argument.
Ale z drugiej strony ta wersja jest mniej czytelna:
elementy krotki nie mają nazw, a nasze kalkulacje stały się enigmatyczne, 
bo dany wymiar prostokąta reprezentowany jest przez indeks elementu krotki.

Dla kalkulacji pola prostokąta akurat nie ma to znaczenia, ale jeśli chcielibyśmy
narysować ten prostokąt na ekranie, wtedy już miałoby to znaczenie!
Musielibyśmy zapamiętać, że szerokość znajduje się w elemencie krotki o indeksie 
`0`, a wysokość w indeksie `1`.
Jeśli ktoś inny pracowałby nad tym kodem musiałby rozgryźć to samemu,
a także to zapamiętać. Omyłkowe pomieszanie tych dwóch wartości nie byłoby zaskakujące,
a następstwem takiej pomyłki byłyby błędy spowodowane brakiem zawarcia
kontekstu i znaczenia danych w naszym kodzie.

### Refaktoryzacja ze strukturami: ukazywanie znaczenia

Struktur używamy, aby przekazać znaczenie poprzez etykietowanie danych.
Możemy przekształcić używaną przez nas krotkę nazywając zarówno
całość jak i pojedyncze jej części, tak jak w Listingu 5-10.

<span class="filename">Plik: src/main.rs</span>

```rust
struct Prostokat {
    szerokosc: u32,
    wysokosc: u32,
}

fn main() {
    let rect1 = Prostokat { szerokosc: 30, wysokosc: 50 };

    println!(
        "Pole prostokąta wynosi {} pikseli kwadratowych."
        pole(&rect1)
    );
}

fn pole(prostokat: &Prostokat) -> u32 {
    prostokat.width * prostokat.height
}
```

<span class="caption">Listing 5-10: Definicja struktury `Prostokat`.

Powyżej zdefiniowaliśmy strukturę i nazwaliśmy ją `Prostokat`.
Wewnątrz nawiasów klamrowych zdefiniowaliśmy atrybuty `szerokosc` i `wysokosc`,
oba mające typ `u32`.
Następnie w funkcji `main` stworzyliśmy konkretną instancję struktury `Prostokat`,
gdzie szerokość wynosi 30 jednostek i wysokość 50 jednostek.

Nasza funkcja `area` przyjmuje teraz jeden parametr,
który nazwaliśmy `prostokat`, którego typ to niezmienne zapożyczenie
instancji struktury `Prostokat`.
Jak wspomnieliśmy w Rozdziale 4, chcemy jedynie pożyczyć strukturę zamiast
wejść w jej posiadanie. Takim sposobem `main` pozostaje właścicielem i może dalej
używać `rect1`, i dlatego używamy `&` w sygnaturze funkcji podczas jej wywołania.

Funkcja `pole` dostaje się do atrybutów `szerokosc` i `wysokosc`
instancji struktury `Prostokat`.
Znaczenie sygnatury funkcji `pole` jest teraz jednakowe jak nasze zamiary:
kalkulacja pola danego prostokąta `Prostokat` poprzez wykorzystanie jego
szerokości i wysokości. Bez niejasności przedstawiamy relację między
szerokością a wysokością i przypisujemy logiczne nazwy wartościom
zamiast indeksowania krotek wartościami `0` oraz `1`.
 
To wygrana dla przejrzystości.

### Dodawanie przydatnej funkcjonalności dzięki cechom derywowanym

Miło byłoby móc wyświetlić instancję struktury `Prostokat` w trakcie
debugowania naszego programu i zobaczyć wartość każdego atrybutu.
Listing 5-11 próbuje użyć makra `println!`,
którego używaliśmy w poprzednich rozdziałach.
To jednakowoż nie zadziała.

<span class="filename">Plik: src/main.rs</span>

```rust,ignore,does_not_compile
struct Prostokat {
    szerokosc: u32,
    wysokosc: u32,
}

fn main() {
    let rect1 = Prostokat { szerokosc: 30, wysokosc: 50 };

    println!("rect1 to {}", rect1);
}
```

<span class="caption">Listing 5-11: Próba wyświetlenia instancji `Prostokat` </span>

Podczas próby uruchomienia tego kodu wyświetlany jest błąd z poniższym komunikatem:

```text
error[E0277]: the trait bound `Prostokat: std::fmt::Display` is not satisfied
```

Makro `println!` może formatować na wiele sposobów, a domyślnie para nawiasów klamrowych
daje `println!` znać, że chcemy wykorzystać formatowanie `Display` (ang. wyświetlenie).
Jest to wyjście (*ang. output*) przeznaczone dla bezpośredniej konsumpcji przez
docelowego użytkownika.
Widziane przez nas wcześniej prymitywne typy implementują `Display` automatycznie,
bo przecież jest tylko jeden sposób wyświetlenia użytkownikowi symbolu `1` czy 
jakiegokolwiek innego prymitywnego typu.
Ale kiedy w grę wchodzą struktury, sposób w jaki `println!` powinno formatować
wyjście jest mniej oczywiste, bo wyświetlać strukturę można na wiele sposobów:
z przecinkami, czy bez?
Chcesz wyświetlić nawiasy klamrowe?
Czy każdy atrybut powinien być wyświetlony?
Przez tę wieloznaczność Rust nie zakłada z góry co jest dla nas najlepsze, 
więc z tego powodu struktury nie implementują automatycznie cechy `Display`.

Jeśli będziemy czytać dalej znajdziemy taką przydatną informację:

```text
`Prostokat` cannot be formatted with the default formatter; try using
`:?` instead if you are using a format string
```

Jesteśmy poinformowani, że podana przez nas struktura nie może być użyta
z domyślnym formaterem i zasugerowane jest nam użycie specyfikatora formatowania `:?`.
To tak też zróbmy! Wywołanie makra `println!` teraz będzie wyglądać następująco:
`println!("rect1 to {:?}", rect1);`. 
Wprowadzenie specyfikatora `:?` wewnątrz pary nawiasów klamrowych
przekazuje `println!`, że chcemy użyć formatu wyjścia o nazwie `Debug`.
Cecha `Debug` pozwala nam wypisać strukturę w sposób użyteczny dla deweloperów,
czyli ułatwia nam zajrzeć do wartości wewnątrz struktury podczas debugowania przez nas kodu.

Uruchom kod z tymi zmianami. A niech to! Nadal pojawia się komunikat o błędzie:

```text
error[E0277]: the trait bound `Prostokat: std::fmt::Debug` is not satisfied
```

Ale kompilator nas nie opuszcza:

```text
`Prostokat` cannot be formatted using `:?`; if it is defined in your
crate, add `#[derive(Debug)]` or manually implement it
```

Powyższy komunikat informuje nas, że cecha `Debug` nie jest zaimplementowana dla
struktury `Prostokat` i zaleca nam dodanie adnotacji. Rust *doprawdy* zawiera
funkcjonalność pozwalającą wyświetlić informacje pomocne w debugowaniu, ale
wymaga od nas, abyśmy ręcznie i wyraźnie zaznaczyli naszą decyzję
o dodaniu tej funkcjonalności do naszej struktury.
W tym celu dodajemy adnotację `#[derive(Debug)]` przed samą definicją
struktury, jak w Listingu 5-12.

<span class="filename">Plik: src/main.rs</span>

```rust
#[derive(Debug)]
struct Prostokat {
    szerokosc: u32,
    wysokosc: u32,
}

fn main() {
    let rect1 = Prostokat { szerokosc: 30, wysokosc: 50 };

    println!("rect1 to {}", rect1);
}
```

<span class="caption">Listing 5-12: Dodanie adnotacji derywacji cechy `Debug`
i wyświetlanie instancji `Rectangle` formatowaniem przeznaczonym do celów debugowania</span> 

Teraz kiedy uruchomimy program nie wyskoczy nam żaden błąd, a naszym oczom
ukaże się poniższe wyjście:

```text
rect1 to Prostokat { szerokosc: 30, wysokosc: 50 }
```

No nieźle! Nie jest to może najpiękniejsza reprezentacja, ale spełnia swoje zadanie i
pokazuje wartości wszystkich atrybutów tej instancji,
co zdecydowanie by pomogło gdybyśmy polowali na bugi.
Kiedy w grę wchodzą większe struktury miło byłoby też mieć troszkę czytelniejszy wydruk;
w takich sytuacjach możemy użyć `{:#?}` zamiast `{:?}` w makrze `println!`.
Użycie stylu `{:#?}` w tym przypadku wyglądało będzie tak:

```text
rect1 to Prostokat {
    szerokosc: 30,
    wysokosc: 50
}
```

Rust oddaje nam do użytku cały szereg cech, które możemy użyć wspólnie z adnotacją `derive`
dostarczając przydatne funkcjonalności typom zadeklarowanym przez nas.
Te cechy i ich zachowania opisane są w Załączniku C. Jak dodawać takim
cechom własne implementacje oraz także jak tworzyć własne cechy omówimy w Rozdziale 10.

Nasza funkcja `pole` jest dość specyficzna: oblicza pola jedynie prostokątów.
Skoro i tak nie zadziała ona z żadnym innym typem, przydatnym 
byłoby bliższe połączenie poleceń zawartych w tej funkcji z naszą strukturą `Prostokat`.

Kontynuacja tej refaktoryzacji zmieni funkcję `pole` w metodę `pole`, którą
zdefiniujemy w naszym typie *Prostokat*.

[the-tuple-type]: ch03-02-data-types.html#krotka

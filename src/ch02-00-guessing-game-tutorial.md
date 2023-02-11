# Piszemy grę zgadywankę

Rozpocznijmy zabawę z Rustem tworząc razem praktyczny projekt. Ten rozdział zapozna cię z kilkoma podstawowymi
konceptami Rusta, prezentując ich użycie w prawdziwym programie. Dowiesz się, co oznaczają `let`, `match`, metoda,
funkcja powiązana (*associated function*), nauczysz się, jak używać skrzyń (*crates*), i wielu innych rzeczy!
Dokładniejsze omówienie tych tematów znajduje się w dalszych rozdziałach. W tym rozdziale przećwiczysz jedynie podstawy.

Zaimplementujemy klasyczny problem programistyczny dla początkujących: grę zgadywankę. Oto zasady: program generuje
losową liczbę całkowitą z przedziału od 1 do 100. Następnie prosi użytkownika o wprowadzenie liczby z tego przedziału.
Gdy użytkownik wprowadzi swoją odpowiedź, program informuje, czy podana liczba jest niższa czy wyższa od wylosowanej.
Gdy gracz odgadnie wylosowaną liczbę, program wyświetla gratulacje dla zwycięzcy i kończy działanie.

## Tworzenie nowego projektu

Aby stworzyć nowy projekt, wejdź do folderu *projects* utworzonego w rozdziale 1 i za pomocą Cargo 
wygeneruj szkielet projektu, w ten sposób:

```console
$ cargo new guessing_game
$ cd guessing_game
```

Pierwsza komenda, `cargo new`, jako argument przyjmuje nazwę projektu (`guessing_game`).
W kolejnej linii komenda `cd` przenosi nas do nowo utworzonego folderu projektu.

Spójrz na wygenerowany plik *Cargo.toml*:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial
rm -rf no-listing-01-cargo-new
cargo new no-listing-01-cargo-new --name guessing_game
cd no-listing-01-cargo-new
cargo run > output.txt 2>&1
cd ../../..
-->

<span class="filename">Plik: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

Jak już widziałeś w rozdziale 1, `cargo new` tworzy dla ciebie program 
„Hello World!”. Otwórz plik *src/main.rs*:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Teraz skompilujemy i uruchomimy ten program w jednym kroku za pomocą komendy `cargo run`:


```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

Komenda `run` jest przydatna, kiedy chcesz w szybki sposób testować kolejne iteracje rozwoju projektu.
Tak właśnie jest w przypadku naszej gry: chcemy testować każdy krok, zanim przejdziemy do kolejnego.

Otwórz jeszcze raz plik *src/main.rs*. W tym pliku będziesz pisał kod programu.

## Przetwarzanie odpowiedzi

Pierwsza część programu będzie prosiła użytkownika o podanie liczby, przetwarzała jego odpowiedź i sprawdzała,
czy wpisane przez niego znaki mają oczekiwaną postać. Zaczynamy od wczytania odpowiedzi gracza. 
Przepisz kod z listingu 2-1 do pliku *src/main.rs*.

<span class="filename">Plik: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

<span class="caption">Listing 2-1: Implementacja wczytująca odpowiedź użytkownika i wypisująca ją na ekran</span>

Powyższy fragment kodu zawiera dużo informacji - przeanalizujmy go kawałek po kawałku. Aby wczytać odpowiedź gracza
a następnie wyświetlić ją na ekranie, musimy dołączyć do programu bibliotekę `io` (input/output).
Biblioteka `io` pochodzi z biblioteki standardowej (znanej jako `std`):

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Domyślnie Rust posiada zestaw elementów zdefiniowanych w bibliotece standardowej, które
importuje do każdego programu. Ten zestaw nazywany jest *prelude* i
można zobaczyć co zawiera [w dokumentacji biblioteki standardowej][prelude].

Jeśli typu, którego chcesz użyć, nie ma w prelude, musisz go jawnie zaciągnąć używając słowa `use`.
Skorzystanie z biblioteki `std::io` dostarcza wielu pożytecznych mechanizmów związanych z `io`,
włącznie z funkcjonalnością do wczytywania danych wprowadzonych przez użytkownika.

Tak jak mówiliśmy już w rozdziale 1, każdy program rozpoczyna wykonanie w funkcji `main`.

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

`fn` deklaruje nową funkcję, `()` informuje, że funkcja ta nie przyjmuje żadnych parametrów,
a `{` otwiera ciało funkcji.

W rozdziale 1 nauczyłeś się również, że `println!` jest makrem, które wyświetla zawartość stringa na ekranie:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Powyższy fragment kodu wypisuje na ekranie informację, na czym polega gra, i prosi użytkownika
o wprowadzenie odgadniętej przez niego liczby.


### Zapisywanie wartości w zmiennych

Teraz stworzymy *zmienną* do zapisywania odpowiedzi użytkownika, w ten sposób:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}

```

Program robi się coraz bardziej interesujący! W tej krótkiej linii wiele się dzieje. 
Instrukcji `let` używamy do utworzenia *zmiennej*. Tutaj kolejny przykład:

```rust,ignore
let apples = 5;
```

W tej linii tworzona jest nowa zmienna o nazwie `apples`, do której przypisana jest wartość 5.
W Ruście wszystkie zmienne są domyślnie niemutowalne (stałe), co oznacza, że nadana im na początku wartość nie zmieni się.
We’ll be discussing this concept in detail in the [„Variables and Mutability”][variables-and-mutability]<!-- ignore -->
section in Chapter 3.
Poniższy przykład pokazuje, jak stawiając słowo kluczowe `mut` przed nazwą zmiennej stworzyć zmienną mutowalną:

```rust,ignore
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

> Uwaga: Znaki `//` rozpoczynają komentarz, który ciągnie się do końca linii.
> Rust ignoruje zawartość komentarzy.
> Komentarze omówimy bardziej szczegółowo w [rozdziale 3][comments]<!-- ignore -->.

Powróćmy do naszej gry-zgadywanki.
Teraz już wiesz, że `let mut guess` utworzy mutowalną zmienną o nazwie `guess`.
Po prawej stronie znaku przypisania (`=`) jest wartość, która jest przypisywana do `guess`,
i która jest wynikiem wywołania funkcji `String::new`, tworzącej nową instancję `Stringa`.
[`String`][string]<!-- ignore --> to dostarczany przez bibliotekę standardową typ tekstowy,
gdzie tekst ma postać UTF-8 i może się swobodnie rozrastać.

Znaki `::` w wyrażeniu `::new` wskazują na to, że `new` jest funkcją powiązaną (*associated
function*) z typem `String`. *Funkcje powiązane* są zaimplementowane na danym typie, w tym 
przypadku na `Stringu`, a nie na konkretnej instancji typu `String`
(niektóre języki programowania nazywają to *metodą statyczną*).
Funkcja `new` tworzy nowy, pusty `String`. W przyszłości spotkasz się z funkcjami `new` 
dla wielu różnych typów, ponieważ jest to standardowa nazwa dla funkcji tworzącej nową
instancję danego typu.

Podsumowując, linia `let mut guess = String::new();` stworzyła mutowalną zmienną, która jest obecnie przypisania do nowej, pustej instancji typu `String`. Uff!

### Pobieranie danych od użytkownika

Przypominasz sobie, że załączyliśmy do programu obsługę wejścia/wyjścia z biblioteki
standardowej przy pomocy linii `use std::io;`?
Teraz wywołamy z `stdin` funkcję znajdującą się w module `io`, które pozwoli nam na pobranie danych od użytkownika:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

Gdybyśmy nie zaimportowali `io` za pomocą `use std::io` na początku programu, aby wywołać tę funkcję musielibyśmy
napisać `std::io::stdin`.
Funkcja `stdin` zwraca instancję [`std::io::Stdin`][iostdin]<!-- ignore -->,
która jest typem reprezentującym uchwyt do standardowego wejścia dla twojego terminala.

Dalszy fragment kodu, `.read_line(&mut guess)`, wywołuje metodę [`read_line`][read_line]<!-- ignore -->
na uchwycie wejścia standardowego, aby w ten sposób wczytać znaki wprowadzone przez gracza.
Do metody `read_line` podajemy argument `&mut guess`, by wskazać gdzie zapisać wczytane znaki.

Zadaniem metody `read_line` jest wziąć to, co użytkownik wpisze na wejście standardowe i dodać to
do podanego string (bez nadpisania jego zawartości), który przyjmuje ona jako argument.
String ten musi być mutowalny, aby metoda była w stanie go zmodyfikować.

Znak `&` wskazuje na to, że argument `guess` jest referencją. Referencja oznacza, że wiele kawałków kodu może operować
na jednej instancji danych, bez konieczności kopiowania tej danej kilkakrotnie. Referencje są skomplikowane,
a jedną z głównych zalet Rusta jest to, jak bezpiecznie i łatwo można ich używać.
Do dokończenia tego programu nie musisz znać wielu szczegółów na ten temat: rozdział 4 omówi referencje bardziej
wnikliwie. Póki co wszystko co musisz wiedzieć o referencjach to to, że podobnie jak zmienne, domyślnie są niemutowalne.
Dlatego musimy napisać `&mut guess`, a nie `&guess`, aby dało się tę referencję modyfikować.


### Obsługa potencjalnych błędów z użyciem `Result`

Nie skończyliśmy jeszcze analizy tej linii kodu. Pomimo tego że doszliśmy już do trzeciej linii tekstu, wciąż jest to część pojedynczej, logicznej linii kodu. Kolejną częścią jest ta metoda:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

Moglibyśmy napisać ten kod tak:

```rust,ignore
io::stdin().read_line(&mut guess).expect("Błąd wczytania linii");
```

Jednakże taka długa linia jest trudna do czytania, więc lepiej ją podzielić. Często warto złamać linię i wprowadzić dodatkowe wcięcie, by poprawić czytelność długich wywołań ze składnią typu `.nazwa_metody()`.
Teraz omówimy, co ta linia robi.

Jak już wspomnieliśmy wcześniej, `read_line` zapisuje tekst wpisany przez użytkownika do stringa przekazanego jako argument. Ale również zwraca wartość typu `Result`.
[`Result`][result]<!--ignore --> jest [*enumeracją*][enums]<!-- ignore -->, często nazywaną *enumem* lub *typamem wyliczeniowym*.
Typ wyliczeniowy to typ, który może mieć stały zestaw wartości, nazywanych *wariantami* (*variants*).

[Chapter 6][enums]<!-- ignore --> will cover enums in more detail. The purpose of these `Result` types is to encode error-handling information.

Możliwe wartości `Result` to `Ok` i `Err`. `Ok` oznacza, że operacja powiodła się sukcesem i wewnątrz obiektu `Ok` znajduje się poprawnie wygenerowana wartość. `Err` oznacza, że operacja nie powiodła się, i obiekt `Err` zawiera informację o przyczynach niepowodzenia.

Obiekty typu `Result`, tak jak obiekty innych typów,
mają zdefiniowane dla siebie metody. Instancja `Result` ma metodę [`expect`][expect]<!-- ignore -->,
którą możesz wywołać. Jeśli dana instancja `Result` będzie miała wartość `Err`, wywołanie metody `expect` spowoduje zakończenie się programu i wyświetlenie na ekranie wiadomości, którą podałeś jako argument do `expect`. Sytuacje, gdy metoda `read_line` zwraca `Err`, najprawdopodobniej są wynikiem błędu pochodzącego z systemu operacyjnego. Gdy zaś zwrócony `Result` ma wartość `Ok`,
`expect` odczyta wartość właściwą, przechowywaną przez `Ok`, i zwróci tę wartość, gotową do użycia w programie.
W tym przypadku wartość ta odpowiada liczbie bajtów, które użytkownik wprowadził na wejście.

Gdybyśmy pominęli wywołanie `expect`, program skompilowałby się z ostrzeżeniem:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust ostrzega, że nie zrobiliśmy nic z wartością `Result` zwróconą z `read_line`, a co za tym idzie,
program nie obsłużył potencjalnego błędu. Sposobem na pozbycie się tego ostrzeżenia jest dopisanie obsługi błędów. Tutaj jednak chcemy, by program zakończył się, gdy nie uda się odczytać odpowiedzi użytkownika,
więc możemy użyć `expect`. O wychodzeniu ze stanu błędu przeczytasz w [rozdziale 9][recover]<!-- ignore -->.

### Wypisywanie wartości z pomocą `println!` i placeholderów

Poza klamrą zamykającą program, w kodzie który dotychczas napisaliśmy została już tylko jedna linia do omówienia:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

Ta linia wyświetla na ekranie łańcuch, w którym zapisaliśmy odpowiedź użytkownika. Zestaw `{}` nawiasów
nawiasów klamrowych to placeholder: pomyśl o `{}` jak o małych szczypcach kraba, które trzymają
wartość w miejscu. Podczas wypisywania wartości zmiennej, nazwa zmiennej może
znajdować się wewnątrz nawiasów klamrowych. By wypisać wynik
wyrażenia, umieść puste nawiasy klamrowe w łańcuchu formatującym,
a za łańcuchem same wyrażenia, oddzielone przecinkami, po jednym dla kolejnych pustych nawiasów klamrowych.
Wyświetlanie zmiennej i wyniku wyrażenia w jednym wywołaniu `println!` wyglądałoby tak:

```rust
let x = 5;
let y = 10;

println!("x = {x} i y + 2 = {}", y + 2);
```
Ten kod wypisze na ekran `x = 5 i y + 2 = 12`.

### Testowanie pierwszej część programu

Przetestujmy pierwszą część *Zgadywanki*. Uruchom grę poleceniem `cargo run`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
     Running `target/debug/guessing_game`
Zgadnij numer!
Podaj swoją liczbę:
6
Wybrana przez ciebie liczba: 6
```

W tym miejscu pierwsza część gry jest gotowa: wczytujemy odpowiedź użytkownika z klawiatury i wypisujemy ją na ekranie.

## Generowanie sekretnej liczby

Następnie musimy wygenerować sekretną liczbę, którą gracz będzie próbował odgadnąć.
Sekretna liczba powinna zmieniać się przy każdym uruchomieniu programu, aby gra bawiła więcej niż raz.
Użyjmy losowej liczby z przedziału od 1 do 100, żeby odgadnięcie jej nie było zbyt trudne.
W bibliotece standardowej Rusta nie ma jeszcze obsługi liczb losowych, dlatego musimy sięgnąć do skrzyni
[`rand`][randcrate].

### Więcej funkcjonalności z użyciem skrzyń

Zapamiętaj: skrzynia (ang. *crate*) to kolejkcja plików źródłowych Rusta.
Projekt, który budujemy, to skrzynia binarna
(*binary crate*), czyli plik wykonywalny. Skrzynia `rand` to *library crate*, czyli biblioteka stworzona do używania w
innych programach.

Z użyciem Cargo dodawanie zewnętrznych pakietów jest bajecznie proste. Aby móc używać `rand` w naszym kodzie,
wystarczy zmodyfikować plik *Cargo.toml* tak, aby zaciągał skrzynię `rand` jako zależność do projektu.
Otwórz *Cargo.toml* i dodaj na końcu, pod nagłówkiem sekcji `[dependencies]`, poniższą linię.
Upewnij się, że podałeś `rand` dokładnie tak jak poniżej, z
z tym samym numerem wersji. Inaczej kody zawarte w tym tutorialu mogą nie zadziałać:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Plik: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:8:}}
```

Plik *Cargo.toml* podzielony jest na sekcje, których ciało zaczyna się po nagłówku i kończy się w miejscu, gdzie zaczyna się kolejna sekcja. W sekcji `[dependencies]` informujesz Cargo, jakich zewnętrznych skrzyń i w której wersji wymaga twój projekt. Tutaj przy skrzyni `rand` znajduje się specyfikator wersji `0.8.5`.
Cargo rozumie [Semantic Versioning][semver]<!-- ignore --> (nazywane tez czasem *SemVer*), które to jest standardem zapisywania numeru wersji. Numer `0.8.5` jest właściwie skrótem do `^0.8.5`, które oznacza wersję conajmniej 0.8.5, ale poniżej 0.9.0.

Cargo considers these versions to have public APIs compatible with version
0.8.5, and this specification ensures you’ll get the latest patch release that
will still compile with the code in this chapter. Any version 0.9.0 or greater
is not guaranteed to have the same API as what the following examples use.

Teraz bez zmieniania niczego w kodzie przekompilujmy projekt, tak jak przedstawia listing 2-2:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
rm Cargo.lock
cargo clean
cargo build -->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
  Downloaded libc v0.2.127
  Downloaded getrandom v0.2.7
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.16
  Downloaded rand_chacha v0.3.1
  Downloaded rand_core v0.6.3
   Compiling libc v0.2.127
   Compiling getrandom v0.2.7
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.16
   Compiling rand_core v0.6.3
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

<span class="caption">Listing 2-2: Wynik po wywołaniu `cargo build` po dodaniu zależności do skrzyni rand</span>

Być może u siebie zobaczysz inne numery wersji (jednak wszystkie będą kompatybilne z kodem, dzięki SemVer!), inne linie (zależnie od systemu operacyjnego),
lub linie wydrukowane w innej kolejności.

Teraz kiedy mamy już zdefiniowaną jakąś zewnętrzną zależność, Cargo ściąga najnowsze wersje wszystkich skrzyń z *rejestru*,
który jest kopią danych z [Crates.io][cratesio]. Crates.io to miejsce, gdzie ludzie związani z Rustem publikują dla innych
swoje otwarto-źródłowe projekty.

Po zaktualizowaniu rejestru Cargo sprawdza sekcję `[dependencies]` i ściąga skrzynie, jeśli jakichś brakuje. 
W tym przypadku, pomimo że podaliśmy do zależności jedynie skrzynę `rand`, Cargo ściągnął jeszcze inne skrzynie, 
od których zależny jest `rand`. Po ich ściągnięciu Rust je kompiluje, a następnie, mając już dostępne 
niezbędne zależności, kompiluje projekt.

Gdybyś teraz bez wprowadzania jakichkolwiek zmian wywołał ponownie `cargo build`, nie zobaczyłbyś nic ponad linię `Finished`.
Cargo wie, że zależności są już ściągnięte i skompilowane, i że nie zmieniałeś nic w ich kwestii w pliku *Cargo.toml*. 
Cargo również wie, że nie zmieniałeś nic w swoim kodzie, więc jego też nie rekompiluje. Nie ma nic do zrobienia, 
więc po prostu kończy swoje działanie. 

Jeśli wprowadzisz jakąś trywialną zmianę w pliku *src/main.rs*, zapiszesz,
a następnie ponownie zbudujesz projekt, zobaczysz jedynie dwie linijki na wyjściu:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

Te dwie linie pokazują, że Cargo przebudował uwzględniając jedynie twoją maleńką zmianą z pliku *src/main.rs*.
Zależności nie zmieniły się, więc Cargo wie, że może użyć ponownie te, które już raz ściągnął i skompilował.

#### Plik *Cargo.lock* zapewnia powtarzalność kompilacji

Cargo posiada mechanizm, który zapewnia że za każdym razem, gdy ty lub ktokolwiek inny będziecie przebudowywać projekt, 
kompilowane będą te same artefakty: Cargo użyje zależności w konkretnych wersjach, chyba że wskażesz inaczej.
Na przykład, co by się stało, gdyby za tydzień wyszła nowa wersja skrzyni `rand` 0.8.6, która zawierałaby poprawkę istotnego błedu,
ale jednocześnie wprowadza regresję, która zepsuje twój kod?
Odpowiedzią na ten problem jest plik *Cargo.lock*, który został stworzony w momencie,
gdy po raz pierwszy wywołałeś `cargo build`. Znajduje się on teraz w twoim folderze *guessing_game*.

Kiedy po raz pierwszy budujesz dany projekt, Cargo sprawdza wersje każdej z zależności, tak by kryteria były spełnione,
i wynik zapisuje w pliku *Cargo.lock*. Od tego czasu przy każdym kolejnym budowaniu, Cargo widząc, że plik *Cargo.lock*
istnieje, będzie odczytywać z niego wersje zależności do pobrania, zamiast na nowo próbować je określać.
Dzięki temu twoje kompilacje są automatycznie reprodukowalne. Innymi słowy, twój projekt będzie wciąż używał wersji `0.8.5`, 
do czasu aż sam jawnie nie wykonasz aktualizacji.
Ponieważ plik *Cargo.lock* jest ważny dla powtarzalnych kompilacji,
jest on często umieszczany w systemie kontroli wersji wraz z resztą kodu w projekcie.

#### Aktualizowanie skrzyni do nowszej wersji

Kiedy *chcesz* zmienić wersję skrzyni na nowszą, możesz skorzystać z komendy `update` dostarczanej przez Cargo, która
zignoruje plik *Cargo.lock* i wydedukuje na nowo najświeższe wersje skrzyń, które pasują do twojej specyfikacji z *Cargo.toml*.
Cargo zapisze te wersje do pliku *Cargo.lock*.
Jednak domyślnie Cargo będzie szukało jedynie wersji większej od `0.8.5` i mniejszej od `0.9.0`.
Jeśli skrzynia `rand` została wypuszczona w dwóch nowych wersjach, `0.8.6` i `0.9.0`,
po uruchomieniu `cargo update` zobaczysz taki wynik:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.8.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
    Updating rand v0.8.5 -> v0.8.6
```

Cargo ignoruje wydanie 0.9.0.
Teraz zauważysz również zmianę w pliku *Cargo.lock* - wersja skrzyni `rand` będzie ustawiona na `0.8.6`.
Gdybyś chciał używać `rand` w wersji 0.9.0 lub jakiejkolwiek z serii 0.9.*x*,
musiałbyś zaktualizować plik *Cargo.toml* do takiej postaci:

```toml
[dependencies]
rand = "0.9.0"
```

Następnym razem gdy wywołasz `cargo build`, Cargo zaktualizuje rejestr dostępnych skrzyń i 
zastosuje nowe wymagania co do wersji skrzyni `rand`, zgodnie z tym co zamieściłeś w pliku.

Można by jeszcze wiele mówić o [Cargo][doccargo]<!-- ignore --> i [jego ekosystemie][doccratesio]<!-- ignore -->.
Wrócimy do tego w rozdziale 14. Na razie wiesz wszystko, co w tej chwili potrzebujesz.
Dzięki Cargo ponowne używanie bibliotek jest bardzo łatwe, więc Rustowcy mogą pisać mniejsze projekty, składające się z wielu skrzyń.

<!-- Generating a Random Number -->
### Generowanie Losowej Liczby

A teraz użyjmy w końcu skrzyni `rand` by wygerować liczbę do zgadnięcia.
Zmodyfikujmy plik *src/main.rs*, tak jak pokazano na listingu 2-3:

<span class="filename">Plik: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

<span class="caption">Listing 2-3: Zmiany potrzebne do wygenerowania losowej liczby</span>

Najpierw dodajmy linię `use rand::Rng;`. `Rng` to cecha (ang. *trait*), która definiuje metody implementowane przez generator liczb losowych. Cecha ta musi być widoczna w zasięgu, w którym chcemy tych metod używać.
Cechy szczegółowo omówimy w rozdziale 10.

Dodajemy również dwie linie w środku. W pierwszej linii wywołujemy funkcję `rand::thread_rng`, która daje nam gotowy do użycia konkretny generator liczb losowych:
taki, który jest lokalny dla wątku wywołującego i seedowany z systemu operacyjnego. 
Następnie wywołujemy metodę `gen_range` tego generatora. Ta metoda zdefiniowana jest w cesze `Rng`,
którą włączyliśmy wyrażeniem `use rand::Rng;`. 
Zakres typu `start..=koniec` jest inkluzywny, zawiera obie podane wartości granicznej, dolną i górną.
Dlatego podaliśmy `1..=100`, aby zażądać liczby pomiędzy 1 a 100.

> Uwaga: Wiedza, której cechy użyć i które funkcje i metody ze skrzyni wywoływać, nie jest czymś co po prostu *wiesz*.
> Instrukcja jak używać danej skrzyni znajduje się zawsze w jej dokumentacji.
> Kolejną przydatną komendą Cargo jest polecenie `cargo doc --open`, które lokalnie zbuduje dokumentację dostarczaną przez wszystkie zależności, jakich używasz, i otworzy ją w przeglądarce.
> Gdyby, przykładowo, interesowały cię inne funkcjonalności ze skrzyni `rand`, wpisz `cargo doc --open` i wybierz `rand` z paska po lewej.

Druga dodana przez nas linia wypisuje na ekranie sekretną liczbę. Jest to przydatne podczas implementowania do testowania programu i zostanie usunięte w finalnej wersji. Gra nie byłaby zbyt ekscytująca, gdyby program podawał sekretną liczbę od razu na starcie!

Spróbuj uruchomić program kilka razy:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
     Running `target/debug/guessing_game`
Zgadnij liczbę!
Sekretna liczba to: 7
Podaj swoją liczbę:
4
Wybrana przez ciebie liczba: 4
$ cargo run
     Running `target/debug/guessing_game`
Zgadnij liczbę!
Sekretna liczba to: 83
Podaj swoją liczbę:
5
Wybrana przez ciebie liczba: 5
```

Za każdym razem powinieneś/powinnaś otrzymać inny sekretny numer, jednak zawsze z zakresu od 1 do 100. Dobra robota!


## Porównywanie Odpowiedzi z Sekretnym Numerem

Teraz, kiedy już mamy odpowiedź gracza i wylosowaną sekretną liczbę, możemy je porównać. Ten krok przedstawiony jest na listingu 2-4. Ten kod nie będzie się jeszcze kompilował. Zaraz wyjaśnimy dlaczego.

<span class="filename">Plik: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

<span class="caption">Listing 2-4: Obsługa możliwych rezultatów operacji porównywania dwóch liczb</span>



Po pierwsze dodaliśmy kolejne `use`, które wprowadza nam do zasięgu typ `std::cmp::Ordering` z biblioteki standardowej. 
`Ordering` jest enumem, takim jak `Result`, ale ma inne warianty: `Less`, `Greater`, i `Equal`. Są to trzy możliwe wyniki porównywania dwóch wartości.

Następnie dopisaliśmy na końcu pięć nowych linii wykorzystujących typ `Ordering`.
Metoda `cmp` porównuje dwie wartości. Można wywołać ją na dowolnym obiekcie, który da się porównywać.
Przyjmuje ona referencję do drugiego obiektu, z którym chcemy porównać pierwszy:
tutaj porównujemy `guess` do `secret_number`. `cmp` zwraca wariant enuma `Ordering` 
(którego typ zaciągnęliśmy poprzez wyrażenie `use`). Za pomocą wyrażenia [`match`][match]<!-- ignore -->, na podstawie wartości `Ordering` zwróconej przez wywołanie `cmp` z wartościami `guess` z `secret_number`, decydujemy, co zrobić dalej.

Wyrażenie `match` składa się z *odnóg*. Każda odnoga składa się ze *wzorca* dopasowania i kodu, który ma się wykonać, jeśli wartość podana na początku wyrażenia `match` będzie pasowała do danego wzorca.
Rust bierze wartość podaną do `match` i przegląda kolejno wzorce ze wszystkich odnóg.
Wzorce i konstrukcja `match` to potężne mechanizmy w Ruście, które pozwolą wyrazić w kodzie wiele różnych scenariuszy i pomogą zapewnić obsługę ich wszystkich.
Zostaną one omówione szczegółowo, odpowiednio w rozdziale 6 i 18.

Przeanalizujmy na przykładzie, co dokładnie dzieje się z użytym tutaj wyrażeniem `match`.
Powiedzmy, że użytkownik wybrał liczbę 50, a losowo wygenerowana sekretna liczba to 38.

Kiedy kod porówna 50 do 38, metoda `cmp` zwróci wartość `Ordering::Greater`, ponieważ 50 jest większe niż 38.
Zatem `match` otrzymuje tutaj wartość `Ordering::Greater`.
`Match` sprawdza wzorzec w pierwszej odnodze, `Ordering::Less`, ale wartość `Ordering::Greater` nie pasuje do wzorca `Ordering::Less`, więc kod z tej odnogi jest pomijany i sprawdzana jest następna odnoga.
Wzorzec z następnej odnogi, `Ordering::Greater`, *pasuje* do `Ordering::Greater`!
Powiązany kod w tej odnodze jest wykonywany i na ekranie pojawia się napis `Za duża!`.
Wyrażenie `match` kończy wykonanie po pierwszym znalezionym dopasowaniu, więc ostatnia odnoga nie będzie już w tym przypadku sprawdzana.

Niemniej, kod z listingu 2-4 jeszcze się nie skompiluje. Spróbujmy:


<!--
The error numbers in this output should be that of the code **WITHOUT** the
anchor or snip comments
-->

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

Komunikat błędu wskazuje, że *typy są niezgodne*. Rust jest silnie statycznym typowanym językiem. Jednak również wspiera dedukcję typów.
Kiedy napisaliśmy `let guess = String::new()`, Rust potrafił wywnioskować, że `guess` powinno być `Stringiem`, 
dzięki czemu nie musieliśmy pisać typu jawnie.
Z drugiej strony, `secret_number` jest typem numerycznym. Wiele typów numerycznych może przyjmować wartość spomiędzy 1 a 100:
`i32`, 32-bitowa liczba całkowita; `u32`, 32-bitowa liczba całkowita bez znaku; `i64`, 64-bitowa liczba całkowita; a także inne.
Jeśli nie wskazano inaczej, to domyślnie Rust wybiera `i32`, co jest typem `secret_number`, jeśli nie wpisaliśmy gdzieś indziej w kodzie jakiejś informacji,
która spowoduje że Rust wybierze inny typ. Przyczyną błędu jest to, że Rust nie potrafi porównywać
stringa z typem numerycznym.

Ostatecznie musimy przekonwertować stringa, którego program wczytał jako wejście z klawiatury,
do postaci typu numerycznego, który można porównać matematycznie do sekretnej liczby. Osiągamy to dodając kolejną linię do ciała funkcji `main`:

<span class="filename">Plik: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

Dodana linia to:

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Podaj liczbę!");
```

Tworzymy tu zmienną o nazwie `guess`. Ale czekaj, czy program przypadkiem nie ma już zmiennej o takiej nazwie? Owszem ma, ale szczęśliwie Rust pozwala *przesłaniać* poprzednią wartość
zmiennej `guess` nową wartością. Przesłanianie (*shadowing*)
pozwala użyć ponownie nazwy `guess`, zamiast zmuszać nas do tworzenia dwóch osobnych zmiennych, takich jak przykładowo `guess_str` i `guess`. Rozdział 3 opowiada więcej o przesłanianiu zmiennych. Teraz wspomnimy jedynie, że funkcjonalność ta jest często używana w sytuacjach, gdy konieczna jest konwersja wartości z jednego typu do drugiego. 

Nowej zmiennej `guess` nadajemy wartość wyrażenia `guess.trim().parse()`. Zmienna `guess` w tym wyrażeniu
odnosi się do pierwotnej zmiennej `guess`, która była stringiem zawierającym dane wczytane z klawiatury.
Metoda `trim` z interfejsu `Stringa` spowoduje usunięcie wszelkich białych znaków znajdujących
się na początku lub końcu stringa. Jest to niezbędne, bo aby sparsować `String` do typu `u32`, `String` ten powinien zawierać jedynie znaki numeryczne.
Jednakże, aby zadowolić funkcję `read_line`, użytkownik musi
wcisnąć <span class="keystroke">enter</span>. Po wciśnięciu <span class="keystroke">enter</span> znak nowej linii jest dopisywany do stringa. Przykładowo, jeśli użytkownik wpisał <span class="keystroke">5</span> i wcisnął <span class="keystroke">enter</span>, to `guess` przyjmie postać: `5\n`.
Znak `\n` reprezentuje nową linię, czyli wynik wciśnięcia <span class="keystroke">enter</span>. (Pod Windowsem w wyniku wciśnięcia <span class="keystroke">enter</span> otrzymujemy `\r\n`.)
Metoda `trim` usunie niechciane `\n`, dzięki czemu w stringu pozostanie jedynie `5`.

[Metoda `parse`][parse]<!-- ignore --> parsuje stringa do innego typu. Tu używamy jej by otrzymać typ liczbowy. Co więcej, musimy powiedzieć Rustowi, jakiego dokładnie typu oczekujemy, używając wyrażenia `let guess: u32`.
Dwukropek (`:`) po `guess` informuje Rusta, że dalej podany będzie typ zmiennej.
Rust ma kilka wbudowanych typów numerycznych;
`u32`, którą tu podaliśmy, to 32-bitowa liczba całkowita bez znaku. Jest to dobry domyślny wybór dla małych liczb dodatnich.
O innych typach numerycznych przeczytasz w rozdziale 3.

Dodatkowo, dzięki anotacji `u32` w tym przykładowym programie
i porównaniu tej liczby z `secret_number`, Rust wywnioskuje, że `secret_number` też powinien być typu `u32`. Zatem
porównanie zachodzi pomiędzy dwiema wartościami tego samego typu!

Wywołanie `parse` często może zakończyć się niepowodzeniem. Jeśli, na przykład, string będzie zawierał
`A👍%`, to jego konwersja do liczby nie może się udać. Z tego względu metoda `parse` zwraca
typ `Result`, podobnie jak metoda `read_line` (wspominaliśmy o tym wcześniej w sekcji
[„Obsługa potencjalnych błędów z użyciem `Result`”](#handling-potential-failure-with-the-result-type)<!-- ignore
-->). Potraktujemy ten `Result` w ten sam sposób, używając ponownie metody `expect`. Jeśli `parse` zwróci wariant `Err`
(ponieważ nie udało się stworzyć liczby ze stringa), wywołanie `expect` spowoduje zawieszenie się gry i wypisanie na ekran
podanego przez nas tekstu. Gdy zaś `parse` powiedzie się i poprawnie skonwertuje stringa do liczby, zwrócony `Result`
będzie wariantem `Ok`, a `expect` zwróci liczbę zaszytą w wartości `Ok`.

Teraz uruchomimy program!

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
cargo run
  76
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43 secs
     Running `target/debug/guessing_game`
Zgadnij liczbę!
Sekretna liczba to: 58
Podaj swoją liczbę:
  76
Wybrana przez ciebie liczba: 76
Za duża!
```

Nieźle! Pomimo tego że dodaliśmy spacje przed liczbą, program wciąż poprawnie rozpoznał,
że użytkownik wybrał liczbę 76. Uruchom program kilka razy, aby sprawdzić jak program reaguje na
różne wejścia: podaj właściwą liczbę, za wysoką, następnie za niską.

Nasza gra już z grubsza działa, ale użytkownik może odgadywać liczbę tylko jeden raz. Zmieńmy to
dodając pętlę!

## Wielokrotne zgadywanie dzięki pętli

Słowo kluczowe `loop` (*pętla*) tworzy pętlę nieskończoną. Dodamy taką pętlę, żeby dać graczowi więcej szans na odgadnięcie liczby:

<span class="filename">Plik: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

Jak widzisz, przenieśliśmy do pętli cały kod następujący po zachęcie gracza do odgadnięcia liczby.
Pamiętaj, żeby zwiększyć wcięcia linii wewnątrz pętli o kolejne cztery spacje, następnie uruchom program
ponownie. 
Niestety teraz program pyta o wprowadzenie odgadniętej liczby w nieskończoność i użytkownik nie może z niego łatwo wyjść!

Użytkownik może zawsze zatrzymać program używając skrótu klawiszowego <span class="keystroke">ctrl-c</span>. Lecz
jest jeszcze inny sposób, żeby uciec temu nienasyconemu potworowi, jak wspomnieliśmy w dyskusji o `parse`
w [„Porównywanie odpowiedzi gracza z sekretnym numerem”](#comparing-the-guess-to-the-secret-number)<!--
ignore -->: wprowadzenie znaku, który nie jest liczbą, spowoduje zawieszenie się programu. Można z tego skorzystać,
aby wyjść z programu, tak jak pokazujemy poniżej:


<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50 secs
     Running `target/debug/guessing_game`
Zgadnij liczbę!
Sekretna liczba to: 59
Podaj swoją liczbę:
45
Wybrana przez ciebie liczba: 45
Za mała!
Podaj swoją liczbę:
60
Wybrana przez ciebie liczba: 60
Za duża!
Podaj swoją liczbę:
59
Wybrana przez ciebie liczba: 59
Jesteś zwycięzcą!
Podaj swoją liczbę:
quit
thread 'main' panicked at 'Podaj liczbę!: ParseIntError { kind: InvalidDigit }', src/main.rs:28:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Wpisanie `quit` faktycznie powoduje wyjście z programu, ale taki sam skutek daje wprowadzenie dowolnego innego ciągu znaków nienumerycznych. Zamykanie programu w ten sposób nie jest zbyt optymalne. Dodatkowo, chcielibyśmy, aby gra zatrzymała się, kiedy gracz wprowadzi poprawny numer.


### Wychodzenie z programu po poprawnym odgadnięciu

Dodanie wyrażenia `break` sprawi, że gra zakończy się, kiedy gracz wygra.

<span class="filename">Plik: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

Dodanie linii `break` po `Jesteś zwycięzcą!` powoduje, że program opuszcza pętlę, gdy gracz odgadnie poprawnie
sekretny numer. Wyjście z pętli jest równoważne z zakończeniem pracy programu, ponieważ pętla jest ostatnią
częścią funkcji `main`.


### Obsługa niepoprawnych danych wejściowych

W celu dalszego ulepszenia gry zróbmy tak, żeby program, zamiast zawieszać się, ignorował wprowadzone dane nienumeryczne,
a użytkownik mógł zgadywać dalej. Możemy to osiągnąć edytując linię, w której `guess` jest konwertowane ze `Stringa` do
`u32`, w sposób przedstawiony na listingu 2-5.


<span class="filename">Plik: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

<span class="caption">Listing 2-5: Ignorowanie wejścia nieliczbowego i pytanie o kolejne liczby,
zamiast zawieszania programu</span>

Zamieniliśmy `expect` na wyrażenie `match` by zamienić zakończenie się programu na obsługuję błędów. Pamiętaj, że typem zwracanym przez
`parse` jest `Result`, a `Result` jest typem wyliczeniowym, który ma warianty `Ok` oraz `Err`.
Używamy tutaj wyrażenia `match`, podobnie jak robiliśmy to z wynikiem `Ordering` zwracanym przez
metodę `cmp`.

Jeśli `parse` jest w stanie pomyślnie zamienić stringa w liczbę, zwróci wariant `Ok`, zawierający
w sobie liczbę otrzymaną w konwersji. Wartość `Ok` odpowiada wzorcowi z pierwszej gałęzi `match`, zatem
`match` zwróci wartość `num`, która została obliczona i zapisana wewnątrz wartości `Ok`.
Ta liczba zostanie przypisana do nowoutworzonej przez nas zmiennej `guess`.

Jeśli jednak `parse` *nie* jest w stanie przekonwertować stringa na liczbę, zwróci wartość `Err`,
która zawiera dodatkowe informacje o błędzie. Wartość `Err` nie pasuje do wzorca `Ok(num)` z pierwszej odnogi `match`, ale pasuje do wzorca `Err(_)` z drugiej odnogi. Znak podkreślenia, `_`, pasuje do wszystkich wartości;
w tym przypadku mówimy, że do wzorca mają pasować wszystkie wartości `Err`, bez znaczenia na to jakie dodatkowe informacje
mają one w środku. Program zatem wykona instrukcje z drugiego ramienia, `continue`, co oznacza że program ma przejść
do kolejnej iteracji pętli i poprosić o nową liczbę. Dzięki temu program ignoruje wszystkie problemy jakie może napotkać
`parse`!

Teraz wszystko w naszym programie powinno działać zgodnie z oczekiwaniami. Wypróbujmy to:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/guessing_game`
Zgadnij liczbę!
Sekretna liczba to: 61
Podaj swoją liczbę:
10
Wybrana przez ciebie liczba: 10
Za mała!
Podaj swoją liczbę:
99
Wybrana przez ciebie liczba: 99
Za duża!
Podaj swoją liczbę:
foo
Podaj swoją liczbę:
61
Wybrana przez ciebie liczba: 61
Jesteś zwycięzcą!
```

Wspaniale! Jeszcze jedna drobna poprawka i nasza gra w zgadywankę będzie już skończona.
Program wciąż wyświetla sekretny numer. To było przydatne podczas testów, ale na dłuższą metę psułoby zabawę.
Usuńmy `println!` odpowiedzialną za wyświetlanie sekretnego numeru. Listing 2-6 pokazuje końcową wersję programu.

<span class="filename">Plik: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

<span class="caption">Listing 2-6: Kompletna gra w zgadywankę</span>

## Podsumowanie

Właśnie udało ci się zbudować grę w zgadywankę. Gratulacje!

Ten projekt w praktyczny sposób zapoznał cię z wieloma konceptami Rusta:
`let`, `match`, funkcjami, używaniem zewnętrznych skrzyń,
i innymi. W najbliższych rozdziałach koncepty te będą omówione bardziej szczegółowo.
Rozdział 3 omawia koncepty obecne w większości języków programowania, takie jak zmienne,
typy danych czy funkcje, i prezentuje jak należy w nich korzystać w Ruście.
Rozdział 4 odkrywa system własności, mechanizm który wyróżna Rusta spośród innych języków.
Rozdział 5 omawia składnię struktur i metod, a rozdział 6 wyjaśnia, jak działają typy numeryczne.


This project was a hands-on way to introduce you to many new Rust concepts:
`let`, `match`, functions, the use of external crates, and more. In the next
few chapters, you’ll learn about these concepts in more detail. Chapter 3
covers concepts that most programming languages have, such as variables, data
types, and functions, and shows how to use them in Rust. Chapter 4 explores
ownership, a feature that makes Rust different from other languages. Chapter 5
discusses structs and method syntax, and Chapter 6 explains how enums work.

[prelude]: ../std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#variables-and-mutability
[comments]: ch03-04-comments.html
[string]: ../std/string/struct.String.html
[iostdin]: ../std/io/struct.Stdin.html
[read_line]: ../std/io/struct.Stdin.html#method.read_line
[result]: ../std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: ../std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html
[match]: ch06-02-match.html
[shadowing]: ch03-01-variables-and-mutability.html#shadowing
[parse]: ../std/primitive.str.html#method.parse
[integers]: ch03-02-data-types.html#integer-types

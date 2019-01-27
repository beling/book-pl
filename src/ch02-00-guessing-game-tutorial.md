# Guessing Game

Rozpocznijmy zabawę z Rustem pisząc razem praktyczny program. Ten rozdział zapozna cię z niektórymi podstawowymi konceptami Rusta, prezentując ich użycie w prawdziwym programie. Dowiesz się, co oznaczają `let`, `match`, metody, funkcje powiązane (*associated functions*), nauczysz się, jak używać skrzyń (*crates*), i wielu innych rzeczy! Dokładniejsze omówienie tych tematów znajduje się w dalszych rozdziałach. W tym rozdziale przećwiczysz jedynie podstawy.

Zaimplementujemy klasyczny problem programistyczny dla początkujących: grę zgadywankę, czyli guessing game. Oto zasady: program generuje losową liczbę całkowitą z przedziału od 1 do 100. Następnie prosi użytkownika o wprowadzenie liczby z tego przedziału. Gdy użytkownik wprowadzi swoją odpowiedź, program informuje, czy podana liczba jest niższa czy wyższa od wylosowanej. Gdy gracz odgadnie wylosowaną liczbę, program wypisuje gratulacje dla zwycięzcy i kończy działanie.

## Tworzenie nowego projektu

Aby stworzyć nowy projekt, wejdź do folderu *projects* utworzonego w Rozdziale 1, i wygeneruj szkielet projektu poprzez użycie Cargo, w ten sposób:

```text
$ cargo new guessing_game
$ cd guessing_game
```

Pierwsza komenda, `cargo new`, jako pierwszy argument przyjmuje nazwę projektu (`guessing_game`). Flaga `--bin` informuje Cargo, że ma wykonać projekt binarny, podobny do tego z Rozdziału 1. W kolejnej linii komenda `cd` przenosi nas do nowo utworzonego folderu projektu.

Spójrz na wygenerowany plik *Cargo.toml*:

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

W tagu `authors` Cargo automatycznie wpisało dane uzyskane z twojego środowiska. Jeśli te informacje są błędne, popraw je i zapisz plik.

Jak już widziałeś w Rozdziale 1, `cargo new` generuje program “Hello, world!”. Otwórz plik *src/main.rs*:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Teraz skompilujemy i uruchomimy ten program w jednym kroku za pomocą komendy `cargo run`:


```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50 secs
     Running `target/debug/guessing_game`
Hello, world!
```

Komenda `run` jest przydatna, kiedy chcesz w szybki sposób testować kolejne iteracje rozwoju projektu. Tak właśnie jest w przypadku naszej gry: chcemy testować każdy krok, zanim przejdziemy do kolejnego.

Otwórz jeszcze raz plik *src/main.rs*. W tym pliku będziesz pisał kod programu.

## Przetwarzanie odpowiedzi

Pierwsza część programu będzie prosiła użytkownika o podanie liczby, przetwarzała jego odpowiedź i sprawdzała, czy wpisane przez niego znaki mają oczekiwaną postać. Zaczynamy od wczytania odpowiedzi gracza. Przepisz kod z listingu Listing 2-1 do pliku *src/main.rs*.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

<span class="caption">Listing 2-1: Implementacja wczytująca input użytkownika i wypisująca go na ekran</span>

Powyższy fragment kodu zawiera dużo informacji, więc przeanalizujemy go kawałek po kawałku. Aby uzyskać input od gracza a następnie wyświetlić go na ekranie, musimy włączyć do programu bibliotekę `io` (input/output). Biblioteka `io` pochodzi z biblioteki standardowej (znanej jako `std`):

```rust,ignore
use std::io;
```

Domyślnie Rust importuje do każdego programu tylko kilka podstawowych typów poprzez [*prelude*][prelude]<!-- ignore -->. Jeśli typu, którego chcesz użyć, nie ma w prelude, musisz go jawnie zaciągnąć używając słowa `use`. Skorzystanie z biblioteki `std::io` dostarcza wielu pożytecznych featurów związanych z `io`, włącznie z funkcjonalnością do akceptowania danych wprowadzonych przez użytkownika.

[prelude]: ../../std/prelude/index.html

Tak jak mówiliśmy już w Rozdziale 1, każdy program rozpoczyna wykonywanie się od funkcji `main`.


```rust,ignore
fn main() {
```

`fn` deklaruje nową funkcję, `()` informuje, że funkcja ta nie przyjmuje żadnych parametrów, a `{` otwiera ciało funkcji.

W Rozdziale 1 nauczyłeś się również, że `println!` jest makrem, które drukuje zawartość stringa na ekranie:


```rust,ignore
println!("Guess the number!");

println!("Please input your guess.");
```

Powyższy fragment kodu wypisuje na ekranie informację, na czym polega gra, i prosi użytkownika o wprowadzenie odgadniętej przez niego liczby.


### Zapisywanie wartości w zmiennych

Teraz stworzymy miejsce do zapisywania odpowiedzi użytkownika, w ten sposób:

```rust,ignore
let mut guess = String::new();
```

Program robi się coraz bardziej interesujący! W tej krótkiej linii wiele się dzieje. Zauważ, że jest to instrukcja `let`, która jest używana do utworzenia *zmiennej*. Tutaj kolejny przykład:


```rust,ignore
let foo = bar;
```

W tej linii tworzona jest nowa zmienna o nazwie `foo`, do której przypisana jest wartość `bar`. W Rust wszystkie zmienne są domyślnie niemodyfikowalne (stałe). Poniższy przykład pokazuje, jak stawiając słowo kluczowe `mut` przed nazwą zmiennej, tworzy się zmienną modyfikowalną:


```rust
let foo = 5; // immutable
let mut bar = 5; // mutable
```


> Note: Znaki `//` rozpoczynają komentarz, który ciągnie się do końca linii.
> Rust ignoruje zawartość komentarzy.


Teraz już wiesz, że `let mut guess` utworzy modyfikowalną zmienną o nazwie `guess`. Po prawej stronie znaku przypisania (`=`) jest wartość, która jest przypisywana do `guess`, i która jest wynikiem wywołania funkcji `String::new`, tworzącej nową instancję `String`. [`String`][string]<!-- ignore --> to dostarczany przez bibliotekę standardową typ tekstowy, gdzie tekst ma postać UTF-8 i może się swobodnie rozrastać.


[string]: ../../std/string/struct.String.html

Znaki `::` w linii z `::new` wskazują na to, że `new` jest funkcją powiązaną (*associated
function*) z typem `String`. *Associated functions* są zaimplementowane na danym typie, w tym przypadku na `String`, a nie na konkretnej instancji typu `String`. Niektóre języki programowania nazywają to *metodą statyczną*.


Funkcja `new` tworzy nowy, pusty `String`. W przyszłości spotkasz się z funkcjami `new` dla wielu różnych typów, ponieważ jest to typowa nazwa dla funkcji, która tworzy nową instancję danego typu.

Podsumowując, linia `let mut guess = String::new();` stworzyła modyfikowalną zmienną, która jest obecnie przypisania do nowej, pustej instancji typu `String`. Wow!

Przypominasz sobie, że załączyliśmy do programu obsługę input/output z biblioteki standardowej przy pomocy linii `use std::io;`? Teraz zawołamy funkcję powiązaną `stdin` z `io`:


```rust,ignore
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

Gdybyśmy pominęli `use std::io` na początku programu, wciąż moglibyśmy wywołać tę funkcję pisząc `std::io::stdin`. Funkcja `stdin` zwraca instancję [`std::io::Stdin`][iostdin]<!-- ignore -->, która jest typem reprezentującym uchwyt do standardowego wejścia dla twojego terminala.


[iostdin]: ../../std/io/struct.Stdin.html

Dalszy fragment kodu, `.read_line(&mut guess)`, woła metodę [`read_line`][read_line]<!-- ignore --> na uchwycie wejścia standardowego, aby w ten sposób wczytać znaki wprowadzone przez gracza. Do metody `read_line` wprowadzamy argument: `&mut guess`.

[read_line]: ../../std/io/struct.Stdin.html#method.read_line

Zadaniem metody `read_line` jest wziąć to, co użytkownik wpisze na wejście standardowe i zrobić z tego string, więc przyjmuje ona string jako argument. String ten musi być `mutable`, aby metoda była w stanie zmodyfikować go - dopisać do niego input użytkownika.

Znak `&` wskazuje na to, że argument `guess` jest referencją. Referencja oznacza, że wiele kawałków kodu może operować na jednej instancji danych, bez konieczności kopiowania tej danej kilkakrotnie. Referencje są skompikowaną funkcjonalnością, a jedną z głównych zalet Rusta jest to, jak bezpiecznie i łatwo można ich używać. Nie musisz znać wielu szczegółów na ten temat, aby dokończyć implementację tego programu: Rozdział 4 omówi referencje bardziej wnikliwie. Póki co wszystko co musisz wiedzieć o referencjach to to, że podobnie jak zmienne, domyślnie są niemodyfikowalne. Dlatego musimy napisać `&mut guess`, a nie `&guess`, aby dało się tę referencję modyfikować.

To jeszcze nie wszystko, co jest w tej linii kodu. Pomimo tego że była to już cała linia tekstu, jest to jedynie pierwsza część pojedynczej, logicznej linii kodu. Drugą częścią jest metoda:


```rust,ignore
.expect("Failed to read line");
```

Kiedy używasz składni `.foo()` do wołania kolejnych metod, często warto złamać linię i wprowadzić dodatkowe wcięcie, by poprawić czytelność długich wywołań. Moglibyśmy napisać ten kod tak:


```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

Jednakże taka długa linia jest trudna do czytania. Lepiej ją podzielić, umieszczając każde wywołanie metody w osobnej linii. Teraz omówimy, co te linie robią.

### Obsługa potencjalnych błędów z użyciem typu `Result`

Jak już wspomnieliśmy wcześniej, `read_line` wpisuje input użytkownika do stringa, którego przekażemy jako argument, ale również zwraca pewną wartość - w tym przypadku jest to [`io::Result`][ioresult]<!-- ignore -->. Rust ma w bibliotece standardowej wiele typów o nazwie `Result`: zarówno generyczny [`Result`][result]<!-- ignore --> jak i specyficzne wersje dla submodułów takich jak `io::Result`.

[ioresult]: ../../std/io/type.Result.html
[result]: ../../std/result/enum.Result.html

Typy `Result` są [*enumeracjami*][enums]<!-- ignore -->, często nazywanymi *enumami*. Enumeracja to typ, który może mieć stały zestaw wartości, nazywanych *wariantami* enuma (*enum’s variants*). Dokładniejszy opis enumów znajduje się w Rozdziale 6.


[enums]: ch06-00-enums.html

Możliwe wartości enuma `Result` to `Ok` i `Err`. `Ok` oznacza, że operacja powiodła się sukcesem, i wewnątrz obiektu `Ok` znajduje się poprawnie wygenerowana wartość. `Err` oznacza, że operacja nie powiodła się, i obiekt `Err` zawiera informację o przyczynach niepowodzenia.

Celem typów `Result` jest zakodowanie informacji o błędach. Obiekty typu `Result`, tak jak obiekty innych typów, mają zdefiniowane dla siebie metody. Instancja `io::Result` ma metodę [`expect`][expect]<!-- ignore -->, którą możesz wywołać. Jeśli dana instancja `io::Result` będzie miała wartość `Err`, wywołanie metody `expect` spowoduje crash programu i wyświetlenie na ekranie wiadomości, którą podałeś jako argument do `expect`. Sytuacje, gdy metoda `read_line` zwraca `Err`, najprawdopodobniej są wynikiem błędu pochodzącego z systemu operacyjnego. Gdy zaś zwraca `io::Result` o wartości `Ok`, `expect` odczyta wartość właściwą, przechowywaną przez `Ok`, i zwróci tę wartość, abyś mógł jej używać. W tym przypadku wartość ta odpowiada liczbie bajtów, które użytkownik wprowadził na wejście standardowe.

[expect]: ../../std/result/enum.Result.html#method.expect

Gdybyśmy pominęli wywołanie `expect`, program skompilowałby się z warningiem:

```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `std::result::Result` which must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default
```

Rust ostrzega, że nie użyliśmy wartości `Result` zwracanej z `read_line`, a co za tym idzie, program nie obsłużył potencjalnego błędu. Sposobem na wyciszenie tego warninga jest dopisanie obsługi błędów, jednak tutaj chcemy by program scrashował się, gdy błąd wystąpi, więc możemy użyć `expect`. O wychodzeniu ze stanu błędu przeczytasz w Rozdziale 9.

### Wypisywanie wartości z pomocą `println!` i placeholderów

Poza klamrą zamykającą program, w kodzie który dotychczas napisaliśmy została już tylko jedna linia do omówienia, która wygląda tak:

```rust,ignore
println!("You guessed: {}", guess);
```

Ta linia drukuje na ekran string, w którym zapisaliśmy input użytkownika. Klamry `{}` są placeholderem, który wkłada wartość do wyświetlenia w danym miejscu. Użycie klamr `{}` pozwala na wyprintowanie więcej niż jednej wartości: pierwsze klamry przyjmą pierwszą z wartości wymienionych po stringu formatującym, drugie klamry przyjmą drugą wartość, i tak dalej. Wyświetlanie wielu wartości w jednym wywołaniu `println!` wyglądałoby tak:

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```
Ten kod wypisze na ekran `x = 5 and y = 10`.

### Testowanie pierwszej część programu

Przetestujmy pierwszą część *guessing game*. Uruchom grę poleceniem `cargo run`:

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

W tym miejscu pierwsza część gry jest gotowa: pobieramy input użytkownika z klawiatury i wypisujemy go na ekranie.

## Generowanie sekretnej liczby

Następnie musimy wygenerować sekretną liczbę, którą gracz będzie próbował odgadnąć. Sekretna liczba powinna zmieniać się przy każdym uruchomieniu programu, aby gra była zabawna więcej niż raz. Użyjmy losowej liczby z przedziału od 1 do 100, żeby odgadnięcię jej nie było zbyt trudne. W bibliotece standardowej Rusta nie ma jeszcze obsługi liczb losowych, dlatego musimy sięgnąć do skrzyni [`rand`][randcrate].

[randcrate]: https://crates.io/crates/rand

### Więcej funkcjonalności z użyciem skrzyń

Zapamiętaj: skrzynia (ang. *crate*) to paczka Rustowego kodu. Projekt, który budujemy, to skrzynia binarna (*binary crate*), czyli plik wykonywalny. Skrzynia `rand` to *library crate*, czyli biblioteka stworzona by używać jej w innych programach.

Z użyciem Cargo, dodawanie zewnętrznych bibliotek jest bajecznie proste. Aby móc używać `rand` w naszym kodzie, wystarczy zmodyfikować plik *Cargo.toml*, tak aby zaciągał skrzynię `rand` jako zależność do projektu. Otwórz *Cargo.toml* i dodaj na końcu, pod headerem sekcji `[dependencies]`, poniższą linię:

<span class="filename">Filename: Cargo.toml</span>

```toml
[dependencies]

rand = "0.3.14"
```

Plik *Cargo.toml* podzielony jest na sekcje, których ciało zaczyna się po headerze i kończy się w miejscu, gdzie zaczyna się kolejna sekcja. W sekcji `[dependencies]` informujesz Cargo, jakich zewnętrznych skrzyń i w której wersji wymaga twój projekt. Tutaj przy skrzyni `rand` znajduje się specyfikator wersji `0.3.14`. Cargo rozumie [Semantic Versioning][semver]<!-- ignore --> (nazywane tez czasem *SemVer*), które to jest standardem zapisywania numeru wersji. Numer `0.3.14` jest właściwie skrótem do `^0.3.14`, które oznacza “jakakolwiek wersja, której API publiczne jest kompatybilne z wersją 0.3.14.”

[semver]: http://semver.org

Teraz bez zmieniania niczego w kodzie przekompilujmy projekt, tak jak przedstawia Listing 2-2:

```text
$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading rand v0.3.14
 Downloading libc v0.2.14
   Compiling libc v0.2.14
   Compiling rand v0.3.14
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

<span class="caption">Listing 2-2: Wynik po wywołaniu `cargo build` po dodaniu zależności do skrzyni rand</span>

Być może u siebie zobaczysz inne numery wersji (jednak wszystkie będą kompatybilne z kodem, dzięki SemVer!), lub wydrukowane linie będą w innej kolejności.

Teraz kiedy mamy już zdefiniowaną jakąś zewnętrzną zależność, Cargo ściąga najnowsze wersje wszystkich skrzyń z *rejestru*, który jest kopią danych z [Crates.io][cratesio]. Crates.io to miejsce, gdzie ludzie związani z Rustem publikują dla innych swoje open source'owe rozwiązania.

[cratesio]: https://crates.io

Po zaktualizowaniu rejestru, Cargo sprawdza sekcję `[dependencies]` i ściąga skrzynie, jeśli jakichś brakuje. W tym przypadku, pomimo że podaliśmy do zależności jedynie skrzyni `rand`, Cargo ściągnął jeszcze kopię `libc`, ponieważ `rand` jest zależne od `libc`. Po ściągnięciu ich Rust je kompiluje, a następnie, mając już dostępne niezbędne zależności, kompiluje projekt.

Gdybyś teraz bez wprowadzania jakichkolwiek zmian wywołał ponownie `cargo build`, nie zobaczyłbyś żadnego outputu. Cargo wie, że zależności są już ściągnięte i skompilowane, i że nie zmieniałeś nic w ich kwestii w pliku *Cargo.toml*. Cargo również wie, że nie zmieniałeś nic w swoim kodzie, więc jego też nie rekompiluje. Nie ma nic do zrobienia, więc po prostu kończy swoje działanie. Jeśli wprowadzisz jakąś trywialną zmianę w pliku *src/main.rs*, zapiszesz, a następnie ponownie zbudujesz projekt, zobaczysz jedynie dwie linijki na wyjściu:

```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

Te dwie linie pokazują, że Cargo aktualizuje builda jedynie twoją maleńką zmianą z pliku *src/main.rs*. Zależności nie zmieniły się, więc Cargo wie, że może użyć ponownie tych, które już raz ściągnął i skompilował. Jedynie fragment twojego kodu wymaga przebudowania.

#### Plik *Cargo.lock* zapewnia reprodukowalność buildów

Cargo posiada mechanizm, który zapewnia że za każdym razem gdy ty lub ktokolwiek inny będziecie przebudowywać projekt, kompilowane będą te same artefakty: Cargo użyje zależności w konkretnych wersjach. Na przykład, co by się stało, gdyby za tydzień wyszła nowa wersja skrzyni `rand` `v0.3.15`, która zawiera poprawkę na istotnego buga, ale jednocześnie wprowadza regresję, która zepsuje twój kod?

Odpowiedzią na ten problem jest plik *Cargo.lock*, który został stworzony w momencie, gdy po raz pierwszy wywołałeś `cargo build`. Znajduje się on teraz w twoim folderze *guessing_game*. Kiedy po raz pierwszy budujesz dany projekt, Cargo sprawdza wersje każdej z zależności, tak by kryteria były spełnione, i wynik zapisuje w pliku *Cargo.lock*. Od tego czasu przy każdym kolejnym budowaniu, Cargo widząc, że plik *Cargo.lock* istnieje, będzie pobierało z niego wersje zależności do pobrania, zamiast na nowo próbować je określać. Dzięki temu twoje buildy będą reprodukowalne. Innymi słowy, twój projekt będzie wciąż używał wersji `0.3.14`, do czasu aż sam jawnie nie podbijesz wersji do wyższej.


#### Podbijanie skrzyni do nowszej wersji

Kiedy *chcesz* zmienić wersję skrzyni na nowszą, możesz skorzystać z komendy `update` dostarczanej przez Cargo, która:

1. Zignoruje plik *Cargo.lock* i wydedukuje na nowo najświeższe wersje skrzyń, które pasują do twojej specyfikacji z *Cargo.toml*.
2. Jeśli to się powiedzie, Cargo zapisze te wersje do pliku *Cargo.lock*.

Jednak domyślnie Cargo będzie szukało jedynie wersji większej od `0.3.0` i mniejszej od `0.4.0`. Jeśli skrzynia `rand` została wypuszczona w dwóch nowych wersjach, `0.3.15` i `0.4.0`, po uruchomieniu `cargo update` zobaczysz taki wynik:

```text
$ cargo update
    Updating registry `https://github.com/rust-lang/crates.io-index`
    Updating rand v0.3.14 -> v0.3.15
```

Teraz zauważysz również zmianę w pliku *Cargo.lock* - wersja skrzyni `rand` będzie ustawiona na `0.3.15`.

Gdybyś chciał używać `rand` w wersji `0.4.0` lub jakiejkolwiek z serii `0.4.x`, musiałbyś zaktualizować plik *Cargo.toml* do takiej postaci:


```toml
[dependencies]

rand = "0.4.0"
```

Następnym razem gdy wywołasz `cargo build`, Cargo zaktualizuje rejestr dostępnych skrzyń i zastosuje nowe wymagania co do wersji skrzyni `rand`, zgodnie z tym co zamieściłeś w pliku.

Można by jeszcze wiele mówić o [Cargo][doccargo]<!-- ignore --> i [jego ekosystemie][doccratesio]<!-- ignore -->, i wrócimy do tego w rozdziale 14. Na teraz wiesz już wszystko, co musisz wiedzieć. Dzięki Cargo ponowne używanie bibliotek jest bardzo łatwe, więc Rustowcy mogą pisać małe projekty, składające się z wielu skrzyń.

[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html

### Generowanie losowej liczby

Zacznijmy w końcu *używać* skrzyni `rand`. Zmodyfikuj plik *src/main.rs*, tak jak pokazano na Listingu 2-3:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

<span class="caption">Listing 2-3: Zmiany potrzebne do wygenerowania losowej liczby</span>

Na górze dodajemy linię `extern crate rand;`, żeby poinformować Rusta, że będziemy używać zewnętrznej zależności. To również skutkuje tym, co wywołanie `use rand`, więc teraz możemy wywoływać rzeczy z wnętrza skrzyni `rand` poprzez użycie prefiksu `rand::`.

Następnie dodajemy kolejną linię z `use`: `use rand::Rng`. `Rng` to cecha (ang. *trait*), która definiuje metody implementowane przez generator liczb losowych, i cecha ta musi być widoczna w zakresie, gdzie chcemy tych metod używać. Rozdział 10 omówi cechy szczegółowo.

Dodajemy również dwie linie w mainie. Funkcja `rand::thread_rng` dostarczy nam do użycia konkretny generator liczb losowych: taki, który jest lokalny dla wątku wywołującego i seedowany z systemu operacyjnego. Następnie wywołujemy metodę `gen_range` tego generatora. Ta metoda zdefiniowana jest w cesze `Rng`, którą zaciągnęliśmy poprzez wyrażenie `use rand::Rng`. Metoda `gen_range` przyjmuje dwa argumenty liczbowe i generuje liczbę losową z zakresu pomiędzy tymi liczbami. Do zakresu wchodzi dolna wartość graniczna, ale górna już nie, zatem aby uzyskać liczbę spomiędzy 1 a 100, musimy przekazać liczby `1` i `101`.

Wiedza, której cechy użyć i które funkcje i metody ze skrzyni wywoływać, nie jest czymś co po prostu *wiesz*. Instrukcja jak używać danej skrzyni znajduje się zawsze w jej dokumentacji.

Kolejną przydatną komendą Cargo jest polecenie `cargo doc --open`, które lokalnie zbuduje dokumentację ddostarczaną przez wszystkie zależności, jakich używasz, i otworzy ją w przeglądarce. Gdyby, przykładowo, interesowały cię inne funkcjonalności ze skrzyni `rand`, wpisz `cargo doc --open`i wybierz `rand` z paska po lewej.

Druga dodana przez nas linia wypisuje na ekranie sekretną liczbę. Jest to przydatne podczas tworzenia programu, aby móc go testować, i zostanie usunięte w finalnej wersji. Gra nie byłaby zbyt ekscytująca, gdyby program podawał sekretną liczbę od razu na starcie!

Spróbuj uruchomić program kilka razy:


```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4
$ cargo run
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

Za każdym razem powinieneś otrzymać inny sekretny numer, jednak zawsze z zakresu od 1 do 100. Dobra robota!


## Porównywanie odpowiedzi gracza z sekretnym numerem


Teraz, kiedy już mamy odpowiedź gracza i wylosowaną sekretną liczbę, możemy je porównać. Ten krok przedstawiony jest na Listingu 2-4:


<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

<span class="caption">Listing 2-4: Obsługa możliwych rezultatów operacji porównywania dwóch liczb</span>

Doszło tu kolejne `use`, które wprowadza nam do zakresu typ `std::cmp::Ordering` z biblioteki standardowej. `Ordering` jest enumem, takim jak `Result`, ale ma inne warianty: `Less`,
`Greater`, i `Equal`. Są to trzy możliwe wyniki porównywania dwóch wartości.

Następnie dopisaliśmy na końcu pięć nowych linii wykorzystujących typ `Ordering`:

```rust,ignore
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

Metoda `cmp` porównuje dwie wartości. Można wywołać ją na dowolnym obiekcie, który da się porównywać. Przyjmuje ona referencję do drugiego obiektu, z którym chcemy porównać pierwszy: tutaj porównujemy `guess` do `secret_number`. `cmp` zwraca wariant enuma `Ordering` (którego typ zaciągnęliśmy poprzez wyrażenie `use`). Za pomocą wyrażenia [`match`][match]<!-- ignore -->, na podstawie wartości `Ordering` zwróconej przez wywołanie `cmp` z wartościami `guess` z `secret_number`, decydujemy co zrobić dalej.


[match]: ch06-02-match.html

Wyrażenie `match` składa się z *ramion*. Ramię składa się ze *wzorca* i kodu, który ma się wykonać, jeśli wartość podana na początku wyrażenia `match` będzie pasowała do wzorca z danego ramienia.
Rust bierze wartość podaną do `match` i przegląda kolejno wzorce ze wszystkich ramion. Konstrukcja `match` i wzorce to potężne featury w Ruście, które pozwolą ci wyrazić wiele różnych scenariuszy w twoim kodzie i pomogą ci zapewnić obsługę ich wszystkich.
Te featury zostaną omówione szczegółowo odpowiednio w Rodziale 6 i Rozdziale 18.

Przeanalizujmy na przykładzie, co dokładnie dzieje się z użytym tutaj wyrażeniem `match`. Powiedzmy, że użytkownik wybrał liczbę 50, a losowo wygenerowana sekretna liczba to 38.
Kiedy kod porówna 50 do 38, metoda `cmp` zwróci wartość `Ordering::Greater`, ponieważ 50 jest większe niż 38. Zatem `match` otrzymuje tutaj wartość `Ordering::Greater`.
`Match` sprawdza wzorzec w pierwszym ramieniu, `Ordering::Less`, ale wartość `Ordering::Greater` nie pasuje do wzorca `Ordering::Less`, więc ignoruje kod w tym ramieniu i przechodzi do następnego.
Wzorzec z następnego ramienia, `Ordering::Greater`, *pasuje* do `Ordering::Greater`! Powiązany kod w tym ramieniu jest wykonywany i na ekranie pojawia się napis `Too big!`.
Wyrażenie `match` kończy wykonanie, ponieważ nie ma potrzeby sprawdzać już ostatniego ramienia w tym scenariuszu.


Jednakże kod z Listingu 2-4 jeszcze się nie skompiluje. Spróbujmy:


```text
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:23:21
   |
23 |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected struct `std::string::String`, found integral variable
   |
   = note: expected type `&std::string::String`
   = note:    found type `&{integer}`

error: aborting due to previous error
Could not compile `guessing_game`.
```

Komunikat błędu wskazuje, że *typy są niezgodne*. Rust jest silnie, statycznie typowanym językiem. Jednak również wspiera dedukcję typów.
Kiedy napisaliśmy `let guess = String::new()`, Rust potrafił wywnioskować, że `guess` powinno być `Stringiem`, dzięki czemu nie musieliśmy pisać typu jawnie.
Z drugiej strony, `secret_number` jest typem numerycznym. Wiele typów numerycznych może przyjmować wartość spomiędzy 1 a 100: `i32`, 32-bitowa liczba całkowita;
`u32`, 32-bitowa liczba całkowita bez znaku; `i64`, 64-bitowa liczba całkowita; a także inne. Domyślnie Rust wybiera `i32`, co jest typem `secret_number`,
jeśli nie wpisaliśmy gdzieś indziej w kodzie jakiejś informacji, która spowoduje że Rust wybierze inny typ. Przyczyną błędu jest to, że Rust nie potrafi porównywać
stringa z typem numerycznym.

Ostatecznie musimy przekonwertować stringa, którego program wczytał jako wejście z klawiatury,
do postaci typu numerycznego, który można porównać matematycznie do sekretnej liczby. Możemy to osiągnąć, dodając takie
dwie linie do ciała funkcji `main`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
// --snip--

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

Dodane linie to:

```rust,ignore
let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
```

Tworzymy zmienną o nazwie `guess`. Ale czekaj, czy program przypadkiem nie ma już
zmiennej o takiej nazwie? Owszem ma, ale Rust pozwala *przesłaniać* poprzednią wartość
zmiennej `guess` nową wartością. Ta funkcjonalność jest często używana w sytuacjach, gdy
konieczna jest konwersja wartości z jednego typu do drugiego. Przesłanianie (*shadowing*)
pozwala użyć ponownie nazwy `guess`, zamiast zmuszać nas do tworzenia dwóch osobnych zmiennych,
takich jak przykładowo `guess_str` i `guess`. (Rozdział 3 opowiada więcej o przesłanianiu zmiennych.)

Przypisujemy do zmiennej `guess` wartość wyrażenia `guess.trim().parse()`. Tutaj zmienna `guess`
odnosi się do pierwotnej zmiennej `guess`, która była `Stringiem` z wczytanymi danymi z klawiatury.
Metoda `trim` z interfejsu `Stringa` spowoduje usunięcie wszelkich białych znaków znajdujących
się na początku lub końcu stringa. Aby sparsować `String` do typu `u32`, `String` ten powinien zawierać jedynie znaki numeryczne, jednak użytkownik musi
wcisnąć <span class="keystroke">enter</span>, aby zadowolić funkcję `read_line`. Po wciśnięciu <span class="keystroke">enter</span>
znak nowej linii jest dopisywany do stringa. Przykładowo, jeśli użytkownik wpisał <span
class="keystroke">5</span> i wcisnął <span class="keystroke">enter</span>, to `guess` przyjmie postać: `5\n`.
Znak `\n` reprezentuje nową linię, czyli wynik wciśnięcia <span class="keystroke">enter</span>. Metoda `trim`
usunie niechciane `\n`, dzięki czemu w stringu pozostanie jedynie `5`.

[Metoda `parse`][parse]<!-- ignore --> parsuje string do postaci jakiegoś typu numerycznego. Ponieważ wynikowa liczba
może być różnego typu, musimy powiedzieć Rustowi, jakiego dokładnie typu oczekujemy, używając wyrażenia `let guess: u32`.
Dwukropek (`:`) po `guess` informuje Rusta, że podajemy typ zmiennej. Rust ma kilka wbudowanych typów numerycznych;
`u32`, jaką tu podaliśmy, to 32-bitowa liczba całkowita bez znaku. Jest to dobry domyślny wybór dla małych liczb dodatnich.
O innych typach numerycznych przeczytasz w Rozdziale 3. Dodatkowo, dzięki anotacji `u32` w tym przykładowym programie
i porównaniu tej liczby z `secret_number`, Rust wywnioskuje, że `secret_number` powinien też być typu `u32`. Zatem
teraz porównanie zachodzi pomiędzy dwiema wartościami tego samego typu!

[parse]: ../std/primitive.str.html#method.parse

Wywołanie `parse` często może zakończyć się niepowodzeniem. Jeśli, na przykład, string będzie zawierał
`A👍%`, to jego konwersja do liczby nie może się udać. Z tego względu metoda `parse` zwraca
typ `Result`, podobnie jak metoda `read_line` (wspominaliśmy o tym wcześniej w sekcji
[“Obsługa potencjalnych błędów z użyciem typu `Result`”](#handling-potential-failure-with-the-result-type)<!-- ignore
-->). Potraktujemy ten `Result` w ten sam sposób, używając ponownie metody `expect`. Jeśli `parse` zwróci wariant `Err`
(ponieważ nie udało się stworzyć liczby ze stringa), wywołanie `expect` scrashuje grę i wypisze na ekran
podany przez nas tekst. Gdy zaś `parse` powiedzie się i poprawnie skonwertuje stringa do liczby, zwrócony `Result`
będzie wariantem `Ok`, a `expect` zwróci liczbę zaszytą w wartości `Ok`.

Teraz uruchomimy program!

```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43 secs
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

Nieźle! Pomimo tego że dodaliśmy spacje przed liczbą, program wciąż poprawnie rozpoznał,
że użytkownik wybrał liczbę 76. Uruchom program kilka razy, aby sprawdzić jak program reaguje na
różne wejścia: podaj właściwą liczbę, za wysoką, następnie za niską.

Nasza gra już z grubsza działa, ale użytkownik może odgadywać liczbę tylko jeden raz. Zmieńmy to
dodając pętlę!

## Wielokrotne zgadywanie dzięki pętli

Słowo kluczowe `loop` (*pętla*) tworzy pętlę nieskończoną. Dodamy taką pętlę, żeby dać
graczowi więcej szans na odgadnięcie liczby:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
// --snip--

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

Jak widzisz, przenieślismy do pętli cały kod następujący po zachęcie gracza do odgadnięcia liczby.
Pamiętaj, żeby zwiększyć wcięcia linii wewnątrz pętli o kolejne cztery spacje, następnie uruchom program
ponownie. Zapewne zauważyłeś nowy problem - program robi dokładnie to, o co go poprosiliśmy: pyta o wprowadzenie
odgadniętej liczby w nieskończoność! Wygląda na to, że użytkownik nie może wyjść z tego programu!

Użytkownik może zawsze zatrzymać program używając skrótu klawiszowego <span class="keystroke">ctrl-c</span>. Lecz
jest jeszcze inny sposób, żeby uciec temu nienasyconemu potworowi, jak wspomnieliśmy w dyskusji o `parse`
w [“Porównywanie odpowiedzi gracza z sekretnym numerem”](#comparing-the-guess-to-the-secret-number)<!--
ignore -->: wprowadzenie znaku, który nie jest liczbą, spowoduje zawieszenie się programu. Można z tego skorzystać,
aby wyjść z programu, tak jak pokazujemy poniżej:


```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50 secs
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:785
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/guess` (exit code: 101)
```

Wpisanie `quit` faktycznie spowodowało wyjście z programu, ale taki sam skutek miałoby wprowadzenie
dowolnego innego ciągu znaków nienumerycznych. Co nie zmienia faktu, że zamykanie programu w ten sposób nie jest
zbyt optymalne. Chcielibyśmy raczej, aby gra zatrzymała się automatycznie, kiedy gracz wprowadzi poprawny numer.


### Wychodzenie z programu po poprawnym odgadnięciu liczby

Let’s program the game to quit when the user wins by adding a `break` statement:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
// --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

Dodanie linii `break` po `You win!` powoduje, że program opuszcza pętlę, gdy gracz odgadnie poprawnie
sekretny numer. Wyjście z pętli jest równoważne z zakończeniem pracy programu, ponieważ pętla jest ostatnią
częścią funkcji `main`.


### Obsługa niepoprawnych danych wejściowych

W celu dalszego ulepszenia gry zróbmy tak, żeby program, zamiast zawieszać się, ignorował wprowadzone dane nienumeryczne,
a użytkownik mógł zgadywać dalej. Możemy to osiągnąć edytując linię, w której `guess` jest konwertowane ze `Stringa` do
`u32`, w sposób przedstawiony na Listingu 2-5.


<span class="filename">Filename: src/main.rs</span>

```rust,ignore
// --snip--

io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

println!("You guessed: {}", guess);

// --snip--
```

<span class="caption">Listing 2-5: Ignorowanie wejścia nieliczbowego i pytanie o kolejne liczby,
zamiast zawieszania programu</span>

Zamiana wywołania `expect` na wyrażenie `match` to ogólny sposób, w jaki zamienia się
program zawieszający się na program obsługujący błędy. Pamiętaj, że typem zwracanym przez
`parse` jest `Result`, a `Result` jest typem wyliczeniowym, który ma warianty `Ok` oraz `Err`.
Używamy tutaj wyrażenia `match`, podobnie jak robiliśmy to z wynikiem `Ordering` zwracanym przez
metodę `cmp`.

Jeśli `parse` jest w stanie pomyślnie zamienić stringa w liczbę, zwróci wartość `Ok`, zawierającą
liczbę otrzymaną w konwersji. Wartość `Ok` odpowiada wzorcowi z pierwszej gałęzi `match`, zatem
`match` zwróci wartość `num`, która została obliczona i zapisana wewnątrz wartości `Ok` przez metodę `parse`.
Ta liczba zostanie zapisana do nowoutworzonej przez nas zmiennej `guess`.

Jeśli jednak `parse` *nie* jest w stanie przekonwertować stringa na liczbę, zwróci wartość `Err`,
która zawiera dodatkowe informacje o błędzie. Wartość `Err` nie pasuje do wzorca `Ok(num)` z pierwszej
gałęzi `match`, ale pasuje do wzorca `Err(_)` z drugiej gałęzi. Podkreślnik, `_`, pasuje do wszystkich wartości;
w tym przypadku mówimy, że do wzorca mają pasować wszystkie wartości `Err`, bez znaczenia na to jakie dodatkowe informacje
mają one w środku. Program zatem wykona instrukcje z drugiego ramienia, `continue`, co oznacza że program ma przejść
do kolejnej iteracji pętli i poprosić o nową liczbę. Dzięki temu program ignoruje wszystkie problemy jakie może napotkać
`parse`!

Teraz wszystko w naszym programie powinno działać zgodnie z oczekiwaniami. Wypróbujmy to:


```text
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

Wspaniale! Jeszcze jedna drobna poprawka i nasza gra w zgadywankę będzie już skończona.
Program wciąż wyświetla sekretny numer. To było przydatne podczas testów, ale na dłuższą metę psułoby zabawę.
Usuńmy `println!` odpowiedzialną za wyświetlanie sekretnego numeru. Listing 2-6 pokazuje końcową wersję programu.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

<span class="caption">Listing 2-6: Kompletna gra w zgadywankę</span>

## Podsumowanie

Właśnie udało ci się zbudować grę w zgadywankę. Gratulacje!

Ten projekt w praktyczny sposób zapoznał cię z wieloma konceptami Rusta:
`let`, `match`, metodami, funkcjami powiązanymi, używaniem zewnętrznych skrzyń,
i innymi. W najbliższych rozdziałach koncepty te będą omówione bardziej szczegółowo.
Rozdział 3 omawia koncepty obecne w większości języków programowania, takie jak zmienne,
typy danych czy funkcje, i prezentuje jak należy w nich korzystać w Ruście.
Rozdział 4 odkrywa system własności, mechanizm który wyróżna Rusta spośród innych języków.
Rozdział 5 omawia składnię struktur i metod, a Rozdział 6 wyjaśnia, jak działają typy numeryczne.


[variables-and-mutability]:
ch03-01-variables-and-mutability.html#variables-and-mutability

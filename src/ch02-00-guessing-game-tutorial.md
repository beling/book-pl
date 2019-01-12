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
Z drugiej strony, `secret_number` jest typem numerycznym. Wiele typów numerycznych może przyjmować wartość spomiędzy 1 a 100: `i32`, a 32-bitowa liczba całkowita;
`u32`, 32-bitowa liczba całkowita bez znaku; `i64`, 64-bitowa liczba całkowita; a także inne. Domyślnie Rust wybiera `i32`, co jest typem `secret_number`,
jeśli nie wpisaliśmy gdzieś indziej w kodzie jakiejś informacji, która spowoduje że Rust wybierze inny typ. Przyczyną błędu jest to, że Rust nie potrafi porównywać
stringa z typem numerycznym.

Ultimately, we want to convert the `String` the program reads as input into a
real number type so we can compare it numerically to the guess. We can do that
by adding the following two lines to the `main` function body:

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

The two new lines are:

```rust,ignore
let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
```

We create a variable named `guess`. But wait, doesn’t the program already have
a variable named `guess`? It does, but Rust allows us to *shadow* the previous
value of `guess` with a new one. This feature is often used in situations in
which you want to convert a value from one type to another type. Shadowing lets
us reuse the `guess` variable name rather than forcing us to create two unique
variables, such as `guess_str` and `guess` for example. (Chapter 3 covers
shadowing in more detail.)

We bind `guess` to the expression `guess.trim().parse()`. The `guess` in the
expression refers to the original `guess` that was a `String` with the input in
it. The `trim` method on a `String` instance will eliminate any whitespace at
the beginning and end. Although `u32` can contain only numerical characters,
the user must press <span class="keystroke">enter</span> to satisfy
`read_line`. When the user presses <span class="keystroke">enter</span>, a
newline character is added to the string. For example, if the user types <span
class="keystroke">5</span> and presses <span class="keystroke">enter</span>,
`guess` looks like this: `5\n`. The `\n` represents “newline,” the result of
pressing <span class="keystroke">enter</span>. The `trim` method eliminates
`\n`, resulting in just `5`.

The [`parse` method on strings][parse]<!-- ignore --> parses a string into some
kind of number. Because this method can parse a variety of number types, we
need to tell Rust the exact number type we want by using `let guess: u32`. The
colon (`:`) after `guess` tells Rust we’ll annotate the variable’s type. Rust
has a few built-in number types; the `u32` seen here is an unsigned, 32-bit
integer. It’s a good default choice for a small positive number. You’ll learn
about other number types in Chapter 3. Additionally, the `u32` annotation in
this example program and the comparison with `secret_number` means that Rust
will infer that `secret_number` should be a `u32` as well. So now the
comparison will be between two values of the same type!

[parse]: ../std/primitive.str.html#method.parse

The call to `parse` could easily cause an error. If, for example, the string
contained `A👍%`, there would be no way to convert that to a number. Because it
might fail, the `parse` method returns a `Result` type, much as the `read_line`
method does (discussed earlier in [“Handling Potential Failure with the
`Result` Type”](#handling-potential-failure-with-the-result-type)<!-- ignore
-->). We’ll treat this `Result` the same way by using the `expect` method
again. If `parse` returns an `Err` `Result` variant because it couldn’t create
a number from the string, the `expect` call will crash the game and print the
message we give it. If `parse` can successfully convert the string to a number,
it will return the `Ok` variant of `Result`, and `expect` will return the
number that we want from the `Ok` value.

Let’s run the program now!

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

Nice! Even though spaces were added before the guess, the program still figured
out that the user guessed 76. Run the program a few times to verify the
different behavior with different kinds of input: guess the number correctly,
guess a number that is too high, and guess a number that is too low.

We have most of the game working now, but the user can make only one guess.
Let’s change that by adding a loop!

## Allowing Multiple Guesses with Looping

The `loop` keyword creates an infinite loop. We’ll add that now to give users
more chances at guessing the number:

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

As you can see, we’ve moved everything into a loop from the guess input prompt
onward. Be sure to indent the lines inside the loop another four spaces each
and run the program again. Notice that there is a new problem because the
program is doing exactly what we told it to do: ask for another guess forever!
It doesn’t seem like the user can quit!

The user could always halt the program by using the keyboard shortcut <span
class="keystroke">ctrl-c</span>. But there’s another way to escape this
insatiable monster, as mentioned in the `parse` discussion in [“Comparing the
Guess to the Secret Number”](#comparing-the-guess-to-the-secret-number)<!--
ignore -->: if the user enters a non-number answer, the program will crash. The
user can take advantage of that in order to quit, as shown here:

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

Typing `quit` actually quits the game, but so will any other non-number input.
However, this is suboptimal to say the least. We want the game to automatically
stop when the correct number is guessed.

### Quitting After a Correct Guess

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

Adding the `break` line after `You win!` makes the program exit the loop when
the user guesses the secret number correctly. Exiting the loop also means
exiting the program, because the loop is the last part of `main`.

### Handling Invalid Input

To further refine the game’s behavior, rather than crashing the program when
the user inputs a non-number, let’s make the game ignore a non-number so the
user can continue guessing. We can do that by altering the line where `guess`
is converted from a `String` to a `u32`, as shown in Listing 2-5.

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

<span class="caption">Listing 2-5: Ignoring a non-number guess and asking for
another guess instead of crashing the program</span>

Switching from an `expect` call to a `match` expression is how you generally
move from crashing on an error to handling the error. Remember that `parse`
returns a `Result` type and `Result` is an enum that has the variants `Ok` or
`Err`. We’re using a `match` expression here, as we did with the `Ordering`
result of the `cmp` method.

If `parse` is able to successfully turn the string into a number, it will
return an `Ok` value that contains the resulting number. That `Ok` value will
match the first arm’s pattern, and the `match` expression will just return the
`num` value that `parse` produced and put inside the `Ok` value. That number
will end up right where we want it in the new `guess` variable we’re creating.

If `parse` is *not* able to turn the string into a number, it will return an
`Err` value that contains more information about the error. The `Err` value
does not match the `Ok(num)` pattern in the first `match` arm, but it does
match the `Err(_)` pattern in the second arm. The underscore, `_`, is a
catchall value; in this example, we’re saying we want to match all `Err`
values, no matter what information they have inside them. So the program will
execute the second arm’s code, `continue`, which tells the program to go to the
next iteration of the `loop` and ask for another guess. So effectively, the
program ignores all errors that `parse` might encounter!

Now everything in the program should work as expected. Let’s try it:

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

Awesome! With one tiny final tweak, we will finish the guessing game. Recall
that the program is still printing the secret number. That worked well for
testing, but it ruins the game. Let’s delete the `println!` that outputs the
secret number. Listing 2-6 shows the final code.

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

<span class="caption">Listing 2-6: Complete guessing game code</span>

## Summary

At this point, you’ve successfully built the guessing game. Congratulations!

This project was a hands-on way to introduce you to many new Rust concepts:
`let`, `match`, methods, associated functions, the use of external crates, and
more. In the next few chapters, you’ll learn about these concepts in more
detail. Chapter 3 covers concepts that most programming languages have, such as
variables, data types, and functions, and shows how to use them in Rust.
Chapter 4 explores ownership, a feature that makes Rust different from other
languages. Chapter 5 discusses structs and method syntax, and Chapter 6
explains how enums work.

[variables-and-mutability]:
ch03-01-variables-and-mutability.html#variables-and-mutability

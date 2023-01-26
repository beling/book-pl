## Witaj, Cargo!

Cargo jest menedżerem pakietów i systemem budowania Rusta. Większość Rustowców
używa tego narzędzia do zarządzania swoimi projektami, ponieważ Cargo potrafi
wykonać za nich wiele zadań, takich jak budowanie kodu oraz ściąganie i
budowanie bibliotek, od których kod jest zależny. Biblioteki, których wymaga
twój kod nazywamy *zależnościami* (*dependencies*).

Najprostsze programy w Ruście, takie jak ten, który właśnie napisaliśmy, nie
mają żadnych zależności, więc gdybyśmy zbudowali projekt „Witaj, świecie!” (ang. „Hello World!”) za pomocą
Cargo, zostałaby użyta tylko ta część narzędzia, która zajmuje się budowaniem
kodu. W miarę pisania bardziej skomplikowanych programów, zechcesz dodawać
zależności i jeśli swój projekt rozpoczniesz z użyciem Cargo, będzie to
o wiele łatwiejsze do zrobienia.

Jako że przeważająca większość projektów w Ruście używa Cargo, w dalszym ciągu
książki założymy, że ty również go używasz. Jeśli korzystałeś(-aś) z oficjalnych
instalatorów, zgodnie z opisem w sekcji
[„Instalacja”][installation]<!-- ignore -->, Cargo zainstalował się razem z
Rustem. Jeżeli instalowałeś(-aś) Rusta w inny sposób, możesz sprawdzić, czy Cargo
jest zainstalowany, przez wprowadzenie w terminalu następującej komendy:

```console
$ cargo --version
```

Jeżeli widzisz numer wersji, Cargo jest zainstalowane! Jeśli natomiast pojawia
się błąd z rodzaju `komendy nie znaleziono`, powinieneś/powinnaś zajrzeć do dokumentacji
swojej metody instalacji celem ustalenia, jak zainstalować Cargo osobno.

### Tworzenie projektu z Cargo

Stwórzmy nowy projekt z pomocą Cargo i przyjrzyjmy się, czym różni się on od
naszego pierwotnego projektu „Witaj, świecie!”. Przejdź z powrotem do swojego
katalogu *projects* (lub innego, w którym zdecydowałeś(-aś) się trzymać swój kod) i
bez względu na posiadany system operacyjny wprowadź polecenie:

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

Pierwsze polecenie stworzy nowy katalog o nazwie *hello_cargo*. Ponieważ nadaliśmy naszemu
projektowi nazwę *hello_cargo*, Cargo tworzy jego pliki źródłowe w katalogu o
tej samej nazwie.

Wejdź do katalogu *hello_cargo* i wyświetl listę plików. Powinieneś(-aś) zobaczyć, że
Cargo utworzył dla nas dwa pliki i jeden podkatalog: plik *Cargo.toml* oraz
katalog *src* z plikiem *main.rs* wewnątrz. Zainicjował również nowe
repozytorium Gita, w komplecie z plikiem *.gitignore*.
Gdybyś jednak wykonał(a) komendę `cargo new` w folderze, w którym istnieje już
repozytorium Gita, to pliki związane z Gitem nie zostałyby stworzone.
Możesz zmienić to zachowanie używając komendy `cargo new --vcs=git`.

> Uwaga: Git jest często stosowanym systemem kontroli wersji. Możesz zlecić
> `cargo new` zastosowanie innego systemu kontroli wersji lub też nie stosowanie
> żadnego, za pomocą flagi `--vcs`. Uruchom `cargo new --help`, żeby zobaczyć
> dostępne opcje.

Otwórz plik *Cargo.toml* w wybranym przez siebie edytorze tekstu. Zawartość
powinna wyglądać podobnie do kodu z listingu 1-2:

<span class="filename">Plik: Cargo.toml</span>

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

<span class="caption">Listing 1-2: Zawartość pliku *Cargo.toml* wygenerowanego
przez `cargo new`</span>

Plik jest w formacie [*TOML*][toml]<!-- ignore --> (Tom’s Obvious, Minimal
Language) (*Oczywisty, Minimalistyczny Język Toma - przyp. tłum.*), którego
Cargo używa do konfiguracji.

Pierwsza linia, `[package]`, jest nagłówkiem sekcji, której kolejne wyrażenia
konfigurują pakiet. W miarę dodawania informacji do tego pliku, dodamy też inne
sekcje.

Następne trzy linie ustalają informacje konfiguracyjne, których Cargo
potrzebuje do kompilacji twojego programu: nazwę, wersję, i dane o
używanej edycji Rusta. O kluczu `edition` będzie mowa w [Dodatku E][appendix-e]<!-- ignore -->.

Ostatnia linia, `[dependencies]`, rozpoczyna sekcję, gdzie wyszczególnia się
wszystkie zależności twojego projektu. W Ruście pakiety z kodem źródłowym
nazywane są *skrzyniami* (*crates*). Do tego projektu nie będziemy potrzebować
żadnych skrzyń, ale do programu w rozdziale drugim owszem. Wówczas przyda nam
się sekcja zależności.

Otwórz teraz plik *src/main.rs* i przyjrzyj się zawartości:

<span class="filename">Plik: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo wygenerował dla ciebie program „Witaj, świecie!”, prawie taki sam jak ten, jak
napisaliśmy w listingu 1-1 (tyle, że w języku angielskim)! Różnice między naszym poprzednim
projektem, a tym wygenerowanym przez Cargo są takie, że w Cargo kod źródłowy
trafia do podkatalogu *src*, a w katalogu głównym pozostaje plik
konfiguracyjny *Cargo.toml*.

Cargo zakłada, że kod źródłowy znajduje się w podkatalogu *src*, dzięki czemu
katalog główny wykorzystany jest tylko na pliki README, informacje o licencjach,
pliki konfiguracyjne i wszystko inne, co nie jest kodem. Używanie Cargo pomaga
utrzymać ci własne projekty w należytym porządku. Na wszystko jest miejsce i
wszystko jest na swoim miejscu.

Jeżeli zacząłeś(-aś) jakiś projekt bez użycia Cargo, taki jak nasz poprzedni z
katalogu *hello_world*, możesz przekonwertować go na wersję kompatybilną z
Cargo, przenosząc kod źródłowy do podkatalogu *src* i tworząc odpowiedni
plik *Cargo.toml*.

### Budowanie i uruchamianie projektu z Cargo

Przyjrzyjmy się teraz, jakie są różnice w budowaniu i uruchamianiu programu
„Witaj, świecie!” poprzez Cargo. Aby zbudować swój projekt, z poziomu głównego
katalogu *hello_cargo* wprowadź polecenie:

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

Spowoduje to utworzenie pliku wykonywalnego w *target/debug/hello_cargo*
(lub *target\debug\hello_cargo.exe* pod Windowsem), zamiast w katalogu bieżącym.
Ponieważ domyślnie budowana jest wersja debug programu, Cargo umieszcza
skompilowany plik binarny w katalogu *debug*.
Plik można uruchomić następującym poleceniem:

```console
$ ./target/debug/hello_cargo # lub .\target\debug\hello_cargo.exe pod Windowsem
Hello, world!
```

Jeśli wszystko przebiegło prawidłowo, `Hello, world!` powinno
wyświetlić się w oknie terminala. Uruchomienie `cargo build` za pierwszym razem
powoduje dodatkowo utworzenie przez Cargo nowego pliku w katalogu głównym o
nazwie *Cargo.lock*, który wykorzystywany jest do śledzenia dokładnych wersji
zależności w twoim projekcie. Ponieważ bieżący projekt nie posiada zależności,
zawartość pliku jest dość licha. Nie będziesz musiał własnoręcznie modyfikować
tego pliku; Cargo zajmie się tym za ciebie.

Właśnie zbudowaliśmy projekt poleceniem `cargo build` i uruchomiliśmy go przez
`./target/debug/hello_cargo`. Możemy również użyć `cargo run`, żeby skompilować
i uruchomić program za jednym rzutem:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Większość programistów używa `cargo run`, bo jest ono wygodniejsze i łatwiejsze do zapamiętania od użycia `cargo build` i pełnej ścieżki do zbudowanego pliku binarnego.

Zauważ, że tym razem nie wyświetliła się informacja o tym, że Cargo kompilował
`hello_cargo`. Program wywnioskował, że zawartość plików nie uległa zmianie,
więc po prostu uruchomił binarkę. Gdyby kod źródłowy został zmodyfikowany, Cargo
przebudowałby projekt przed jego uruchomieniem i wówczas informacja na ekranie
wyglądałaby następująco:

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Mamy jeszcze do dyspozycji `cargo check`. To polecenie szybko sprawdzi twój kod,
celem upewnienia się, że skompilowałby się on prawidłowo, ale nie tworzy pliku
wykonywalnego:

```console
$ cargo check
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Jaki jest cel pomijania tworzenia binarki? Taki, że `cargo check` jest często o
wiele szybsze od `cargo build`, ponieważ cały krok generowania pliku
wykonywalnego jest pomijany. Jeśli masz zwyczaj sprawdzać swoją pracę w trakcie
pisania kodu, użycie `cargo check` przyspieszy proces! Wielu Rustowców okresowo
uruchamia `cargo check`, żeby sprawdzić, czy wszystko się kompiluje, a
`cargo build` dopiero, kiedy zdecydują się na uruchomienie binarki.

Podsumujmy, co do tej pory nauczyliśmy się o Cargo:

* Możemy stworzyć projekt używając `cargo new`.
* Możemy zbudować projekt poleceniami `cargo build`.
* Możemy zbudować i uruchomić projekt w jednym kroku z użyciem `cargo run`.
* Możemy zbudować projekt bez tworzenia binarki, aby sprawdzić błędy używając
  `cargo check`.
* Zamiast umieszczać pliki wynikowe budowania w tym samym katalogu co nasz kod,
  Cargo umieści je w podkatalogu *target/debug*.

Dodatkową zaletą używania Cargo jest to, że polecenia są identyczne, bez
względu na system operacyjny, pod którym pracujesz. Od tego momentu nie będziemy
zatem podawać osobnych instrukcji dla Linuksa i macOS czy Windowsa.

### Budowanie do publikacji

Gdy twój projekt jest już gotowy do publikacji, możesz użyć polecenia
`cargo build --release` celem przeprowadzenia kompilacji z optymalizacjami.
Spowoduje to utworzenie pliku wykonywalnego w podkatalogu *target/release*
zamiast w *target/debug*. Przeprowadzone optymalizacje sprawiają, że twój
program w Ruście wykonuje się szybciej, ale jednocześnie jego kompilacja trwa
dłużej. Dlatego też istnieją dwa różne profile: jeden rozwojowy, kiedy zależy ci
na szybkim i częstym budowaniu oraz drugi, przeznaczony do zbudowania
ostatecznej wersji, która trafi do użytkownika. Nie będzie ona wielokrotnie
przebudowywana, ale wykonywać się będzie tak szybko, jak to tylko możliwe.
Jeżeli prowadzisz testy czasu wykonywania swojego kodu, pamiętaj o wywołaniu
`cargo build --release` i testowaniu pliku wykonywalnego z
podkatalogu *target/release*.

### Cargo jako konwencja

W przypadku prostych projektów, Cargo nie wnosi wielu korzyści w porównaniu z
używaniem prostego `rustc`, ale udowodni swoją wartość w miarę postępów. Przy
skomplikowanych projektach złożonych z wielu plików lub mających wiele zależności,
zezwolenie Cargo na koordynację budowania znacznie ułatwia pracę.

Nawet jeśli projekt `hello_cargo` jest prosty, używa już sporej części arsenału
narzędzi, z którymi będziesz mieć do czynienia przez pozostały okres swojej
kariery z Rustem. Rzeczywiście, rozpoczęcie pracy nad jakimkolwiek istniejącym
projektem sprowadza się do wydania kilku poleceń: check out kodu w Git,
wejście do katalogu roboczego i budowanie:

```console
$ git clone przyklad.org/jakisprojekt
$ cd jakisprojekt
$ cargo build
```

Więcej informacji o Cargo można znaleźć w [jego dokumentacji][cargo].

## Podsumowanie

Jesteś już na dobrej drodze do rozpoczęcia podróży z Rustem! W tym rozdziale
nauczyłeś(-aś) się, jak:

* Zainstalować najnowszą, stabilną wersję Rusta z użyciem `rustup`,
* Uaktualnić Rusta do nowszej wersji,
* Otwierać lokalną kopię dokumentacji,
* Napisać program „Witaj, świecie!” używając bezpośrednio `rustc`,
* Stworzyć i uruchomić nowy projekt używając konwencji Cargo.

To świetna pora na stworzenie poważniejszego programu, aby przyzwyczaić się do
czytania i pisania kodu w Ruście. W następnym rozdziale zbudujemy program
grający w zgadywankę. Jeśli jednak chcesz zacząć naukę o działaniu powszechnych
pojęć programistycznych w Ruście, przejdź do rozdziału 3, a następnie wróć do
rozdziału 2.

[installation]: ch01-01-installation.html#instalacja
[toml]: https://toml.io
[appendix-e]: appendix-05-editions.html
[cargo]: https://doc.rust-lang.org/cargo/

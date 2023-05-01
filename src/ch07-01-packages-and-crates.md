<!-- ## Packages and Crates -->
## Pakiety i Skrzynie

Pierwszymi elementami systemu modułów, które omówimy są pakiety i skrzynie.

*Skrzynka* (ang. *crate*) jest najmniejszą jednostką kodu braną jednorazowo pod uwagę przez kompilator Rusta.
Nawet jeżeli uruchomimy `rustc` zamiast `cargo` i podamy pojedynczy plik z kodem źródłowym (tak jak zrobiliśmy to w sekcji „Pisanie i uruchamianie programu w Rust“ rozdziału 1), kompilator potraktuje ten plik jako skrzynię.
Skrzynie mogą zawierać moduły, a moduły mogą być zdefiniowane w innych plikach, które są kompilowane razem ze skrzynią, co zostanie pokazane dalej.

Skrzynia może przybrać jedną z dwóch form: skrzyni binarnej lub skrzyni bibliotecznej.
*Skrzynie binarne* (ang. *binary crate*) to programy, które można skompilować do postaci wykonywalnej, możliwej do uruchomienia. Może to być np. program linii poleceń lub serwer. Każda skrzynia binarna musi posiadać funkcję o nazwie `main`, która definiuje, co się stanie, gdy program zostanie uruchomiony. Wszystkie skrzynie, które stworzyliśmy do tej pory były skrzyniami binarnymi.

*Skrzynie biblioteczne* (ang. *library crate*) nie mają funkcji `main` i nie kompilują się do pliku wykonywalnego. Zamiast tego definiują funkcjonalność przewidzianą do współdzielenia przez wiele projektów. Na przykład skrzynia `rand`, której używaliśmy w [rozdziale 2][rand]<!-- ignore -->, zapewnia funkcjonalność generującą liczby losowe.
Gdy Rustowcy mówią po prostu „skrzynia“, zazwyczaj mają na myśli właśnie skrzynię biblioteczną i używają „skrzyni“ zamiennie z ogólnym, programistycznym pojęciem „biblioteki“.

*Korzeń skrzyni* (ang. *crate root*) jest plikiem źródłowym, od którego kompilator Rust rozpoczyna i który stanowi główny moduł skrzyni
(moduły wyjaśnimy dogłębnie w sekcji [„Defining Modules to Control Scope and Privacy“][modules]<!-- ignore -->).

*Pakiet* (ang. *package*) jest zapewniającym pewną funkcjonalność zestawem jednej lub więcej skrzyń.
Pakiet zawiera plik *Cargo.toml*, który opisuje jak te skrzynie zbudować.
Cargo w rzeczywistości jest pakietem, który zawiera binarną skrzynię z narzędziem wiersza poleceń, służącym do budowania kodu.
Pakiet Cargo zawiera również skrzynię biblioteczną, od której zależy skrzynia binarna.
Inne projekty mogą zależeć od skrzyni bibliotecznej Cargo, aby używać tej samej logiki, której używa narzędzie wiersza poleceń Cargo.

Pakiet może zawierać dowolną liczbę skrzyń binarnych i co najwyżej jedną skrzynię biblioteczną. Pakiet musi zawierać przynajmniej jedną skrzynię, niezależnie od tego czy jest to skrzynia biblioteczna czy binarna.

Prześledźmy, co się dzieje, gdy tworzymy pakiet.
Najpierw wpisujemy polecenie `cargo new`:

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

Po uruchomieniu `cargo new`, użyliśmy `ls` aby zobaczyć co Cargo utworzyło. W katalogu projektu znajduje się plik *Cargo.toml* konstytuujący pakiet. Jest też katalog *src*, który zawiera *main.rs*. Po otworzeniu *Cargo.toml* w edytorze tekstu, można zauważyć, że nie ma tam wzmianki o *src/main.rs*. Cargo stosuje konwencję, zgodnie z którą *src/main.rs* jest korzeniem skrzyni binarnej o tej samej nazwie co pakiet. Podobnie, Cargo wie, że jeśli katalog pakietu zawiera *src/lib.rs*, to pakiet zawiera bibliotekę o tej samej nazwie co pakiet, a *src/lib.rs* jest korzeniem jej skrzyni. Cargo przekazuje pliki skrzyni do `rustc`, aby zbudować bibliotekę lub program.

Tutaj mamy pakiet, który zawiera tylko *src/main.rs*, co oznacza, że zawiera tylko skrzynię binarną o nazwie `my-project`. Jeśli pakiet zawiera *src/main.rs* i *src/lib.rs*, to ma dwie skrzynie: binarną i biblioteczną, obie o tej samej nazwie co pakiet. Pakiet może mieć wiele skrzyń binarnych poprzez umieszczenie plików w katalogu *src/bin*: każdy plik będzie oddzielną skrzynią binarną.

[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#generowanie-losowej-liczby

## Możliwe do Odzyskania Błędy z `Result`

Większość błędów nie jest na tyle poważna, by wymagać całkowitego zatrzymania programu. 
Czasami, gdy funkcja zawodzi, dzieje się tak z powodu, który można łatwo 
zinterpretować i na który można łatwo zareagować. Na przykład, jeśli próbujesz 
otworzyć plik i operacja ta kończy się niepowodzeniem, ponieważ plik nie istnieje,
możesz chcieć utworzyć plik zamiast kończyć proces.

Przypomnijmy z [„Obsługa potencjalnych błędów z użyciem `Result`”][handle_failure]<!--
ignore --> w rozdziale 2, że enum `Result` jest zdefiniowane jako posiadające dwa
warianty, `Ok` i `Err`, w następujący sposób:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` i `E` są parametrami typów generycznych: omówimy je bardziej szczegółowo w rozdziale 10. W tym momencie należy wiedzieć, że `T` reprezentuje typ wartości, która zostanie zwrócona w przypadku sukcesu w wariancie `Ok`, a `E` reprezentuje typ błędu, który zostanie zwrócony w przypadku niepowodzenia w wariancie `Err`. Ponieważ `Result` ma te ogólne parametry typu, możemy użyć typu `Result` i funkcji zdefiniowanych na nim w wielu różnych sytuacjach, w których wartość sukcesu i wartość błędu, którą chcemy zwrócić, mogą się różnić.

Wywołajmy funkcję, która zwraca wartość `Result`, ponieważ funkcja może się nie powieść. 
Na listingu 9-3 próbujemy otworzyć plik.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

<span class="caption">Listing 9-3: Otwarcie pliku</span>

Typem zwracanym przez `File::open` jest `Result<T, E>`. Ogólny parametr `T`
został wypełniony przez implementację `File::open` typem wartości sukcesu,
`std::fs::File`, który jest uchwytem pliku. Typ `E` użyty w wartości błędu to
`std::io::Error`. Ten typ zwrotu oznacza, że wywołanie funkcji `File::open`
może się powieść i zwrócić uchwyt pliku, z którego możemy czytać lub do 
którego możemy pisać. Wywołanie funkcji może również zakończyć się niepowodzeniem:
na przykład plik może nie istnieć lub możemy nie mieć uprawnień dostępu do pliku.
Funkcja `File::open` musi mieć sposób na poinformowanie nas o powodzeniu 
lub niepowodzeniu i jednocześnie przekazać nam uchwyt pliku lub informacje o błędzie.
Te informacje są dokładnie tym, co przekazuje wyliczenie `Result`.

W przypadku gdy `File::open` się powiedzie, wartością w zmiennej `greeting_file_result` będzie instancja `Ok` zawierająca uchwyt pliku. W przypadku niepowodzenia, wartość w `greeting_file_result` będzie instancją `Err`, która zawiera więcej informacji o rodzaju błędu, który wystąpił.

Musimy dodać do kodu z listingu 9-3 różne akcje w zależności od wartości zwracanej przez `File::open`. Listing 9-4 pokazuje jeden ze sposobów obsługi `Result` przy użyciu podstawowego narzędzia, wyrażenia `match`, które omówiliśmy w rozdziale 6.

<span class="filename">Plik: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

<span class="caption">Listing 9-4: Użycie wyrażenia `match` do obsługi wariantów `Result`, które mogą zostać zwrócone</span>

Zauważ, że podobnie jak wyliczenie `Option`, wyliczenie `Result` i jego warianty zostały wprowadzone do zakresu przez preludium, więc nie musimy podawać `Result::` przed wariantami `Ok` i `Err` w elementach `match`.

Gdy wynikiem jest `Ok`, kod ten zwróci wewnętrzną wartość `file` z wariantu `Ok`, a następnie przypiszemy tę wartość uchwytu pliku do zmiennej `greeting_file`. Po `match`, możemy użyć uchwytu pliku do odczytu lub zapisu.

Drugie pole `match` obsługuje przypadek, w którym otrzymujemy wartość `Err` z `File::open`. W tym przykładzie wybraliśmy wywołanie makra `panic!`. Jeśli w naszym bieżącym katalogu nie ma pliku o nazwie *hello.txt* i uruchomimy ten kod, zobaczymy następujące dane wyjściowe z makra `panic!`:

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

Jak zwykle, te dane wyjściowe mówią nam dokładnie, co poszło nie tak.

### Dopasowanie na Podstawie Różnych Błędów

Kod z listingu 9-4 zgłosi `panic!` bez względu na przyczynę niepowodzenia `File::open`. Chcemy jednak podjąć różne działania dla różnych przyczyn niepowodzenia: jeśli `File::open` nie powiodło się, ponieważ plik nie istnieje, chcemy utworzyć plik i zwrócić uchwyt do nowego pliku. Jeśli `File::open` nie powiodło się z jakiegokolwiek innego powodu - na przykład, ponieważ nie mieliśmy uprawnień do otwarcia pliku - nadal chcemy, aby kod zgłosił `panic!` w taki sam sposób, jak na listingu 9-4. W tym celu dodajemy wewnętrzne wyrażenie `match`, pokazane na listingu 9-5.

<span class="filename">Plik: src/main.rs</span>

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

<span class="caption">Listing 9-5: Obsługa różnych rodzajów błędów na różne sposoby</span>

Typ wartości zwracanej przez `File::open` wewnątrz wariantu `Err` to `io::Error`, który jest strukturą dostarczaną przez bibliotekę standardową. Struktura ta posiada metodę `kind`, którą możemy wywołać, aby uzyskać wartość `io::ErrorKind`. Enum `io::ErrorKind` jest dostarczane przez bibliotekę standardową i posiada warianty reprezentujące różne rodzaje błędów, które mogą wynikać z operacji `io`. Wariant, którego chcemy użyć to `ErrorKind::NotFound`, który wskazuje, że plik, który próbujemy otworzyć jeszcze nie istnieje. Dopasowujemy się więc do `greeting_file_result`, ale mamy też wewnętrzne dopasowanie do `error.kind()`.

Warunkiem, który chcemy sprawdzić w wewnętrznym dopasowaniu jest to, czy wartość zwrócona przez `error.kind()` jest wariantem `NotFound` wyliczenia `ErrorKind`. Jeśli tak, próbujemy utworzyć plik za pomocą `File::create`. Jednakże, ponieważ `File::create` może również zawieść, potrzebujemy drugiego argumentu w wewnętrznym wyrażeniu `match`. Jeśli plik nie może zostać utworzony, drukowany jest inny komunikat o błędzie. Drugie dopasowanie zewnętrznego wyrażenia `match` pozostaje takie samo, więc program panikuje przy każdym błędzie poza błędem braku pliku.

> ### Alternatywy do Użycia `match` z `Result<T, E>`
>
> Tak dużo tych `match`! Wyrażenie `match` jest bardzo użyteczne, ale także
> bardzo prymitywne. W rozdziale 13 dowiesz się o domknięciach, które są używane
> z wieloma metodami zdefiniowanymi w `Result<T, E>`. Metody te mogą być
> bardziej zwięzłe niż użycie `match` podczas obsługi wartości `Result<T, E>` w
> kodzie.
>
> Na przykład, oto inny sposób na napisanie tej samej logiki, jak pokazano na
> listingu 9-5, tym razem przy użyciu domknięć i metody `unwrap_or_else`:
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic!("Problem creating the file: {:?}", error);
>             })
>         } else {
>             panic!("Problem opening the file: {:?}", error);
>         }
>     });
> }
> ```
>
> Chociaż ten kod zachowuje się tak samo jak z listingu 9-5, nie zawiera żadnych
> wyrażeń `match` i jest bardziej czytelny. Wróć do tego przykładu po
> przeczytaniu rozdziału 13 i poszukaj metody `unwrap_or_else` w dokumentacji
> biblioteki standardowej. Wiele z tych metod może oczyścić ogromne zagnieżdżone
> wyrażenia `match`, gdy masz do czynienia z błędami.

### Skróty do Obsłużenia Paniki z Błędu: `unwrap` and `expect`

Używanie `match` działa wystarczająco dobrze, ale może być nieco rozwlekłe i nie zawsze dobrze komunikuje intencje. Typ `Result<T, E>` ma zdefiniowanych wiele metod pomocniczych do wykonywania różnych, bardziej specyficznych zadań. Metoda `unwrap` jest metodą skrótu zaimplementowaną podobnie jak wyrażenie `match`, które napisaliśmy na listingu 9-4. Jeśli wartość `Result` jest wariantem `Ok`, `unwrap` zwróci wartość wewnątrz `Ok`. Jeśli `Result` jest wariantem `Err`, `unwrap` wywoła dla nas makro `panic!`. Oto przykład użycia `unwrap`:

<span class="filename">Plik: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

Jeśli uruchomimy ten kod bez pliku *hello.txt*, zobaczymy komunikat o błędzie
z wywołania `panic!`, które wykonuje metoda `unwrap`:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-04-unwrap
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:4:49
```

Podobnie, metoda `expect` pozwala nam również wybrać komunikat błędu `panic!`. 
Używanie `expect` zamiast `unwrap` i dostarczanie dobrych komunikatów o błędach
może przekazać twoje intencje i ułatwić śledzenie źródła paniki.
Składnia `expect` wygląda następująco:

<span class="filename">Plik: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

Używamy `expect` w taki sam sposób jak `unwrap`: by zwrócić uchwyt pliku lub wywołać 
makro `panic!`. Komunikat błędu użyty przez `expect` w wywołaniu `panic!` będzie parametrem,
który przekażemy do `expect`, zamiast domyślnego komunikatu `panic!`, którego używa `unwrap`. 
Oto jak to wygląda:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-05-expect
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at 'hello.txt should be included in this project: Os {
code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:5:10
```

W kodzie produkcyjnym, większość Rustowców wybiera `expect` zamiast `unwrap`
i podaje więcej kontekstu na temat tego, dlaczego operacja ma się zawsze udać. 
W ten sposób, jeśli twoje założenia okażą się błędne, masz więcej informacji 
do wykorzystania podczas debugowania.

### Propagowanie Błędów

Gdy implementacja funkcji wywołuje coś, co może się nie powieść, zamiast obsługiwać
błąd w samej funkcji, można zwrócić błąd do kodu wywołującego, aby mógł zdecydować,
co zrobić. Jest to znane jako *propagowanie* błędu i daje większą kontrolę kodowi
wywołującemu, gdzie może być więcej informacji lub logiki, która dyktuje sposób
obsługi błędu niż to, co jest dostępne w kontekście twojego kodu.

Na przykład, listing 9-6 pokazuje funkcję, która odczytuje nazwę użytkownika z pliku. 
Jeśli plik nie istnieje lub nie można go odczytać, funkcja ta zwróci te błędy do kodu,
który ją wywołał.

<span class="filename">Plik: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

<span class="caption">Listing 9-6: Funkcja zwracająca błędy do kodu wywołującego
przy użyciu `match`</span>

Ta funkcja może być napisana w znacznie krótszy sposób, ale zaczniemy od zrobienia
wielu z nich ręcznie, aby zbadać obsługę błędów; na koniec pokażemy krótszy sposób.
Przyjrzyjmy się najpierw typowi zwracanemu funkcji: `Result<String, io::Error>`.
Oznacza to, że funkcja zwraca wartość typu `Result<T, E>`, gdzie parametr generyczny
`T` został wypełniony konkretnym typem `String`, a typ generyczny `E` został wypełniony
konkretnym typem `io::Error`.

Jeśli ta funkcja powiedzie się bez żadnych problemów, kod wywołujący tę funkcję
otrzyma wartość `Ok`, która zawiera `String` - nazwę użytkownika, którą ta funkcja
odczytała z pliku. Jeśli ta funkcja napotka jakiekolwiek problemy, kod wywołujący
otrzyma wartość `Err`, która zawiera instancję `io::Error`, która zawiera więcej
informacji o problemach. Wybraliśmy `io::Error` jako typ zwracany tej funkcji,
ponieważ tak się składa, że jest to typ wartości błędu zwracanej z obu operacji,
które wywołujemy w ciele tej funkcji, a które mogą się nie powieść: 
funkcji `File::open` i metody `read_to_string`.

Ciało funkcji rozpoczyna się od wywołania funkcji `File::open`. Następnie obsługujemy
wartość `Result` za pomocą `match` podobnego do `match` z listingu 9-4. Jeśli
`File::open` się powiedzie, uchwyt pliku w zmiennej wzorca `file` staje się wartością
w zmiennej mutowalnej `username_file` i funkcja jest kontynuowana. W przypadku `Err`, 
zamiast wywoływać `panic!`, używamy słowa kluczowego `return`, aby całkowicie powrócić
z funkcji i przekazać wartość błędu z `File::open`, teraz w zmiennej wzorca `e`,
z powrotem do kodu wywołującego jako wartość błędu tej funkcji.

Jeśli więc mamy uchwyt pliku w `username_file`, funkcja tworzy nowy `String` w zmiennej
`username` i wywołuje metodę `read_to_string` na uchwycie pliku w `username_file`,
aby odczytać zawartość pliku do `username`. Metoda `read_to_string` również zwraca `Result`,
ponieważ może się nie powieść, nawet jeśli `File::open` się powiedzie. Potrzebujemy
więc kolejnego `match` do obsługi tego `Result`: jeśli `read_to_string` się powiedzie,
to nasza funkcja się powiedzie i zwrócimy nazwę użytkownika z pliku, która jest teraz
w `username` zawinięta w `Ok`. Jeśli `read_to_string` się nie powiedzie, zwracamy
wartość błędu w ten sam sposób, w jaki zwróciliśmy wartość błędu w `match`, który
obsłużył wartość zwracaną `File::open`. Nie musimy jednak wyraźnie używać wyrażenia
`return`, ponieważ jest to ostatnie wyrażenie w funkcji.

Kod, który wywołuje ten kod będzie następnie obsługiwał otrzymanie wartości `Ok` zawierającej
nazwę użytkownika lub wartości `Err` zawierającej `io::Error`. Do kodu wywołującego należy
decyzja, co zrobić z tymi wartościami. Jeśli kod wywołujący otrzyma wartość `Err`,
może na przykład wywołać `panic!` i zawiesić program, użyć domyślnej nazwy użytkownika lub
wyszukać nazwę użytkownika z innego miejsca niż plik. Nie mamy wystarczających informacji
na temat tego, co kod wywołujący faktycznie próbuje zrobić, więc propagujemy wszystkie
informacje o powodzeniu lub błędzie w górę, aby mógł je odpowiednio obsłużyć.

Ten wzorzec propagacji błędów jest tak powszechny w Rust, że zapewnia operator znaku
zapytania `?`, aby to ułatwić.

#### Skrót do Propagawania Błędów: Operator `?`

Listing 9-7 pokazuje implementację `read_username_from_file`, która ma taką samą
funkcjonalność jak na listingu 9-6, ale ta implementacja używa operatora `?`.

<span class="filename">Plik: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

<span class="caption">Listing 9-7: Funkcja zwracająca błędy do kodu wywołującego
przy użyciu operatora `?`</span>

Wyrażenie `?` umieszczone po wartości `Result` jest zdefiniowane tak, aby działało
w prawie taki sam sposób jak wyrażenia `match`, które zdefiniowaliśmy do obsługi
wartości `Result` na listingu 9-6. Jeśli wartością `Result` jest `Ok`, wartość
wewnątrz `Ok` zostanie zwrócona z tego wyrażenia, a program będzie kontynuowany. 
Jeśli wartością jest `Err`, `Err` zostanie zwrócone z całej funkcji, tak jakbyśmy 
użyli słowa kluczowego `return`, więc wartość błędu zostanie rozpropagowana 
do kodu wywołującego.

Istnieje różnica między tym, co robi wyrażenie `match` z listingu 9-6, a tym, 
co robi operator `?`: wartości błędów, które mają wywołany operator `?` przechodzą
przez funkcję `from`, zdefiniowaną w cechach `From` w bibliotece standardowej,
która jest używana do konwersji wartości z jednego typu na inny. Gdy operator `?` 
wywołuje funkcję `from`, otrzymany typ błędu jest konwertowany na typ błędu 
zdefiniowany w typie zwracanym bieżącej funkcji. Jest to przydatne, gdy funkcja 
zwraca jeden typ błędu, aby reprezentować wszystkie sposoby, w jakie funkcja 
może zawieść, nawet jeśli jej części mogą zawieść z wielu różnych powodów.

Na przykład, możemy zmienić funkcję `read_username_from_file` z listingu 9-7,
aby zwracała niestandardowy typ błędu o nazwie `OurError`, który zdefiniujemy. 
Jeśli zdefiniujemy również `impl From<io::Error> for OurError`, aby skonstruować
instancję `OurError` z `io::Error`, wtedy wywołania operatora `?` w ciele 
`read_username_from_file` wywołają `from` i przekonwertują typy błędów bez potrzeby
dodawania dodatkowego kodu do funkcji.

W kontekście Listingu 9-7, `?` na końcu wywołania `File::open` zwróci wartość 
wewnątrz `Ok` do zmiennej `username_file`. Jeśli wystąpi błąd, operator `?` powróci
wcześniej z całej funkcji i przekaże dowolną wartość `Err` do kodu wywołującego.
To samo dotyczy `?` na końcu wywołania `read_to_string`.

Operator `?` eliminuje wiele szablonów i upraszcza implementację tej funkcji. 
Moglibyśmy nawet jeszcze bardziej skrócić ten kod, łącząc wywołania metod 
bezpośrednio po `?`, jak pokazano na listingu 9-8.

<span class="filename">Plik: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

<span class="caption">Listing 9-8: Łączenie wywołań metod po operatorze `?`</span>

Przenieśliśmy tworzenie nowego `String` w `username` na początek funkcji;
ta część nie uległa zmianie. Zamiast tworzyć zmienną `username_file`,
połączyliśmy wywołanie `read_to_string` bezpośrednio z wynikiem `File::open(„hello.txt”)?`.
Nadal mamy `?` na końcu wywołania `read_to_string` i nadal zwracamy wartość `Ok`
zawierającą `username`, gdy zarówno `File::open` jak i `read_to_string` się powiodą,
zamiast zwracać błędy. Funkcjonalność jest ponownie taka sama jak na listingu 9-6
i listingu 9-7; jest to po prostu inny, bardziej ergonomiczny sposób zapisu.

Listing 9-9 pokazuje sposób na uczynienie tego jeszcze krótszym przy użyciu `fs::read_to_string`.

<span class="filename">Plik: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

<span class="caption">Listing 9-9: Użycie `fs::read_to_string` zamiast otwierania
i odczytywania pliku</span>

Odczytywanie pliku do łańcucha jest dość powszechną operacją, więc biblioteka
standardowa dostarcza wygodną funkcję `fs::read_to_string`, która otwiera plik,
tworzy nowy `String`, odczytuje zawartość pliku, umieszcza zawartość w tym `String`
i zwraca go. Oczywiście użycie `fs::read_to_string` nie daje nam możliwości wyjaśnienia
całej obsługi błędów, więc najpierw zrobiliśmy to w dłuższy sposób.

#### Gdzie Operator `?` Może Być Używany

Operator `?` może być używany tylko w funkcjach, których typ zwracany jest zgodny
z wartością, na której `?` jest używany. Dzieje się tak dlatego, że operator `?`
jest zdefiniowany do wczesnego zwracania wartości z funkcji, w taki sam sposób jak
wyrażenie `match`, które zdefiniowaliśmy na listingu 9-6. Na listingu 9-6, wyrażenie 
`match` używało wartości `Result`, a element wczesnego powrotu zwracał wartość `Err(e)`.
Typ zwracany funkcji musi być `Result`, aby był kompatybilny z tym `return`.

Na Listingu 9-10 przyjrzyjmy się błędowi, który otrzymamy, jeśli użyjemy operatora `?`
w funkcji `main` z typem zwracanym niezgodnym z typem wartości, na której używamy `?`:

<span class="filename">Plik: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

<span class="caption">Listing 9-10: Próba użycia `?` w funkcji `main` zwracającej `()` 
nie zostanie skompilowana</span>

Ten kod otwiera plik, co może się nie udać. Operator `?` podąża za wartością `Result`
zwracaną przez `File::open`, ale ta funkcja `main` ma typ zwracany `()`, a nie `Result`.
Kiedy skompilujemy ten kod, otrzymamy następujący komunikat o błędzie:

```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

Ten błąd wskazuje, że możemy używać operatora `?` tylko w funkcji, która zwraca
`Result`, `Option` lub inny typ, który implementuje `FromResidual`.

Aby naprawić błąd, masz dwie możliwości. Jedną z nich jest zmiana typu zwracanego funkcji,
aby był zgodny z wartością, na której używasz operatora `?`, o ile nie masz żadnych
ograniczeń, które by to uniemożliwiały. Inną techniką jest użycie `match` lub jednej
z metod `Result<T, E>` do obsługi `Result<T, E>` w dowolny odpowiedni sposób.

Komunikat o błędzie wspomina, że `?` może być również używany z wartościami 
`Option<T>`. Podobnie jak w przypadku użycia `?` na `Result`, możesz użyć `?` na 
`Option` tylko w funkcji, która zwraca `Option`. Zachowanie operatora `?` po wywołaniu
na `Option<T>` jest podobne do jego zachowania po wywołaniu na `Result<T, E>`: jeśli 
wartością jest `None`, to `None` zostanie zwrócone wcześniej z funkcji w tym momencie.
Jeśli wartością jest `Some`, wartość wewnątrz `Some` jest wartością wynikową wyrażenia
i funkcja jest kontynuowana. Listing 9-11 zawiera przykład funkcji, która znajduje
ostatni znak pierwszej linii w podanym tekście:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

<span class="caption">Listing 9-11: Użycie operatora `?` na wartości `Option<T>`</span>

Ta funkcja zwraca `Option<char>`, ponieważ możliwe jest, że jest tam znak, ale możliwe
jest również, że go tam nie ma. Ten kod pobiera argument `text` z wycinka łańcucha
i wywołuje na nim metodę `lines`, która zwraca iterator nad liniami w łańcuchu. 
Ponieważ funkcja ta chce sprawdzić pierwszą linię, wywołuje `next` na iteratorze,
aby uzyskać pierwszą wartość z iteratora. Jeśli `text` jest pustym łańcuchem,
to wywołanie `next` zwróci `None`, w którym to przypadku użyjemy `?` by zatrzymać
i zwrócić `None` z `last_char_of_first_line`. Jeśli `text` nie jest pustym łańcuchem,
`next` zwróci wartość `Some` zawierającą wycinek łańcucha pierwszej linii w `text`.

Znak `?` wyodrębnia wycinek łańcucha i możemy wywołać `chars` na tym wycinku łańcucha,
aby uzyskać iterator jego znaków. Interesuje nas ostatni znak w tej pierwszej linii,
więc wywołujemy `last`, aby zwrócić ostatni element w iteratorze. Jest to `Option`,
ponieważ możliwe jest, że pierwsza linia jest pustym ciągiem, na przykład jeśli `text`
zaczyna się od pustej linii, ale zawiera znaki w innych liniach, jak w `„\nhi”`. 
Jednakże, jeśli w pierwszej linii znajduje się ostatni znak, zostanie on zwrócony
w wariancie `Some`. Operator `?` w środku daje nam zwięzły sposób na wyrażenie tej
logiki, pozwalając nam zaimplementować funkcję w jednej linii. Gdybyśmy nie mogli 
użyć operatora `?` w `Option`, musielibyśmy zaimplementować tę logikę używając więcej
wywołań metod lub wyrażenia `match`.

Zauważ, że możesz użyć operatora `?` na `Result` w funkcji, która zwraca `Result` i możesz
użyć operatora `?` na `Option` w funkcji, która zwraca `Option`, ale nie możesz mieszać
i dopasowywać. Operator `?` nie konwertuje automatycznie `Result` na `Option` lub odwrotnie;
w takich przypadkach można użyć metod takich jak `ok` na `Result` lub `ok_or` na `Option`, 
aby dokonać konwersji w sposób jawny.

Jak dotąd, wszystkie funkcje `main`, których używaliśmy zwracały `()`. Funkcja `main`
jest wyjątkowa, ponieważ jest punktem wejścia i wyjścia programów wykonywalnych i istnieją 
ograniczenia dotyczące tego, jaki może być jej typ zwracany, aby programy zachowywały się 
zgodnie z oczekiwaniami.

Na szczęście `main` może również zwrócić `Result<(), E>`. Listing 9-12 zawiera kod z listingu 9-10,
ale zmieniliśmy typ zwracanego `main` na `Result<(), Box<dyn Error>>` i dodaliśmy wartość zwracaną
`Ok(())` na końcu. Ten kod zostanie teraz skompilowany:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

<span class="caption">Listing 9-12: Zmiana `main` by zwracał `Result<(), E>` pozwala
na użycie operatora `?` na wartościach `Result`.</span>

Typ `Box<dyn Error>` jest *trait object*, o którym będziemy mówić w sekcji 
[„Using Trait Objects that Allow for Values of Different Types”][trait-objects]<!-- ignore --> 
w rozdziale 17. Na razie można odczytać `Box<dyn Error>` jako „dowolny rodzaj błędu”. 
Użycie `?` na wartości `Result` w funkcji `main` z typem błędu `Box<dyn Error>` 
jest dozwolone, ponieważ pozwala na wczesne zwrócenie dowolnej wartości `Err`. 
Nawet jeśli ciało tej funkcji `main` będzie zwracać tylko błędy typu `std::io::Error`,
poprzez określenie `Box<dyn Error>`, ta sygnatura będzie nadal poprawna, nawet jeśli więcej
kodu zwracającego inne błędy zostanie dodane do ciała `main`.

Gdy funkcja `main` zwraca `Result<(), E>`, program wykonywalny zakończy działanie
z wartością `0` jeśli `main` zwróci `Ok())` i zakończy działanie z wartością
niezerową jeśli `main` zwróci `Err`. Programy wykonywalne napisane w C zwracają liczby
całkowite, gdy kończą działanie: programy, które zakończą działanie pomyślnie zwracają
liczbę całkowitą `0`, a programy, które zwrócą błąd zwracają liczbę całkowitą inną niż `0`.
Rust również zwraca liczby całkowite z plików wykonywalnych, aby być zgodnym z tą konwencją.

Funkcja `main` może zwracać dowolne typy, które implementują 
[cechę `std::process::Termination` trait][termination]<!-- ignore -->, który zawiera
funkcję `report` zwracającą `ExitCode`. Więcej informacji na temat implementacji
cechy `Termination` dla własnych typów można znaleźć w dokumentacji biblioteki standardowej.

Teraz, gdy omówiliśmy szczegóły wywoływania `panic!` lub zwracania `Result`, 
powróćmy do tematu tego, jak zdecydować, który z nich jest odpowiedni do użycia
w poszczególnych przypadkach.

[handle_failure]: ch02-00-guessing-game-tutorial.html#obsługa-potencjalnych-błędów-z-użyciem-result
[trait-objects]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[termination]: ../std/process/trait.Termination.html

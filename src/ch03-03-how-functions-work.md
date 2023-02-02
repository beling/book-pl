## Funkcje

Funkcje są wszechobecne w kodzie Rusta. Widzieliśmy już jedną z najważniejszych funkcji w całym języku: funkcję `main`, która jest punktem wejściowym wielu programów. Widzieliśmy już też słowo kluczowe `fn`, za pomocą którego można deklarować nowe funkcje.

W kodzie Rusta konwencjonalnym stylem zapisu nazw funkcji i zmiennych jest użycie tzw. *snake case*. W tym stylu wszystkie człony pisane są małymi literami, a poszczególne wyrazy oddzielone są podkreślnikami. Poniżej program zawierający przykładową definicję funkcji:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

Definicje funkcji w Ruście składają się ze słowa kluczowego `fn`, nazwy funkcji i pary nawiasów okrągłych. Nawiasy klamrowe informują kompilator, gdzie zaczyna i kończy się ciało funkcji.

Zdefiniowane przez nas funkcje możemy wywołać, pisząc ich nazwę wraz z parą nawiasów.
Ponieważ funkcja `another_function` jest już zdefiniowana programie, możemy ją wywołać z wnętrza funkcji `main`.
Proszę zauważyć, że definicja funkcji `another_function` znajduje się w kodzie źródłowym *po* ciele funkcji `main`;
moglibyśmy równie dobrze umieścić ją przed funkcją `main`. Rusta nie obchodzi, gdzie umieszczasz definicje swoich funkcji, a jedynie to żeby te definicje były w zasięgu widzianym przez wywołującego.

Stwórzmy nowy projekt o nazwie *functions*, dzięki któremu zapoznamy się z głębiej z funkcjami w Ruście. Umieść powyższy przykład z `another_function` w pliku *src/main.rs* i uruchom program. Powinieneś zobaczyć taki wynik:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

Linie kodu wykonywane są w kolejności, w jakiej pojawiają się w funkcji `main`. Najpierw pokaże się tekst „Witaj, świecie!”, a następnie wywołana jest funkcja `another_function` i ukazuje się wiadomość z jej wnętrza.


### Parametry funkcji

Funkcje mogą również przyjmować *parametry*, które są specjalnymi zmiennymi, będącymi częścią
sygnatury funkcji. Jeśli funkcja, którą wywołujesz, posiada parametry, możesz jej podać konkretne wartości tych parametrów.
Technicznie rzecz biorąc, konkretne wartości przekazywane do funkcji nazywają się *argumentami*, jednak
w swobodnych rozmowach ludzie mają w zwyczaju używać słów *parametry* i *argumenty* zamiennie, zarówno dla zmiennych
w definicji funkcji, jak i konkretnych wartości przekazywanych podczas wywołania funkcji.

Poniższa zaktualizowana wersja funkcji `another_function` prezentuje, jak wyglądają parametry w Ruście:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

Spróbuj uruchomić ten program; powinieneś otrzymać następujący wynik:


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

Deklaracja funkcji `another_function` ma jeden parametr o nazwie `x`. Typ `x` jest określony jako `i32`.
Kiedy wartość `5` jest przekazana do `another_function`, makro `println!` umieszcza `5` w miejscu, gdzie
string formatujący zawiera `x` w nawiasach klamrowych.

W sygnaturze funkcji *trzeba* podać typ każdego z parametrów. To celowa decyzja projektantów Rusta: wymaganie adnotacji typów w definicjach funkcji powoduje, że nie trzeba już podawać ich niemal nigdzie więcej, a Rust i tak wie, co mamy na myśli.
Kompilator jest również w stanie wypisać bardziej pomocne komunikaty o błędach, jeśli wie, jakich typów oczekuje funkcja.

Jeśli chcesz, żeby funkcja przyjmowała wiele parametrów, rozdziel kolejne deklaracje parametrów przecinkami,
jak poniżej:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

W tym przykładzie stworzyliśmy funkcję `print_labeled_measurement` z dwoma parametrami.
Pierwszy parametr ma nazwę `value` i typ `i32`. Drugi jest
nazwany `unit_label` i jest typu `char`. Funkcja drukuje tekst zawierający wartości zarówno `value` jak i `unit_label`.

Spróbujmy uruchomić ten kod. Otwórz plik *src/main.rs* w twoim projekcie *functions* i zastąp jego zawartość kodem z powyższego przykładu. Uruchom program poleceniem `cargo run`:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

Ponieważ wywołaliśmy tę funkcję z argumentem `5` jako wartość dla `value` oraz `'h'` jako wartość dla `unit_label`, program wypisał właśnie te wartości.

### Instrukcje i wyrażenia

Ciało funkcji jest składa sie z serii instrukcji (*statements*) i opcjonalnie jest zakończone wyrażeniem (*expression*). Jak dotąd analizowaliśmy jedynie funkcje bez końcowego wyrażenia, jednakże widziałeś już wyrażenia będące częścią instrukcji. Ponieważ Rust jest językiem opartym
o wyrażenia, ważne jest, aby zrozumieć różnicę między tymi dwoma. Podobne rozróżnienie nie występuje w innych językach, więc przyjrzyjmy się teraz instrukcjom i wyrażeniom oraz jak różnice między nimi wpływają na postać funkcji.

* **Instrukcje** to polecenia wykonania jakichś akcji; nie zwracają one wartości.
* **Wyrażenia** zaś rozwijają się do wartości zwracanej. Spójrzmy na przykłady. 

Tworzenie zmiennej i przypisanie do niej wartości z użyciem słowa kluczowego `let` jest instrukcją. W listingu 3-1, `let y = 6;` to instrukcja.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

<span class="caption">Listing 3-1: Deklaracja funkcji `main` zawierającej jedną instrukcję</span>


Definicje funkcji są również instrukcjami; cały powyższy przykład jest instrukcją sam w sobie.

Instrukcje nie mają wartości zwracanej. To znaczy, że nie możesz przypisać instrukcji `let` do innej
zmiennej, tak jak poniższy kod próbuje zrobić; Rust zwróci błąd:


<span class="filename">Plik: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

Po uruchomieniu tego programu dostaniesz taki błąd:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

Instrukcja `let y = 6` nie zwraca żadnej wartości, więc nie ma nic, co moglibyśmy przypisać do `x`.
Tym Rust różni się od innych języków, takich jak C lub Ruby, w których operacja przypisania zwraca wartość
tego przypisania. W tych językach można napisać `x = y = 6` i zarówno `x` i `y` będą miały wartość `6`;
w Ruście tak się jednak nie stanie.

Wyrażenia rozwijają się do pewnej wartości i zaraz po instrukcjach stanowią większość kodu,
jaki napiszesz w Ruście. Rozważmy prostą operację matematyczną, taką jak `5 + 6`, która to jest
wyrażeniem rozwijającym się do wartości `11`. Wyrażenia mogą być częścią instrukcji: w listingu 3-1
liczba `6` w instrukcji `let y = 6;` jest wyrażeniem, które rozwija się do wartości `6`. Wywołanie funkcji
jest wyrażeniem. Wywołanie makra jest wyrażeniem. Blok, który tworzymy za pomocą nawiasów klamrowych dla zdefiniowania nowego zasięgu, również jest wyrażeniem, na przykład:


<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

To wyrażenie:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

jest blokiem, który rozwija się do wartości `4`. Ta wartość w ramach instrukcji `let` wpisywana jest do `y`.
Zwróć uwagę na końcową linię `x + 1` bez średnika, która różni się od większości linii, jakie
widziałeś do tej pory. Wyrażenia nie kończą się średnikiem. Jeśli dodasz średnik na końcu wyrażenia,
zmienisz je w instrukcję, która nie będzie rozwijać się do żadnej wartości. Pamiętaj o tym, gdy będziesz
zagłębiać się w zagadnienie wartości zwracanych z funkcji i wyrażeń w kolejnej sekcji.


### Funkcje zwracające wartość

Funkcje mogą zwracać wartości do miejsca w kodzie, gdzie zostały wywołane. Wartości zwracanej
z funkcji nie nadajemy nazwy, a jedynie deklarujemy jej typ po strzałce (`->`). W Ruście
wartość zwracana z funkcji jest równoważna wartości ostatniego wyrażenia z bloku ciała funkcji.
Możesz też użyć instrukcji wcześniejszego wyjścia z funkcji poprzez użycie słowa kluczowego `return`,
jednak większość funkcji zwraca niejawnie ostatnie wyrażenie. Poniżej przykład funkcji zwracającej wartość:


<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

W funkcji `five` nie ma żadnych wywołań funkcji, makr, ani nawet instrukcji `let`
- tylko sama liczba `5`. Jest to całkowicie poprawna funkcja w Ruście. Zauważ,
że typ zwracany z funkcji jest również zdefiniowany, jako `-> i32`. Spróbuj uruchomić ten kod;
rezultat powinien wyglądać tak:


```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

W funkcji `five` `5` jest wartością zwracaną, dlatego też typ wartości zwracanej to `i32`.
Przyjrzyjmy się temu bliżej. Są tu dwa istotne fragmenty: pierwszy, linia `let x = five();`
pokazuje, że używamy wartości zwracanej z funkcji `five` do zainicjalizowania zmiennej.
Ponieważ funkcja `five` zwraca `5`, to linia ta jest równoważna poniższej:


```rust
let x = 5;
```

Druga rzecz, to postać funkcji `five`, która nie przyjmuje żadnych parametrów
i określa typ zwracany, ale jej ciało to samotne `5` bez średnika, ponieważ jest to
wyrażenie, którego wartość chcemy zwrócić.

Spójrzmy na kolejny przykład:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

Po uruchomieniu tego kodu na ekranie zostanie wypisane `Wartość x wynosi: 6`. Jednak gdybyśmy dopisali średnik po linii zawierającej `x + 1`, zmienilibyśmy wyrażenie w instrukcję i Rust zgłosiłby błąd.

<span class="filename">Plik: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

Próba kompilacji poskutkuje następującym błędem:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

Główny fragment wiadomości o błędzie, `mismatched types`, czyli niezgodność typów, informuje o podstawowym problemie w tym kodzie. Definicja funkcji `plus_one` określa typ zwracany jako `i32`, jednak instrukcje nie rozwijają się do żadnej wartości, i wartość zwrócona z funkcji przyjmuje postać `()`, czyli pustej krotki. Jest to sprzeczne z definicją funkcji i powoduje błąd.
W komunikacie błędu Rust podaje przypuszczalne rozwiązanie tego problemu: sugeruje, aby usunąć średnik z końca linii, co naprawi błąd kompilacji.

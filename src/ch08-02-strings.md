## Przechowywanie Tekstów UTF-8 za Pomocą Łańcuchów

W rozdziale 4 poświęciliśmy trochę czasu łańcuchom (*ang.* strings), 
ale teraz zagłębimy się w ten temat. Świeżo upieczeni Rustowcy bardzo 
często zatrzymują się na łańcuchach i nie mogą ruszyć dalej z trzech powodów: 
Rust ma skłonność do ujawniania możliwych błędów, łańcuchy są o wiele bardziej 
złożoną strukturą danych niż wielu programistów przyznaje, a także system 
kodowania UTF-8. Jeśli uczyliście się wcześniej innych języków programowania, 
taka mieszanka może wydawać się trudna.

Łańcuchy znaków omówimy w kontekście kolekcji, ponieważ są one zaimplementowane 
jako kolekcja bajtów i paru metod zapewniających przydatną funkcjonalność, 
kiedy bajty są interpretowane jako tekst. W tej sekcji, poruszymy też kwestię 
operacji wykonywanych na `String`, takich jak tworzenie, modyfikowanie i 
czytanie i które są dostępne w każdej kolekcji. Określimy też różnice między 
`String` a innymi kolekcjami, a dokładniej jak rozbieżność między interpretacją 
danych w `String` przez człowieka i przez komputer komplikuje indeksowanie w
`String`.

### Czym Jest Łańcuch Znaków?

Zacznijmy od wyjaśnienia czym jest *łańcuch znaków*. W rdzeniu językowym 
Rusta znajdziemy tylko jeden rodzaj łańcucha znaków i jest nim wycinek 
łańcucha `str` , który zazwyczaj znaleźć można w formie zapożyczonej `&str`. 
W rozdziale 4 wspominaliśmy o *wycinkach łańcuchów*, które są referencją do 
pewnego łańcucha tekstu UTF-8 i zapisanego w innym miejscu. Na przykład, 
literały łańcuchów są zapisane w pliku binarnym programu, a więc są wycinkami 
łańcuchów. 

Rodzaj łańcucha znaków `String` jest zapewniany przez bibliotekę standardową 
Rusta a nie wkodowany w rdzeń języka. Przechowuje tekst UTF-8, może się powiększać, 
mutować, a także być własnością. Rozmawiając o *łańcuchach* Rustowcy nie odwołują 
się do jednego konkretnego jego typu, mogą mieć na myśli albo`String` albo wycinek 
łańcucha `&str`. Chociaż ta sekcja poświęcona jest w dużej mierze `String`, oba 
typy są często wykorzystywane w bibliotece standardowej Rusta i oba przechowują 
tekst UTF-8

### Tworzenie Nowego Łańcucha Znaków

Jeśli przyjrzymy się operacjom dostępnym w `Vec<T>` i w `String`,
zauważymy, że wiele z nich się powtarza. Dzieje się tak, ponieważ `String` 
opakowuje wektor bajtów i posiada dodatkowo pewne zabezpieczenia, 
ograniczenia, a także możliwości. Przykładem funkcji działającej 
tak samo na `Vec<T>` i `String` jest funkcja `new`
za pomocą której możemy stworzyć nowe instancje i która jest 
pokazana na listingu 8-11. 

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```

<span class="caption">Listing 8-11: Tworzenie nowego, pustego `String`</span>

Powyższa linijka kodu tworzy nowy, pusty łańcuch `s`, do którego możemy 
wprowadzić dane. Częściej jednak mamy już jakieś wstępne dane, z którymi 
chcielibyśmy stworzyć łańcuch. By to osiągnąć, możemy posłużyć się metodą 
`to_string`, dostępną w każdym typie, który implementuje cechę `Display` 
tak jak robią to literały łańcuchów. Na listingu 8-12 znajdziemy dwa przykłady 
zastosowania tej metody.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```

<span class="caption">Listing 8-12: Użycie metody `to_string` w celu stworzenia
`String` z literału łańcuchowego</span>

Powyższy kod tworzy łańcuch zawierający `wstępna zawartość`.

By stworzyć `String` z literału łańcuchowego możemy również użyć funkcji 
`String::from`. Kod widoczny na listingu 8-13 stanowi równowartość kodu 
z listingu 8-12. Pierwszy utylizuje `to_string` a drugi `String::from`.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```

<span class="caption">Listing 8-13: Użycie funkcji `String::from` 
w celu stworzenia `String` z literału łańcuchowego</span>

Ponieważ istnieje mnóstwo zastosowań łańcuchów, możemy do nich 
używać wielu interfejsów generycznych. W konsekwencji, mamy przed 
sobą cały szereg opcji i choć niektóre z nich mogą wydawać się 
niepotrzebne, wszystkie mają swoją rolę do odegrania! W przypadku gdy 
`String::from` i `to_string` wykonują tę samą czynność, wybór 
między nimi sprowadza się do kwestii stylu i czytelności. 

Należy pamiętać, że łańcuchy są zakodowane w UTF-8 i dzięki 
temu możemy wprowadzić do nich jakiekolwiek poprawnie zakodowane dane, 
co pokazane jest na listingu 8-14.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```

<span class="caption">Listing 8-14: Przechowywanie odpowiednika 
“Dzień dobry/Cześć” w różnych językach za pomocą łańcuchów </span>

Wszystkie powyższe wartości `String` są poprawne.

### Modyfikowanie Łańcucha Znaków

Podobnie do `Vec<T>`, `String` może się powiększać a jego zawartość 
zmieniać jeśli wprowadzimy do niego więcej danych. Ponadto, możemy użyć 
operatora `+` i makra `format!` , żeby w poręczny sposób połączyć ze sobą 
wartości `String`. 

#### Dodawanie Elementów do Łańcucha za Pomocą `push_str` i `push`

Używając metody `push_str` możemy powiększyć `String` i dodać do niego 
wycinek łańcucha tak jak jest to pokazane na listingu 8-15.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```

<span class="caption">Listing 8-15: Dodawanie wycinka łańcucha do `String`
za pomocą metody `push_str`</span>

Po napisaniu powyższych dwóch linijek kodu, `s` będzie zawierać w sobie
 `foobar`. Przy `push_str` używamy wycinka łańcucha ponieważ możemy nie 
 chcieć, żeby metoda przejęła własność nad tym parametrem. Za przykład 
 weźmy kod pokazany na listingu 8-16, gdzie chcemy móc ponownie użyć 
`s2` po dołączeniu jego zawartości do `s1`.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```

<span class="caption">Listing 8-16: Użycie wycinka łańcucha po 
dołączeniu jego zawartości do `String`</span>

Gdyby metoda `push_str` przejęła własność nad `s2`, wyświetlenie 
jego zawartości w ostatniej linijce byłoby niemożliwe. Zamiast 
tego, kod działa tak jak tego oczekiwaliśmy!

Metoda `push` przyjmuje pojedynczy znak jako parametr i dodaje go do 
`String`. Kod na listingu 8-17 dodaje literę "l" do `String` za pomocą metody `push`.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

<span class="caption">Listing 8-17: Dodanie jednego znaku do wartości `String` za pomocą 
 `push`</span>

W rezultacie, `s` będzie zawierać w sobie `lol`.

#### Łączenie za Pomocą Operatora `+` lub Makra `format!`

Często wykonywaną operacją jest łączenie dwóch istniejących już łańcuchów. 
Pierwszym sposobem by to osiągnąć jest użycie operatora `+` tak jak jest 
to pokazane na listingu 8-18.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

<span class="caption">Listing 8-18: Użycie operatora `+` by połączyć dwie wartości
`String` i w ten sposób stworzyć nową wartość `String`</span>

Łańcuch `s3` będzie teraz zawierał `Witaj, Świecie!`. Utrata ważności przez `s1` 
po dodaniu do łańcucha i użycie referencji do `s2` są spowodowane sygnaturą 
metody wywoływanej przy użyciu operatora `+`. Operator `+` używa metody `add`, której sygnatura wygląda mniej więcej tak:

```rust,ignore
fn add(self, s: &str) -> String {
```

W bibliotece standardowej, `add` jest definiowane za pomocą typów generycznych 
i powiązanych. Tutaj wymieniliśmy je na typy konkretne, co jest prawidłowym 
zjawiskiem przy wywoływaniu metody `add` z wartościami `String`. 
Typami generycznymi zajmiemy się w rozdziale 10. Powyższa sygnatura 
dostarcza nam wskazówek potrzebnych do zrozumienia podchwytliwych 
elementów operatora `+`.

Po pierwsze, znak `&` stojący przed `s2` oznacza, że do pierwszego 
łańcucha dodajemy *referencję* do drugiego łańcucha. Dzieje się tak za sprawą
parametru `s` funkcji `add`. Dodanie do siebie dwóch wartości `String` jest 
niemożliwe, jedyne co możemy dodać do `String` to `&str`. Ale przecież typem `&s2` 
jest `&String` a nie `&str` tak jak było pokazane w drugim parametrze funkcji `add`. 
Dlaczego w takim razie listing 8-18 kompiluje się? 

Możemy użyć `&s2` podczas wywoływania funkcji `add` ponieważ kompilator potrafi 
*wymusić* zmianę argumentu `&String` w argument `&str`. Podczas wywoływania metody 
`add` Rust *wymusza dereferencję*, która w tym przypadku zmienia `&s2` w `&s2[..]`.
Wymuszanie dereferencji omówimy szczegółowo w rozdziale 15. Ponieważ `add` nie 
przejmuje własności nad parametrem `s`, `s2` wciąż będzie poprawnym `String`
po tej operacji. 

Po drugie, z sygnatury wynika, że `add` przejmuje własność nad `self`,
ponieważ `self` *nie* zawiera `&`. Oznacza to, że w listingu 8-18 funkcja `add` 
przejmuje własność nad `s1` i `s1` traci ważność. Chociaż moglibyśmy pomyśleć, że
`let s3 = s1 + &s2;` zduplikuje oba łańcuchy i stworzy nowy, nic takiego się nie
dzieje. Zamiast tego, instrukcja przejmuje własność nad `s1` dodaje do niego kopię 
zawartości `s2`, a potem oddaje własność nad rezultatem. Innymi słowy, `let s3 = s1 + &
s2;` sprawia wrażenie tworzenia wielu duplikatów, ale tego nie robi. Ta implementacja 
jest wydajniejsza od kopiowania. 


Przy łączeniu ze sobą wielu łańcuchów, operator `+` przestaje być poręczny:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

Po napisaniu powyższego kodu, `s` będzie zawierać `tic-tac-toe`. Niestety, ilość 
znaków `+` i `"` utrudnia zrozumienie co się dzieje. Do bardziej złożonego łączenia 
łańcuchów, możemy użyć makra `format!`:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

Powyższy kod również przypisuje `s` wartość `tic-tac-toe`. Działanie makra `format!` 
można porównać do `println!` z taką różnicą, że makro zwróci `String` razem z jego 
zawartością zamiast tylko wyświetlić rezultat. Użycie `format!` znacznie ułatwia 
czytanie kodu. Co więcej, kod generowany przez `format!` utylizuje referencje, żeby 
wywołanie metody nie przejęło własności na żadnym z jego parametrów.

### Indeksowanie do Łańcuchów

W wielu językach programowania można uzyskać dostęp do pojedynczych znaków w łańcuchu 
używając indeksu i tworząc referencję do interesujących nas znaków. Jest to operacja 
poprawna i powszechnie używana, ale nie zadziała w Ruście. Jeśli spróbujemy użyć 
składni indeksowej by uzyskać dostęp do części `String` w Ruście, wyświetli nam się 
błąd. Spójrzmy na niepoprawny kod na listingu 8-19.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

<span class="caption">Listing 8-19: Próba użycia składni indeksowej z
`String`</span>

Przy kompilowaniu powyższego kodu, otrzymamy poniższy błąd:

```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

Powyższy błąd i notatka przekazują nam, że łańcuchy w Ruście nie obsługują 
indeksowania. Dlaczego? By odpowiedzieć na to pytanie, musimy porozmawiać o tym jak 
Rust przechowuje łańcuchy w pamięci.

#### Wewnętrzna Reprezentacja

`String` opakowuje `Vec<u8>`. Przyjrzyjmy się paru poprawnym łańcuchom UTF-8 z 
listingu 8-14. Jako pierwszy, omówimy poniższy łańcuch:


```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```

W tym przypadku, funkcja `len` zwróci wynik 4, co oznacza, że wektor przechowujący 
“Hola” ma długość czterech bajtów. Każda z tych liter waży jeden bajt jeśli 
została zakodowana w UTF-8. Następna linijka kodu może cię zaskoczyć. (Zauważ, że ten 
łańcuch rozpoczyna litera *Ze* alfabetu rosyjskiego a nie arabska cyfra 3).

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```

Zapytany o długość tego łańcucha, mógłbyś odpowiedzieć 12 bajtów, ale tak naprawdę są 
to 24 bajty. Ponieważ każda wartość skalarna Unicode w tym łańcuchu zajmuje dwa bajty 
pamięci, potrzeba 24 bajtów by zakodować “Здравствуйте” w UTF-8. Dlatego też, indeks do bajtów łańcucha nie zawsze będzie korelował z poprawną wartością skalarną Unicode. W ramach przykładu, spójrz na poniższy kod:

```rust,ignore,does_not_compile
let hello = "Здравствуйте";
let answer = &hello[0];
```

Wiesz już, że `answer` nie zwróci pierwszej litery czyli `З`. Ponieważ “Здравствуйте”
jest zakodowane w UTF-8, pierwszym bajtem `З` jest `208` a drugim `151`. Wydawałoby 
się zatem, że `answer` powinien zwracać `208`, ale samo `208` nie jest poprawnym 
znakiem. Użytkownik prawdopodobnie nie chce, żeby program wyświetlił `208` kiedy 
prosi się go o pierwszą literę w łańcuchu, ale są to jedyne dane jakie Rust posiada 
przy indeksie bajtu 0. Wyświetlenie wartości bajtu zazwyczaj nie jest pożądane 
nawet jeśli łańcuch zawiera tylko litery alfabetu łacińskiego. Gdyby `&"hello"[0]` 
było poprawnie napisanym kodem i wyświetlało wartość bajtu, zwróciłoby `104` zamiast 
`h`.

Aby uniknąć zwracania niespodziewanych wartości i błędów, które nie rzucają się w 
oczy, Rust nie kompiluje tego kodu i zapobiega nieporozumieniom na wczesnym 
etapie rozwoju programu.

#### Bajty i Wartości Skalarne i Zbitki Grafemów! O Matko!

Porozmawiajmy teraz o kolejnej ważnej kwestii związanej z UTF-8, a mianowicie 
o sposobach patrzenia na łańcuchy przez Rusta. Spośród nich możemy wyłonić 
trzy naprawdę istotne perspektywy: łańcuchy jako bajty, wartości skalarne i 
jako zbitki grafemów (jednostka, której najbliżej do *litery*).


Przykładowo, słowo hindi “नमस्ते” zapisane w piśmie dewanagari jest 
przechowywane jako wektor wartości `u8`, który wygląda tak:


```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

Powyżej widzimy 18 bajtów i sposób w jaki komputery koniec końców 
przechowują te dane. Jeśli spojrzymy na te bajty jako wartości 
skalarne Unicode, które są rodzajem  `char` Rusta, będą one wyglądać tak:


```text
['न', 'म', 'स', '्', 'त', 'े']
```

Mamy tutaj sześć wartości `char`,  ale czwarta i szósta nie są 
literami. Są znakami diakrytycznymi, które same w sobie nie 
mają sensu. Wreszcie, jeśli spojrzymy na nie jako na zbitki 
grafemów, otrzymamy coś co można by nazwać czterema literami, 
z których składa się słowo “नमस्ते”:


```text
["न", "म", "स्", "ते"]
```

Rust pozwala na różne interpretacje surowych danych z łańcucha, 
które są przechowywane przez komputer. Dzięki temu, każdy program 
może wybrać interpretację, której potrzebuje bez względu na to w 
jakim ludzkim języku te dane są napisane. 

Ostatnim powodem, dla którego Rust nie pozwala indeksować do `String` 
w celu uzyskania dostępu do znaku jest wymóg by operacje indeksowania 
(O(1)) zawsze zajmowały tyle samo czasu. W przypadku `String` nie 
można tego zagwarantować, ponieważ Rust musiałby przejść przez zawartość 
łańcucha od początku do indeksu by określić ilość poprawnych znaków w łańcuchu. 


### Cięcie Łańcuchów

Indeksowanie do łańcucha często jest złym pomysłem ponieważ nie wiadomo 
jaki powinien być rodzaj wyniku takiej operacji. Czy powinna to być wartość 
bajtu, znak, zbitek grafemów czy wycinek łańcucha? Dlatego jeśli naprawdę 
musisz użyć indeksów by stworzyć wycinki łańcucha, Rust prosi cię o konkrety.


Zamiast indeksować za pomocą `[]` z pojedynczą cyfrą, możesz użyć `[]` 
z zakresem by stworzyć wycinek łańcucha zawierający konkretne bajty:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Tutaj, `s` będzie `&str`, która zawiera pierwsze cztery bajty 
łańcucha. Wcześniej, wspomnieliśmy, że każda z tych liter waży 
2 bajty, co oznacza, że `s` będzie równe `Зд`.

Jeśli mielibyśmy spróbować wyciąć tylko część bajtów litery za pomocą czegoś w rodzaju 
`&hello[0..1]`, Rust spanikowałby w trakcie wykonywania programu.
Zachowałby się tak samo gdybyś spróbował uzyskać dostęp do indeksu wykraczającego poza wektor:


```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```

Używaj zakresów do tworzenia wycinków łańcuchów ostrożnie,
bo może to zakończyć się błędem.

### Sposoby na Iterację po Łańcuchach

Na łańcuchach najlepiej operuje się jeśli dokładnie określimy czy wynikiem operacji
mają być bajty czy znaki. Jeśli chcesz uzyskać wartości skalarne Unicode, zastosuj metodę
`chars`. Wywołanie `chars` na “Зд” rozdziela i zwraca dwie wartości typu `char`. 
By uzyskać dostęp do każdego elementu, możesz iterować po wyniku:


```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

Powyższy kod wyświetli:

```text
З
д
```

Ewentualnie, bardziej może ci odpowiadać metoda `bytes`, która wyświetla każdy surowy bajt:

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

Powyższy kod wyświetli cztery bajty, które znajdują się w tym łańcuchu:

```text
208
151
208
180
```

Nie zapomnij, że poprawne wartości skalarne Unicode mogą składać się z więcej niż jednego bajta.

Proces otrzymania zbitka grafemów z łańcucha, jak przy piśmie dewanagari, jest 
złożony więc ta funkcjonalność nie jest dostępna w bibliotece standardowej. 
Jeżeli potrzebujesz tej funkcjonalności, na [crates.io](https://crates.io/)<!-- ignore --> 
możesz znaleźć odpowiednie skrzynie.

### Łańcuchy Nie Są Takie Proste
Podsumowując, łańcuchy są skomplikowane. Różne języki programowania wybierają 
różne sposoby na pokazanie tej złożoności programiście. Rust wybrał poprawne 
obchodzenie się z danymi ze `String` jako domyślne zachowanie wszystkich 
programów w nim napisanych. Oznacza to, że programiści muszą mieć z góry przemyślane 
co zrobią z danymi w UTF-8. Ten kompromis ujawnia więcej złożoności łańcuchów niż 
sposoby wybrane przez inne języki programowania, ale zapobiega konieczności radzenia 
sobie z błędami obejmującymi znaki inne niż ASCII na późniejszym etapie cyklu rozwoju 
oprogramowania. 

Dobre wieści są takie, że biblioteka standardowa oferuje wiele funkcjonalności 
opartych na typach `String` i `&str` by pomóc ci poprawnie zająć się takimi 
złożonymi sytuacjami. Upewnij się, że przejrzałeś dokumentację i zapoznałeś się z
takimi użytecznymi metodami jak `contains`, żeby przeszukać łańcuch i `replace` 
by zastąpić części łańcucha innym łańcuchem. 


Przejdźmy teraz do czegoś nieco łatwiejszego: mapy z haszowaniem!

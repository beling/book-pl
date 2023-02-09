<!-- ## Defining Modules to Control Scope and Privacy -->
## Definiowanie Modułów by Kontrolować Zasięg i Prywatności

W tym rozdziale porozmawiamy o modułach i innych częściach systemu modułów, mianowicie o *ścieżkach*, które pozwalają na nazywanie elementów; słowie kluczowym `use`, które włącza ścieżkę w zasięg; oraz słowie kluczowym `pub`, które upublicznia elementy.
Omówimy również słowo kluczowe `as`, pakiety zewnętrzne i operator glob.

Zaczniemy od podania listy reguł, będącej swoistą ściągą, przydatną podczas organizowania własnego kodu.
Następnie szczegółowo wyjaśnimy poszczególne reguły.

### Ściąga z Modułów

Przedstawiamy tutaj krótkie kompendium omawiające jak moduły, ścieżki, słowo kluczowe `use` i słowo kluczowe `pub` działają w kompilatorze i jak większość programistów organizuje swój kod.
Jest to świetne miejsce, do którego można sięgnąć, aby przypomnieć sobie, jak działają moduły.
Zaś przykłady każdej z podanych reguł będziemy omawiać w dalszej części rozdziału.

- **Start z korzenia skrzyni**: Podczas kompilowania skrzyni, kompilator najpierw zagląda do pliku głównego skrzyni (zazwyczaj *src/lib.rs* dla skrzyni bibliotecznej lub *src/main.rs* dla skrzyni binarnej) w poszukiwaniu kodu do skompilowania.
- **Deklarowanie modułów**: W pliku głównym skrzyni, można deklarować nowe moduły; powiedzmy, że zadeklarujemy moduł „garden“ (z ang. ogród) za pomocą `mod garden;`. Kompilator będzie szukał kodu tego modułu w następujących miejscach:
  - Zaraz za `mod garden`, w nawiasach klamrowych, które zastępują średnik po `mod garden`
  - W pliku *src/garden.rs*
  - W pliku *src/garden/mod.rs*
- **Deklarowanie podmodułów**: W każdym pliku innym niż główny plik skrzyni, można zadeklarować podmoduły. Na przykład, można zadeklarować `mod vegetables;` (z ang. warzywa) w *src/garden.rs*. Kompilator będzie szukał kodu podmodułu w katalogu o nazwie zgodnej z modułem nadrzędnym, w następujących miejscach:
  - Zaraz za `mod vegetables`, w nawiasach klamrowych, które zastępują średnik po `mod vegetables`
  - W pliku *src/garden/vegetables.rs*
  - W pliku *src/garden/vegetables/mod.rs*
- **Ścieżki do kodu w modułach**: Gdy moduł jest częścią skrzyni, można odwołać się do kodu w tym module z dowolnego innego miejsca tej skrzyni, gdy tylko pozwalają na to zasady prywatności, używając ścieżki do kodu. Na przykład, do typu `Asparagus` (z ang. szparag) w podmodule `vegetables` modułu `garden` można odwołać się za pomocą `crate::garden::vegetables::Asparagus`.
- **Prywatne a publiczne**: Kod zawarty w module domyślnie jest prywatny i niedostępny dla modułów nadrzędnych. Aby upublicznić moduł, trzeba go zadeklarować za pomocą `pub mod` zamiast `mod`. By upublicznić zawarte w module elementy, należy umieścić `pub` przed ich deklaracjami.
- **Słowo kluczowe `use`**: Słowo kluczowe `use` tworzy skróty do elementów, ograniczając tym samym powtarzanie długich ścieżek. W dowolnym zasięgu, w którym typ `crate::garden::vegetables::Asparagus` jest dostępny, można z pomocą `use crate::garden::vegetables::Asparagus;` utworzyć do niego skrót i, od tego momentu, pisać `Asparagus`, aby ten typ wykorzystać.

Powyższe zasady zilustrujemy na przykładzie skrzyni binarnej o nazwie `backyard` (z ang. podwórko). Katalog tej skrzyni, również nazwany `backyard`, zawiera następujące pliki i katalogi:

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

Plikiem głównym tej skrzyni jest *src/main.rs* o następującej zawartości:

<span class="filename">Plik: src/main.rs</span>

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```

Linia `pub mod garden;` mówi kompilatorowi, aby uwzględnił kod, który znajduje się w *src/garden.rs*, czyli:

<span class="filename">Filename: src/garden.rs</span>

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden.rs}}
```

Tutaj, `pub mod vegetables;` oznacza uwzględnienie także kodu z *src/garden/vegetables.rs*. Oto ten kod:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden/vegetables.rs}}
```

Teraz dogłębniej omówmy powyższe reguły i zademonstrujmy je w działaniu!

<!-- ### Grouping Related Code in Modules -->
### Grupowanie Spokrewnionego Kodu w Modułach

*Moduły* pozwalają nam tak zorganizować kod w obrębie skrzyni, by był czytelny i łatwy do wielokrotnego wykorzystania.
Moduły pozwalają nam również kontrolować *prywatność* elementów, ponieważ kod wewnątrz modułu jest domyślnie prywatny. Elementy prywatne stanowią wewnętrzne szczegóły implementacji, niedostępne z zewnątrz.
Możemy zdecydować się na upublicznienie modułów i zawartych w nich elementów, aby zewnętrzny kod mógł je wykorzystywać i być od nich zależny.

Jako przykład napiszmy skrzynię biblioteczną, dostarczającą funkcjonalność restauracji.
Zdefiniujemy sygnatury funkcji, ale pozostawimy ich ciała puste, aby skupić się na organizacji kodu, a nie na implementacji restauracji.

W branży restauracyjnej niektóre części restauracji określane są jako *front of house*, a inne jako *back of house*.
Front of house to obszar, w którym przebywają klienci; w nim gospodarze sadzają gości, kelnerzy przyjmują zamówienia i płatności, a barmani przygotowują drinki.
Back of house to miejsca, w których pracują kucharze przygotowujący posiłki, zmywacze myjący naczynia, oraz kierownicy wykonujący prace administracyjne.

Aby zorganizować naszą skrzynię zgodnie z powyższym podziałem, uporządkujemy jej funkcjonalności w zagnieżdżonych modułach. 
Utworzymy nową bibliotekę o nazwie `restaurant`, uruchamiając `cargo new restaurant --lib`; następnie wpiszemy kod z Listingu 7-1 do *src/lib.rs*, aby zdefiniować niektóre moduły i sygnatury funkcji. Oto sekcja frontowa:

<span class="filename">Plik: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

<span class="caption">Listing 7-1: Moduł `front_of_house` zawierający inne moduły, które zawierają funkcje</span>

Moduł definiujemy za pomocą słowa kluczowego `mod`, po którym następuje nazwa modułu (w tym przypadku `front_of_house`). Następnie umieszczamy ciało modułu w nawiasach klamrowych. Wewnątrz modułów możemy umieszczać inne moduły, co w tym przypadku uczyniliśmy z modułami `hosting` i `serving`. Moduły mogą również zawierać definicje innych elementów, takich jak strukty, enumy, stałe, cechy i—jak na listingu 7-1—funkcje.

Moduły pozwalają na pogrupowanie powiązanych ze sobą definicji i nazwanie relacji pomiędzy nimi.
Dzięki pogrupowaniu, programiści mogą łatwiej poruszać się po kodzie i nie muszą czytać wszystkiego by odnaleźć interesujące ich definicje.
Zaś dodając nową funkcjonalność wiedzą, gdzie umieścić kod.

Wcześniej wspomnieliśmy, że *src/main.rs* i *src/lib.rs* nazywane są korzeniami skrzyni. Przyczyną nadania im takiej nazwy jest fakt, że zawartość każdego z tych plików konstytuuje moduł o nazwie `crate`, będący korzeniem struktury złożonej z modułów skrzyni, zwanej *drzewem modułów*.

Listing 7-2 pokazuje drzewo modułów dla struktury z Listingu 7-1.

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

<span class="caption">Listing 7-2: Drzewo modułów dla kodu pokazanego na Listingu 7-1</span>

Drzewo to pokazuje w jaki sposób moduły zagnieżdżone są w innych; na przykład, `hosting` jest zagnieżdżony w `front_of_house`. Drzewo ukazuje również, że niektóre moduły są swoim *rodzeństwem*, co oznacza, że są zdefiniowane w tym samym module; `hosting` i `serving` są rodzeństwem zdefiniowanym w `front_of_house`.
Jeśli moduł A jest zawarty wewnątrz modułu B, mówimy, że moduł A jest *dzieckiem* modułu B oraz, że moduł B jest *rodzicem* modułu A.
Proszę zauważyć, że korzeniem drzewa modułów jest zdefiniowany domyślnie i niejawnie moduł o nazwie `crate`.

Drzewo modułów przypomina drzewo katalogów w systemie plików na komputerze. Podobnie do katalogów w systemie plików, moduły służą organizacji (w ich przypadku, chodzi o organizację kodu). I analogicznie do plików w katalogach, potrzebujemy sposobu na odnajdywanie naszych modułów.
## Przechowywanie Kluczy z Powiązanymi Wartościami w Mapach z Haszowaniem

Ostatnią z naszych popularnych kolekcji jest *mapa z haszowaniem*. Typ `HashMap<K, V>` 
przechowuje mapowanie kluczy typu `K` na wartości typu `V` przy użyciu *funkcji 
haszującej*, która określa sposób umieszczania tych kluczy i wartości w pamięci. 
Wiele języków programowania obsługuje tego rodzaju struktury danych, ale często 
używają one innych nazw, takich jak *hash*, *mapa*, *obiekt*, *tablica hash*, 
*słownik* lub *tablica asocjacyjna*, by wymienić tylko kilka.

Mapy z haszowaniem są przydatne, gdy chcesz wyszukać dane nie za pomocą indeksu, jak
w przypadku wektorów, ale za pomocą klucza, który może być dowolnego typu. Na przykład,
w grze można śledzić wyniki każdej drużyny w mapie z haszowaniem, w której
każdy klucz to nazwa drużyny, a wartości to wyniki każdej drużyny. Podając nazwę drużyny,
można pobrać jej wynik.

W tej sekcji omówimy podstawowy interfejs API map z haszowaniem, ale wiele innych ciekawostek
kryje się w funkcjach zdefiniowanych na `HashMap<K, V>` przez bibliotekę standardową. 
Jak zawsze, sprawdź dokumentację biblioteki standardowej, aby uzyskać więcej informacji.

### Tworzenie Nowej Mapy z Haszowaniem

Jednym ze sposobów utworzenia pustej mapy z haszowaniem jest użycie `new` i dodanie elementów za pomocą
`insert`. Na listingu 8-20 śledzimy wyniki dwóch drużyn o 
nazwach *Blue* i *Yellow*. Niebieska drużyna zaczyna z 10 punktami, a żółta
z 50.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

<span class="caption">Listing 8-20: Tworzenie nowej mapy z haszowaniem i wstawianie kluczy i wartości</span>

Zauważ, że musimy najpierw zadeklarować `use` `HashMap` z części kolekcji biblioteki standardowej.
Spośród naszych trzech popularnych kolekcji, ta jest najrzadziej używana, 
więc nie jest uwzględniona w funkcjach automatycznie wprowadzanych do zakresu w preludium. 
Mapy z haszowaniem mają również mniejsze wsparcie ze strony biblioteki standardowej; 
na przykład nie ma wbudowanego makra do ich konstruowania.

Podobnie jak wektory, mapy z haszowaniem przechowują swoje dane na stercie.
Ta `HashMap` ma klucze typu `String` i wartości typu `i32`. Podobnie jak wektory, 
mapy z haszowaniem są jednorodne: wszystkie klucze i wartości muszą mieć ten sam typ.

### Uzyskiwanie Dostępu do Wartości w Mapie z Haszowaniem
Możemy uzyskać wartość z mapy z haszowaniem poprzez podanie jej klucza do metody `get`, 
jak pokazano na listingu 8-21.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

<span class="caption">Listing 8-21: Uzyskanie dostępu do wyniku drużyny niebieskiej przechowywanego w mapie z haszowaniem</span>

W tym przypadku `score` będzie miał wartość powiązaną z niebieską drużyną,
a wynikiem będzie `10`. Metoda `get` zwraca `Option<&V>`; jeśli nie ma
wartości dla tego klucza w mapie z haszowaniem, `get` zwróci `None`. Ten program
obsługuje `Option`, wywołując `copied,` aby uzyskać `Option<i32>` zamiast `Option<&i32>`,
a następnie `unwrap_or`, aby ustawić `score` na zero, jeśli `scores` nie ma wpisu dla klucza.

Możemy iterować po każdej parze klucz-wartość w mapie z haszowaniem w podobny sposób jak
z wektorami, używając pętli `for`:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

Ten kod wydrukuje każdą parę w dowolnej kolejności:

```text
Yellow: 50
Blue: 10
```

### Mapy z Haszowaniem i System Własności

Dla typów, które implementują cechę `Copy`, jak `i32`, wartości są kopiowane
do mapy z haszowaniem. Dla wartości będących własnością, takich jak `String`, 
wartości zostaną przeniesione i mapa z haszowaniem będzie właścicielem tych wartości,
jak pokazano na listingu 8-22.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

<span class="caption">Listing 8-22: Pokazanie, że klucze i wartości są własnością mapy z haszowaniem po ich wstawieniu</span>

Nie jesteśmy w stanie użyć zmiennych `field_name` i `field_value` po tym, jak
zostały one przeniesione do mapy z haszowaniem za pomocą wywołania `insert`.

Jeśli wstawimy odniesienia do wartości w mapie z haszowaniem, wartości nie zostaną przeniesione
do mapy z haszowaniem. Wartości, na które wskazują odniesienia, muszą być ważne przez
co najmniej tak długo, jak ważna jest mapa z haszowaniem. Więcej o tych kwestiach powiemy w
["Walidacja referencji za pomocą cyklu życia"][validating-references-with-lifetimes]<!-- ignore --> w rozdziale 10.

### Aktualizowanie Mapy z Haszowaniem

Chociaż liczba par kluczy i wartości może rosnąć, każdy unikalny klucz może 
mieć tylko jedną wartość powiązaną z nim w danym momencie (ale nie odwrotnie:
na przykład zarówno drużyna Niebieska, jak i Żółta mogą mieć wartość `10`
przechowywaną w mapie z haszowaniem `scores`).

Kiedy chcesz zmienić dane w mapie z haszowaniem, musisz zdecydować, w jaki sposób
obsłużyć przypadek, gdy klucz ma już przypisaną wartość. Można zastąpić
starą wartość nową wartością, całkowicie ignorując starą wartość. Można
zachować starą wartość i zignorować nową, dodając nową wartość tylko wtedy, gdy
klucz *nie* ma jeszcze wartości. Można też połączyć starą i nową wartość.
Przyjrzyjmy się, jak wykonać każdą z tych metod!

#### Zastępowanie Wartości

Jeśli wstawimy klucz i wartość do mapy z haszowaniem, a następnie wstawimy ten sam klucz
z inną wartością, wartość powiązana z tym kluczem zostanie zastąpiona.
Mimo że kod na listingu 8-23 wywołuje `insert` dwa razy, mapa z haszowaniem
będzie zawierała tylko jedną parę klucz-wartość, ponieważ za każdym razem wstawiamy 
wartość klucza drużyny Blue.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

<span class="caption">Listing 8-23: Zastępowanie wartości przechowywanej z określonym kluczem</span>

Ten kod wypisze `{„Blue”: 25}`. Oryginalna wartość `10` została
nadpisana.

<!-- Old headings. Do not remove or links may break. -->
<a id="only-inserting-a-value-if-the-key-has-no-value"></a>

#### Dodawanie Klucza i Wartości Tylko w Przypadku Braku Klucza

Powszechne jest sprawdzanie, czy dany klucz już istnieje w mapie z haszowaniem
z wartością, a następnie podjęcie następujących działań: jeśli klucz istnieje w
mapie haszującej, istniejąca wartość powinna pozostać bez zmian; jeśli klucz
nie istnieje, należy wstawić go wraz z wartością.

Mapy z haszowaniem mają do tego specjalne API o nazwie `entry`, które przyjmuje klucz,
który chcemy sprawdzić jako parametr. Wartością zwrotną metody `entry` jest enum
o nazwie `Entry`, która reprezentuje wartość, która może istnieć lub nie. Powiedzmy, że
chcemy sprawdzić, czy klucz dla drużyny Yellow ma wartość z nią powiązaną.
Jeśli nie, chcemy wstawić wartość `50`, i to samo dla zespołu
Blue team. Używając API `entry`, kod wygląda jak na listingu 8-24.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

<span class="caption">Listing 8-24: Używanie metody `entry` do wstawiania tylko wtedy, gdy klucz nie ma jeszcze wartości</span>

Metoda `or_insert` na `Entry` jest zdefiniowana by zwracać mutowalne odniesienie do
wartości odpowiedniego klucza `Entry`, jeśli ten klucz istnieje, a jeśli nie, to
wstawia parametr jako nową wartość dla tego klucza i zwraca mutowalne
odniesienie do nowej wartości. Ta technika jest znacznie czystsza niż samodzielne pisanie
logiki, a ponadto lepiej współgra ze sprawdzaniem zapożyczeń.

Uruchomienie kodu z listingu 8-24 spowoduje wypisanie `{„Yellow”: 50, „Niebieski”: 10}`. 
Pierwsze wywołanie `entry` wstawi klucz dla drużyny Yellow z wartością
`50`, ponieważ żółta drużyna nie ma jeszcze wartości. Drugie wywołanie
`entry` nie zmieni mapy z haszowaniem, ponieważ drużyna Blue ma już wartość
wartość `10`.

#### Aktualizacja Wartości na Podstawie Starej Wartości

Innym częstym przypadkiem użycia map z haszowaniem jest wyszukiwanie wartości klucza,
a następnie aktualizacja na podstawie starej wartości. Na przykład, listing 8-25 pokazuje kod,
który zlicza ile razy każde słowo pojawia się w tekście. Używamy mapy z haszowaniem z
słowami jako kluczami i zwiększamy wartość, aby śledzić, ile razy widzieliśmy to słowo.
Jeśli widzimy dane słowo po raz pierwszy, najpierw wstawiamy wartość wartość `0`.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

<span class="caption">Listing 8-25: Zliczanie wystąpień słów przy użyciu mapy z haszowaniem, która przechowuje słowa i liczby</span>

Ten kod wypisze `{„world”: 2, „hello”: 1, „wonderful”: 1}`. Możesz zobaczyć
te same pary klucz-wartość wydrukowane w innej kolejności: przypomnijmy sobie z sekcji
[„Uzyskiwanie dostępu do wartości z mapy z haszowaniem”][access]<!-- ignore -->, że
iteracja po mapie z haszowaniem odbywa się w dowolnej kolejności.

Metoda `split_whitespace` zwraca iterator nad subwycinkami, oddzielonymi
białymi znakami, wartości w `text`. Metoda `or_insert` zwraca mutowalną referencję
(`&mut V`) do wartości dla określonego klucza. Tutaj przechowujemy
mutowalną referencję w zmiennej `count`, więc aby przypisać ją do tej wartości,
musimy najpierw odnieść się do `count` używając gwiazdki (`*`). Zmienna
wychodzi poza zakres na końcu pętli `for`, więc wszystkie te
zmiany są bezpieczne i dozwolone przez reguły pożyczania.

### Funkcje Haszujące

Domyślnie, `HashMap` używa funkcji haszującej o nazwie *SipHash*, która może zapewnić
odporność na ataki typu DoS (denial-of-service) z użyciem hash tablic
[^siphash]<!-- ignore -->. Nie jest to najszybszy dostępny algorytm haszujący,
ale kompromis dla lepszego bezpieczeństwa, który wiąże się ze spadkiem
wydajności ale jest tego wart. Jeśli profilujesz swój kod i stwierdzisz, że domyślna
funkcja haszująca jest zbyt wolna dla twoich celów, możesz przełączyć się na inną funkcję
określając inny hasher. *hasher* jest typem, który implementuje cechę
`BuildHasher`. Porozmawiamy o cechach i sposobach ich implementacji w
[Rozdział 10][traits]<!-- ignore -->. Nie musisz koniecznie implementować
własnego hashera od zera; [crates.io](https://crates.io/)<!-- ignore -->
zawiera biblioteki współdzielone przez innych użytkowników Rusta, które dostarczają 
hashery implementujące wiele powszechnych algorytmów haszujących.

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

## Podsumowanie

Wektory, ciągi znaków i mapy z haszowaniem zapewniają dużą funkcjonalność
niezbędną w programach do przechowywania, pobierania i modyfikowania danych. Oto
kilka ćwiczeń, które powinieneś być teraz w stanie rozwiązać:

1. Dla podanej listy liczb całkowitych, użyj wektora i zwróć medianę (po posortowaniu,
   wartość na środkowej pozycji) i dominantę (wartość, która występuje najczęściej;
   pomocna będzie tutaj mapa z haszowaniem) listy.
1. Przekonwertuj ciągi znaków na łacinę świńską. Pierwsza spółgłoska każdego słowa jest przenoszona na
   na koniec słowa i dodawane jest *ay*, więc *first* staje się *irst-fay*. Słowa
   zaczynające się od samogłoski mają zamiast tego dodane *hay* na końcu (*apple* staje się
   *apple-hay*). Należy pamiętać o szczegółach kodowania UTF-8!
1. Korzystając z mapy z haszowaniem i wektorów, utwórz interfejs tekstowy, aby umożliwić użytkownikowi dodawanie
   pracowników do działów w firmie; na przykład „Dodaj Sally do działu inżynierii” lub 
   »Dodaj Amir do działu sprzedaży«. Następnie pozwól użytkownikowi pobrać listę wszystkich
   osób w dziale lub wszystkich osób w firmie według działu, posortowanych alfabetycznie.
   
Dokumentacja API biblioteki standardowej opisuje metody wektorów, łańcuchów
i mapy z haszowaniem, które będą pomocne w tych ćwiczeniach!

Wchodzimy w bardziej złożone programy, w których operacje mogą się nie powieść, więc jest to
idealny czas na omówienie obsługi błędów. Zrobimy to w następnej kolejności!

[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[access]: #uzyskiwanie-dostępu-do-wartości-w-mapie-z-haszowaniem
[traits]: ch10-02-traits.html

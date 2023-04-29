<!-- ## Paths for Referring to an Item in the Module Tree -->
## Ścieżki Do Elementów W Drzewie Modułów

Podobnie jak podczas nawigowania po systemie plików, elementy w drzewie modułów wskazujemy za pomocą ścieżek. Aby wywołać funkcję, musimy znać do niej ścieżkę.

Każda ścieżka jest jednego z następujących dwóch rodzajów:

* *Ścieżka bezwzględna* jest pełną ścieżką startującą od korzenia skrzyni; dla kodu z zewnętrznej skrzyni, notacja ścieżki bezwzględnej zaczyna się od nazwy skrzyni, a dla kodu z bieżącej skrzyni, od słowa `crate`.
* *Ścieżka względna* startuje z bieżącego modułu i jej zapis zaczyna się od `self`, `super`, lub identyfikatora w bieżącym module.

Zarówno ścieżki bezwzględne jak i względne notujemy za pomocą jednego lub więcej identyfikatorów oddzielonych podwójnymi dwukropkami (`::`).

Powróćmy do listingu 7-1 i załóżmy, że chcemy wywołać funkcję `add_to_waitlist`.
By to uczynić, musimy wpierw odpowiedzieć na pytanie: jaka jest ścieżka do funkcji `add_to_waitlist`?
Listing 7-3 obejmuje skrót listingu 7-1, pozbawiony niektórych modułów i funkcji.

Prezentujemy dwa sposoby wywołania funkcji `add_to_waitlist` z nowej funkcji `eat_at_restaurant` zdefiniowanej w korzeniu skrzyni.
Pomimo że ścieżki podane w przykładzie są poprawne, to jego skompilowanie nie jest możliwe, ze względu na inny problem.
Za chwilę wyjaśnimy jaki.

Funkcja `eat_at_restaurant` jest częścią publicznego API naszej skrzyni bibliotecznej, więc oznaczyliśmy ją słowem kluczowym `pub`.
Bardziej szczegółowo omawiamy to słowo w sekcji ["Exposing Paths with the `pub` Keyword"][pub]<!-- ignore -->.

<span class="filename">Plik: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

<span class="caption">Listing 7-3: Wywołanie funkcji `add_to_waitlist` przy wykorzystaniu bezwzględnej oraz względnej ścieżki</span>

W pierwszym wywołaniu funkcji `add_to_waitlist` w `eat_at_restaurant`, używamy ścieżki bezwzględnej.
Ponieważ funkcja `add_to_waitlist` jest zdefiniowana w tej samej skrzyni co `eat_at_restaurant`, to zapis tej ścieżki zaczynamy słowem `crate`.
Po tym słowie wymieniamy kolejne moduły, aż dotrzemy do `add_to_waitlist`.
Można sobie wyobrazić system plików o takiej samej strukturze: aby uruchomić program `add_to_waitlist`, podalibyśmy ścieżkę `/front_of_house/hosting/add_to_waitlist`; użycie nazwy `crate` by zacząć od korzenia skrzyni jest jak użycie `/` by zacząć od korzenia systemu plików.

W drugim wywołaniu funkcji `add_to_waitlist` w `eat_at_restaurant`, używamy ścieżki względnej.
Ścieżka ta zaczyna się od `front_of_house`, czyli nazwy modułu zdefiniowanego na tym samym poziomie drzewa modułów co `eat_at_restaurant`.
Jej odpowiednikiem w systemie plików byłoba ścieżka `front_of_house/hosting/add_to_waitlist`.
Rozpoczęcie od nazwy modułu oznacza, że ścieżka jest względna.

Decyzja czy użyć ścieżki względnej czy bezwzględnej zależy od projektu.
Od tego, czy bardziej prawdopodobne jest przeniesienie kodu definiującego element osobno czy razem z kodem, który go używa.
Na przykład, jeśli przeniesiemy moduł `front_of_house` i funkcję `eat_at_restaurant` do modułu o nazwie `customer_experience`, będziemy musieli zaktualizować bezwzględną ścieżkę do `add_to_waitlist`, ale ścieżka względna nadal będzie poprawna.
Jeśli jednak przeniesiemy samą funkcję `eat_at_restaurant` do modułu o nazwie `dining`, bezwzględna ścieżka do wywołania `add_to_waitlist` pozostanie taka sama, zaś względna ścieżka będzie wymagała uaktualnienia.
Ogólnie powinniśmy preferować podawanie ścieżek bezwzględnych, ponieważ jest bardziej prawdopodobne, że będziemy chcieli przenieść definicje kodu i wywołania elementów niezależnie od siebie.

Spróbujmy skompilować listing 7-3 i dowiedzmy się, dlaczego nie jest to jeszcze możliwe.
Otrzymany błąd jest pokazany na listingu 7-4.

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

<span class="caption">Listing 7-4: Błędy kompilatora otrzymane podczas próby zbudowania kodu z listing 7-3</span>

Komunikaty błędów mówią, że moduł `hosting` jest prywatny.
Innymi słowy, mamy poprawne ścieżki do modułu `hosting` i funkcji `add_to_waitlist`, ale Rust nie pozwoli nam ich użyć, ponieważ nie ma dostępu do prywatnych sekcji.
W Ruście wszystkie elementy (funkcje, metody, strukty, enumy, moduły i stałe) są domyślnie prywatne, niedostępne dla modułów nadrzędnych.
Dlatego by uczynić element taki jak funkcja lub struktura prywatnym, wystarczy umieścić go w module.

Elementy w module nadrzędnym nie mogą używać prywatnych elementów wewnątrz modułów podrzędnych, ale elementy w modułach podrzędnych mogą używać elementów w swoich modułach nadrzędnych.
Dzieje się tak dlatego, że moduły podrzędne opakowują i ukrywają swoje szczegóły implementacji, ale moduły podrzędne widzą kontekst, w którym są zdefiniowane.
Kontynuując naszą metaforę, pomyślmy o zasadach prywatności jak o zapleczu restauracji: to, co się tam dzieje, jest prywatne, niedostępne dla klientów restauracji. Ale menedżerowie biura mogą zobaczyć i zrobić wszystko w restauracji, którą prowadzą.

System modułów w Ruście ukrywa domyślnie wewnętrzne szczegóły implementacji.
Dzięki temu wiadomo, które części wewnętrznego kodu można bezpiecznie zmienić, nie psując przy tym kodu zewnętrznego.
Równocześnie, za pomocą słowa kluczowego `pub` można upublicznić element, tym samym udostępniając nadrzędnym modułom część wewnętrznego kodu z modułu podrzędnego.

<!-- ### Exposing Paths with the `pub` Keyword -->
### Eksponowanie Ścieżek Za Pomocą Słowa Kluczowego `pub`.

Wróćmy do błędu z listingu 7-4, który mówił, że moduł `hosting` jest prywatny.
Chcemy, aby funkcja `eat_at_restaurant` w module nadrzędnym miała dostęp do funkcji `add_to_waitlist` w module podrzędnym.
Dlatego oznaczamy moduł `hosting` słowem kluczowym `pub`, co pokazano na listingu 7-5.

<span class="filename">Plik: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs}}
```

<span class="caption">Listing 7-5: Deklarowanie modułu `hosting` jako `pub` by użyć go z `eat_at_restaurant`</span>

Niestety, próba skompilowania kodu z listingu 7-5 nadal kończy się błędem, co pokazano na listingu 7-6.

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

<span class="caption">Listing 7-6: Błędy kompilatora przy budowaniu kodu z listing 7-5</span>

Co się stało? Dodanie słowa kluczowego `pub` przed `mod hosting` upublicznia moduł.
Dzięki tej zmianie, jeśli mamy dostęp do `front_of_house`, to mamy też dostęp do `hosting`.
Ale *zawartość* `hosting` nadal jest prywatna; upublicznienie modułu nie upublicznia jego zawartości.
Słowo kluczowe `pub` dla modułu pozwala jedynie na odwołanie się do niego przez kod w modułach nadrzędnych, a nie na dostęp do jego wewnętrznego kodu.
Ponieważ moduły są kontenerami, nie wystarczy upublicznić jedynie modułu; należy pójść dalej i zdecydować się na upublicznienie jednego lub więcej elementów z jego wnętrza.

Błędy na listingu 7-6 mówią, że funkcja `add_to_waitlist` jest prywatna.
Tak samo jak modułów, zasady prywatności dotyczą również struktur, typów wyliczeniowych, funkcji i metod.

Upublicznijmy również funkcję `add_to_waitlist`, dodając przed jej definicją słowo kluczowe `pub`, jak na listingu 7-7.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs}}
```

<span class="caption">Listing 7-7: Dodanie słowa kluczowego `pub` do `mod hosting` i `fn add_to_waitlist` pozwala nam wywołać tę funkcję z `eat_at_restaurant`</span>

Teraz kod się kompiluje! Aby zobaczyć dlaczego dodanie słowa kluczowego `pub` spowodowało, że można użyć ścieżek zawartych w `eat_at_restaurant` z poszanowaniem reguł prywatności, przeanalizujmy ścieżki bezwzględne i względne.

Nasza ścieżka bezwzględna zaczyna się od `crate`, czyli korzenia drzewa modułów naszej skrzyni.
Moduł `front_of_house` jest zdefiniowany w korzeniu skrzyni.
Mimo że `front_of_house` nie jest publiczny, to ponieważ funkcja `eat_at_restaurant` jest zdefiniowana w tym samym module co `front_of_house` (czyli `eat_at_restaurant` i `front_of_house` są równorzędne), możemy odwoływać się do `front_of_house` z `eat_at_restaurant`.
Następny jest moduł `hosting` oznaczony symbolem `pub`.
A ponieważ możemy uzyskać dostęp do modułu nadrzędnego w stosunku do `hosting`, to możemy też uzyskać dostęp do `hosting`.
Ostatecznie, ponieważ funkcja `add_to_waitlist` jest oznaczona jako `pub` i możemy uzyskać dostęp do jej modułu nadrzędnego, to mamy prawo ją wywołać!

W przypadku ścieżki względnej, logika jest taka sama jak w przypadku ścieżki bezwzględnej, z wyjątkiem pierwszego kroku: zamiast zaczynać od korzenia skrzyni, ścieżka zaczyna się od `front_of_house`.
Rozpoczęcie ścieżki względnej od modułu, w którym zdefiniowany jest `eat_at_restaurant` (a tak jest w przypadku `front_of_house`) oczywiście zadziała.
Następnie, ponieważ `hosting` i `add_to_waitlist` są oznaczone jako `pub`, to reszta ścieżki również zadziała, tak jak i samo wywołanie funkcji!

Jeśli planujesz udostępnić swoją skrzynię biblioteczną, aby inne projekty mogły korzystać z twojego kodu, twoje publiczne API jest umową z użytkownikami skrzyni, która określa, w jaki sposób mogą oni korzystać z twojego kodu.
Jest wiele aspektów dotyczących zarządzania zmianami w publicznym API tak, by ułatwić utrzymanie od niego zależności.
Rozważania na ich temat wykraczają jednak poza zakres tej książki; zainteresowanych tym tematem odsyłamy do [The Rust API Guidelines][api-guidelines].

> #### Pakiety z Programami i Bibliotekami - Najlepsze Praktyki
>
> Wspomnieliśmy, że pakiet może równocześnie zawierać zarówno korzeń skrzyni binarnej *src/main.rs* jak i korzeń skrzyni bibliotecznej *src/lib.rs*, i obie skrzynie będą miały domyślnie nazwę pakietu.
> Zazwyczaj takie pakiety w skrzyni binarnej będą miały jedynie kod niezbędny do uruchomienia programu wykonywalnego i wywołania kodu ze skrzyni bibliotecznej.
> Dzięki temu inne projekty będą mogły wykorzystać prawie całą funkcjonalność zapewnianą przez pakiet, ponieważ kod skrzyni bibliotecznej może być współdzielony.
>
> Drzewo modułów powinno być zdefiniowane w *src/lib.rs*.
> Wtedy skrzynia binarna będzie miała dostęp do wszystkich jego publiczne elementów, rozpoczynając ich ścieżki od nazwy pakietu.
> Skrzynia binarna może użytkować skrzynię biblioteczną na takich samych zasadach jak skrzynia całkowicie zewnętrzna, tj. może korzystać tylko z publicznego interfejsu API.
> To pomaga zaprojektować dobry interfejs API; jesteś nie tylko jego autorem, ale także użytkownikiem!
>
> W [rozdziale 12][ch12]<!-- ignore --> zademonstrujemy taką organizację na przykładzie programu wiersza poleceń.

<!-- ### Starting Relative Paths with `super` -->
### Rozpoczynanie Ścieżek Względnych Od `super`

Możemy skonstruować względne ścieżki, które zaczynają się w module nadrzędnym, a nie w bieżącym lub korzeniu skrzyni, poprzez użycie `super` na początku ścieżki.
To tak jakby rozpocząć ścieżkę systemu plików od `..`.
Użycie `super` pozwala odwołać się do elementu znajdującego się w module nadrzędnym i ułatwia modyfikację drzewa modułów, gdy moduł jest ściśle związany ze swoim modułem nadrzędnym, który chcemy przenieść w inne miejsce drzewa.

Rozważmy kod z listingu 7-8, który modeluje sytuację, w której szef kuchni naprawia błędne zamówienie i osobiście przynosi je klientowi.
Funkcja `fix_incorrect_order` zdefiniowana w module `back_of_house` wywołuje funkcję `deliver_order` zdefiniowaną w module nadrzędnym, rozpoczynając ścieżkę do `deliver_order` od `super`:

<span class="filename">Plik: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

<span class="caption">Listing 7-8: Wywołanie funkcji przy użyciu ścieżki względnej zaczynającej się od `super`</span>

Funkcja `fix_incorrect_order` znajduje się w module `back_of_house`.
Za pomocą `super` osiągamy jego modułu nadrzędny, którym w tym przypadku jest `crate`, czyli korzeń.
W nim zaś odnajdujemy `deliver_order`.
Sukces!!! Zakładamy, że podczas ewentualnej reorganizację drzewa modułów skrzyni, moduł `back_of_house` i funkcja `deliver_order` prawdopodobnie zachowają względem siebie tę samą relację i zostaną przeniesione razem.
Dlatego użyliśmy `super`, abyśmy mieli mniej miejsc do zaktualizowania, jeśli ten kod zostanie przeniesiony w przyszłości do innego modułu.

### Upublicznianie Struktur i Typów Wyliczeniowych

Możemy również użyć `pub` by oznaczyć struktury i enumy jako publiczne.
Wiążą się z tym jednak pewne dodatkowe szczegóły.
Jeśli użyjemy `pub` przed definicją struktury, uczynimy ją publiczną, ale jej pola pozostaną prywatne.
W zależności od potrzeb, możemy uczynić każde z pól publicznym lub nie.
Na listingu 7-9 zdefiniowano publiczną strukturę `back_of_house::Breakfast`, której pole `toast` jest publiczne, zaś `seasonal_fruit` prywatne.
To modeluje restaurację, w której klient może wybrać rodzaj dołączonego do posiłku chleba, ale szef kuchni decyduje o tym, jakie owoce towarzyszą posiłkowi na podstawie tego, co akurat jest w sezonie i na stanie.
Dostępne owoce często się zmieniają, więc klienci nie mogą ich wybrać, ani nawet zobaczyć, jakie owoce dostaną.

<span class="filename">Plik: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

<span class="caption">Listing 7-9: Struktura, której część pól jest publiczna, zaś część prywatna</span>

Ponieważ w strukturze `back_of_house::Breakfast`, pole `toast` jest publiczne, to w `eat_at_restaurant` jest ono dostępne do zapisu i odczyt przy użyciu notacji kropkowej.
Równocześnie nie możemy użyć pola `seasonal_fruit` w `eat_at_restaurant`, ponieważ jest ono prywatne.
Proszę spróbować odkomentować linię modyfikującą wartość pola `seasonal_fruit` i zobaczyć do jakiego błędu to doprowadzi!

Inaczej jest w przypadku typów wyliczeniowych.
Jeśli uczynimy enum publicznym, to wszystkie jego warianty także staną się publiczne.
Wystarczy postawić `pub` przed słowem kluczowym `enum`, tak jak pokazano na listingu 7-10.

<span class="filename">Plik: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

<span class="caption">Listing 7-10: Oznaczenie enum jako publicznego, upublicznia też wszystkie jego warianty</span>

Ponieważ upubliczniliśmy enum `Appetizer`, to możemy używać wariantów `Soup` i `Salad` w `eat_at_restaurant`.

Typ wyliczeniowy, którego nie wszystkie warianty byłyby publiczne, nie byłyby zbyt użyteczny.
Równocześnie byłoby denerwujące, gdybyśmy musieli za każdym razem opatrywać wszystkie warianty enuma adnotacją `pub`.
Dlatego domyślnie warianty enuma są publiczne.
Inaczej jest ze strukturami, które często są użyteczne, pomimo że ich pola nie są publiczne.
Dlatego pola struktury podążają za ogólną zasadą, że wszystko jest domyślnie prywatne, chyba że opatrzone jest adnotacją `pub`.

Jest jeszcze jedna nieomówiona kwestia związana z `pub`, która związana jest z ostatnią funkcjonalnością systemu modułów pozostałą do opisania, czyli ze słowem kluczowym `use`.
Najpierw omówimy `use` samo w sobie, a następnie pokażemy jak połączyć `pub` i `use`.

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#eksponowanie-Ścieżek-za-pomocą-słowa-kluczowego-pub
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-an-io-project.html

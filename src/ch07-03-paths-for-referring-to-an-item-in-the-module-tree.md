<!-- ## Paths for Referring to an Item in the Module Tree -->
## Ścieżki Do Elementów W Drzewie Modułów

Podobnie jak podczas nawigowania po systemie plików, elementy w drzewie modułów wskazujemy za pomocą ścieżek. Aby wywołać funkcję, musimy znać do niej ścieżkę.

Każda ścieżka jest jednego z następujących dwóch rodzajów:

* *Ścieżka bezwzględna* jest pełną ścieżką startującą od korzenia skrzyni; dla kodu z zewnętrznej skrzyni, notacja ścieżki bezwzględnej zaczyna się od nazwy skrzyni, a dla kodu z bieżącej skrzyni, od słowa `crate`.
* *Ścieżka względna* startuje z bieżącego modułu i jej zapis zaczyna się od `self`, `super`, lub identyfikatora w bieżącym module.

Zarówno ścieżki bezwzględne jak i względne notujemy za pomocą jednego lub więcej identyfikatorów oddzielonych podwójnymi dwukropkami (`::`).

Powróćmy do Listingu 7-1 i załóżmy, że chcemy wywołać funkcję `add_to_waitlist`.
By to uczynić, musimy wpierw odpowiedzieć na pytanie: jaka jest ścieżka do funkcji `add_to_waitlist`?
Listing 7-3 obejmuje skrót Listingu 7-1, pozbawiony niektórych modułów i funkcji.

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

Spróbujmy skompilować Listing 7-3 i dowiedzmy się, dlaczego nie jest to jeszcze możliwe.
Otrzymany błąd jest pokazany na Listingu 7-4.

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

<span class="caption">Listing 7-4: Błędy kompilatora otrzymane podczas próby zbudowania kodu z Listing 7-3</span>

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

Wróćmy do błędu z Listingu 7-4, który mówił, że moduł `hosting` jest prywatny.
Chcemy, aby funkcja `eat_at_restaurant` w module nadrzędnym miała dostęp do funkcji `add_to_waitlist` w module podrzędnym.
Dlatego oznaczamy moduł `hosting` słowem kluczowym `pub`, co pokazano na listingu 7-5.

<span class="filename">Plik: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs}}
```

<span class="caption">Listing 7-5: Deklarowanie modułu `hosting` jako `pub` by użyć go z `eat_at_restaurant`</span>

Niestety, próba skompilowania kodu z Listingu 7-5 nadal kończy się błędem, co pokazano na Listingu 7-6.

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

<span class="caption">Listing 7-6: Błędy kompilatora przy budowaniu kodu z Listing 7-5</span>

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
Mimo że `front_of_house` nie jest publiczny, to ponieważ funkcja `eat_at_restaurant` jest zdefiniowana w tym samym module co `front_of_house` (czyli `eat_at_restaurant` i `front_of_house` są rodzeństwem), możemy odwoływać się do `front_of_house` z `eat_at_restaurant`.
Następny jest moduł `hosting` oznaczony symbolem `pub`.
A ponieważ możemy uzyskać dostęp do modułu nadrzędnego w stosunku do `hosting`, to możemy też uzyskać dostęp do `hosting`.
Ostatecznie, ponieważ funkcja `add_to_waitlist` jest oznaczona jako `pub` i możemy uzyskać dostęp do jej modułu nadrzędnego, to mamy prawo ją wywołać!

W przypadku ścieżki względnej, logika jest taka sama jak w przypadku ścieżki bezwzględnej, z wyjątkiem pierwszego kroku: zamiast zaczynać od korzenia skrzyni, ścieżka zaczyna się od `front_of_house`.
Rozpoczęcie ścieżki względnej od modułu, w którym zdefiniowany jest `eat_at_restaurant` (a tak jest w przypadku `front_of_house`) oczywiście zadziała.
Następnie, ponieważ `hosting` i `add_to_waitlist` są oznaczone jako `pub`, to reszta ścieżki również zadziała, tak jak i samo wywołanie funkcji!

Jeśli planujesz udostępnić swoją skrzynię biblioteczną, aby inne projekty mogły korzystać z twojego kodu, twoje publiczne API jest umową z użytkownikami skrzyni, która określa, w jaki sposób mogą oni korzystać z twojego kodu.
Jest wiele aspektów dotyczących zarządzania zmianami w publicznym API tak, by ułatwić utrzymanie od niego zależności.
Rozważania na ich temat wykraczają jednak poza zakres tej książki; zainteresowanych tym tematem odsyłamy do [The Rust API Guidelines][api-guidelines].

> #### Best Practices for Packages with a Binary and a Library
>
> We mentioned a package can contain both a *src/main.rs* binary crate root as
> well as a *src/lib.rs* library crate root, and both crates will have the
> package name by default. Typically, packages with this pattern of containing
> both a library and a binary crate will have just enough code in the binary
> crate to start an executable that calls code with the library crate. This
> lets other projects benefit from the most functionality that the package
> provides, because the library crate’s code can be shared.
>
> The module tree should be defined in *src/lib.rs*. Then, any public items can
> be used in the binary crate by starting paths with the name of the package.
> The binary crate becomes a user of the library crate just like a completely
> external crate would use the library crate: it can only use the public API.
> This helps you design a good API; not only are you the author, you’re also a
> client!
>
> In [Chapter 12][ch12]<!-- ignore -->, we’ll demonstrate this organizational
> practice with a command-line program that will contain both a binary crate
> and a library crate.

### Starting Relative Paths with `super`

We can construct relative paths that begin in the parent module, rather than
the current module or the crate root, by using `super` at the start of the
path. This is like starting a filesystem path with the `..` syntax. Using
`super` allows us to reference an item that we know is in the parent module,
which can make rearranging the module tree easier when the module is closely
related to the parent, but the parent might be moved elsewhere in the module
tree someday.

Consider the code in Listing 7-8 that models the situation in which a chef
fixes an incorrect order and personally brings it out to the customer. The
function `fix_incorrect_order` defined in the `back_of_house` module calls the
function `deliver_order` defined in the parent module by specifying the path to
`deliver_order` starting with `super`:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

<span class="caption">Listing 7-8: Calling a function using a relative path
starting with `super`</span>

The `fix_incorrect_order` function is in the `back_of_house` module, so we can
use `super` to go to the parent module of `back_of_house`, which in this case
is `crate`, the root. From there, we look for `deliver_order` and find it.
Success! We think the `back_of_house` module and the `deliver_order` function
are likely to stay in the same relationship to each other and get moved
together should we decide to reorganize the crate’s module tree. Therefore, we
used `super` so we’ll have fewer places to update code in the future if this
code gets moved to a different module.

### Making Structs and Enums Public

We can also use `pub` to designate structs and enums as public, but there are a
few details extra to the usage of `pub` with structs and enums. If we use `pub`
before a struct definition, we make the struct public, but the struct’s fields
will still be private. We can make each field public or not on a case-by-case
basis. In Listing 7-9, we’ve defined a public `back_of_house::Breakfast` struct
with a public `toast` field but a private `seasonal_fruit` field. This models
the case in a restaurant where the customer can pick the type of bread that
comes with a meal, but the chef decides which fruit accompanies the meal based
on what’s in season and in stock. The available fruit changes quickly, so
customers can’t choose the fruit or even see which fruit they’ll get.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

<span class="caption">Listing 7-9: A struct with some public fields and some
private fields</span>

Because the `toast` field in the `back_of_house::Breakfast` struct is public,
in `eat_at_restaurant` we can write and read to the `toast` field using dot
notation. Notice that we can’t use the `seasonal_fruit` field in
`eat_at_restaurant` because `seasonal_fruit` is private. Try uncommenting the
line modifying the `seasonal_fruit` field value to see what error you get!

Also, note that because `back_of_house::Breakfast` has a private field, the
struct needs to provide a public associated function that constructs an
instance of `Breakfast` (we’ve named it `summer` here). If `Breakfast` didn’t
have such a function, we couldn’t create an instance of `Breakfast` in
`eat_at_restaurant` because we couldn’t set the value of the private
`seasonal_fruit` field in `eat_at_restaurant`.

In contrast, if we make an enum public, all of its variants are then public. We
only need the `pub` before the `enum` keyword, as shown in Listing 7-10.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

<span class="caption">Listing 7-10: Designating an enum as public makes all its
variants public</span>

Because we made the `Appetizer` enum public, we can use the `Soup` and `Salad`
variants in `eat_at_restaurant`.

Enums aren’t very useful unless their variants are public; it would be annoying
to have to annotate all enum variants with `pub` in every case, so the default
for enum variants is to be public. Structs are often useful without their
fields being public, so struct fields follow the general rule of everything
being private by default unless annotated with `pub`.

There’s one more situation involving `pub` that we haven’t covered, and that is
our last module system feature: the `use` keyword. We’ll cover `use` by itself
first, and then we’ll show how to combine `pub` and `use`.

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-an-io-project.html

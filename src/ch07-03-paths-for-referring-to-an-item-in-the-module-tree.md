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
Równocześnie, za pomocą słowa kluczowego `pub` można upublicznić element i tym samym udostępnić nadrzędnym modułom część wewnętrznego kodu zawartego w module podrzędnym.

### Exposing Paths with the `pub` Keyword

Let’s return to the error in Listing 7-4 that told us the `hosting` module is
private. We want the `eat_at_restaurant` function in the parent module to have
access to the `add_to_waitlist` function in the child module, so we mark the
`hosting` module with the `pub` keyword, as shown in Listing 7-5.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs}}
```

<span class="caption">Listing 7-5: Declaring the `hosting` module as `pub` to
use it from `eat_at_restaurant`</span>

Unfortunately, the code in Listing 7-5 still results in an error, as shown in
Listing 7-6.

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

<span class="caption">Listing 7-6: Compiler errors from building the code in
Listing 7-5</span>

What happened? Adding the `pub` keyword in front of `mod hosting` makes the
module public. With this change, if we can access `front_of_house`, we can
access `hosting`. But the *contents* of `hosting` are still private; making the
module public doesn’t make its contents public. The `pub` keyword on a module
only lets code in its ancestor modules refer to it, not access its inner code.
Because modules are containers, there’s not much we can do by only making the
module public; we need to go further and choose to make one or more of the
items within the module public as well.

The errors in Listing 7-6 say that the `add_to_waitlist` function is private.
The privacy rules apply to structs, enums, functions, and methods as well as
modules.

Let’s also make the `add_to_waitlist` function public by adding the `pub`
keyword before its definition, as in Listing 7-7.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs}}
```

<span class="caption">Listing 7-7: Adding the `pub` keyword to `mod hosting`
and `fn add_to_waitlist` lets us call the function from
`eat_at_restaurant`</span>

Now the code will compile! To see why adding the `pub` keyword lets us use
these paths in `add_to_waitlist` with respect to the privacy rules, let’s look
at the absolute and the relative paths.

In the absolute path, we start with `crate`, the root of our crate’s module
tree. The `front_of_house` module is defined in the crate root. While
`front_of_house` isn’t public, because the `eat_at_restaurant` function is
defined in the same module as `front_of_house` (that is, `eat_at_restaurant`
and `front_of_house` are siblings), we can refer to `front_of_house` from
`eat_at_restaurant`. Next is the `hosting` module marked with `pub`. We can
access the parent module of `hosting`, so we can access `hosting`. Finally, the
`add_to_waitlist` function is marked with `pub` and we can access its parent
module, so this function call works!

In the relative path, the logic is the same as the absolute path except for the
first step: rather than starting from the crate root, the path starts from
`front_of_house`. The `front_of_house` module is defined within the same module
as `eat_at_restaurant`, so the relative path starting from the module in which
`eat_at_restaurant` is defined works. Then, because `hosting` and
`add_to_waitlist` are marked with `pub`, the rest of the path works, and this
function call is valid!

If you plan on sharing your library crate so other projects can use your code,
your public API is your contract with users of your crate that determines how
they can interact with your code. There are many considerations around managing
changes to your public API to make it easier for people to depend on your
crate. These considerations are out of the scope of this book; if you’re
interested in this topic, see [The Rust API Guidelines][api-guidelines].

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

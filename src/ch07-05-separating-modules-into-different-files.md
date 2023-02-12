<!-- ## Separating Modules into Different Files -->
## Umieszczanie Modułów w Osobnych Plikach

Jak dotąd, wszystkie przykłady w tym rozdziale definiowały wiele modułów w pojedynczym pliku.
Kiedy moduły stają się duże, warto przenieść ich definicje do osobnych plików, aby ułatwić poruszanie się po kodzie.

Na przykład, zacznijmy od kodu z listingu 7-17, który zawiera wiele modułów restauracji.
Zamiast trzymać wszystkie moduły w pliku głównym skrzyni, przeniesiemy je do plików.
W tym przypadku plikiem głównym skrzyni jest *src/lib.rs*, ale ta procedura działa również w przypadku skrzyń binarnych, których plikiem głównym jest *src/main.rs*.

Najpierw przeniesiemy moduł `front_of_house` do jego własnego pliku.
Usuwamy kod modułu `front_of_house` zawarty w nawiasach klamrowych, pozostawiając tylko deklarację `mod front_of_house;`, tak że *src/lib.rs* zawiera kod pokazany na listingu 7-21.
Ten kod nie się skompiluje, dopóki nie utworzymy *src/front_of_house.rs*, o zawartości pokazanej na listingu 7-22.

<span class="filename">Plik: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

<span class="caption">Listing 7-21: Deklaracja modułu `front_of_house`, którego ciało znajdzie się w *src/front_of_house.rs*</span>

Następnie umieszczamy kod, który znajdował się w nawiasach klamrowych, w nowym pliku o nazwie *src/front_of_house.rs*, jak pokazano na Listingu 7-22.
Kompilator wie, że ma szukać kodu w tym pliku, ponieważ natrafił w korzeniu skrzyni na deklarację modułu o nazwie `front_of_house`.

<span class="filename">Plik: src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

<span class="caption">Listing 7-22: Definicje wewnątrz modułu `front_of_house` w *src/front_of_house.rs*</span>

Warto zauważyć, że plik za pomocą deklaracji `mod` wystarczy załadować w *jednym miejscu* drzewa modułów.
Kiedy kompilator wie, że plik jest częścią projektu (i wie gdzie w drzewie modułów rezyduje kod, ponieważ umieszczono tam deklarację `mod`), inne pliki w projekcie powinny odwoływać się do kodu załadowanego z pliku używając ścieżki do miejsca, w którym został zadeklarowany, tak jak to zostało opisane w sekcji [„Paths for Referring to an Item in the Module Tree“][paths]<!-- ignore -->.
Innymi słowy, `mod` *nie* jest znaną z niektórych języków programowania operacją „include“.

Następnie przeniesiemy moduł `hosting` do jego własnego pliku.
Proces będzie nieco inny, bo `hosting` jest modułem podrzędnym w stosunku do `front_of_house`, a nie modułem głównym.
Umieścimy plik dla `hosting` w nowym katalogu, nazwanym zgodnie z jego przodkami w drzewie modułów, czyli w *src/front_of_house/*.

Przenoszenie `hosting` rozpoczynamy od zmieniamy *src/front_of_house.rs* tak, aby zawierał tylko deklarację modułu `hosting`:

<span class="filename">Plik: src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

Następnie tworzymy katalog *src/front_of_house*, a w nim plik *hosting.rs* zawierający definicje z modułu `hosting`:

<span class="filename">Plik: src/front_of_house/hosting.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

Jeśli zamiast tego umieścilibyśmy *hosting.rs* w katalogu *src*, to kompilator oczekiwałby, że kod *hosting.rs* zostanie zadeklarowany w module `hosting` znajdującym się w korzeniu skrzyni, a nie w module `front_of_house`.
Reguły kompilatora dotyczące tego, które pliki sprawdzać pod kątem kodu poszczególnych modułów, sprawiają, że drzewo modułów jest odzwierciedlone przez katalogi i pliki.

> ### Alternatywne Ścieżki do Plików
>
> Dotychczas omówiliśmy najbardziej idiomatyczne nazewnictwo ścieżek do plików, które używa kompilator Rusta.
> Ale Rust obsługuje również nazewnictwo w starszym stylu.
> Kompilator będzie szukał kodu dla modułu `front_of_house` zadeklarowanego w korzeniu skrzyni, w:
>
> * *src/front_of_house.rs* (co omówiliśmy)
> * *src/front_of_house/mod.rs* (starszy styl, nadal obsługiwany)
>
> Kompilator będzie szukał kodu dla modułu `hosting` zadeklarowanego w module `front_of_house`, w:
>
> * *src/front_of_house/hosting.rs* (co omówiliśmy)
> * *src/front_of_house/hosting/mod.rs* (starszy styl, nadal obsługiwany)
>
> Próba użycia obu stylów dla tego samego modułu poskutkuje błędem kompilatora.
> Używanie mieszanki obu stylów dla różnych modułów w tym samym projekcie jest dozwolone, ale może być mylące dla osób nawigujących po projekcie.
>
> Głównym minusem używania plików o nazwie *mod.rs* jest to, że może być ich w projekcie wiele.
> To zaś może prowadzić do pomyłek, gdy kilka z nich będzie równocześnie otwartych w edytorze.

Przenieśliśmy kod każdego modułu do osobnego pliku, ale drzewo modułów nie zmieniło się.
Wywołania funkcji w `eat_at_restaurant` będą działać bez żadnych modyfikacji, mimo że ich definicje znajdują się w różnych plikach.
Dzięki tej technice można przenosić moduły do nowych plików w miarę jak rosną ich rozmiary.

Proszę zauważyć, że deklaracja `pub use crate::front_of_house::hosting` w *src/lib.rs* również nie uległa zmianie, oraz że `use` nie ma wpływu na to, które pliki są kompilowane jako część skrzyni.
Słowo kluczowe `mod` deklaruje moduły, a Rust szuka kodu wchodzącego w skład każdego z nich w pliku nazwanym tak jak moduł.

## Podsumowanie

Rust pozwala podzielić pakiet na wiele skrzyń, a każdą ze skrzyń na moduły, tak aby można było odwoływać się do elementów zdefiniowanych w jednym module z innego modułu.
Pozwalają na to ścieżki bezwzględne i względne.
Ścieżki te mogą być włączone w zasięg za pomocą deklaracji `use`, dzięki czemu można w nim używać krótszej ścieżki, co jest przydatne gdy elementy są używane w nim wielokrotnie.
Kod modułu jest domyślnie prywatny, ale można upublicznić jego definicje za pomocą słowa kluczowego `pub`.

W następnym rozdziale przyjrzymy się niektórym, zawartym w bibliotece standardowej, strukturom danych dla kolekcji, które można wykorzystać w swoim starannie zorganizowanym kodzie.

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html

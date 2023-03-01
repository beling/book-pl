<!-- ## Storing Lists of Values with Vectors -->
## Przechowywanie List Wartości w Wektorach

Pierwszym typem kolekcji, któremu się przyjrzymy jest `Vec<T>`, znany również jako *wektor*.
Wektory pozwalają przechować pewną liczbę wartości, umieszczając je w pamięci jedna obok drugiej.
Wektory mogą zawierać tylko wartości tego samego typu.
Są one przydatne, gdy mamy listę elementów, takich jak linie tekstu z pliku lub ceny towarów w koszyku zakupowym.

<!-- ### Creating a New Vector -->
## Tworzenie Nowego Wektora

Aby utworzyć nowy pusty wektor, wywołujemy funkcję `Vec::new`, tak jak pokazano na Listingu 8-1.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-01/src/main.rs:here}}
```

<span class="caption">Listing 8-1: Utworzenie nowego, pustego wektora do przechowywania wartości typu `i32`</span>

Proszę zauważyć, że dodaliśmy w kodzie adnotację typu.
Ponieważ nie wstawiamy do tego wektora żadnych wartości, Rust nie wie, jakiego rodzaju elementy zamierzamy przechowywać.
Co zaś istotne, wektory są implementowane z wykorzystaniem generyczności; w rozdziale 10 opowiem jak jej użyć we własnych typach.
Na razie wystarczy wiedzieć, że typ `Vec<T>` z biblioteki standardowej może przechowywać elementy dowolnego, zadanego typu.
Ten typ możemy wskazać w nawiasach kątowych podczas tworzenia wektora.
Na listingu 8-1 powiedzieliśmy Rustowi, że `Vec<T>` w `v` będzie przechowywał elementy typu `i32`.

Zazwyczaj jednak `Vec<T>` będzie tworzony z wartościami początkowymi i Rust wywnioskuje typ przechowywanej wartości.
Adnotacja typu nie będzie wtedy wymagana.
Rust dostarcza wygodne makro `vec!`, tworzące nowy wektor z podanymi wartościami.
Przykładowo, na listing 8-2 tworzony jest `Vec<i32>` zawierający `1`, `2`, i `3`.
Typem wartości jest tam `i32`, ponieważ jest to domyślny typ dla liczb całkowitych, co omówiliśmy w sekcji [„Typy danych“][data-types]<!-- ignore --> rozdziału 3.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-02/src/main.rs:here}}
```

<span class="caption">Listing 8-2: Tworzenie nowego wektora zawierającego wartości</span>

Ponieważ podano początkowe wartości typu `i32`, to Rust może wywnioskować, że typem `v` jest `Vec<i32>`, nawet bez adnotacji typu.
Za chwilę omówimy, jak modyfikować wektor.

<!-- ### Updating a Vector -->
### Uaktualnianie Wektora

Listing 8-3 pokazuje jak utworzyć wektor, a następnie dodać do niego elementy za pomocą metody `push`.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-03/src/main.rs:here}}
```

<span class="caption">Listing 8-3: Dodawanie elementów do wektora za pomocą metody `push`</span>

Jak w przypadku każdej zmiennej, jeśli chcemy mieć możliwość zmiany jej wartości, musimy uczynić ją mutowalną za pomocą słowa kluczowego `mut`, tak jak zostało to omówione w rozdziale 3.
Ponieważ wszystkie dodawane liczby są typu `i32`, to Rust wywnioskuje odpowiedni typ wektora, nawet bez adnotacji `Vec<i32>`.

<!-- Reading Elements of Vectors -->
### Czytanie Elementów Wektora

Istnieją dwa sposoby odwołania się do wartości przechowywanej w wektorze: poprzez indeksowanie lub użycie metody `get`.
W poniższych przykładach, dla przejrzystości, opatrzyliśmy adnotacjami typy wartości zwracanych przez te funkcje.

Listing 8-4 pokazuje obie metody dostępu do wartości w wektorze. Pierwsza wykorzystuje składnie indeksowania, druga zaś metodę `get`.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

<span class="caption">Listing 8-4: Użycie indeksowania lub metody `get` by uzyskać dostęp do elementu w wektorze</span>

W tym miejscu warto zwrócić uwagę na kilka szczegółów.
Ponieważ wektory są indeksowane od zera, to by uzyskać trzeci element, używamy indeksu `2`.
Użycie `&` i `[]` daje referencję do elementu znajdującego się pod zadanym indeksem.
Metoda `get`, której argumentem jest żądany indeks, zwraca `Option<&T>`, który możemy użyć z `match`.

Powodem, dla którego Rust udostępnia dwa sposoby odwoływania się do elementu, jest danie możliwość wyboru, jak ma zachować się program, podczas próby użycia indeksu spoza zakresu indeksów istniejących elementów.
Na przykład zobaczmy co się stanie, gdy za pomocą każdej z tych technik spróbujemy uzyskać dostęp do elementu o indeksie 100 z wektora o pięciu elementach, jak pokazano na listingu 8-5.

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

<span class="caption">Listing 8-5: Próba uzyskania dostęp do elementu o indeksie 100 z wektora o pięciu elementach</span>

Kiedy uruchomimy ten kod, odwołanie się do nieistniejącego elementu pierwszą metodą `[]` spowoduje, że program spanikuje.
Tej metody najlepiej więc użyć, gdy chcemy, aby podczas próby dostępu do elementu poza końcem wektora, program został zakończony.

Dla odmiany metoda `get` wywołana z nieistniejącym indeksem nie panikuje, tylko zwraca `None`.
To przydaje się, gdy sporadyczny dostęp do elementu poza zakresem wektora jest spodziewany.
Należy wtedy wyposażyć kod w logikę obsługującą oba możliwe rezultaty tej metody, zarówno `Some(&element)` jak i `None`, tak jak to omówiono w rozdziale 6.
Na przykład, indeks może pochodzić od osoby wprowadzającej numer.
Jeśli przypadkowo wprowadzi ona zbyt dużą liczbę i program otrzyma wartość `None`, można zakomunikować ile elementów znajduje się w bieżącym wektorze i dać użytkownikowi kolejną szansę na wprowadzenie poprawnej wartości.
Będzie to dla niego bardziej przyjazne niż zakończenie programu z powodu literówki!

Nadzorca pożyczania, egzekwując zasady własności i reguły pożyczania (omówione w rozdziale 4), zapewni że wszelkie reference do zawartości wektora, będą poprawne.
Proszę przypomnieć sobie regułę mówiącą, że nie można mieć mutowalnych i niemutowalnych referencji w tym samym zasięgu.
Ta zasada ujawnia się na listingu 8-6, gdzie trzymając niemutowalną referencję do pierwszego elementu wektora, próbujemy dodać element na jego koniec.
Jeśli dodatkowo spróbujemy odwołać się do tego elementu w dalszej części funkcji, to ten program się nie skompiluje:


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-06/src/main.rs:here}}
```

<span class="caption">Listing 8-6: Próba dodania do wektora, do którego elementu trzymamy referencję</span>

Próba skompilowania tego kodu daje następujący błąd:


```console
{{#include ../listings/ch08-common-collections/listing-08-06/output.txt}}
```

Wydaje się, że kod na listingu 8-6 powinien działać: dlaczego referencja do pierwszego elementu miałaby się przejmować zmianami na końcu wektora?
Błąd wynika ze sposobu działania wektorów: ponieważ wektory umieszczają wartości w pamięci obok siebie, to dodanie nowego elementu na końcu wektora może wymagać przydzielenia nowej pamięci i skopiowania do niej uprzednio dodanych elementów, gdy zabraknie miejsca tam, gdzie są one obecnie przechowywane.
W takim przypadku referencja do pierwszego elementu wskazywałaby na zwolniony obszar pamięci.
Zaś reguły pożyczania uniemożliwiają doprowadzenie programu do takiej sytuacji.

> Uwaga: Więcej o implementacji typu `Vec<T>` można znaleźć w [“The
> Rustonomicon”][nomicon].

<!-- ### Iterating over the Values in a Vector -->
### Iterowanie po Zawartych w Wektorze Wartościach

Aby uzyskać dostęp kolejno do wszystkich elementów wektora, to zamiast używać ich indeksów, lepiej go przeiterować.
Listing 8-7 pokazuje, jak za pomocą pętli `for` uzyskać niemutowalne referencje do wszystkich liczb (typu `i32`) zawartych w wektorze i te liczby wypisać.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

<span class="caption">Listing 8-7: Wypisanie zawartości wektora za pomocą pętli `for` iterującej po jego elementach</span>

Iterując po mutowalnych referencjach do elementów, możemy również zmodyfikować wszystkie elementy mutowalnego wektora. Pętla `for` na Listingu 8-8 dodaje do każdego `50`.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

<span class="caption">Listing 8-8: Iterowanie po mutowalnych referencjach do elementów wektora</span>

Aby za pomocą operatora `+=` zmienić wartość, do której odnosi się mutowalna referencja `i`, wpierw musimy się do tej wartości dostać za pomocą operatora dereferencji `*`.
Więcej o operatorze dereferencji powiemy w sekcji ["Following the Pointer to the Value with the Dereference Operator"][deref]<!-- ignore --> rozdziału 15.

Dzięki regułom nadzorcy pożyczania, zarówno niemutowalne jak i mutowalne iterowanie po wektorze, jest bezpieczne.
Gdybyśmy na Listingu 8-7 lub 8-8 spróbowali w ciałach pętli `for` wstawiać do wektora elementy lub je usuwać, otrzymalibyśmy błąd kompilatora podobny do tego, który powodował kod na Listingu 8-6.
Ponieważ pętla `for` posiada referencję do wektora, to nie można go równocześnie modyfikować inaczej, niż poprzez tę referencję.

<!-- ### Using an Enum to Store Multiple Types -->
### Przechowywanie Wartości Różnych Typów za Pomocą Typu Wyliczeniowego

Wektory mogą przechowywać tylko wartości tego samego typu, co może być uciążliwe;
nierzadko zdarza się potrzeba przechowani listy elementów różnych typów.
Na szczęście warianty wyliczenia są zdefiniowane w ramach tego samego typu enum, więc kiedy potrzebujemy jednego typu do reprezentowania elementów różnych typów, możemy zdefiniować i użyć enuma!

Na przykład, załóżmy, że chcemy uzyskać wartości z wiersza arkusza kalkulacyjnego, w którym niektóre kolumny w wierszu zawierają liczby całkowite, niektóre liczby zmiennoprzecinkowe, zaś niektóre łańcuchy.
Możemy zdefiniować typ wyliczeniowy, którego warianty będą trzymać wartości różnych typów, a jednocześnie wszystkie te warianty będą traktowane jako ten sam typ.
Następnie możemy utworzyć wektor takich enumów, efektywnie przechowujący wartości różnych typów.
Takie działanie ilustruje listing 8-9.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

<span class="caption">Listing 8-9: Zdefiniowanie `enum` do przechowywania wartości różnych typów w jednym wektorze</span>

Rust już w czasie kompilacji musi wiedzieć, jakiego typu elementy są dozwolone w wektorze, aby dokładnie przewidzieć, ile pamięci na stercie będzie potrzebne do ich przechowania.
Dlatego musimy jednoznacznie te typy określić.
Gdyby Rust pozwolił wektorowi przechowywać elementy dowolnych typów, istniałoby zagrożenie, że jeden lub więcej typów spowodowałoby błędy w operacjach wykonywanych na elementach wektora.
Użycie enum plus wyrażenia `match` oznacza, że Rust zapewni w czasie kompilacji, że każdy możliwy przypadek zostanie obsłużony, jak omówiono w rozdziale 6.

Gdybyśmy nie znali wyczerpującego zestawu typów, których wartości w czasie wykonywania programu będą dodawane do wektora, to nie moglibyśmy użyć omówionej techniki wykorzystującej enum.
Instead, you can use a trait object, which we’ll cover in Chapter 17.

Now that we’ve discussed some of the most common ways to use vectors, be sure
to review [the API documentation][vec-api]<!-- ignore --> for all the many
useful methods defined on `Vec<T>` by the standard library. For example, in
addition to `push`, a `pop` method removes and returns the last element.

### Dropping a Vector Drops Its Elements

Like any other `struct`, a vector is freed when it goes out of scope, as
annotated in Listing 8-10.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

<span class="caption">Listing 8-10: Showing where the vector and its elements
are dropped</span>

When the vector gets dropped, all of its contents are also dropped, meaning the
integers it holds will be cleaned up. The borrow checker ensures that any
references to contents of a vector are only used while the vector itself is
valid.

Let’s move on to the next collection type: `String`!

[data-types]: ch03-02-data-types.html#data-types
[nomicon]: ../nomicon/vec/vec.html
[vec-api]: ../std/vec/struct.Vec.html
[deref]: ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator

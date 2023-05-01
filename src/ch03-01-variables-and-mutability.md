## Zmienne i ich modyfikowalność

Tak jak wspomniano w rozdziale [„Storing Values with
Variables”][storing-values-with-variables]<!-- ignore --> , zmienne są domyślnie niemodyfikowalne (niemutowalne, ang. *immutable*). To jeden z wielu prztyczków, którymi Rust zachęca cię do tworzenia kodu w pełni wykorzystującego mechanizmy bezpieczeństwa i prostoty współbieżności, które oferuje ten język programowania. Jednakże nadal możesz
uczynić zmienne modyfikowalnymi. Przyjrzyjmy się bliżej temu, jak i dlaczego Rust zachęca cię do preferowania niemodyfikowalności zmiennych oraz czemu czasem możesz chcieć zrezygnować z tej właściwości.

Gdy zmienna jest niemodyfikowalna, po przypisaniu wartości do danej nazwy, nie można później zmienić tej wartości. Aby to zobrazować, utworzymy nowy projekt o nazwie *variables* w folderze *projects* korzystając z komendy
`cargo new --bin variables`.

Następnie w nowo utworzonym folderze *variables*, odnajdź i otwórz *src/main.rs*, zmień kod w tym pliku na poniższy, który jednak jeszcze nie skompiluje się poprawnie:

<span class="filename">Plik: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

Zapiszmy zmiany i uruchommy program, używając `cargo run`. Powinniśmy otrzymać następujący komunikat o błędzie związanym z niemutowalnością:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

Ten przykład pokazuje, jak kompilator pomaga ci odnajdywać błędy w twoich programach. Mimo że błędy kompilacji mogą być denerwujące, świadczą jedynie o tym, że twój program jeszcze nie działa prawidłowo. Nie wykonuje w bezpieczny sposób tego, co chcesz, by robił. Tw błędy *nie* oznaczają jednak, że nie jesteś dobrym programistą! Nawet doświadczeni Rustowcy nadal napotykają błędy podczas kompilacji.

Otrzymany komunikat błędu `` cannot assign twice to immutable variable `x``` oznacza, że nie można dwukrotnie przypisać wartości do niemodyfikowalnej zmiennej `x`.
Pierwotnie nadanej wartości nie można zmienić.

To ważne, że napotykamy błędy w trakcie kompilacji, gdy próbujemy zmienić wartość, którą wcześniej określiliśmy jako niemodyfikowalną, gdyż takie działanie może prowadzić do podatności i błędów w programie. Jeżeli pierwsza część kodu opiera się na założeniu, że dana wartość nigdy nie ulegnie zmianie, a inna część kodu zmienia tę wartość, pierwsza część kodu może przestać wykonywać swoje zadanie poprawnie. Przyczyna tego rodzaju błędów może być trudna do zidentyfikowania po wystąpieniu, szczególnie gdy druga część kodu zmienia daną wartość tylko *czasami*.

W Ruście, kompilator gwarantuje, że jeżeli ustawimy wartość na niemodyfikowalną, to naprawdę nigdy nie ulegnie zmianie. Oznacza to, że czytając i pisząc kod, nie musisz ciągle sprawdzać gdzie i jak wartość może się zmienić. W związku
z tym tworzony przez ciebie kod staje się łatwiejszy do zrozumienia.

Jednak modyfikowalność może być też bardzo użyteczna. Zmienne są tylko domyślnie niemodyfikowalne. Można uczynić
je modyfikowalnymi, dodając `mut` przed nazwą zmiennej. Poza tym, że dzięki dodaniu `mut` możliwa jest modyfikacja wartości zmiennej, jest ono też wyraźnym sygnałem dla osób, które będą czytały kod w przyszłości. Informuje, że inne
części kodu będą modyfikować wartość danej zmiennej.

Na przykład, zmieńmy kod w *src/main.rs* na poniższy:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

Gdy teraz uruchomimy program, otrzymamy:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

Możemy zmienić wartość, do której odwołuje się `x` z `5` na `6`, dzięki wykorzystaniu `mut`. W niektórych przypadkach będziesz chciał uczynić zmienną modyfikowalną, ponieważ sprawi to, że pisanie kodu stanie się wygodniejsze niż
gdyby tworzono go tylko z użyciem niemodyfikowalnych zmiennych.

### Stałe

Brak możliwości modyfikacji wartości zmiennej może przypominać ci inne rozwiązanie programistyczne, które wykorzystuje wiele języków programowania:
stałe (*constants*). Podobnie jak zmienne niemodyfikowalne, stałe to wartości, których nie można zmienić, przypisane do nazw, ale występuje też kilka różnic między stałymi i zmiennymi.

Po pierwsze, nie możesz używać `mut` do stałych. Stałe są nie tylko domyślnie niemodyfikowalne. One są zawsze niemodyfikowalne.
Do deklaracji stałej wykorzystujemy słowo kluczowe `const` zamiast `let` i *zawsze* musimy określić typ wartości. Typy danych i ich adnotacje omówimy już niedługo, w następnym podrozdziale ["Typy Danych"][data-types]<!-- ignore-->,
więc nie przejmuj się na razie szczegółami. Po prostu zapamiętaj, że zawsze musisz nadać stałej typ danych.

Stałe mogą być deklarowane w każdym zasięgu, włączając w to zasięg globalny, dzięki czemu są bardzo użyteczne w przypadku wartości, z których korzysta wiele części kodu.

Ostatnia różnica to, że stałym można nadać wartości tylko za pomocą stałych wyrażeń, a nie takich obliczanych dopiero trakcie działania programu.

Oto przykład deklaracji stałej:

```rust
const TRZY_GODZINY_W_SEKUNDACH: u32 = 60 * 60 * 3;
```

Nazwą stałej jest `TRZY_GODZINY_W_SEKUNDACH`, zaś jej wartością jest iloczyn: 60 (liczba sekund w minucie), kolejnej 60 (liczba minut w godzienie) i 3 (liczba godzin którą chcemy odliczyć w programie). Konwencja nazewnicza Rusta dla stałych
zobowiązuje do wykorzystywanie tylko dużych liter z podkreśleniami między słowami.
The compiler is able to evaluate a limited set of operations at compile time, which lets us choose to write out this value in a way that’s easier to understand and verify, rather than setting this constant to the value 10,800. See the [Rust Reference’s section on constant evaluation][const-eval] for more information on what operations can be used when declaring constants.

Stałe są dostępne przez cały okres działania programu w zasięgu, w którym zostały zadeklarowane, stają się tym samym dobrym wyborem dla wartości w twojej domenie aplikacji, które mogą być wykorzystywane przez różne elementy programu,
takich jak maksymalna liczba punktów, które może uzyskać gracz, czy też prędkość światła.

Nazywanie predefiniowanych wartości używanych przez twój program stałymi jest użyteczne w przekazywaniu znaczenia wykorzystywanych wartości dla przyszłych współtwórców kodu. Pomaga to w utrzymaniu predefiniowanych wartości w jednym
miejscu i ułatwia ich późniejsze uaktualnianie.

### Przesłanianie

Jak widzieliśmy w poradniku do gry zgadywanki w [rozdziale 2][comparing-the-guess-to-the-secret-number]<!-- ignore -->, można zadeklarować nową zmienną o takiej samej nazwie, jak miała dawna zmienna, i ta nowa zmienna przesłania dawną zmienną. Rustowcy mówią, że pierwsza zmienna jest *przesłoniona* przez drugą. I właśnie tą nową zmienną użyje kompilator w miejscach wystąpienia jej nazwy, aż do czasu gdy i ona nie zostanie przesłonięta, albo nie skończy się zasięg jej życia.
Możemy przesłonić zmienną poprzez wykorzystanie tej
samej nazwy zmiennej i ponowne użycie słowa kluczowego `let`, tak jak poniżej:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

Ten program najpierw deklaruje zmienną `x` o wartość `5`. Następnie tworzy nową zmienną `x` powtarzając `let x =`, pobiera oryginalną wartość zmiennej i dodaje do niej `1` w wyniku czego wartość `x` to obecnie `6`. Użycie deklaracji `let` po raz trzeci również przesłania `x` i tworzy kolejną zmienną, której wartość jest ustalona poprzez przemnożenie poprzedniej wartość przez `2`, czyli na `12`. Gdy
uruchomimy ten program, otrzymamy:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

To nie to samo, co nadanie `mut` zmiennej, gdyż jeżeli przypadkowo spróbujemy ponownie przypisać wartość do zmiennej, nie wykorzystując słowa kluczowego `let` otrzymamy błąd kompilacji. Dzięki użyciu `let`, możemy przeprowadzić
kilka transformacji na wartości, pozostawiając przy tym zmienną niemodyfikowalną.

Inna różnica między `mut` i przesłanianiem to, że za każdym razem, gdy używamy słowa kluczowego `let`, tworzymy nową zmienną, co oznacza, że możemy wybrać inny typ danych, ale ponownie użyć tej samej nazwy zmiennej. Na przykład, powiedzmy, że nasz program prosi użytkownika o pokazanie ilości spacji, jaka ma zostać umieszczona między jakimś tekstem, poprzez wpisanie tych spacji, ale my tak naprawdę chcemy przechowywać tę wartość jako liczbę:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

Powyższa konstrukcja jest dozwolona, gdyż pierwsza zmienna `spaces` typu string, to zupełnie inna zmienna niż druga zmienna `spaces` typu numerycznego. Dzięki przesłanianiu nie musimy wykorzystywać dwóch różnych nazw np. `spaces_str` i `spaces_num`. Zamiast tego, ponownie korzystamy z prostszej nazwy `spaces`. Jednak, jeżeli spróbowalibyśmy użyć
`mut` dla tej zmiennej otrzymalibyśmy błąd kompilacji:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

Błąd mówi o tym, że nie możemy zmodyfikować typu zmiennej:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}

```

Teraz gdy poznaliśmy już działanie zmiennych, przyjrzyjmy się bliżej typom danych, jakich mogą być zmienne.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#porównywanie-odpowiedzi-z-sekretnym-numerem
[data-types]: ch03-02-data-types.html#typy-danych
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#zapisywanie-wartości-w-zmiennych
[const-eval]: ../reference/const_eval.html

<!-- Old heading. Do not remove or links may break. -->
<a id="the-match-control-flow-operator"></a>
<!-- ## The `match` Control Flow Construct -->
## Konstrukcja Przepływu Sterowania `match`

Rust posiada niezwykle potężną konstrukcję przepływu sterowania `match`, która pozwala na porównanie wartości z serią wzorców, a następnie wykonanie kodu przypisanego do pasującego wzorca.
Wzorce mogą składać się z literałów, nazw zmiennych, wieloznaczników (ang. woldcards) i wielu innych rzeczy;
[rozdział 18][ch18-00-patterns]<!-- ignore --> objaśnia działanie wszystkich rodzajów wzorców.
Siła `match` wynika z ekspresyjności wzorców i faktu, że kompilator sprawdza, czy wszystkie możliwe przypadki są obsługiwane.

Na wyrażeniu `match` można spojrzeć jak na maszynę do sortowania monet:
monety zjeżdżają po torze wzdłuż którego znajdują się różnej wielkości otwory i każda wpada w pierwszy napotkany otwór, do którego pasuje. W ten sam sposób wartości przechodzą przez każdy wzorzec w `match`, aż do napotkania pierwszego wzorca, do którego wartość "pasuje". Po jego napotkaniu, wartość wpada do powiązanego z tym wzorcem bloku kodu, który zostaje wykonany.

Skoro mowa o monetach, to użyjmy ich jako przykładu z wykorzystaniem `match`!
Możemy napisać funkcję, która pobiera nieznaną amerykańską monetę i tak jak maszyna licząca określa, jaka to moneta, oraz zwraca jej wartość w centach, jak pokazano na listingu 6-3.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

<span class="caption">Listing 6-3: Wyrażenie `match` dopasowujące warianty enuma do wzorców</span>

Rozłóżmy `match` w funkcji `value_in_cents` na czynniki pierwsze.
Najpierw umieszczamy słowo kluczowe `match`, po którym następuje wyrażenie, którym w tym przypadku jest wartość `coin`.
To wyrażenie pełni podobną rolę do wyrażenia warunkowego używanego z `if`, ale jest duża różnica: w `if` warunek musi dawać wartość boolowską, zaś tutaj wyrażnie może być dowolnego typu. Typem `coin` w tym przykładzie jest enum `Coin`, który zdefiniowaliśmy w pierwszej linii.

Następne są odnogi `match`. Odnoga składa się z dwóch części: wzorca i kodu. Pierwsza odnoga ma wzorzec, którym jest wartość `Coin::Penny`, a następnie operator `=>`, który oddziela wzorzec i kod do uruchomienia. Kodem w tym przypadku jest po prostu wartość `1`. Każda odnoga jest oddzielone od następnej przecinkiem.

Wykonanie wyrażenia `match` polega na porównaniu wartości wynikowej z wzorcem każdej z odnóg, w kolejności ich wystąpienia.
Jeśli wzorzec pasuje do wartości, wykonywany jest kod związany z tym wzorcem.
Jeśli wzorzec nie pasuje do wartości, wykonanie przechodzi do następnej odnogi, zupełnie jak w maszynie do sortowania monet.
Możemy mieć tyle odnóg ile potrzebujemy. Na Listingu 6-3, nasz `match` ma ich cztery.

Kod związany z każdą odnogą jest wyrażeniem, a wartość wynikowa wyrażenia w pasującej odnodze jest wartością, która zostaje zwrócona z całego wyrażenia `match`.

Zazwyczaj nie używamy nawiasów klamrowych, jeśli kod ramienia odnogi jest krótki, tak jak na Listingu 6-3, gdzie każda odnoga jedynie zwraca wartość. Chcąc uruchomić wiele linii kodu w jednej odnodze należy użyć nawiasów klamrowych, a przecinek po odnodze jest wtedy opcjonalny. Na przykład poniższy kod drukuje "Lucky penny!" za każdym razem, gdy metoda jest wywoływana z `Coin::Penny`, ale wciąż zwraca ostatnią wartość bloku, `1`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

<!-- ### Patterns That Bind to Values -->
### Wzorce Deklarujące Zmienne

Inną przydatną cechą odnóg match jest to, że mogą one tworzyć zmienne zainicjowane fragmentami wartości pasującej do wzorca.
Tym samym pozwalają wyodrębnić wartości z wariantów enuma.

By to zilustrować, zmienimy jeden z wariantów naszego wyliczenia tak, aby przechowywał on wewnątrz dane.
Od 1999 do 2008 roku Stany Zjednoczone biły ćwierćdolarówki mające po jednej ze stron różne wzory dla każdego z 50 stanów.
Żadna inna moneta nie miała wzorów stanowych, więc tylko ćwiartki będą miały dodatkową wartość. Możemy ją uwzględnić w naszym typie `enum` poprzez zmianę wariantu `Quarter` tak, aby zawierał wartość typu `UsState`, co zrobiliśmy na Listingu 6-4.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

<span class="caption">Listing 6-4: Enum `Coin` z wariantem `Quarter` trzymającym wartość typu `UsState`</span>

Wyobraźmy sobie, że znajomy chce zebrać wszystkie 50 ćwiartek stanowych.
Segregując nasze drobniaki według typów monet, będziemy podawać nazwę stanu związanego z każdą ćwiartką,
by nasz przyjaciel mógł dodać ją do swojej kolekcji, gdy jeszcze takiej ćwiartki nie posiada.

W kodzie wyrażenia match dodajemy zmienną o nazwie `state` do wzorca dopasowującego wariant `Coin::Quarter`.
Kiedy `Coin::Quarter` zostanie dopasowane, zmienna `state` zostanie utworzona i zainicjowana wartością wskazującą stan ćwiartki.
Następnie `state` może zostać użyte w kodzie tej odnogi, co pokazuje przykład:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

W wywołaniu `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin` miałoby wartość `Coin::Quarter(UsState::Alaska)`.
Próby dopasowania tej wartości do kolejnych odnóg match zakończyłyby się sukcesem dopiero po dotarciu do `Coin::Quarter(state)`.
Wtedy zostałaby utworzona zmienna `state` o wartość `UsState::Alaska`.
Ta zmienna zostałaby użyta w wyrażeniu `println!`, dając mu dostęp do wartości przechowywanej wewnątrz wariantu `Quarter` enuma `Coin`.

<!-- ### Matching with `Option<T>` -->
### Dopasowywanie do `Option<T>`

W poprzedniej sekcji chcieliśmy wydobyć wewnętrzną wartość typu `T` z wariantu `Some` enuma `Option<T>`; możemy również obsłużyć `Option<T>` używając `match`, podobnie jak zrobiliśmy to z typem wyliczeniowym `Coin`!
Zamiast porównywać monety, będziemy porównywać warianty `Option<T>`, ale sposób działania wyrażenia `match` będzie taki sam.

Powiedzmy, że chcemy napisać funkcję, która przyjmuje `Option<i32>` i jeśli w środku jest jakaś wartość, to dodaje do niej 1.
Jeśli w środku nie ma żadnej wartości, funkcja powinna zwrócić wartość `None`, nie robiąc nic więcej.

Dzięki `match` taka funkcja jest bardzo łatwa do napisania i wygląda jak na Listingu 6-5.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

<span class="caption">Listing 6-5: Funkcja używająca wyrażenia `match` dla `Option<i32>`</span>

Przeanalizujmy bardziej szczegółowo pierwsze wykonanie `plus_one`.
W wywołaniu `plus_one(five)`, zmienna `x` ma wartość `Some(5)` i zostanie porównana z każdą odnogą `match`:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

Wartość `Some(5)` nie pasuje do wzorca `None`, nastąpi więc przejęcie do kolejnej odnogi:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

Czy `Some(5)` pasuje do `Some(i)`? Tak! To ten sam wariant.
Zostaje zadeklarowana zmienna `i`, zainicjowana wartością zawartą w `Some`, czyli `5`.
Następnie wykonywany jest kod w wybranej odnodze match, który dodaje 1 do wartości `i` i tworzymy nową wartość `Some` z uzyskaną sumą `6` w środku.

Rozważmy teraz drugie wywołanie `plus_one` z Listingu 6-5, w którym `x` jest `None`.
Następuje jego porównanie do pierwszej odnogi `match`:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

Pasuje! Nie ma żadnej wartości do dodania, więc program zatrzymuje się i zwraca wartość `None` po prawej stronie `=>`.
Ponieważ pierwsza odnoga pasowała, to pozostałe nie są już sprawdzane.

Używanie `match` z typami wyliczeniowymi jest przydatne w wielu sytuacjach.
Często można spotkać następujący scenariusz: enum jest dopasowywany za pomocą `match`, następnie z jego wewnętrznymi danymi związywana jest zmienna, która jest używana w kodzie przewidzianym dla wybranego wariantu. Początkowo może się to wydawać nieco trudne, ale po przyzwyczajeniu, okazuje się bardzo wygodne. Jest to niezmiennie ulubione narzędzie Rustowców.

### Matches Are Exhaustive

There’s one other aspect of `match` we need to discuss: the arms’ patterns must
cover all possibilities. Consider this version of our `plus_one` function,
which has a bug and won’t compile:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

We didn’t handle the `None` case, so this code will cause a bug. Luckily, it’s
a bug Rust knows how to catch. If we try to compile this code, we’ll get this
error:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

Rust knows that we didn’t cover every possible case, and even knows which
pattern we forgot! Matches in Rust are *exhaustive*: we must exhaust every last
possibility in order for the code to be valid. Especially in the case of
`Option<T>`, when Rust prevents us from forgetting to explicitly handle the
`None` case, it protects us from assuming that we have a value when we might
have null, thus making the billion-dollar mistake discussed earlier impossible.

### Catch-all Patterns and the `_` Placeholder

Using enums, we can also take special actions for a few particular values, but
for all other values take one default action. Imagine we’re implementing a game
where, if you roll a 3 on a dice roll, your player doesn’t move, but instead
gets a new fancy hat. If you roll a 7, your player loses a fancy hat. For all
other values, your player moves that number of spaces on the game board. Here’s
a `match` that implements that logic, with the result of the dice roll
hardcoded rather than a random value, and all other logic represented by
functions without bodies because actually implementing them is out of scope for
this example:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

For the first two arms, the patterns are the literal values `3` and `7`. For
the last arm that covers every other possible value, the pattern is the
variable we’ve chosen to name `other`. The code that runs for the `other` arm
uses the variable by passing it to the `move_player` function.

This code compiles, even though we haven’t listed all the possible values a
`u8` can have, because the last pattern will match all values not specifically
listed. This catch-all pattern meets the requirement that `match` must be
exhaustive. Note that we have to put the catch-all arm last because the
patterns are evaluated in order. If we put the catch-all arm earlier, the other
arms would never run, so Rust will warn us if we add arms after a catch-all!

Rust also has a pattern we can use when we want a catch-all but don’t want to
*use* the value in the catch-all pattern: `_` is a special pattern that matches
any value and does not bind to that value. This tells Rust we aren’t going to
use the value, so Rust won’t warn us about an unused variable.

Let’s change the rules of the game: now, if you roll anything other than a 3 or
a 7, you must roll again. We no longer need to use the catch-all value, so we
can change our code to use `_` instead of the variable named `other`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

This example also meets the exhaustiveness requirement because we’re explicitly
ignoring all other values in the last arm; we haven’t forgotten anything.

Finally, we’ll change the rules of the game one more time so that nothing else
happens on your turn if you roll anything other than a 3 or a 7. We can express
that by using the unit value (the empty tuple type we mentioned in [“The Tuple
Type”][tuples]<!-- ignore --> section) as the code that goes with the `_` arm:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

Here, we’re telling Rust explicitly that we aren’t going to use any other value
that doesn’t match a pattern in an earlier arm, and we don’t want to run any
code in this case.

There’s more about patterns and matching that we’ll cover in [Chapter
18][ch18-00-patterns]<!-- ignore -->. For now, we’re going to move on to the
`if let` syntax, which can be useful in situations where the `match` expression
is a bit wordy.

[tuples]: ch03-02-data-types.html#the-tuple-type
[ch18-00-patterns]: ch18-00-patterns.html

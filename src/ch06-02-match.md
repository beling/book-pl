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

### Patterns That Bind to Values
<!--?? ### Wzorce Przypisujące Wartości -->

Another useful feature of match arms is that they can bind to the parts of the
values that match the pattern. This is how we can extract values out of enum
variants.

As an example, let’s change one of our enum variants to hold data inside it.
From 1999 through 2008, the United States minted quarters with different
designs for each of the 50 states on one side. No other coins got state
designs, so only quarters have this extra value. We can add this information to
our `enum` by changing the `Quarter` variant to include a `UsState` value
stored inside it, which we’ve done in Listing 6-4.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

<span class="caption">Listing 6-4: A `Coin` enum in which the `Quarter` variant
also holds a `UsState` value</span>

Let’s imagine that a friend is trying to collect all 50 state quarters. While
we sort our loose change by coin type, we’ll also call out the name of the
state associated with each quarter so that if it’s one our friend doesn’t have,
they can add it to their collection.

In the match expression for this code, we add a variable called `state` to the
pattern that matches values of the variant `Coin::Quarter`. When a
`Coin::Quarter` matches, the `state` variable will bind to the value of that
quarter’s state. Then we can use `state` in the code for that arm, like so:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

If we were to call `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin`
would be `Coin::Quarter(UsState::Alaska)`. When we compare that value with each
of the match arms, none of them match until we reach `Coin::Quarter(state)`. At
that point, the binding for `state` will be the value `UsState::Alaska`. We can
then use that binding in the `println!` expression, thus getting the inner
state value out of the `Coin` enum variant for `Quarter`.

### Matching with `Option<T>`

In the previous section, we wanted to get the inner `T` value out of the `Some`
case when using `Option<T>`; we can also handle `Option<T>` using `match`, as
we did with the `Coin` enum! Instead of comparing coins, we’ll compare the
variants of `Option<T>`, but the way the `match` expression works remains the
same.

Let’s say we want to write a function that takes an `Option<i32>` and, if
there’s a value inside, adds 1 to that value. If there isn’t a value inside,
the function should return the `None` value and not attempt to perform any
operations.

This function is very easy to write, thanks to `match`, and will look like
Listing 6-5.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

<span class="caption">Listing 6-5: A function that uses a `match` expression on
an `Option<i32>`</span>

Let’s examine the first execution of `plus_one` in more detail. When we call
`plus_one(five)`, the variable `x` in the body of `plus_one` will have the
value `Some(5)`. We then compare that against each match arm:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

The `Some(5)` value doesn’t match the pattern `None`, so we continue to the
next arm:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

Does `Some(5)` match `Some(i)`? It does! We have the same variant. The `i`
binds to the value contained in `Some`, so `i` takes the value `5`. The code in
the match arm is then executed, so we add 1 to the value of `i` and create a
new `Some` value with our total `6` inside.

Now let’s consider the second call of `plus_one` in Listing 6-5, where `x` is
`None`. We enter the `match` and compare to the first arm:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

It matches! There’s no value to add to, so the program stops and returns the
`None` value on the right side of `=>`. Because the first arm matched, no other
arms are compared.

Combining `match` and enums is useful in many situations. You’ll see this
pattern a lot in Rust code: `match` against an enum, bind a variable to the
data inside, and then execute code based on it. It’s a bit tricky at first, but
once you get used to it, you’ll wish you had it in all languages. It’s
consistently a user favorite.

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

## Referencje i Pożyczanie

Z rozwiązaniem z krotką zastosowanym na Listingu 4-5 związana jest taka niedogodność,
że musieliśmy zwrócić `String` do funkcji wywołującej, by ta mogła dalej z niego korzystać
(po wywołaniu `calculate_length`). Było tak dlatego że wspomniany `String` był przenoszony do `calculate_length`.

Funkcję `calculate_length` można by zdefiniować w następujący sposób, w którym jako parametr przekazywana jest jedynie referencja do obiektu i dzięki temu nie jest on przez `calculate_length` przejmowany na własność:

<span class="filename">Plik: src/main.rs</span>

```rust
fn main() {
    let s1 = String::from("witaj");

    let len = calculate_length(&s1);

    println!("Długość '{}' wynosi {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```
Po pierwsze proszę zauważyć, że krotki nie są nam już dalej potrzebne. Nie ma ich ani przy deklaracji zmiennej, ani w typie zwracanym przez funkcję `calculate_length`. Po drugie, proszę zwrócić uwagę, że przekazujemy do tej funkcji `&s1` oraz, że typ jej parametru został zmieniony z `String` na `&String`.

Te ampersandy oznaczają *referencje* (ang. references) i pozwalają na odnoszenie się (referowanie) do wartości bez przejmowania jej na własność.
Jest to zilustrowane na Rysunku 4-5.

<img alt="&String s wskazuje na String s1" src="img/trpl04-05.svg" class="center" />

<span class="caption">Rysunek 4-5: `&String s` wskazuje na `String s1`</span>

> Uwaga: Przeciwieństwem referowania za pomocą `&` jest *dereferowanie*,
> które realizowane jest za pomocą operatora dereferencji, `*`.
> W rozdziale 8 przedstawiono przykładowe zastosowania operatora dereferencji,
> zaś więcej szczegółów na jego temat można znaleźć w rozdziale 15.

Przyjrzyjmy się nieco bliżej temu wywołaniu funkcji:

```rust
# fn calculate_length(s: &String) -> usize {
#     s.len()
# }
let s1 = String::from("witaj");

let len = calculate_length(&s1);
```
Składnia `&s1` tworzy referencję, która co prawda *referuje* do `s1`, ale
nie posiada go na własność.
Skoro zaś go nie posiada, to wskazywana przez nią wartość nie zostanie
usunięta wraz z końcem zakresu życia samej referencji.

Sygnatura funkcji także używa `&` do wskazania, że `s` jest referencją.
Poniżej dodano kilka komentarzy z wyjaśnieniami:

```rust
fn calculate_length(s: &String) -> usize { // s jest referencją do Stringa
    s.len()
} // Tu kończy się zakres życia s. Ale ponieważ s nie posiada na własność tego
  // na co wskazuje, nic się nie dzieje.
```
Zakres, w którym zmienna `s` jest dostępna, jest taki sam jak zakres każdego innego parametru funkcji.
Jednakże, ponieważ `s` nie posiada tego, na co wskazuje, to nie jest to kasowane gdy `s` wyjdzie poza swój zakres.
W przeciwieństwie do argumentów przekazywanych przez wartość, te przekazywane przez referencje nie są funkcji dawane na własność.
Dlatego też funkcja nie musi więcej zwracać ich za pomocą `return`, by je oddać.

Przekazywanie referencji jako parametrów funkcji nazywamy *pożyczaniem* (ang. borrowing).
I jak w prawdziwym życiu, jeśli ktoś coś posiada, możemy to od niego pożyczyć.
W końcu jednak musimy mu to także oddać.

Co więc się stanie gdy spróbujemy zmodyfikować coś, co pożyczyliśmy?
Wypróbujmy kod z Listingu 4-6. Uwaga: on nie zadziała!

<span class="filename">Plik: src/main.rs</span>

```rust,ignore,does_not_compile
fn main() {
    let s = String::from("witaj");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", świecie");
}
```

<span class="caption">Listing 4-6: Próba modyfikacji pożyczonej wartości</span>

Otrzymamy następujący błąd:

```text
error[E0596]: cannot borrow immutable borrowed content `*some_string` as mutable
 --> error.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- use `&mut String` here to make mutable
8 |     some_string.push_str(", świecie");
  |     ^^^^^^^^^^^ cannot borrow as mutable
```

Tak jak zmienne, referencje są domyślnie niemutowalne.
Nie możemy zmieniać czegoś do czego mamy referencję.

### Mutowalne Referencje

Możemy wyeliminować błąd z kodu z listingu 4-6 wprowadzając drobną poprawkę:

<span class="filename">Plik: src/main.rs</span>

```rust
fn main() {
    let mut s = String::from("witaj");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", świecie");
}
```

Po pierwsze, zmieniliśmy `s` by było `mut`. Następnie, utworzyliśmy mutowalną referencję za pomocą `&mut s` i ją przyjęliśmy za pomocą `some_string: &mut String`.

Jednakże mutowalne referencję posiadają jedno spore ograniczenie: w danym zakresie można mieć tylko jedną mutowalną referencję do konkretnych danych. Ten kod nie skompiluje się:

<span class="filename">Plik: src/main.rs</span>

```rust,ignore,does_not_compile
let mut s = String::from("witaj");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

Otrzymamy następujący błąd:

```text
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:10
  |
4 | let r1 = &mut s;
  |          ------ first mutable borrow occurs here
5 | let r2 = &mut s;
  |          ^^^^^^ second mutable borrow occurs here
6 | println!("{}, {}", r1, r2);
  |                    -- borrow later used here
```

This restriction allows for mutation but in a very controlled fashion. It’s
something that new Rustaceans struggle with, because most languages let you
mutate whenever you’d like.

The benefit of having this restriction is that Rust can prevent data races at
compile time. A *data race* is similar to a race condition and happens when
these three behaviors occur:

* Two or more pointers access the same data at the same time.
* At least one of the pointers is being used to write to the data.
* There’s no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix
when you’re trying to track them down at runtime; Rust prevents this problem
from happening because it won’t even compile code with data races!

As always, we can use curly brackets to create a new scope, allowing for
multiple mutable references, just not *simultaneous* ones:

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

A similar rule exists for combining mutable and immutable references. This code
results in an error:

```rust,ignore,does_not_compile
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

Here’s the error:

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:10
  |
4 | let r1 = &s; // no problem
  |          -- immutable borrow occurs here
5 | let r2 = &s; // no problem
6 | let r3 = &mut s; // BIG PROBLEM
  |          ^^^^^^ mutable borrow occurs here
7 |
8 | println!("{}, {}, and {}", r1, r2, r3);
  |                            -- borrow later used here
```

Whew! We *also* cannot have a mutable reference while we have an immutable one.
Users of an immutable reference don’t expect the values to suddenly change out
from under them! However, multiple immutable references are okay because no one
who is just reading the data has the ability to affect anyone else’s reading of
the data.

Even though these errors may be frustrating at times, remember that it’s the
Rust compiler pointing out a potential bug early (at compile time rather than
at runtime) and showing you exactly where the problem is. Then you don’t have
to track down why your data isn’t what you thought it was.

### Dangling References

In languages with pointers, it’s easy to erroneously create a *dangling
pointer*, a pointer that references a location in memory that may have been
given to someone else, by freeing some memory while preserving a pointer to
that memory. In Rust, by contrast, the compiler guarantees that references will
never be dangling references: if you have a reference to some data, the
compiler will ensure that the data will not go out of scope before the
reference to the data does.

Let’s try to create a dangling reference, which Rust will prevent with a
compile-time error:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

Here’s the error:

```text
error[E0106]: missing lifetime specifier
 --> main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is
  no value for it to be borrowed from
  = help: consider giving it a 'static lifetime
```

This error message refers to a feature we haven’t covered yet: lifetimes. We’ll
discuss lifetimes in detail in Chapter 10. But, if you disregard the parts
about lifetimes, the message does contain the key to why this code is a problem:

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from.
```

Let’s take a closer look at exactly what’s happening at each stage of our
`dangle` code:

```rust,ignore
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

Because `s` is created inside `dangle`, when the code of `dangle` is finished,
`s` will be deallocated. But we tried to return a reference to it. That means
this reference would be pointing to an invalid `String`. That’s no good! Rust
won’t let us do this.

The solution here is to return the `String` directly:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

This works without any problems. Ownership is moved out, and nothing is
deallocated.

### The Rules of References

Let’s recap what we’ve discussed about references:

* At any given time, you can have *either* one mutable reference *or* any
  number of immutable references.
* References must always be valid.

Next, we’ll look at a different kind of reference: slices.

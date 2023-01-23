<!-- Old heading. Do not remove or links may break. -->
<a id="closures-anonymous-functions-that-can-capture-their-environment"></a>

## Closures: Anonymous Functions that Capture Their Environment

Rust’s closures are anonymous functions you can save in a variable or pass as
arguments to other functions. You can create the closure in one place and then
call the closure elsewhere to evaluate it in a different context. Unlike
functions, closures can capture values from the scope in which they’re defined.
We’ll demonstrate how these closure features allow for code reuse and behavior
customization.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>

### Capturing the Environment with Closures

We’ll first examine how we can use closures to capture values from the
environment they’re defined in for later use. Here’s the scenario: Every so
often, our t-shirt company gives away an exclusive, limited-edition shirt to
someone on our mailing list as a promotion. People on the mailing list can
optionally add their favorite color to their profile. If the person chosen for
a free shirt has their favorite color set, they get that color shirt. If the
person hasn’t specified a favorite color, they get whatever color the company
currently has the most of.

<<<<<<< HEAD
We’ll simulate calling this hypothetical algorithm with the function
`simulated_expensive_calculation` shown in listing 13-1, which will print
`calculating slowly...`, wait for two seconds, and then return whatever number
we passed in.
=======
There are many ways to implement this. For this example, we’re going to use an
enum called `ShirtColor` that has the variants `Red` and `Blue` (limiting the
number of colors available for simplicity). We represent the company’s
inventory with an `Inventory` struct that has a field named `shirts` that
contains a `Vec<ShirtColor>` representing the shirt colors currently in stock.
The method `giveaway` defined on `Inventory` gets the optional shirt
color preference of the free shirt winner, and returns the shirt color the
person will get. This setup is shown in Listing 13-1:
>>>>>>> english/main

<span class="filename">Plik: src/main.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

<span class="caption">Listing 13-1: Shirt company giveaway situation</span>

The `store` defined in `main` has two blue shirts and one red shirt remaining
to distribute for this limited-edition promotion. We call the `giveaway` method
for a user with a preference for a red shirt and a user without any preference.

Again, this code could be implemented in many ways, and here, to focus on
closures, we’ve stuck to concepts you’ve already learned except for the body of
the `giveaway` method that uses a closure. In the `giveaway` method, we get the
user preference as a parameter of type `Option<ShirtColor>` and call the
`unwrap_or_else` method on `user_preference`. The [`unwrap_or_else` method on
`Option<T>`][unwrap-or-else]<!-- ignore --> is defined by the standard library.
It takes one argument: a closure without any arguments that returns a value `T`
(the same type stored in the `Some` variant of the `Option<T>`, in this case
`ShirtColor`). If the `Option<T>` is the `Some` variant, `unwrap_or_else`
returns the value from within the `Some`. If the `Option<T>` is the `None`
variant, `unwrap_or_else` calls the closure and returns the value returned by
the closure.

We specify the closure expression `|| self.most_stocked()` as the argument to
`unwrap_or_else`. This is a closure that takes no parameters itself (if the
closure had parameters, they would appear between the two vertical bars). The
body of the closure calls `self.most_stocked()`. We’re defining the closure
here, and the implementation of `unwrap_or_else` will evaluate the closure
later if the result is needed.

Running this code prints:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

One interesting aspect here is that we’ve passed a closure that calls
`self.most_stocked()` on the current `Inventory` instance. The standard library
didn’t need to know anything about the `Inventory` or `ShirtColor` types we
defined, or the logic we want to use in this scenario. The closure captures an
immutable reference to the `self` `Inventory` instance and passes it with the
code we specify to the `unwrap_or_else` method. Functions, on the other hand,
are not able to capture their environment in this way.

### Closure Type Inference and Annotation

There are more differences between functions and closures. Closures don’t
usually require you to annotate the types of the parameters or the return value
like `fn` functions do. Type annotations are required on functions because the
types are part of an explicit interface exposed to your users. Defining this
interface rigidly is important for ensuring that everyone agrees on what types
of values a function uses and returns. Closures, on the other hand, aren’t used
in an exposed interface like this: they’re stored in variables and used without
naming them and exposing them to users of our library.

Closures are typically short and relevant only within a narrow context rather
than in any arbitrary scenario. Within these limited contexts, the compiler can
infer the types of the parameters and the return type, similar to how it’s able
to infer the types of most variables (there are rare cases where the compiler
needs closure type annotations too).

As with variables, we can add type annotations if we want to increase
explicitness and clarity at the cost of being more verbose than is strictly
necessary. Annotating the types for a closure would look like the definition
shown in Listing 13-2. In this example, we’re defining a closure and storing it
in a variable rather than defining the closure in the spot we pass it as an
argument as we did in Listing 13-1.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

<<<<<<< HEAD
<span class="caption">Listing 13-2: A `main` function with hardcoded values to
simulate user input and random number generation</span>

We’ve hardcoded the variable `simulated_user_specified_value` as 10 and the
variable `simulated_random_number` as 7 for simplicity’s sake; in an actual
program, we’d get the intensity number from the app frontend, and we’d use the
`rand` crate to generate a random number, as we did in the Guessing Game
example in Chapter 2. The `main` function calls a `generate_workout` function
with the simulated input values.

Now that we have the context, let’s get to the algorithm. The function
`generate_workout` in listing 13-3 contains the business logic of the
app that we’re most concerned with in this example. The rest of the code
changes in this example will be made to this function.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

<span class="caption">Listing 13-3: The business logic that prints the workout
plans based on the inputs and calls to the `simulated_expensive_calculation`
function</span>

The code in listing 13-3 has multiple calls to the slow calculation function.
The first `if` block calls `simulated_expensive_calculation` twice, the `if`
inside the outer `else` doesn’t call it at all, and the code inside the
second `else` case calls it once.

The desired behavior of the `generate_workout` function is to first check
whether the user wants a low-intensity workout (indicated by a number less than
25) or a high-intensity workout (a number of 25 or greater).

Low-intensity workout plans will recommend a number of push-ups and sit-ups
based on the complex algorithm we’re simulating.

If the user wants a high-intensity workout, there’s some additional logic: if
the value of the random number generated by the app happens to be 3, the app
will recommend a break and hydration. If not, the user will get a number of
minutes of running based on the complex algorithm.

This code works the way the business wants it to now, but let’s say the data
science team decides that we need to make some changes to the way we call the
`simulated_expensive_calculation` function in the future. To simplify the
update when those changes happen, we want to refactor this code so it calls the
`simulated_expensive_calculation` function only once. We also want to cut the
place where we’re currently unnecessarily calling the function twice without
adding any other calls to that function in the process. That is, we don’t want
to call it if the result isn’t needed, and we still want to call it only once.

#### Refactoring Using Functions

We could restructure the workout program in many ways. First, we’ll try
extracting the duplicated call to the `simulated_expensive_calculation`
function into a variable, as shown in listing 13-4.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs:here}}
```

<span class="caption">Listing 13-4: Extracting the calls to
`simulated_expensive_calculation` to one place and storing the result in the
`expensive_result` variable</span>

This change unifies all the calls to `simulated_expensive_calculation` and
solves the problem of the first `if` block unnecessarily calling the function
twice. Unfortunately, we’re now calling this function and waiting for the
result in all cases, which includes the inner `if` block that doesn’t use the
result value at all.

We want to define code in one place in our program, but only *execute* that
code where we actually need the result. This is a use case for closures!

#### Refactoring with Closures to Store Code

Instead of always calling the `simulated_expensive_calculation` function before
the `if` blocks, we can define a closure and store the *closure* in a variable
rather than storing the result of the function call, as shown in listing 13-5.
We can actually move the whole body of `simulated_expensive_calculation` within
the closure we’re introducing here.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs:here}}
```

<span class="caption">Listing 13-5: Defining a closure and storing it in the
`expensive_closure` variable</span>

The closure definition comes after the `=` to assign it to the variable
`expensive_closure`. To define a closure, we start with a pair of vertical
pipes (`|`), inside which we specify the parameters to the closure; this syntax
was chosen because of its similarity to closure definitions in Smalltalk and
Ruby. This closure has one parameter named `num`: if we had more than one
parameter, we would separate them with commas, like `|param1, param2|`.

After the parameters, we place curly brackets that hold the body of the
closure—these are optional if the closure body is a single expression. The end
of the closure, after the curly brackets, needs a semicolon to complete the
`let` statement. The value returned from the last line in the closure body
(`num`) will be the value returned from the closure when it’s called, because
that line doesn’t end in a semicolon; just as in function bodies.

Note that this `let` statement means `expensive_closure` contains the
*definition* of an anonymous function, not the *resulting value* of calling the
anonymous function. Recall that we’re using a closure because we want to define
the code to call at one point, store that code, and call it at a later point;
the code we want to call is now stored in `expensive_closure`.

With the closure defined, we can change the code in the `if` blocks to call the
closure to execute the code and get the resulting value. We call a closure like
we do a function: we specify the variable name that holds the closure
definition and follow it with parentheses containing the argument values we
want to use, as shown in listing 13-6.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs:here}}
```

<span class="caption">Listing 13-6: Calling the `expensive_closure` we’ve
defined</span>

Now the expensive calculation is called in only one place, and we’re only
executing that code where we need the results.

However, we’ve reintroduced one of the problems from listing 13-3: we’re still
calling the closure twice in the first `if` block, which will call the
expensive code twice and make the user wait twice as long as they need to. We
could fix this problem by creating a variable local to that `if` block to hold
the result of calling the closure, but closures provide us with another
solution. We’ll talk about that solution in a bit. But first let’s talk about
why there aren’t type annotations in the closure definition and the traits
involved with closures.

### Closure Type Inference and Annotation

Closures don’t require you to annotate the types of the parameters or the
return value like `fn` functions do. Type annotations are required on functions
because they’re part of an explicit interface exposed to your users. Defining
this interface rigidly is important for ensuring that everyone agrees on what
types of values a function uses and returns. But closures aren’t used in an
exposed interface like this: they’re stored in variables and used without
naming them and exposing them to users of our library.

Closures are usually short and relevant only within a narrow context rather
than in any arbitrary scenario. Within these limited contexts, the compiler is
reliably able to infer the types of the parameters and the return type, similar
to how it’s able to infer the types of most variables.

Making programmers annotate the types in these small, anonymous functions would
be annoying and largely redundant with the information the compiler already has
available.

As with variables, we can add type annotations if we want to increase
explicitness and clarity at the cost of being more verbose than is strictly
necessary. Annotating the types for the closure we defined in listing 13-5
would look like the definition shown in listing 13-7.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs:here}}
```

<span class="caption">Listing 13-7: Adding optional type annotations of the
=======
<span class="caption">Listing 13-2: Adding optional type annotations of the
>>>>>>> english/main
parameter and return value types in the closure</span>

With type annotations added, the syntax of closures looks more similar to the
syntax of functions. Here we define a function that adds 1 to its parameter and
a closure that has the same behavior, for comparison. We’ve added some spaces
to line up the relevant parts. This illustrates how closure syntax is similar
to function syntax except for the use of pipes and the amount of syntax that is
optional:

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

The first line shows a function definition, and the second line shows a fully
annotated closure definition. In the third line, we remove the type annotations
from the closure definition. In the fourth line, we remove the brackets, which
are optional because the closure body has only one expression. These are all
valid definitions that will produce the same behavior when they’re called. The
`add_one_v3` and `add_one_v4` lines require the closures to be evaluated to be
able to compile because the types will be inferred from their usage. This is
similar to `let v = Vec::new();` needing either type annotations or values of
some type to be inserted into the `Vec` for Rust to be able to infer the type.

<<<<<<< HEAD
Closure definitions will have one concrete type inferred for each of their
parameters and for their return value. For instance, listing 13-8 shows the
definition of a short closure that just returns the value it receives as a
=======
For closure definitions, the compiler will infer one concrete type for each of
their parameters and for their return value. For instance, Listing 13-3 shows
the definition of a short closure that just returns the value it receives as a
>>>>>>> english/main
parameter. This closure isn’t very useful except for the purposes of this
example. Note that we haven’t added any type annotations to the definition.
Because there are no type annotations, we can call the closure with any type,
which we’ve done here with `String` the first time. If we then try to call
`example_closure` with an integer, we’ll get an error.

<span class="filename">Plik: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

<span class="caption">Listing 13-3: Attempting to call a closure whose types
are inferred with two different types</span>

The compiler gives us this error:

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

The first time we call `example_closure` with the `String` value, the compiler
infers the type of `x` and the return type of the closure to be `String`. Those
types are then locked into the closure in `example_closure`, and we get a type
error when we next try to use a different type with the same closure.

### Capturing References or Moving Ownership

<<<<<<< HEAD
Let’s return to our workout generation app. In listing 13-6, our code was still
calling the expensive calculation closure more times than it needed to. One
option to solve this issue is to save the result of the expensive closure in a
variable for reuse and use the variable in each place we need the result,
instead of calling the closure again. However, this method could result in a
lot of repeated code.
=======
Closures can capture values from their environment in three ways, which
directly map to the three ways a function can take a parameter: borrowing
immutably, borrowing mutably, and taking ownership. The closure will decide
which of these to use based on what the body of the function does with the
captured values.
>>>>>>> english/main

In Listing 13-4, we define a closure that captures an immutable reference to
the vector named `list` because it only needs an immutable reference to print
the value:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

<span class="caption">Listing 13-4: Defining and calling a closure that
captures an immutable reference</span>

This example also illustrates that a variable can bind to a closure definition,
and we can later call the closure by using the variable name and parentheses as
if the variable name were a function name.

Because we can have multiple immutable references to `list` at the same time,
`list` is still accessible from the code before the closure definition, after
the closure definition but before the closure is called, and after the closure
is called. This code compiles, runs, and prints:

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

Next, in Listing 13-5, we change the closure body so that it adds an element to
the `list` vector. The closure now captures a mutable reference:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

<span class="caption">Listing 13-5: Defining and calling a closure that
captures a mutable reference</span>

This code compiles, runs, and prints:

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

Note that there’s no longer a `println!` between the definition and the call of
the `borrows_mutably` closure: when `borrows_mutably` is defined, it captures a
mutable reference to `list`. We don’t use the closure again after the closure
is called, so the mutable borrow ends. Between the closure definition and the
closure call, an immutable borrow to print isn’t allowed because no other
borrows are allowed when there’s a mutable borrow. Try adding a `println!`
there to see what error message you get!

If you want to force the closure to take ownership of the values it uses in the
environment even though the body of the closure doesn’t strictly need
ownership, you can use the `move` keyword before the parameter list.

This technique is mostly useful when passing a closure to a new thread to move
the data so that it’s owned by the new thread. We’ll discuss threads and why
you would want to use them in detail in Chapter 16 when we talk about
concurrency, but for now, let’s briefly explore spawning a new thread using a
closure that needs the `move` keyword. Listing 13-6 shows Listing 13-4 modified
to print the vector in a new thread rather than in the main thread:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

<span class="caption">Listing 13-6: Using `move` to force the closure for the
thread to take ownership of `list`</span>

We spawn a new thread, giving the thread a closure to run as an argument. The
closure body prints out the list. In Listing 13-4, the closure only captured
`list` using an immutable reference because that's the least amount of access
to `list` needed to print it. In this example, even though the closure body
still only needs an immutable reference, we need to specify that `list` should
be moved into the closure by putting the `move` keyword at the beginning of the
closure definition. The new thread might finish before the rest of the main
thread finishes, or the main thread might finish first. If the main thread
maintained ownership of `list` but ended before the new thread did and dropped
`list`, the immutable reference in the thread would be invalid. Therefore, the
compiler requires that `list` be moved into the closure given to the new thread
so the reference will be valid. Try removing the `move` keyword or using `list`
in the main thread after the closure is defined to see what compiler errors you
get!

<!-- Old headings. Do not remove or links may break. -->
<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>

### Moving Captured Values Out of Closures and the `Fn` Traits

Once a closure has captured a reference or captured ownership of a value from
the environment where the closure is defined (thus affecting what, if anything,
is moved *into* the closure), the code in the body of the closure defines what
happens to the references or values when the closure is evaluated later (thus
affecting what, if anything, is moved *out of* the closure). A closure body can
do any of the following: move a captured value out of the closure, mutate the
captured value, neither move nor mutate the value, or capture nothing from the
environment to begin with.

The way a closure captures and handles values from the environment affects
which traits the closure implements, and traits are how functions and structs
can specify what kinds of closures they can use. Closures will automatically
implement one, two, or all three of these `Fn` traits, in an additive fashion,
depending on how the closure’s body handles the values:

1. `FnOnce` applies to closures that can be called once. All closures implement
   at least this trait, because all closures can be called. A closure that
   moves captured values out of its body will only implement `FnOnce` and none
   of the other `Fn` traits, because it can only be called once.
2. `FnMut` applies to closures that don’t move captured values out of their
   body, but that might mutate the captured values. These closures can be
   called more than once.
3. `Fn` applies to closures that don’t move captured values out of their body
   and that don’t mutate captured values, as well as closures that capture
   nothing from their environment. These closures can be called more than once
   without mutating their environment, which is important in cases such as
   calling a closure multiple times concurrently.

Let’s look at the definition of the `unwrap_or_else` method on `Option<T>` that
we used in Listing 13-1:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

Recall that `T` is the generic type representing the type of the value in the
`Some` variant of an `Option`. That type `T` is also the return type of the
`unwrap_or_else` function: code that calls `unwrap_or_else` on an
`Option<String>`, for example, will get a `String`.

Next, notice that the `unwrap_or_else` function has the additional generic type
parameter `F`. The `F` type is the type of the parameter named `f`, which is
the closure we provide when calling `unwrap_or_else`.

The trait bound specified on the generic type `F` is `FnOnce() -> T`, which
means `F` must be able to be called once, take no arguments, and return a `T`.
Using `FnOnce` in the trait bound expresses the constraint that
`unwrap_or_else` is only going to call `f` at most one time. In the body of
`unwrap_or_else`, we can see that if the `Option` is `Some`, `f` won’t be
called. If the `Option` is `None`, `f` will be called once. Because all
closures implement `FnOnce`, `unwrap_or_else` accepts the most different kinds
of closures and is as flexible as it can be.

> Note: Functions can implement all three of the `Fn` traits too. If what we
> want to do doesn’t require capturing a value from the environment, we can use
> the name of a function rather than a closure where we need something that
> implements one of the `Fn` traits. For example, on an `Option<Vec<T>>` value,
> we could call `unwrap_or_else(Vec::new)` to get a new, empty vector if the
> value is `None`.

<<<<<<< HEAD
The `value` field is of type `Option<u32>`. Before we execute the closure,
`value` will be `None`. When code using a `Cacher` asks for the *result* of the
closure, the `Cacher` will execute the closure at that time and store the
result within a `Some` variant in the `value` field. Then if the code asks for
the result of the closure again, instead of executing the closure again, the
`Cacher` will return the result held in the `Some` variant.

The logic around the `value` field we’ve just described is defined in listing
13-10.
=======
Now let’s look at the standard library method `sort_by_key` defined on slices,
to see how that differs from `unwrap_or_else` and why `sort_by_key` uses
`FnMut` instead of `FnOnce` for the trait bound. The closure gets one argument
in the form of a reference to the current item in the slice being considered,
and returns a value of type `K` that can be ordered. This function is useful
when you want to sort a slice by a particular attribute of each item. In
Listing 13-7, we have a list of `Rectangle` instances and we use `sort_by_key`
to order them by their `width` attribute from low to high:
>>>>>>> english/main

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

<span class="caption">Listing 13-7: Using `sort_by_key` to order rectangles by
width</span>

<<<<<<< HEAD
We want `Cacher` to manage the struct fields’ values rather than letting the
calling code potentially change the values in these fields directly, so these
fields are private.

The `Cacher::new` function takes a generic parameter `T`, which we’ve defined
as having the same trait bound as the `Cacher` struct. Then `Cacher::new`
returns a `Cacher` instance that holds the closure specified in the
`calculation` field and a `None` value in the `value` field, because we haven’t
executed the closure yet.

When the calling code needs the result of evaluating the closure, instead of
calling the closure directly, it will call the `value` method. This method
checks whether we already have a resulting value in `self.value` in a `Some`;
if we do, it returns the value within the `Some` without executing the closure
again.

If `self.value` is `None`, the code calls the closure stored in
`self.calculation`, saves the result in `self.value` for future use, and
returns the value as well.

Listing 13-11 shows how we can use this `Cacher` struct in the function
`generate_workout` from listing 13-6.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

<span class="caption">Listing 13-11: Using `Cacher` in the `generate_workout`
function to abstract away the caching logic</span>

Instead of saving the closure in a variable directly, we save a new instance of
`Cacher` that holds the closure. Then, in each place we want the result, we
call the `value` method on the `Cacher` instance. We can call the `value`
method as many times as we want, or not call it at all, and the expensive
calculation will be run a maximum of once.

Try running this program with the `main` function from listing 13-2. Change the
values in the `simulated_user_specified_value` and `simulated_random_number`
variables to verify that in all the cases in the various `if` and `else`
blocks, `calculating slowly...` appears only once and only when needed. The
`Cacher` takes care of the logic necessary to ensure we aren’t calling the
expensive calculation more than we need to so `generate_workout` can focus on
the business logic.

### Limitations of the `Cacher` Implementation

Caching values is a generally useful behavior that we might want to use in
other parts of our code with different closures. However, there are two
problems with the current implementation of `Cacher` that would make reusing it
in different contexts difficult.

The first problem is that a `Cacher` instance assumes it will always get the
same value for the parameter `arg` to the `value` method. That is, this test of
`Cacher` will fail:

```rust,ignore,panics
{{#rustdoc_include ../listings/ch13-functional-features/no-listing-01-failing-cacher-test/src/lib.rs:here}}
```

This test creates a new `Cacher` instance with a closure that returns the value
passed into it. We call the `value` method on this `Cacher` instance with an
`arg` value of 1 and then an `arg` value of 2, and we expect the call to
`value` with the `arg` value of 2 to return 2.

Run this test with the `Cacher` implementation in listing 13-9 and listing
13-10, and the test will fail on the `assert_eq!` with this message:
=======
This code prints:
>>>>>>> english/main

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

The reason `sort_by_key` is defined to take an `FnMut` closure is that it calls
the closure multiple times: once for each item in the slice. The closure `|r|
r.width` doesn’t capture, mutate, or move out anything from its environment, so
it meets the trait bound requirements.

<<<<<<< HEAD
Try modifying `Cacher` to hold a hash map rather than a single value. The keys
of the hash map will be the `arg` values that are passed in, and the values of
the hash map will be the result of calling the closure on that key. Instead of
looking at whether `self.value` directly has a `Some` or a `None` value, the
`value` function will look up the `arg` in the hash map and return the value if
it’s present. If it’s not present, the `Cacher` will call the closure and save
the resulting value in the hash map associated with its `arg` value.

The second problem with the current `Cacher` implementation is that it only
accepts closures that take one parameter of type `u32` and return a `u32`. We
might want to cache the results of closures that take a string slice and return
`usize` values, for example. To fix this issue, try introducing more generic
parameters to increase the flexibility of the `Cacher` functionality.

### Capturing the Environment with Closures

In the workout generator example, we only used closures as inline anonymous
functions. However, closures have an additional capability that functions don’t
have: they can capture their environment and access variables from the scope in
which they’re defined.

Listing 13-12 has an example of a closure stored in the `equal_to_x` variable
that uses the `x` variable from the closure’s surrounding environment.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/main.rs}}
```

<span class="caption">Listing 13-12: Example of a closure that refers to a
variable in its enclosing scope</span>

Here, even though `x` is not one of the parameters of `equal_to_x`, the
`equal_to_x` closure is allowed to use the `x` variable that’s defined in the
same scope that `equal_to_x` is defined in.

We can’t do the same with functions; if we try with the following example, our
code won’t compile:
=======
In contrast, Listing 13-8 shows an example of a closure that implements just
the `FnOnce` trait, because it moves a value out of the environment. The
compiler won’t let us use this closure with `sort_by_key`:
>>>>>>> english/main

<span class="filename">Plik: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

<span class="caption">Listing 13-8: Attempting to use an `FnOnce` closure with
`sort_by_key`</span>

This is a contrived, convoluted way (that doesn’t work) to try and count the
number of times `sort_by_key` gets called when sorting `list`. This code
attempts to do this counting by pushing `value`—a `String` from the closure’s
environment—into the `sort_operations` vector. The closure captures `value`
then moves `value` out of the closure by transferring ownership of `value` to
the `sort_operations` vector. This closure can be called once; trying to call
it a second time wouldn’t work because `value` would no longer be in the
environment to be pushed into `sort_operations` again! Therefore, this closure
only implements `FnOnce`. When we try to compile this code, we get this error
that `value` can’t be moved out of the closure because the closure must
implement `FnMut`:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

<<<<<<< HEAD
The compiler even reminds us that this only works with closures!

When a closure captures a value from its environment, it uses memory to store
the values for use in the closure body. This use of memory is overhead that we
don’t want to pay in more common cases where we want to execute code that
doesn’t capture its environment. Because functions are never allowed to capture
their environment, defining and using functions will never incur this overhead.

Closures can capture values from their environment in three ways, which
directly map to the three ways a function can take a parameter: taking
ownership, borrowing mutably, and borrowing immutably. These are encoded in the
three `Fn` traits as follows:

* `FnOnce` consumes the variables it captures from its enclosing scope, known
  as the closure’s *environment*. To consume the captured variables, the
  closure must take ownership of these variables and move them into the closure
  when it is defined. The `Once` part of the name represents the fact that the
  closure can’t take ownership of the same variables more than once, so it can
  be called only once.
* `FnMut` can change the environment because it mutably borrows values.
* `Fn` borrows values from the environment immutably.

When you create a closure, Rust infers which trait to use based on how the
closure uses the values from the environment. All closures implement `FnOnce`
because they can all be called at least once. Closures that don’t move the
captured variables also implement `FnMut`, and closures that don’t need mutable
access to the captured variables also implement `Fn`. In listing 13-12, the
`equal_to_x` closure borrows `x` immutably (so `equal_to_x` has the `Fn` trait)
because the body of the closure only needs to read the value in `x`.

If you want to force the closure to take ownership of the values it uses in the
environment, you can use the `move` keyword before the parameter list. This
technique is mostly useful when passing a closure to a new thread to move the
data so it’s owned by the new thread.

> Note: `move` closures may still implement `Fn` or `FnMut`, even though
> they capture variables by move. This is because the traits implemented by a
> closure type are determined by what the closure does with captured values,
> not how it captures them. The `move` keyword only specifies the latter.

We’ll have more examples of `move` closures in Chapter 16 when we talk about
concurrency. For now, here’s the code from listing 13-12 with the `move`
keyword added to the closure definition and using vectors instead of integers,
because integers can be copied rather than moved; note that this code will not
yet compile.
=======
The error points to the line in the closure body that moves `value` out of the
environment. To fix this, we need to change the closure body so that it doesn’t
move values out of the environment. To count the number of times `sort_by_key`
is called, keeping a counter in the environment and incrementing its value in
the closure body is a more straightforward way to calculate that. The closure
in Listing 13-9 works with `sort_by_key` because it is only capturing a mutable
reference to the `num_sort_operations` counter and can therefore be called more
than once:
>>>>>>> english/main

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

<span class="caption">Listing 13-9: Using an `FnMut` closure with `sort_by_key`
is allowed</span>

The `Fn` traits are important when defining or using functions or types that
make use of closures. In the next section, we’ll discuss iterators. Many
iterator methods take closure arguments, so keep these closure details in mind
as we continue!

[unwrap-or-else]: ../std/option/enum.Option.html#method.unwrap_or_else

## Czym jest własność?

System *własności* jest w zasadzie najważniejszą cechą Rusta. Mimo, że
funkcjonalność tę można w prosty sposób wytłumaczyć, wymusza ona poważne
następstwa dla reszty języka.

Wszystkie programy muszą kontrolować sposób wykorzystywania pamięci komputera
podczas swojego działania. Niektóre języki korzystają z automatycznych
systemów odśmiecania (*garbage collection*), które stale poszukują fragmentów
pamięci, których działający program już nie używa. W innych językach, sam
programista ręcznie zajmuje i zwalnia pamięć. Rust wykorzystuje trzecie
podejście: pamięć jest zarządzana przez system własności, obejmujący zestaw
zasad sprawdzanych przez kompilator w trakcie kompilacji. W ten sposób żaden z
aspektów związanych z systemem własności nie spowalnia działania programu.

Ponieważ własność jest nowym pojęciem dla wielu programistów, przyzwyczajenie
się do niej zabiera trochę czasu. Dobra wiadomość jest taka, że w miarę
nabywania doświadczenia z Rustem i zasadami systemu własności, rośnie też twoja
zdolność do naturalnego tworzenia bezpiecznego i wydajnego kodu. Tak trzymaj!

Kiedy zrozumiesz system własności, będziesz mieć solidną podstawę ku zrozumieniu
innych, unikatowych funkcjonalności Rusta. W tym rozdziale nauczysz się, czym
jest własność za pomocą kilku przykładów, które skupiają się na bardzo często
spotykanej strukturze danych: łańcuchach znaków (*string*).

> ### Stos i sterta
>
> W wielu językach programowania nie trzeba zbyt często myśleć o stosie i o
> stercie. Ale w przypadku języka systemowego takiego jak Rust, to, czy zmienna
> zapisana jest na stosie czy na stercie, ma wpływ na zachowanie całego języka
> i określa, dlaczego należy podjąć niektóre decyzje. W dalszej części rozdziału
> opiszemy części systemu własności w odniesieniu do stosu i sterty, zaczynamy
> więc od krótkiego, przygotowawczego wyjaśnienia.
>
> Zarówno stos jak i sterta są częściami pamięci dostępnymi dla programu w
> trakcie działania, ale charakteryzują się one odmienną strukturą. Stos
> przechowuje dane w takiej kolejności, w jakiej tam trafiają, a usuwane są z
> niego w kolejności odwrotnej. Działanie te określa się nazwą *last in, first
> out* (*ostatni na wejściu, pierwszy na wyjściu*). Pomyśl o stosie talerzy:
> kiedy kładziesz na nim nowe talerze, umieszczasz je na szczycie stosu, a kiedy
> jest ci jakiś potrzebny, zdejmujesz go ze szczytu. Dodawanie lub usuwanie
> talerzy ze środka lub ze spodu stosu nie jest już takie łatwe. Dodawanie
> danych nosi nazwę *odkładania na stos* (*pushing onto the stack*), a usuwanie
> ich nazywane jest *zdejmowaniem ze stosu* (*popping off the stack*).
>
> Stos zawdzięcza swoją szybkość metodzie dostępu do danych: nigdy nie trzeba
> szukać miejsca na dodanie danych, ani miejsca, z którego dane należy pobrać,
> ponieważ to miejsce znajduje się zawsze na szczycie stosu. Kolejną
> właściwością, która odpowiada za szybkość stosu jest to, że każda dana na nim
> umieszczona musi mieć znany, ustalony z góry rozmiar.
>
> Dane, których rozmiar jest nieznany na etapie kompilacji lub ulega zmianie,
> mogą być przechowywane na stercie. Sterta jest mniej zorganizowana: kiedy coś
> się na niej umieszcza, należy poprosić o przydzielenie pewnego jej obszaru.
> System operacyjny znajduje na stercie wolne, wystarczająco duże miejsce,
> oznacza je jako będące w użyciu i zwraca *wskaźnik*, zawierający adres
> wybranej lokalizacji. Proces ten nazywamy *alokacją na stercie* lub po prostu
> „alokacją”. Umieszanie danych na stosie nie jest uznawane za alokację. Ze
> względu na to, że zwrócony wskaźnik posiada znany, ustalony rozmiar, możemy
> przechować go na stosie. Jednak gdy chcemy dostać się do właściwych danych,
> musimy podążyć za wskaźnikiem.
>
> Pomyśl o byciu rozsadzanym w restauracji. Przy wejściu podajesz ilość osób
> w swojej grupie, a pracownik znajduje pusty stolik, przy którym wszyscy się
> pomieszczą i prowadzi ich na miejsce. Jeśli ktoś z twojej grupy sie spóźni,
> aby was znaleźć, może zapytać, gdzie was posadzono.
>
> Dostęp do danych na stercie jest wolniejszy od dostępu do danych na stosie,
> ponieważ należy je zlokalizować korzystając ze wskaźnika. Nowoczesne procesory
> działają szybciej, jeżeli nie muszą dużo skakać po pamięci. Kontynuując
> analogię, załóżmy, że kelner w restauracji zbiera zamówienia z wielu stolików.
> Bardziej wydajne jest zebranie wszystkich zamówień z jednego stolika, zanim
> przejdzie się do kolejnego. Zebranie pojedynczego zamówienia ze stolika A,
> następnie jednego ze stolika B, kolejnrgo znów ze stolika A i powtórnie ze
> stolika B, byłoby zdecydowanie wolniejszym procesem. Z tego samego względu,
> procesor wykonuje swoje zadanie lepiej, operując na danych sąsiadujących z
> innymi danymi (jak ma to miejsce na stosie), niż gdyby operował na danych
> oddalonych od siebie (co może się zdarzyć w przypadku sterty). Alokacja
> sporego obszaru pamięci na stercie również może potrwać.
>
> Kiedy twój kod wywołuje funkcję, przekazywane do niej argumenty (łącznie z
> potencjalnymi wskaźnikami do danych na stercie) oraz jej wewnętrzne zmienne
> są odkładane na stosie. Gdy funkcja się kończy, wartości te są zdejmowane ze
> stosu.
>
> Do problemów, którym przeciwstawia się system własności, należą: śledzenie,
> które fragmenty kodu używają których danych na stercie, minimalizowanie
> duplikowania się danych na stercie, a także pozbywanie się ze sterty
> nieużywanych danych, celem uniknięcia wyczerpania się pamięci.
> Po zrozumieniu pojęcia własności, nie będziesz juz musiał zbyt często myśleć o
> stosie czy o stercie. Jednak świadomość tego, że zawiadowanie danymi na
> stercie jest istotą istnienia systemu własności, pomaga wyjaśnić, dlaczego
> działa on tak, jak działa.

### Zasady systemu własności

W pierwszej kolejności przyjrzyjmy się zasadom systemu własności. Miej je na
uwadze, kiedy będziemy omawiać ilustrujące je przykłady:

* Każda wartość w Ruście przyporządkowana jest do zmiennej, którą określa się
mianem jej *właściciela*.
* W danym momencie może istnieć tylko jeden właściciel.
* Kiedy program wychodzi poza zasięg właściciela, przechowywana wartość
zostaje usunięta.

### Zasięg zmiennych

W Rozdziale 2 przebrnęliśmy przez przykład programu napisanego w Ruście. Teraz,
kiedy znamy już podstawy składni, nie będziemy umieszczać w treści przykładów
kodu `fn main() {`. Jeśli zatem przepisujesz kod na bieżąco, musisz ręcznie
umieszczać zaprezentowane dalej fragmenty wewnątrz funkcji `main`. Dzięki temu, 
przykłady będą nieco bardziej zwięzłe, pozwalając nam skupić się na istocie
sprawy zamiast na powtarzalnych frazach.

W pierwszym przykładzie systemu własności, przyjrzymy się *zasięgowi* kilku
zmiennych. Zasięgiem elementu nazywamy obszar programu, wewnątrz którego dany
element zachowuje ważność. Powiedzmy, że mamy zmienną, która wygląda tak:

```rust
let s = "hello";
```

Zmienna `s` odnosi się do literału znakowego, którego wartość jest ustalana w
samym kodzie programu. Zmienna zachowuje ważność od miejsca, w którym ją
zadeklarowano, do końca bieżącego *zasięgu*. Listing 4-1 zawiera komentarze
wyjaśniające, gdzie zmienna `s` zachowuje ważność:

```rust
{                      // s nie jest tu ważna - jeszcze jej nie zadeklarowano
    let s = "hello";   // od tego momentu s jest ważna

    // jakieś operacje na s
}                      // bieżący zasięg się kończy - s traci ważność
```

<span class="caption">Listing 4-1: Zmienna, oraz zasięg, w którym zachowuje ona
ważność</span>

Innymi słowy, mamy do czynienia z dwoma istotnymi momentami w czasie:

* Kiedy zmienna `s` *wchodzi w zasięg*, staje się ona ważna.
* Zmienna pozostaje ważna, dopóki nie *wyjdzie z zasięgu*.

Na tę chwilę zależność między zasięgiem a ważnością zmiennych jest podobna do
sytuacji w innych językach programowania. Posłużymy się tą wiedzą, wprowadzając
nowy typ danych: `String` (*łańcuch znaków*).

### Typ `String`

Aby zilustrować zasady systemu własności, potrzebujemy typu danych, który jest
bardziej złożony od tych, które omawiane były w Rozdziale 3. Wszystkie typy
opisane w sekcji „Typy danych” przechowywane są na stosie i są z niego
zdejmowane, kiedy skończy się ich zasięg. Potrzebny jest nam natomiast typ
przechowujący zawarte w nim dane na stercie . Dowiemy się wówczas, skąd Rust
wie, kiedy te dane usunąć.

W przykładzie użyjemy typu `String`, koncentrując się na tych jego elementach,
które odnoszą sie do systemu własności. Te same elementy mają znaczenie dla
innych złożonych typów, które dostarcza biblioteka standardowa oraz tych, które
stworzysz sam. Typ `String` omawiany będzie dogłębnie w Rozdziale 8.

Widzieliśmy już literały znakowe, których dane na stałe umieszczone są w treści
programu. Takie zmienne są wygodne w użyciu, ale nieprzydatne w wielu
sytuacjach, w których używa się danych tekstowych. Jednym z powodów jest to, że
są one niemodyfikowalne. Innym, że nie każdy łańcuch tekstowy jest znany podczas
pisania programu. Na przykład: co zrobić, jeśli chcemy pobrać dane od
użytkownika i je przechować? Dla takich sytuacji Rust przewiduje drugi typ
znakowy: `String`. Typ ten alokowany jest na stercie i z tego względu może
przechowywać dane, których ilość jest nieznana podczas kompilacji. Można
przekształcić niemodyfikowalny literał znakowy w zmienną typu `String` za pomocą
funkcji `from`. Wygląda to tak:

```rust
let s = String::from("hello");
```

Podwójny dwukropek (`::`) jest operatorem umożliwiającym wykorzystanie funkcji
`from` z przestrzeni nazw typu `String`, zamiast konieczności utworzenia
ogólnej funkcji o przykładowej nazwie `string_from`. Ten rodzaj składni będzie
szerzej omawiany w sekcji „Składnia metod” w Rozdziale 5 oraz podczas rozważań
o przestrzeniach nazw modułów w sekcji „Definicje modułów” w Rozdziale 7.

Ten rodzaj łańcucha znaków *można* modyfikować:

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() dodaje literał do zmiennej String

println!("{}", s); // To spowoduje wyświetlenie tekstu: `hello, world!`
```

Jaka jest zatem różnica? Dlaczego `String` może być modyfikowalny, a literał
nie? Różnica polega na sposobie, w jakim oba te typy korzystają z pamięci.

### Pamięć i alokacja

W przypadku literału znakowego, jego wartość znana jest już w czasie kompilacji,
więc przechowywany tekst jest na stałe zakodowany w docelowym pliku
wykonywalnym, co czyni literały szybkimi i wydajnymi. Ale cechy te wynikają z
niemodyfikowalności literałów. Niestety, nie możemy w pliku binarnym umieścić
bańki pamięci pod każdy potrzebny tekst, którego rozmiar jest nieznany podczas
kompilacji i może się zmienić w trakcie działania programu.

Mając typ `String`, w celu obsługi modyfikowalnego i potencjalnie rosnącego
tekstu, musimy zaalokować pewną ilość pamięci na stercie, nieznaną podczas
kompilacji. To oznacza, że:

* O przydział pamięci należy poprosić system operacyjny w trakcie działania
programu
* Potrzebny jest sposób na zwracanie pamięci do systemu operacyjnego, kiedy
`String` nie będzie już używany.

Pierwszą część robimy sami, wywołując funkcję `String::from`, której
implementacja zawiera prośbę o wymaganą pamięć. Takie rozwiązanie jest w
zasadzie uniwersalne dla wielu języków programowania.

Druga część znacznie się za to różni. W językach wyposażonych w systemy
odśmiecania (*garbage collector - GC*), GC śledzi i zwalnia pamięć, która nie
jest już używana, a my - programiści nie musimy już o tym myśleć. W językach
pozbawionych GC, naszą odpowiedzialnością jest identyfikowanie nieużywanej już
pamięci i wywoływanie bezpośrednio kodu, który tę pamięć zwalnia. Tak samo, jak
kodu, który ją alokuje. Poprawne wykonanie tej operacji stanowiło historycznie
trudny, programistyczny problem. Jeśli zapomnimy, marnujemy pamięć. Jeśli
zrobimy to za wcześnie, zostaniemy z nieważną zmienną. Zrobimy to dwukrotnie -
to też błąd. Musimy połączyć w pary dokładnie jedną alokację z dokładnie jednym
zwolnieniem.

Rust prezentuje inne podejście: pamięć jest automatycznie zwracana do systemu,
kiedy skończy się zasięg zmiennej, będącej jej właścicielem. Oto wersja naszego
przykładu z listingu 4-1, który używa typu `String` zamiast literału:

```rust
{
    let s = String::from("hello"); // s jest ważna od tego momentu

    // jakieś operacje na s
}                                  // bieżący zasięg się kończy - s traci
                                   // ważność
```

Istnieje oczywisty moment, w którym możemy zwrócić pamięć wykorzystywaną przez
nasz `String` do systemu operacyjnego - kiedy kończy się zasięg zmiennej `s`.
Kiedy zasięg jakiejś zmiennej się kończy, Rust wywołuje za nas specjalną
funkcję. Funkcja ta nosi nazwę `drop` (*rzuć, upuść*), a w jej treści autor typu
`String` umieszcza kod zwrotu pamięci. Funkcja `drop` zostaje wywołana przez
Rusta automatycznie, przy zamykającej kod klamrze.

> Uwaga: W C++ schemat dealokacji zasobów przy końcu okresu trwania jakiegoś
> elementu jest czasem nazywany *Inicjowaniem Przy Pozyskaniu Zasobu* (*Resource
> Acquisition Is Initialization (RAII)*). Funkcja `drop` z Rusta może wydać ci
> się znajoma, jeśli miałeś styczność ze schematami RAII.

Schemat ten ma ogromny wpływ na sposób pisania kodu w Ruście. Na tym etapie może
wydawać się to proste, ale program może zachować się niespodziewanie w bardziej
złożonych przypadkach, kiedy chcemy, aby kilka zmiennych używało tej samej
danej, alokowanej na stercie. Zbadajmy teraz kilka takich sytuacji.

#### Metody interakcji między zmiennymi a danymi: Move

Wiele zmiennych może w Ruście odnosić się do tej samej danej na różne sposoby.
Spójrzmy na przykład w Listingu 4-2, z wykorzystaniem liczby całkowitej:

```rust
let x = 5;
let y = x;
```

<span class="caption">Listing 4-2: Przypisanie całkowitej wartości zmiennej `x`
do zmiennej `y`</span>

Z całą pewnością możemy odgadnąć, co ten kod robi: „Przypisz wartość `5` do `x`,
a następnie zrób kopię wartości przechowywanej w `x` i przypisz ją do `y`.”.
Mamy teraz dwie zmienne: `x` i `y`, obie o wartości `5`. Dzieje się dokładnie
tak, ponieważ zmienne dla liczb całkowitych są prostymi elementami o znanym,
ustalonym rozmiarze, więc obie wartości `5` zostają odłożone na stos.

Teraz przyjrzyjmy się wersji z typem `String`:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

Wygląda to bardzo podobnie do wcześniejszego kodu, więc możemy zakładać, że jego
działanie też będzie podobne: w drugiej linii powstaje kopia wartości w `s1` i
zostaje ona przypisana do `s2`. Ale tak się akurat nie dzieje.

To explain this more thoroughly, let’s look at what `String` looks like under
the covers in Figure 4-1. A `String` is made up of three parts, shown on the
left: a pointer to the memory that holds the contents of the string, a length,
and a capacity. This group of data is stored on the stack. On the right is the
memory on the heap that holds the contents.

<img alt="String in memory" src="img/trpl04-01.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-1: Representation in memory of a `String`
holding the value `"hello"` bound to `s1`</span>

The length is how much memory, in bytes, the contents of the `String` is
currently using. The capacity is the total amount of memory, in bytes, that the
`String` has received from the operating system. The difference between length
and capacity matters, but not in this context, so for now, it’s fine to ignore
the capacity.

When we assign `s1` to `s2`, the `String` data is copied, meaning we copy the
pointer, the length, and the capacity that are on the stack. We do not copy the
data on the heap that the pointer refers to. In other words, the data
representation in memory looks like Figure 4-2.

<img alt="s1 and s2 pointing to the same value" src="img/trpl04-02.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-2: Representation in memory of the variable `s2`
that has a copy of the pointer, length, and capacity of `s1`</span>

The representation does *not* look like Figure 4-3, which is what memory would
look like if Rust instead copied the heap data as well. If Rust did this, the
operation `s2 = s1` could potentially be very expensive in terms of runtime
performance if the data on the heap was large.

<img alt="s1 and s2 to two places" src="img/trpl04-03.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-3: Another possibility of what `s2 = s1` might
do if Rust copied the heap data as well</span>

Earlier, we said that when a variable goes out of scope, Rust automatically
calls the `drop` function and cleans up the heap memory for that variable. But
Figure 4-2 shows both data pointers pointing to the same location. This is a
problem: when `s2` and `s1` go out of scope, they will both try to free the
same memory. This is known as a *double free* error and is one of the memory
safety bugs we mentioned previously. Freeing memory twice can lead to memory
corruption, which can potentially lead to security vulnerabilities.

To ensure memory safety, there’s one more detail to what happens in this
situation in Rust. Instead of trying to copy the allocated memory, Rust
considers `s1` to no longer be valid and therefore, Rust doesn’t need to free
anything when `s1` goes out of scope. Check out what happens when you try to
use `s1` after `s2` is created, it won’t work:

```rust,ignore
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

You’ll get an error like this because Rust prevents you from using the
invalidated reference:

```text
error[E0382]: use of moved value: `s1`
 --> src/main.rs:5:28
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`, which does
  not implement the `Copy` trait
```

If you’ve heard the terms “shallow copy” and “deep copy” while working with
other languages, the concept of copying the pointer, length, and capacity
without copying the data probably sounds like a shallow copy. But because Rust
also invalidates the first variable, instead of calling this a shallow copy,
it’s known as a *move*. Here we would read this by saying that `s1` was *moved*
into `s2`. So what actually happens is shown in Figure 4-4.

<img alt="s1 moved to s2" src="img/trpl04-04.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-4: Representation in memory after `s1` has been
invalidated</span>

That solves our problem! With only `s2` valid, when it goes out of scope, it
alone will free the memory, and we’re done.

In addition, there’s a design choice that’s implied by this: Rust will never
automatically create “deep” copies of your data. Therefore, any *automatic*
copying can be assumed to be inexpensive in terms of runtime performance.

#### Ways Variables and Data Interact: Clone

If we *do* want to deeply copy the heap data of the `String`, not just the
stack data, we can use a common method called `clone`. We’ll discuss method
syntax in Chapter 5, but because methods are a common feature in many
programming languages, you’ve probably seen them before.

Here’s an example of the `clone` method in action:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

This works just fine and is how you can explicitly produce the behavior shown
in Figure 4-3, where the heap data *does* get copied.

When you see a call to `clone`, you know that some arbitrary code is being
executed and that code may be expensive. It’s a visual indicator that something
different is going on.

#### Stack-Only Data: Copy

There’s another wrinkle we haven’t talked about yet. This code using integers,
part of which was shown earlier in Listing 4-2, works and is valid:

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

But this code seems to contradict what we just learned: we don’t have a call to
`clone`, but `x` is still valid and wasn’t moved into `y`.

The reason is that types like integers that have a known size at compile time
are stored entirely on the stack, so copies of the actual values are quick to
make. That means there’s no reason we would want to prevent `x` from being
valid after we create the variable `y`. In other words, there’s no difference
between deep and shallow copying here, so calling `clone` wouldn’t do anything
differently from the usual shallow copying and we can leave it out.

Rust has a special annotation called the `Copy` trait that we can place on
types like integers that are stored on the stack (we’ll talk more about traits
in Chapter 10). If a type has the `Copy` trait, an older variable is still
usable after assignment. Rust won’t let us annotate a type with the `Copy`
trait if the type, or any of its parts, has implemented the `Drop` trait. If
the type needs something special to happen when the value goes out of scope and
we add the `Copy` annotation to that type, we’ll get a compile time error. To
learn about how to add the `Copy` annotation to your type, see Appendix C on
Derivable Traits.

So what types are `Copy`? You can check the documentation for the given type to
be sure, but as a general rule, any group of simple scalar values can be
`Copy`, and nothing that requires allocation or is some form of resource is
`Copy`. Here are some of the types that are `Copy`:

* All the integer types, like `u32`.
* The Boolean type, `bool`, with values `true` and `false`.
* The character type, `char`.
* All the floating point types, like `f64`.
* Tuples, but only if they contain types that are also `Copy`. `(i32, i32)` is
`Copy`, but `(i32, String)` is not.

### Ownership and Functions

The semantics for passing a value to a function are similar to assigning a
value to a variable. Passing a variable to a function will move or copy, just
like assignment. Listing 4-3 has an example with some annotations showing where
variables go into and out of scope:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope.

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here.

    let x = 5;                      // x comes into scope.

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward.

} // Here, x goes out of scope, then s. But since s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

<span class="caption">Listing 4-3: Functions with ownership and scope
annotated</span>

If we tried to use `s` after the call to `takes_ownership`, Rust would throw a
compile time error. These static checks protect us from mistakes. Try adding
code to `main` that uses `s` and `x` to see where you can use them and where
the ownership rules prevent you from doing so.

### Return Values and Scope

Returning values can also transfer ownership. Here’s an example with similar
annotations to those in Listing 4-3:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1.

    let s2 = String::from("hello");     // s2 comes into scope.

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3.
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it.

    let some_string = String::from("hello"); // some_string comes into scope.

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function.
}

// takes_and_gives_back will take a String and return one.
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope.

    a_string  // a_string is returned and moves out to the calling function.
}
```

The ownership of a variable follows the same pattern every time: assigning a
value to another variable moves it. When a variable that includes data on the
heap goes out of scope, the value will be cleaned up by `drop` unless the data
has been moved to be owned by another variable.

Taking ownership and then returning ownership with every function is a bit
tedious. What if we want to let a function use a value but not take ownership?
It’s quite annoying that anything we pass in also needs to be passed back if we
want to use it again, in addition to any data resulting from the body of the
function that we might want to return as well.

It’s possible to return multiple values using a tuple, like this:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (s, length)
}
```

But this is too much ceremony and a lot of work for a concept that should be
common. Luckily for us, Rust has a feature for this concept, and it’s called
*references*.

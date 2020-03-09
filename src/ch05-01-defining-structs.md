## Definiowanie i tworzenie instancji struktur

Struktury są podobne do krotek, które omawiane były w Rozdziale 3. 
Podobnie do krotek, poszczególne części struktur mogą różnić się typami.
W odróżnieniu od krotek, każdy fragment danych musisz nazwać,
aby jasne było co każdy oznacza. W wyniku tego nazewnictwa struktury są bardziej
elastyczne od krotek. 
Nie musisz polegać na kolejności danych, aby dostać się do wartości danej struktury.

Aby zdefiniować strukturę posługujemy się słowem kluczowym `struct`,
po którym wstawiamy nazwę struktury.
Nazwa struktury powinna odzwierciedlać znaczenie grupy danych
znajdujących się w danej strukturze.
Następnie, w nawiasach klamrowych definiujemy nazwy i typy fragmentów danych,
które nazywamy *atrybutami*. 
Na przykład, w Listingu 5-1 widzimy strukturę,
w której znajdują się przykładowe dane profilu użytkownika.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

<span class="caption">Listing 5-1: `User` definicja struktury</span>

Aby wykorzystać strukturę po jej zdefiniowaniu tworzymy *instancję* tej struktury
poprzez podanie konkretnych wartości dla każdego atrybutu.
Tworzymy strukturę przez podanie jej nazwy, następnie nawiasy klamrowe zawierające
pary `klucz: wartość`, gdzie klucze to nazwy atrybutów,
a wartości to dane, które chcemy umieścić w danych atrybutach.
Nie musimy podawać atrybutów w tej samej kolejności w jakiej zostały one zdefiniowane
podczas deklaracji struktury.
Innymi słowy, definicja struktury jest ogólnym szablonem, a instancje jakby wypełniają
dany szablon jakimiś danymi tworząc wartości typu struktury.
Przykładowa deklaracja użytkownika pokazana jest w Listingu 5-2.

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

<span class="caption">Listing 5-2: Tworzenie instancji struktury `User`</span>

Aby uzyskać dostęp do danego atrybutu struktury używamy kropki.
Jeśli chcielibyśmy zdobyć tylko adres email danego użytkownika moglibyśmy
napisać `user1.email` gdziekolwiek ta wartość byłaby nam potrzebna.
Jeśli instancja jest mutowalna możemy zmienić atrybut używając
kropki aby uzyskać daną wartości i ją zmienić.
Listing 5-3 pokazuje jak zmienić atrybut `email` w mutowalnej instancji struktury `User`.

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

<span class="caption">Listing 5-3: Zmiana atrybutu `email` instancji struktury `User`.

Należy pamiętać, że cała instancja musi być mutowalna;
Rust nie pozwala nam zaznaczyć poszczególnych atrybutów jako mutowalnych.
Jak z każdym wyrażeniem, możemy skonstruować nową instancję struktury jako
ostatnie wyrażenie w ciele funkcji, aby dana instancja została zwrócona przez funkcję.

Listing 5-4 ukazuje funkcję `build_user`
zwracającą instancję struktury `User` z pewnym emailem
i nazwą użytkownika.
Atrybutowi `active` przypisana jest wartość `true`,
a atrybutowi `sign_in_count` przypisana jest wartość `1`.

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

<span class="caption">Listing 5-4: Funkcja `build_user`, która jako argument 
przyjmuje email i nazwę użytkownika, a zwraca instancję struktury `User`</span>

Nadanie parametrom funkcji tej samej nazwy co atrybutom struktury ma sens, ale
przez to musimy powtarzać nazwy atrybutów `email` i `username`, co jest trochę męczące.
Jeśli jakaś struktura miałaby więcej atrybutów powtarzanie każdego z nich
byłoby jeszcze bardziej męczące. Na szczęście, istnieje wygodny skrótowiec!

### Używanie skrótowej inicjalizacji atrybutów gdy atrybut i wartość mają taką samą nazwę

Ponieważ nazwy parametrów i atrybutów struktury są takie same, w
Listingu 5-4 możemy użyć składni tzw. *skrótowej inicjializacji atrybutów*
(ang. field init shorthand), aby zmienić funkcję `build_user`,
tak aby nie zmieniać jej zachowania, ale też nie powtarzając
`email` i `username`. Taki zabieg widzimy w Listingu 5-5.

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

<span class="caption">Listing 5-5: Funkcja `build_user`
używająca skrótowej inicjalizacji atrybutów `email` oraz `username`,
które mają takie same nazwy jak parametry funkcji</span>

Tutaj tworzymy nową instancję struktury `User`, która posiada atrybut
o nazwie `email`. Chcemy nadać atrybutowi `email` wartość znajdującą się
w parametrze `email` funkcji `build_user`. Skoro atrybut `email` i parametr `email`
mają takie same nazwy wystarczy, że napiszemy `email` jedynie raz zamiast
musieć napisać `email: email`.

### Tworzenie instancji z innych instancji przy użyciu składni zmiany struktury

Czasem bardzo przydatnym jest stworzenie nowej struktury, która jest w zasadzie
kopią innej struktury, w której chcemy zmienić tylko niektóre atrybuty.
Do tego celu zastosujemy *składnię zmiany struktury*.

Listing 5-6 obrazuje tworzenie instancji struktury `User` zapisanej do zmiennej `user2`
bez użycia naszej nowej składni. Nadajemy nowe wartości atrybutom `email` i `username`, ale poza tym
zostawiamy te same wartości w instancji `user1`, które przypisaliśmy w Listingu 5-2.

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
# let user1 = User {
#     email: String::from("someone@example.com"),
#     username: String::from("someusername123"),
#     active: true,
#     sign_in_count: 1,
# };
#
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

<span class="caption">Listing 5-6: Tworzenie nowej instancji struktury `User` pozostawiając
niektóre wartości z instancji struktury `user1`</span>

Przy użyciu składni zmiany struktury możemy osiągnąć ten sam efekt mniejszym nakładem kodu,
co widzimy w Listingu 5-7. Składnia `..` oznacza, że pozostałym atrybutom, którym nie oznaczyliśmy ręcznie
wartości przypisane zostaną wartości z danej, oznaczonej instancji.

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
# let user1 = User {
#     email: String::from("someone@example.com"),
#     username: String::from("someusername123"),
#     active: true,
#     sign_in_count: 1,
# };
#
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

<span class="caption">Listing 5-7: Użycie składni zmiany struktury w celu przypisania
nowych wartości atrybutom `email` oraz `username` z instancji struktury `User`,
jednocześnie pozostawiając wartości atrybutów ze zmiennej `user1`</span>

Kod przedstawiony w Listingu 5-7 tworzy też instancję w zmiennej `user2`, która
zmienia wartości w atrybutach `email` i `username`, ale pozostawia wartości
z atrybutów `active` i `sign_in_count` ze zmiennej `user1`.

### Wykorzystanie braku nazywania atrybutów w strukturach-krotkach do tworzenia nowych typów

Możesz też stworzyć struktury podobne do krotek, nazywane *strukturami-krotkami* (ang. tuple structs).
Atutem struktur-krotek jest przypisanie znaczenia atrybutom bez ich nazywania,
a jedynie przez przypisanie atrybutom ich typu.
Struktury-krotki przydatne są najbardziej, kiedy: chciałbyś użyć krotkę,
chcesz nadać jej nazwę i odróżnić ją od innych poprzez posiadanie innego typu,
oraz kiedy nazywanie każdego atrybutu (jak w normalnej strukturze) byłoby zbyt rozwlekłe lub zbędne.

Aby zdefiniować strukturę-krotkę, najpierw wpisz słowo kluczowe `struct`, po nim nazwę struktury,
a następnie typy twojej nowej krotki.
Dla przykładu, tutaj pokazane są działania na dwóch strukturach-krotkach, tj. `Color` i `Point`:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

Zauważ, że `black` i `origin` mają różne typy, bo są instancjami różnych struktur-krotek.
Każda zdefiniowana struktura ma własny, niepowtarzalny typ mimo, tego że atrybuty w danych
dwóch strukturach mogą mieć identyczne typy.
Na przykład, funkcja przyjmująca parametr typu `Color` nie może także przyjąć argumentu typu `Point`,
mimo że np. oba typy mogą składać się z trzech wartości typu `i32`.
Oprócz tego wyjątku struktury-krotki zachowują się całkiem jak krotki:
możesz użyć składni przypisania destrukturyzującego, aby przypisać atrybuty zmiennym;
możesz także użyć numeru indeksu bezimiennego atrybutu poprzedzonego `.`, aby
uzyskać do niego dostęp.

### Struktura-jednostka bez żadnych atrybutów

Możesz także definiować struktury nie posiadające żadnych atrybutów!
Są to tzw. *struktury-jednostki* (ang. unit-like structs), bo zachowują się podobnie do `()`, czyli jednostki.
Struktury-jednostki mogą być przydatne, kiedy chciałbyś zaimplementować cechę w typie,
ale sama struktura nie zawiera w sobie żadnych danych. Więcej o cechach w Rozdziale 10.

> ### System własności danych struktury
>
> W definicji struktury `User` w Listingu 5-1 użyliśmy posiadanego typu `String`a
> zamiast wycinka *stringowego* typu `&str`. To świadomy wybór, gdyż chcemy, aby instancje struktury posiadały wszystkie
> swoje dane oraz żeby te dane były nienaruszone, jeśli sama struktura jest nienaruszona.
>
> Struktury mogą przechowywać referencje do danych należących do czegoś innego,
> ale do tego potrzebne byłyby informacje o *długości życia* zmiennych (ang. lifetime).
> Jest to funkcja Rusta, o której powiemy więcej w Rozdziale 10. 
> Długość życia gwarantuje nam, że dane wskazywane przez referencję
> są nienaruszone dopóty, dopóki struktura istnieje.
> Załóżmy, że próbujesz przechować referencję do struktury bez podania informacji
> o długości życia tak jak tutaj, co nie zadziała:
>
> <span class="filename">Nazwa pliku: src/main.rs</span>
>
> ```rust,ignore,does_not_compile
> struct User {
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
>     active: bool,
> }
>
> fn main() {
>     let user1 = User {
>         email: "someone@example.com",
>         username: "someusername123",
>         active: true,
>         sign_in_count: 1,
>     };
> }
> ```
>
> Kompilator da ci znać, że potrzebuje specyfikatoru długości życia:
>
> ```text
> error[E0106]: missing lifetime specifier
>  -->
>   |
> 2 |     username: &str,
>   |               ^ expected lifetime parameter
>
> error[E0106]: missing lifetime specifier
>  -->
>   |
> 3 |     email: &str,
>   |            ^ expected lifetime parameter
> ```
>
> W Rozdziale 10 pokażemy jak pozbyć się tych błędów, aby przechować
> referencje do innych struktur, ale póki co pozbędziemy się ich po prostu
> używając posiadanych typów, takich jak `String` zamiast referencji typu `&str`.

## Składnia metod

*Metody* są podobne do funkcji: też deklarujemy je słowem kluczowym `fn`,
po którym umieszczamy nazwę, a czasem parametry i typ zwracanej wartości.
Tak jak funkcje, zawierają jakieś polecenia wykonywane przy wywołaniu. 
Metody jednak różnią się od funkcji tym, że definiujemy je wewnątrz struktur
(lub enumeracji czy obiektów-cech, które omówimy odpowiednio w rozdziałach 6 i 17)
a ich pierwszym parametrem jest zawsze `self` reprezentujące instancję
struktury, na której metoda jest wywołana.

### Definiowanie metod

Zmieńmy funkcję `area` przyjmującą jako parametr instancję struktury `Rectangle`,
w taki sposób aby od teraz `area` było metodą zdefiniowaną na strukturze `Rectangle`.
To przedstawia listing 5-13.

<span class="filename">Plik: src/main.rs</span>

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "Pole prostokąta wynosi {} pikseli kwadratowych."
        rect1.area()
    );
}
```

<span class="caption">Listing 5-13: Definicja metody `area` w strukturze `Rectangle`</span>

Aby zdefiniować funkcję w kontekście `Rectangle` otwieramy blok `impl`, czyli implementacji. 
Następnie przenosimy funkcję `area` do nawiasów klamrowych
należących do bloku `impl` i jako pierwszy parametr funkcji dodajemy
`self` w jej sygnaturze i wszędzie w jej wnętrzu. W funkcji `main`,
zamiast jak poprzednio wywołania funkcji `area`, a jako argument
podawania jej `rect1`, możemy użyć *składni metody* aby wywołać metodę `area`
na instancji struktury `Rectangle`.
Składnia metody pojawia się po nazwie instancji: dodajemy kropkę, a po niej nazwę samej metody, oraz
nawiasy i argumenty jeśli są wymagane.

W sygnaturze funkcji `area` używamy `&self` zamiast `rectangle: &Rectangle`
ponieważ Rust wie, że typ `self` to `Rectangle`, gdyż metoda znajduje się
wewnątrz kontekstu `impl Rectangle`. Zauważ, że nadal musimy użyć `&`
przed `self`, podobnie jak w `&Rectangle`. Metody mogą wejść w posiadanie `self`,
pożyczyć `self` niemutowalnie, lub pożyczyć `self` mutowalnie, 
tak jakby to był jakikolwiek inny parametr.

W tym wypadku użyliśmy `&self` z tego samego powodu, co `&Rectangle`
w wersji z funkcją. Otóż nie chcemy zostać właścicielem, ale nadal powinniśmy móc odczytać dane
ze struktury, nie potrzebujemy jednak jej zmieniać. Jeśli chcielibyśmy zmienić dane instancji w trakcie wywoływania
metody użylibyśmy `&mut self` jako pierwszego parametru. 
Tworzenie metody która wchodzi w posiadanie instancji przy użyciu `self`
jest dość rzadkie; tej techniki używamy głównie jedynie, kiedy metoda przeobraża `self`
w coś innego i chcesz zabronić wywołującemu metody wykorzystanie oryginalnej instancji
po jej transformacji.

Podstawową zaletą używania metod zamiast funkcji, oprócz używania
składni metody oraz braku wymogu podawania typu `self` w każdej sygnaturze,
jest organizacja.  
Umieściliśmy wszystko, co związane jest z instancją danego typu w jednym bloku `impl`.
Dzięki temu oszczędzamy przyszłym użytkownikom kodu szukania zachowań struktury
`Rectangle` po różnych zakątkach naszej biblioteki.



> ### Co z operatorem `->`?
>
> W C i C++ istnieją dwa różne operatory używane do wywoływania metod:
> symbolu `.` używamy do bezpośredniego wywoływania metody na obiekcie, zaś
> symbolu `->`, jeśli metodę wywołujemy na wskaźniku do obiektu, który najpierw
> musimy poddać dereferencji. Innymi słowy, jeśli `object` jest wskaźnikiem,
> `object->something()` jest podobne do `(*object).something()`.
>
> Rust nie ma operatora równoważnego z `->`; a za to w Ruście spotkasz
> funkcję *automatycznej referencji i dereferencji*. Wywoływanie metod jest jednym z niewielu miejsc,
> w których Rust wykorzystuje tę funkcję.
>
> Działa to następująco: kiedy wywołujesz metodę kodem `object.something()`, Rust
> automatycznie dodaje `&`, `&mut`, lub `*` aby `object`
> pasował do sygnatury metody. Inaczej mówiąc, oba te przykłady są równoważne:
>
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> Pierwszy wygląda o wiele bardziej przejrzyście. Zastosowana w niej jest automatyczna referencja,
> ponieważ metoda ma wyraźnie oznaczonego odbierającego (*ang. receiver*) - typ `self`.
> Mając informacje o odbierającym oraz o nazwie metody Rust jest w stanie jednoznacznie
> stwierdzić, czy metoda odczytuje (`&self`), zmienia (`&mut self`) lub konsumuje (`self`).
> Wymaganie przez Rusta oznaczenia pożyczania w odbierającym metody jest
> w dużej części dlaczego mechanizm posiadania jest tak ergonomiczny w używaniu.

### Metody z wieloma parametrami

Poćwiczymy używanie metod implementując drugą metodę na strukturze `Rectangle`.
Tym razem chcemy, żeby instancja struktury `Rectangle` przyjęła inną instancję `Rectangle`,
i zwróciła: wartość `true`, jeśli ta inna instancja `Rectangle`
całkowicie mieści się w instancji `self`; a jeśli nie, wartość `false`.
Innymi słowy, zakładając, że wcześniej zdefiniowaliśmy metodę `can_hold`, chcemy
być w stanie napisać program przedstawiony w listingu 5-14.

<span class="filename">Plik: src/main.rs</span>

```rust,ignore
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Czy rect2 zmieści się wewnątrz rect1? {}", rect1.can_hold(&rect2));
    println!("Czy rect3 zmieści się wewnątrz rect1? {}", rect1.can_hold(&rect3));
}
```

<span class="caption">Listing 5-14: Użycie jeszcze nieistniejącej metody `can_hold`</span>

Skoro wymiary
`rect2` są mniejsze od wymiarów `rect1`, a `rect3` jest szerszy
od `rect1`, spodziewamy się następującego wyniku wykonania powyższej funkcji `main`.

```text
Czy rect2 zmieści się wewnątrz rect1? true
Czy rect3 zmieści się wewnątrz rect1? false
```

Chcemy zdefiniować metodę, więc umieścimy ją w bloku `impl Rectangle`.
Metodę nazwiemy `can_hold`, i przyjmie ona jako parametr niemutowalne wypożyczenie
innej instancji `Rectangle`. Aby dowiedzieć się jaki dokładnie typ powinien
znajdować się w parametrze, spójrzmy na kod wywołujący metodę:
`rect1.can_hold(&rect2)`, przekazuje `&rect2` będące niezmiennym wypożyczeniem
`rect2`, czyli pewnej instancji `Rectangle`. Zależy nam jedynie
na odczytaniu wartości zawartych w `rect2` (do ich zmieniania wymagane byłoby mutowalne wypożyczenie),
a chcemy, żeby `main` pozostało właścicielem `rect2`, abyśmy mogli ponownie wykorzystać `rect2`
po wywołaniu metody `can_hold`. Wartość zwracana przez `can_hold` będzie typem
Boolean, a sama implementacja sprawdzi czy wysokość i szerokość instancji
`self` są większe niż odpowiednio wysokość i szerokość innej instancji `Rectangle`.
Dodanie nowej metody `can_hold` do bloku `impl` z
listingu 5-13 pokazane jest w listingu 5-15.

<span class="filename">Plik: src/main.rs</span>

```rust
# #[derive(Debug)]
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
#
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

<span class="caption">Listing 5-15: Implementacja metody `can_hold` na instancji
`Rectangle`, która przyjmuje inną instancję `Rectangle` jako parametr</span>

Po uruchomieniu tego kodu funkcją `main` z listingu 5-14,
naszym oczom ukaże się oczekiwany przez nas tekst.
Metody mogą przyjmować wiele parametrów, które dodać możemy
do ich sygnatur po parametrze `self`.
Te parametry niczym nie różnią się od parametrów funkcji.

### Funkcje powiązane

Używanie bloków `impl` niesie za sobą kolejną przydatną funkcję, a mianowicie
pozwala nam na definiowanie funkcji w blokach `impl`, które *nie* przyjmują `self` jako parametru. Są to
tzw. *funkcje powiązane* (*ang. associated functions*), ponieważ mają bliski związek ze strukturami.
To jednak nadal są funkcje, a nie metody, bo nie mogą wykonywać operacji na
instancjach struktury. Już miałeś okazję użyć funkcji powiązanej, była nią `String::from`.

Funkcje powiązane są często używane w konstruktorach zwracających nową
instancję pewnej struktury. Na przykład, możemy stworzyć funkcję powiązaną,
która przyjmie tylko jeden wymiar jako parametr i przypisze go zarówno do wysokości i szerokości
ułatwiając stworzenie kwadratowego `Rectangle` zamiast podawania dwa razy
tej samej wartości:

<span class="filename">Plik: src/main.rs</span>

```rust
# #[derive(Debug)]
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
#
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
```

Aby wywołać funkcję powiązaną używamy składni `::` po nazwie struktury; np.
`let sq = Rectangle::square(3)`. Ta funkcja znajduje się w przestrzeni nazw
struktury: składnia `::` używana jest zarówno w kontekście funkcji powiązanych,
ale i też w przestrzeniach nazw modułów. Moduły omówimy w rozdziale 7.

### Wiele bloków `impl`

Każda struktura może mieć wiele bloków `impl`. Dla przykładu, kod w listingu 5-15 jest równoważny z kodem w listingu 5-16
zawierającym każdą metodę w innym bloku `impl`.

```rust
# #[derive(Debug)]
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
#
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

<span class="caption">Listing 5-16: Wersja kodu z listingu 5-15 z wieloma blokami `impl`</span>

W tym przypadku nie ma powodu, by metody były od siebie odseparowane w różne bloki `impl`,
ale jest to poprawna składnia. Przypadek przydatnego wykorzystania wielu bloków `impl`
omówimy w rozdziale 10, w którym omówimy typy uniwersalne i cechy.

## Podsumowanie

Dzięki strukturom możesz tworzyć własne typy, potrzebne do rozwiązania problemów w Twojej domenie. 
Kiedy używasz struktury grupujesz powiązane elementy danych,
a każdemu elementowi nadajesz nazwę, co sprawia, że twój kod staje się przejrzysty. Metody pozwalają określić
zachowanie instancji struktur, a funkcje powiązane pozwalają umieścić funkcję
w przestrzeni nazw danej struktury bez wymogu dostępu do istniejącej instancji struktury.

Ale struktury to nie jest jedyny sposób na tworzenie własnych typów:
poznamy kolejną funkcjonalność Rusta - enumeracje, czyli kolejne
niezbędne narzędzie w twojej kolekcji.


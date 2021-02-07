## Definiowanie wyliczeń

Weźmy na tapetę pewną sytuację, w której wyliczenia są przydatniejsze i bardziej odpowiednie niż struktury.
Załóżmy, że chcemy wykonywać operacje na adresach IP.
Obecnie istnieją dwa standardy adresów IP: wersja czwarta i szósta.
To jedyne możliwe typy adresów IP z jakimi napotka się nasz program:
możemy wyliczyć (*ang. enumerate*) wszystkie możliwe wartości,
stąd nazwa wyliczeń/enumeracji.

Dany adres IP może być albo wersji czwartej albo szóstej, ale nigdy obiema naraz.
Ta właściwość adresów IP sprawia, że wyliczenia będą dobrym wyborem,
skoro mogą przyjąć tylko jedną wartość ze wszystkich swoich wariantów.
Zarówno adresy wersji czwartej, jak i wersji szóstej to nadal adresy IP,
więc kod zajmujący się operacjami niezależnymi od typu adresu
powinien traktować oba adresy jakby były tym samym typem.

Możemy wyrazić tę myśl w kodzie definując wyliczenie `IpAddrKind` i
wymieniając wszystkie możliwe typy adresów IP: `V4` oraz `V6`. To są warianty tego enuma:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:def}}
```

`IpAddrKind` jest teraz niestandardowym typem danych dostępnym dla nas w całym kodzie.

### Wartości wyliczeń

Możemy stworzyć instancje obu wariantów `IpAddrKind` następująco:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:instance}}
```

Zauważ, że warianty wyliczenia dostępne są w przestrzeni jego nazwy, a więc
korzystamy z dwóch dwukropków pomiędzy nazwą enuma a jego wariantem. To okazuje się być przydatne,
bo teraz zarówno wartość `IpAddrKind::V4` oraz `IpAddrKind::V6` mają ten sam typ:
`IpAddrKind`. A co za tym idzie, możemy napisać funkcję przyjmującą jako argument dowolny
`IpAddrKind`.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

tę funkcję wywołać możemy dowolnym wariantem:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

Enumeracje mają jeszcze więcej zalet. Dalej wgłębiając się w nasz typ adresu IP,
na chwilę obecną nie jesteśmy w stanie przechować samego adresu IP, czyli jego *danych*;
a przechowujemy jedynie jego *rodzaj*. Skoro dopiero co w rozdziale 5 poznaliśmy struktury,
mógłbyś/mogłabyś wpaść na pomysł podobny do tego w listingu 6-1.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-01/src/main.rs:here}}
```

<span class="caption">Listing 6-1: Przechowywanie danych i wariantu `IpAddrKind`
adresu IP przy użyciu `struktury`</span>

Zdefiniowaliśmy strukturę `IpAddr` mającą dwa atrybuty: `kind` (ang. rodzaj)
wartość typu `IpAddrKind` (wyliczenie zdefiniowane przez nas wcześniej) oraz atrybut
`address` przechowującą wartość typu `String`. Stworzyliśmy dwie instancje tej struktury.
Pierwsza, `home`, do atrybutu `kind`, przypisaną ma wartość `IpAddrKind::V4`.
Druga instancja, `loopback` ma inny wariant `IpAddrKind` jako wartość pola `kind`,
ta wartość wynosi `V6`; oraz jako adres przypisany ma String `::1`.
Tym samym użyliśmy struktury aby zgrupować wartości `kind` i `address`, dzięki czemu
typ adresu i sam adres są ze sobą powiązane.

Przy użyciu enuma jesteśmy w stanie wyrazić tę samą myśl w bardziej zwięzłej formie, niż
wstawiając enuma do struktury, przez umieszczenie danych bezpośrednio w samym wariancie enuma.
Ta nowa definicja enuma `IpAddr` zawiera zarówno w wariancie `V4` jak i `V6`
nową wartość o typie `String`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-02-enum-with-data/src/main.rs:here}}
```

Bezpośrednio dołączamy dane do każdego warianta enuma, więc dodatkowa struktura
staje się niepotrzebna.

Wykorzystanie enuma zamiast struktury niesie ze sobą jeszcze jedną korzyść:
z każdym wariantem mogą być powiązane inne typy oraz ilości danych. 
Adresy IP wersji czwartej zawsze będą miały cztery liczby, o wartościach pomiędzy
0 a 255. Zapisanie adresu `V4` jako czterech wartości `u8`, a adresu `V6`
nadal jako typ `String` byłoby niemożliwe przy użyciu struktury.
W przypadku wyliczeń jest to proste:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-03-variants-with-different-data/src/main.rs:here}}
```

Pokazaliśmy kilka różnych sposobów definiowania struktur danych przechowujących
adresy IP czwartej i szóstej wersji. Jak się jednak okazuje, przechowywanie adresów IP
wraz z rodzajem ich wersji jest tak powszechne, że [biblioteka standardowa ma gotową
definicję czekającą tylko na to, aby jej użyc!][IpAddr]<!-- ignore -->
Spójrzmy na definicję `IpAddr` w bibliotece standardowej: ma dokładnie taką samą nazwę
i takie sama warianty, ale przechowuje dane o adresach w postaci dwóch różnych struktur
umieszczonych w wyliczeniach. Każda struktura zdefiniowana jest inaczej dla każdego wariantu.

[IpAddr]: ../std/net/enum.IpAddr.html

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Jak demonstruje powyższy wycinek kodu, do wariantów enuma umieścić można
każdy typ danych, np.: ciągi znaków (stringi), typy liczbowe, lub struktury.
W enumie umieścić możesz nawet innego enuma!
Ponadto, typy w standardowej bibliotece często nie są wcale bardziej skomplikowane
od tego co wymyślisz własnoręcznie.

Mimo że standardowa biblioteka definuje własny `IpAddr`, nadal możemy
stworzyć i używać własnej definicji bez żadnych konfliktów, bo nie zaimportowaliśmy
definicji z biblioteki standardowej do zakresu (ang. scope).
Więcej o importowaniu typów do zakresu w rozdziale 7.

Spójrzmy na innego enuma, na przykładzie listingu 6-2: ten w swoich wariantach zawierał będzie
wiele różnych typów.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

<span class="caption">Listing 6-2: Enum `Message`, którego warianty zawierają
różne ilości i typy wartości</span>

Ten enum definiuje cztery warianty, każdy z innymi typami:

* `Quit`, nie zawiera w sobie żadnych danych.
* `Move` zawiera w sobie anonimową strukturę.
* `Write` zawiera w sobie jeden `String`.
* `ChangeColor` zawiera trzy wartości o typach `i32`.

Na przykładzie tego w listingu 6-2 widzimy, że definiowanie wariantów enuma, jest podobne do definiowania
kilku struktur, z taką różnicą, że nie używamy słowa kluczowego `struct` oraz,
wszystkie warianty zgrupowane są w typie `Message`.
Poniższe struktury mogłyby zawierać te same dane co powyższe typy enuma:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

Ale jeśli użylibyśmy różnych struktur, to każda z nich miałaby inny typ.
Zdefiniowanie funkcji mogącej przyjąć jako parametr
różne rodzaje wiadomości, nie byłoby tak proste jak przy użyciu 
enuma `Message` zdefiniowanego w listingu 6-2, który jest tylko jednym typem.

Jest jeszcze jedno podobieństwo między wyliczeniami, a strukturami: tak samo, jak
zdefiniować można metody na strukturach używając bloku `impl`, zdefiniować można
metody na wyliczeniach. Spójrzmy na metodę o nazwie `call` zdefinowaną na naszym enumie `Message`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

Ciało metody użyje wartości `self`, aby uzyskać wariant enuma, na którym ta metoda została wywołana.
W tym przykładzie przypisaliśmy do zmiennej `m` wartość
`Message::Write(String::from("witaj"))` równoważną parametrowi `self`,
który znajduje się w ciele metody `call`, kiedy ta uruchomiona zostanie poprzez `m.call()`.

Spójrzmy na kolejne wyliczenie z biblioteki standardowej,
które jest bardzo przydatne i często używane, czyli `Option`.

### Wyliczenie `Option` i jego przewagi nad wartościami *pustymi* (ang. null)

W poprzedniej sekcji omówiliśmy jak enum `IpAddr` pozwala nam wykorzystać
system typów Rusta, aby zawrzeć w naszym programie więcej informacji, niż tylko same dane.
W tej sekcji znajduje się analiza wyliczenia `Option` - kolejnego enuma z biblioteki standardowej.
Typ `Option` używany jest w wielu miejscach, ponieważ
opisuje bardzo częsty scenariusz, w którym dana wartość może być zarówno czymś albo niczym.
Wyrażając to pojęcie pod względem systemu typów Rusta, oznacza to, że
kompilator jest w stanie sprawdzić, czy wziąłeś pod uwagę wszystkie wymagane przypadki;
ta funkcjonalność pozwala zapobiec błędom (bugom) pojawiającym się bardzo często w innych językach
programowania.  

Przez konstrukcję języka programowania często rozumie się decyzje o zamieszczeniu w nim jakichś funkcji,
ale to jakie funkcje się w nim nie znalazły, jest również istotne.
Rust nie ma wartości `null` znanej z wielu innych języków.
*Null* to wartość oznaczająca brak wartości - jest to wartośc pusta.
W językach z pustymi wartościami, zmienne zawsze mogą być jednym z dwóch stanów: null lub nie-null.

W swojej prezentacji z 2009 roku "Puste referencje: Błąd warty miliard dolarów" 
(oryg. “Null References: The Billion Dollar Mistake,”),
Tony Hoare, wynalazca nulla, miał to do powiedzenia:

> Ten błąd warty jest miliard dolarów. W tamtych czasach projektowałem pierwszy kompleksowy system typów
> referencji dla języków obiektowych. Moim celem była możliwość gwarancji, że każde użycie referencji byłoby całkowicie
> bezpieczne, co automatycznie sprawdzałby kompilator. 
> Ale nie mogłem oprzeć się pokusie implementacji pustych referencji, z prostej przyczyny:
> było to łatwe do zaimplementowania. Ta decyzja doprowadziła do tylu niezliczonych błędów, luk i awarii systemów,
> że łącznie przez ostatnie czterdzieści lat pewnie spowodowała ból i szkody warte miliard dolarów.

Problem z pustymi wartościami polega na tym, że kiedy spróbujesz użyć nulla,
jak gdyby nie był nullem, spowodujesz błąd.
Własność null i nie-null rozpowszechniła się tak szeroko, że
popełnienie takiego błędu jest bardzo łatwe.

Jednak pojęcie jakie null próbuje wyrazić jest samo w sobie przydatne:
wartość pusta jest albo obecnie nieważna albo nieobecna.

Problemem nie jest sam pomysł, ale ta konkretna implementacja pustych wartości.
Rust nie ma jako takich pustych wartości null, ale istnieje w Ruście wyliczenie,
które wyraża pojęcie obecności lub braku obecności danej wartości.
Tym wyliczeniem jest `Option<T>` zdefiniowane przez bibliotekę standardową][option]<!-- ignore -->
w następujący sposób:

[option]: ../std/option/enum.Option.html

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Enum `Option<T>` jest tak przydatny, że znajduje się w preludzie (prelude);
nie musisz samemu go importować.  Ponadto, to samo dotyczy jego wariantów:
możesz użyć `Some` i `None`, bez używania prefiksu `Option::`.
Enum `Option<T>` jest zwykłym wyliczeniem, a `Some(T)` oraz `None`
to nadal zwykłe warianty `Option<T>`.

Składnia `<T>` jest funkcjonalnością Rusta, której jeszcze nie omówiliśmy.
Jest to tzw. parametr generyczny. Bardziej szczegółowo je omówimy w rozdziale 10.
Póki co, wszystko co musisz o nich wiedzieć to, że `<T>` oznacza, że wariant `Some`
enuma `Option` może zawierać w sobie jedną wartość dowolnego typu. Oto niektóre przykłady
używania wartości `Option` do przechowywania typów liczbowych oraz łańcuchów znaków (stringów):

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-06-option-examples/src/main.rs:here}}
```

Używając `None` zamiast `Some` musimy przekazać Rustowi, jaki typ danych
zawierać będzie nasz `Option<T>`, bo kompilator nie jest w stanie samemu wywnioskować
jaki typ danych zawierać będzie wariant `Some` patrząc jedynie na wartość `None`.

Widząc wartość `Some`, wiemy że wartość jest obecna oraz że znajduje
się ona w `Some`. Za to patrząc na takiego `None`, w pewnym sensie oznacza to samo
co null, czyli brak prawidłowej wartości. To dlaczego `Option<T>` jest lepszy od nulla?

W skrócie, `Option<T>` i `T`(gdzie `T` może być dowolnym typem)
są różnymi typami, więc kompilator nie pozwoli nam użyć `Option<T>` tak jakby
była to prawidłowa wartość typu `T`. Na przykład, ten kod się nie skompiluje, bo próbujemy w nim
dodać wartość typu `i8` do typu `Option<i8>`:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/src/main.rs:here}}
```

Uruchamiając ten kod, otrzymamy następujący komunikat o błędzie:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/output.txt}}
```

Bezlitośnie! Ten komunikat oznacza, że Rust nie wie jak ma dodać do siebie typ `i8`
oraz typ `Option<i8>`, ponieważ to dwa różne typy. Kiedy w Ruście posługujemy się typem takim jak `i8`, kompilator zawsze
gwarantuje, że jest to prawidłowa wartość. Możemy być pewni swego i kontynuować kodowanie bez
sprawdzania czy dana wartość jest pusta w środku. Jedynie kiedy posługujemy się typem `Option<i8>`
(czy jakimkolwiek innym typem zawarym wewnątrz `Option`) musimy się upewnić, czy
w środku znajduje się prawidłowa wartość, a kompilator sprawdzi, czy
na pewno wzięliśmy obie sytuacje pod uwagę.

Innymi słowy, musisz przekonwertować wartość typu `Option<T>` na `T` zanim
przyjmie ona zachowania charakterystyczne dla typu `T`.
W większości przypadków pozwala to na pozbycie się
jednego z najczęstszych problemów z nullem: zakładanie, że jakaś wartość istnieje, kiedy
tak na prawdę nie istnieje.

Nie martwiąc się tym, czy prawidłowo założymy, że dana wartość nie jest pusta
mamy pewność co do napisanego kodu. Aby dana wartośc mogła nie istnieć musisz wyrazić na to zgodę
definiując daną wartość jako typ `Option<T>`.
Następnie używając tę wartość do twoich obowiązków należeć będzie powzięcie innych kroków, 
dla przypadków kiedy ta wartość jest pusta.
Gdziekolwiek, gdzie dany typ wartości nie jest typem `Option<T>`, możesz
bezpiecznie założyć, że ta wartość nie jest pusta.
To była przemyślana decyzja w konstrukcji Rusta mająca na celu ograniczenie wszechobecności
nulla oraz zwiększenie bezpieczeństwa kodu napisanego w Ruście.

Więc mając wartość typu `Option<T>`, jak można dostać się do wartości typu `T`
znajdującej się w wariancie `Some`? Enum `Option<T>` ma wiele przydatnych metod odpowiednich dla różnych sytuacji;
możesz je sprawdzić w [dokumentacji][docs]<!-- ignore -->. Zapoznanie się z metodami
typu `Option<T>` będzie bardzo przydatne w twojej podróży z Rustem.

[docs]: ../std/option/enum.Option.html

Zwykle, aby użyć wartości typu `Option<T>`, musisz napisać kod sprawdzający oba warianty.
Jedna część kodu będzie odpowiedzialna za wariant `Some(T)` - ta część będzie miała
dostęp do wewnętrznej wartości typu `T`.
Druga część będzie odpowiedzialna za wariant `None` - ten kod oczywiście nie będzie miał dostępu
do wartości typu `T`. Wyrażenie `match` jest konstruktem umożliwiającym kontrolę przepływu (ang. control flow)
pozwalającym na tego typu zachowanie. 
Wyrażenie `match` uruchomi różny kod w zależności od tego,
jakim wariantem jest dany enum. Ten kod będzie będzie miał dostęp do danych znajdujących się w danym enumie.

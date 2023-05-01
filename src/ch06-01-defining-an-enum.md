## Definiowanie wyliczeń

Podczas gdy struktury pozwalają na grupowanie powiązanych pól i danych, jak `Rectangle` z jego szerokością i wysokością, typy wyliczeniowe pozwalają na określenie wartości jako jednej z możliwych. Na przykład, za ich pomocą możemy wyrazić, że `Rectangle` (prostokąt) jest jednym z możliwych kształtów, które obejmują również `Circle` (koło) i `Triangle` (trójkąt).

Weźmy na tapetę pewną sytuację, w której wyliczenia są przydatniejsze i bardziej odpowiednie niż struktury.
Załóżmy, że chcemy wykonywać operacje na adresach IP.
Obecnie istnieją dwa standardy adresów IP: wersja czwarta i szósta.
Ponieważ to jedyne możliwe typy adresów IP na jakie napotka się nasz program, to
możemy wyliczyć (*ang. enumerate*) wszystkie możliwe wartości, stąd nazwa wyliczeń/enumeracji.

Dany adres IP może być albo w wersji czwartej albo w szóstej, ale nigdy w obu naraz.
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

Proszę zauważyć, że warianty wyliczenia dostępne są w przestrzeni jego nazwy, a więc korzystamy z dwóch dwukropków pomiędzy nazwą enuma a jego wariantem. To jest przydatne, bo teraz zarówno wartość `IpAddrKind::V4`, jak i `IpAddrKind::V6` mają ten sam typ: `IpAddrKind`. A co za tym idzie, możemy napisać funkcję przyjmującą jako argument dowolny `IpAddrKind`.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

tę funkcję możemy wywołać z dowolnym wariantem:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

Enumeracje mają jeszcze więcej zalet. Przyjrzyjmy się naszemu typowi adresu IP dokładniej.
Na chwilę obecną nie jesteśmy w stanie przechować samego adresu IP, czyli jego *danych*;
możemy przechować jedynie jego *rodzaj*. Skoro dopiero co w rozdziale 5 poznaliśmy struktury,
moglibyśmy pokusić się by ich użyć, tak jak pokazano na listingu 6-1.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-01/src/main.rs:here}}
```

<span class="caption">Listing 6-1: Przechowywanie danych i wariantu `IpAddrKind`
adresu IP przy użyciu `struktury`</span>

Zdefiniowaliśmy strukturę `IpAddr` mającą dwa pola: `kind` (ang. rodzaj) typu `IpAddrKind` (wyliczenie zdefiniowane przez nas wcześniej) oraz `address` przechowującą wartość typu `String`.
Stworzyliśmy dwie instancje tej struktury.
Pierwsza, `home`, do pola `kind` ma przypisaną wartość `IpAddrKind::V4`, zaś do `address` wartość `127.0.0.1`.
Druga instancja, `loopback` ma inny wariant `IpAddrKind` jako wartość pola `kind`,
ta wartość wynosi `V6`; oraz jako adres przypisany ma String `::1`.
Tym samym użyliśmy struktury aby zgrupować wartości `kind` i `address`, dzięki czemu typ adresu i sam adres są ze sobą powiązane.

Jednakże, tę samą myśl jesteśmy w stanie wyrazić zwięzłej za pomocą samego enuma:
zamiast wstawiając enuma do struktury, możemy umieścić dane w każdym z wariantów enuma.
Ta nowa definicja enuma `IpAddr` zawiera zarówno w wariancie `V4` jak i `V6` nową wartość o typie `String`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-02-enum-with-data/src/main.rs:here}}
```

Bezpośrednio dołączamy dane do każdego wariantu enuma, więc dodatkowa struktura staje się niepotrzebna.
Tutaj łatwo można też dostrzec inny szczegół działania enuma: nazwa każdego jego wariantu, jest również funkcją konstruującą instancję enuma. Czyli, `IpAddr::V4()` jest wywołaniem funkcji, która przyjmuje argument `String` i zwraca instancję typu `IpAddr`. Ta funkcja konstruującą jest zdefiniowana automatycznie.

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
definicję czekającą tylko na to, aby jej użyć!][IpAddr]<!-- ignore -->
Spójrzmy na definicję `IpAddr` w bibliotece standardowej: ma dokładnie taką samą nazwę
i takie sama warianty, ale przechowuje dane o adresach za pomocą dwóch różnych struktur,
zdefiniowanych osobno oraz umieszczonych w wariantach wyliczenia.

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

Jak demonstruje powyższy wycinek kodu, w wariantach enuma można umieścić każdy typ danych, np.: ciąg znaków (string), typ liczbowy, lub strukturę.
W enumie można umieścić nawet innego enuma!
Ponadto, typy w standardowej bibliotece często nie są wcale bardziej skomplikowane od wymyślonych samodzielnie.

Mimo że standardowa biblioteka definiuje własny `IpAddr`, nadal możemy
stworzyć i używać własnej definicji bez żadnych konfliktów, bo nie zaimportowaliśmy
definicji z biblioteki standardowej do zasięgu (ang. scope).
Więcej informacji o importowaniu typów do zasięgu zawiera rozdział 7.

Spójrzmy na innego enuma, na przykładzie listingu 6-2: ten w swoich wariantach zawierał będzie
wiele różnych typów.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

<span class="caption">Listing 6-2: Enum `Message`, którego warianty zawierają
różne ilości i typy wartości</span>

Ten enum definiuje cztery warianty, każdy z innymi typami:

* `Quit`, nie zawiera w sobie żadnych danych.
* `Move` zawiera nazwane pola, zupełnie jak struktura.
* `Write` zawiera w sobie jeden `String`.
* `ChangeColor` zawiera trzy wartości o typach `i32`.

Na przykładzie enuma w listingu 6-2 widzimy, że definiowanie wariantów jest podobne do definiowania
kilku struktur, z taką różnicą, że nie używamy słowa kluczowego `struct` oraz, że
wszystkie warianty zgrupowane są w typie `Message`.
Poniższe struktury mogą zawierać te same dane co powyższe typy enuma:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

Ale jeśli użylibyśmy różnych struktur, to, w przeciwieństwie do enuma, każda z nich miałaby inny typ.
Zdefiniowanie funkcji mogącej przyjąć jako parametr
różne rodzaje wiadomości, nie byłoby tak proste jak przy użyciu 
enuma `Message` zdefiniowanego w listingu 6-2.

Jest jeszcze jedno podobieństwo między wyliczeniami, a strukturami: tak jak
można zdefiniować metody dla struktur używając bloku `impl`, można je też zdefiniować
dla typu wyliczeniowego. Spójrzmy na metodę o nazwie `call` zdefiniowaną na naszym enumie `Message`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

Ciało metody użyje wartości `self`, aby uzyskać wariant enuma, na którym ta metoda została wywołana.
W tym przykładzie przypisaliśmy do zmiennej `m` wartość
`Message::Write(String::from("witaj"))`, która, w wywołaniu `m.call()`
stanie się parametrem `self`, dostępnym w ciele metody `call`.

Spójrzmy na kolejne wyliczenie z biblioteki standardowej,
które jest bardzo przydatne i często używane, czyli `Option`.

### Wyliczenie `Option` i jego przewagi nad wartościami Null

W tej sekcji znajduje się analiza typu `Option`, kolejnego enuma z biblioteki standardowej.
Typ `Option` używany jest w wielu miejscach, ponieważ opisuje bardzo częsty scenariusz, w którym dana wartość może być zarówno czymś albo niczym.

Na przykład, jeśli zażądamy pierwszego elementu niepustej listy, to otrzymamy jego wartość. Jeśli zażądamy pierwszego elementu pustej listy, nie otrzymamy nic.
Wyrażenie tej koncepcji za pomocą systemu typów Rusta sprawia, że kompilator jest w stanie sprawdzić, czy wzięliśmy pod uwagę wszystkie przypadki, co pozwala zapobiegać błędom (bugom) pojawiającym się bardzo często w innych językach programowania.  

Przez konstrukcję języka programowania często rozumie się decyzje o zawarciu w nim jakichś funkcjonalności.
Ale równie istotne jest to, jakie funkcjonalności się w nim nie znalazły.
Rust nie ma wartości `null` znanej z wielu innych języków.
*Null* to wartość oznaczająca brak wartości - jest to wartość pusta.
W językach z pustymi wartościami, zmienne zawsze mogą być w jednym z dwóch stanów: null lub nie-null.

W swojej prezentacji z 2009 roku "Puste referencje: Błąd warty miliard dolarów" 
(oryg. „Null References: The Billion Dollar Mistake,”),
Tony Hoare, wynalazca nulla, powiedział:

> Ten błąd warty jest miliard dolarów. W tamtych czasach projektowałem pierwszy kompleksowy system typów
> referencji dla języków obiektowych. Moim celem było zapewnienie gwarancji, że każde użycie referencji byłoby całkowicie
> bezpieczne, co automatycznie sprawdzałby kompilator.
> Ale nie mogłem oprzeć się pokusie implementacji pustych referencji, z prostej przyczyny:
> było to łatwe do zaimplementowania. Ta decyzja doprowadziła do tylu niezliczonych błędów, luk i awarii systemów,
> że łącznie przez ostatnie czterdzieści lat pewnie spowodowała ból i szkody warte miliard dolarów.

Problem z pustymi wartościami polega na tym, że próba użycia nulla tak, jak gdyby nie był nullem, spowoduje błąd.
Ponieważ ta właściwość, null lub nie-null, jest wszechobecna, niezwykle łatwo jest popełnić ten rodzaj błędu.

Jednak koncepcja, którą null próbuje wyrazić, jest przydatna:
null oznacza że wartość jest obecnie nieważna lub nieobecna.

Problemem nie jest sam pomysł, ale ta konkretna implementacja pustych wartości.
Rust nie ma jako takich pustych wartości null, ale istnieje w Ruście wyliczenie,
które wyraża pojęcie obecności lub braku obecności danej wartości.
Tym wyliczeniem jest `Option<T>` zdefiniowane przez bibliotekę standardową][option]<!-- ignore -->
w następujący sposób:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

Enum `Option<T>` jest tak przydatny, że znajduje się w prelude;
nie trzeba samemu go importować.  Ponadto, to samo dotyczy jego wariantów:
`Some` i `None` można użyć bez prefiksu `Option::`.
Enum `Option<T>` jest zwykłym wyliczeniem, a `Some(T)` oraz `None`
to nadal zwykłe warianty `Option<T>`.

Składnia `<T>` jest funkcjonalnością Rusta, której jeszcze nie omówiliśmy.
Jest to tzw. parametr generyczny. Bardziej szczegółowo omówimy je w rozdziale 10.
Póki co, wszystko co musisz o nich wiedzieć to to, że `<T>` oznacza, że wariant `Some`
enuma `Option` może zawierać w sobie jedną wartość dowolnego typu.
Co więcej, `Option<T>` jest różnych typów dla różnych, konkretnych typów `T`.
Oto niektóre przykłady używania wartości `Option` do przechowywania typów liczbowych oraz łańcuchowych (stringów):

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-06-option-examples/src/main.rs:here}}
```

Zmienna `some_number` jest typu `Option<i32>`, zaś `some_char` jest typu `Option<char>`, który jest innym typem.
Rust może wydedukować te typy, ponieważ określiliśmy wartość wewnątrz wariantu `Some`. W przypadku `absent_number`, Rust wymaga od nas adnotacji o całościowym type `Option`: kompilator nie może wywnioskować typu, jaki będzie posiadał wariant `Some` widząc tylko wartość `None`. Tutaj mówimy więc Rustowi, że chcemy aby `absent_number` było typu `Option<i32>`.

Widząc `Some`, wiemy że wartość jest obecna oraz że znajduje się ona w `Some`. Za to `None`, w pewnym sensie oznacza to samo co null, czyli brak prawidłowej wartości. To dlaczego `Option<T>` jest lepszy od nulla?

W skrócie, `Option<T>` i `T` (gdzie `T` może być dowolnym typem)
są różnymi typami, więc kompilator nie pozwoli nam użyć `Option<T>` tak jakby
była to prawidłowa wartość typu `T`. Na przykład, ten kod się nie skompiluje, bo próbujemy w nim
dodać do siebie wartość typu `i8` oraz `Option<i8>`:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/src/main.rs:here}}
```

Uruchamiając ten kod, otrzymamy następujący komunikat o błędzie:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/output.txt}}
```

Bezlitośnie! Ten komunikat oznacza, że Rust nie wie jak ma dodać do siebie typy `i8`
oraz `Option<i8>`, ponieważ to dwa różne typy. Kiedy w Ruście posługujemy się typem takim jak `i8`, kompilator zawsze
gwarantuje, że jest to prawidłowa wartość. Możemy być pewni swego i kontynuować kodowanie bez
sprawdzania czy dana wartość jest pusta. Jedynie kiedy posługujemy się typem `Option<i8>`
(czy jakimkolwiek innym typem zawartym wewnątrz `Option`) musimy się martwić o ewentualny brak wartości,
zaś kompilator upewni się, że uwzględniliśmy ten przypadek przed użyciem wartości.

Innymi słowy, musimy skonwertować wartość typu `Option<T>` na `T` zanim
przyjmie ona zachowania charakterystyczne dla typu `T`.
W większości przypadków pozwala to na pozbycie się jednego z najczęstszych problemów z nullem: zakładanie, że jakaś wartość istnieje, kiedy
tak na prawdę nie istnieje.

Wyeliminowanie ryzyka nieprawidłowego założenia, że dana wartość nie jest pusta, daje nam większą pewność co do napisanego kodu. Aby dana wartość mogła nie istnieć musimy wyrazić na to zgodę definiując daną wartość jako typ `Option<T>`.
Następnie, używając tej wartości, musimy jawnie obsłużyć przypadek, gdy wartość jest null.
Wszędzie, gdzie typem wartości nie jest `Option<T>`, można bezpiecznie założyć, że wartość nie jest pusta.
To była celowa decyzja projektowa dla Rust, aby ograniczyć wszechobecność null i zwiększyć bezpieczeństwo napisanego kodu.

Więc mając wartość typu `Option<T>`, jak można dostać się do wartości typu `T`
znajdującej się w wariancie `Some`? Enum `Option<T>` ma wiele przydatnych metod odpowiednich dla różnych sytuacji;
można je znaleźć w [dokumentacji][docs]<!-- ignore -->. Zapoznanie się z metodami
typu `Option<T>` jest bardzo przydatne w przygodzie z Rustem.

Zwykle, aby użyć wartości typu `Option<T>`, musimy napisać kod sprawdzający oba warianty.
Jedna część kodu będzie odpowiedzialna za wariant `Some(T)` i będzie ona miała
dostęp do wewnętrznej wartości typu `T`.
Druga część będzie odpowiedzialna za wariant `None` i ona oczywiście nie będzie miała dostępu
do wartości typu `T`. Wyrażenie `match` jest konstrukcją sterującą wykonaniem, pozwalającą na tego typu zachowanie. 
Wyrażenie `match` uruchomi różny kod w zależności od tego, który wariant ma dany enum.
I ten kod będzie będzie miał dostęp do danych znajdujących się w dopasowanym wariancie.

[IpAddr]: ../std/net/enum.IpAddr.html
[option]: ../std/option/enum.Option.html
[docs]: ../std/option/enum.Option.html
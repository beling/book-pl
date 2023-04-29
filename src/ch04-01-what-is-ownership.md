## Czym jest własność?

*Własność* to zestaw reguł, które determinują, jak program Rusta zarządza pamięcią.
Wszystkie programy muszą kontrolować sposób wykorzystywania pamięci komputera podczas swojego działania. Niektóre języki korzystają z automatycznego systemu odśmiecania (*garbage collection*), który regularnie poszukuje fragmentów pamięci, których działający program już nie używa. W innych językach, sam programista ręcznie zajmuje i zwalnia pamięć. Rust wykorzystuje trzecie podejście: pamięć jest zarządzana przez system własności, obejmujący zestaw zasad sprawdzanych przez kompilator w trakcie kompilacji. Jeśli którakolwiek z reguł zostanie naruszona, program nie skompiluje się. Jednocześnie żaden z aspektów związanych z systemem własności nie spowalnia działania programu.

Ponieważ własność jest nowym pojęciem dla wielu programistów, przyzwyczajenie się do niej zabiera trochę czasu. Dobra wiadomość jest taka, że w miarę nabywania doświadczenia z Rustem i zasadami systemu własności, rośnie też twoja zdolność do naturalnego tworzenia bezpiecznego i wydajnego kodu. Tak trzymaj!

Kiedy zrozumiesz system własności, będziesz mieć solidną podstawę ku zrozumieniu innych, unikatowych funkcjonalności Rusta. W tym rozdziale nauczysz się, czym jest własność za pomocą kilku przykładów, które skupiają się na bardzo często spotykanej strukturze danych: łańcuchach znaków (*string*).

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
> Każda dana umieszczona na stosie musi mieć znany, stały rozmiar.
> Dane, których rozmiar jest nieznany na etapie kompilacji lub może ulegać zmianie,
> muszą być przechowywane na stercie. Sterta jest mniej zorganizowana: kiedy coś
> się na niej umieszcza, należy poprosić o przydzielenie pewnego jej obszaru.
> Alokator pamięci znajduje na stercie wolne, wystarczająco duże miejsce,
> oznacza je jako będące w użyciu i zwraca *wskaźnik* zawierający adres
> wybranej lokalizacji. Proces ten nazywamy *alokacją na stercie* lub po prostu
> *alokacją*. Umieszanie danych na stosie nie jest uznawane za alokację. Ze
> względu na to, że zwrócony wskaźnik posiada znany, ustalony rozmiar, możemy
> przechować go na stosie. Jednak gdy chcemy dostać się do właściwych danych,
> musimy podążyć za wskaźnikiem.
>
> Pomyśl o byciu rozsadzanym w restauracji. Przy wejściu podajesz ilość osób
> w swojej grupie, a pracownik znajduje pusty stolik, przy którym wszyscy się
> pomieszczą i prowadzi ich na miejsce. Jeśli ktoś z twojej grupy sie spóźni,
> aby was znaleźć, może zapytać, gdzie was posadzono.
>
> Odkładanie na stosie jest szybsze od alokacji na stercie, ponieważ alokator 
> nigdy nie musi szukać miejsca na dodanie nowych danych;
> to miejsce znajduje się zawsze na szczycie stosu.
> Alokacja na stercie natomiast wymaga więcej pracy, ponieważ
> system operacyjny musi w pierwszej kolejności znaleźć wystarczająco dużo
> miejsca, aby dane się zmieściły. a następnie przeprowadzić niezbędne operacje,
> by przygotować się na następną alokację.
>
> Dostęp do danych na stercie jest wolniejszy od dostępu do danych na stosie,
> ponieważ należy je zlokalizować korzystając ze wskaźnika. Nowoczesne procesory
> działają szybciej, jeżeli nie muszą dużo skakać po pamięci. Kontynuując
> analogię, załóżmy, że kelner w restauracji zbiera zamówienia z wielu stolików.
> Bardziej wydajne jest zebranie wszystkich zamówień z jednego stolika, zanim
> przejdzie się do kolejnego. Zebranie pojedynczego zamówienia ze stolika A,
> następnie jednego ze stolika B, kolejnego znów ze stolika A i powtórnie ze
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
> stosie czy o stercie. Jednak świadomość tego, że zarządzanie danymi na
> stercie jest istotą istnienia systemu własności, pomaga wyjaśnić, dlaczego
> działa on tak, jak działa.

### Zasady systemu własności

W pierwszej kolejności przyjrzyjmy się zasadom systemu własności. Proszę mieć je na
uwadze, kiedy będziemy omawiać ilustrujące je przykłady:

* Każda wartość w Ruście ma *właściciela*.
* W danym momencie może istnieć tylko jeden właściciel.
* Kiedy sterowanie wychodzi poza zasięg właściciela, wartość zostaje zwolniona.

### Zasięg zmiennych

Teraz, kiedy znamy już podstawy składni, nie będziemy umieszczać w treści przykładów kodu `fn main() {`. Jeśli zatem przepisujesz kod na bieżąco, musisz ręcznie umieszczać zaprezentowane dalej fragmenty wewnątrz funkcji `main`. Dzięki temu, przykłady będą nieco bardziej zwięzłe, pozwalając nam skupić się na istocie sprawy zamiast na powtarzalnych frazach.

W pierwszym przykładzie systemu własności, przyjrzymy się *zasięgowi* kilku zmiennych. Zasięgiem elementu nazywamy obszar programu, wewnątrz którego dany element zachowuje ważność (istnieje). Powiedzmy, że mamy zmienną, która wygląda tak:

```rust
let s = "witaj";
```

Zmienna `s` odnosi się do literału łańcuchowego, którego wartość jest ustalona w samym kodzie programu. Zmienna zachowuje ważność od miejsca, w którym ją zadeklarowano, do końca bieżącego *zasięgu*. Listing 4-1 zawiera komentarze
wyjaśniające, gdzie zmienna `s` zachowuje ważność.

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-01/src/main.rs:here}}
```

<span class="caption">Listing 4-1: Zmienna, oraz zasięg, w którym zachowuje ona ważność</span>

Innymi słowy, mamy do czynienia z dwoma istotnymi momentami w czasie:

* Kiedy zmienna `s` wchodzi *w* zasięg, zyskuje ważność.
* Zmienna pozostaje ważna, dopóki nie wyjdzie *z* zasięgu.

Na tę chwilę zależność między zasięgiem a ważnością zmiennych jest podobna do
sytuacji w innych językach programowania. Posłużymy się tą wiedzą, wprowadzając
nowy typ danych: `String` (*łańcuch znaków*).

### Typ `String`

Aby zilustrować zasady systemu własności, potrzebujemy typu danych, który jest
bardziej złożony od tych, które omawiane były w sekcji
[„Typy danych”][data-types]<!-- ignore --> rozdziału 3. Wszystkie opisane tam
typy przechowywane są na stosie i są z niego zdejmowane, kiedy skończy się ich
zasięg. Potrzebny jest nam natomiast typ przechowujący zawarte w nim dane na
stercie. Dowiemy się wówczas, skąd Rust wie, kiedy te dane usunąć.

W przykładzie użyjemy typu `String`, koncentrując się na tych jego elementach,
które odnoszą się do systemu własności. Te same elementy mają znaczenie dla
innych złożonych typów, które dostarcza biblioteka standardowa oraz tych, które
stworzysz sam. Typ `String` omawiany będzie dogłębnie w [rozdziale 8][ch8].

Widzieliśmy już literały łańcuchowe, których dane na stałe umieszczone są w treści
programu. Takie zmienne są wygodne w użyciu, ale nieprzydatne w wielu
sytuacjach, w których używa się danych tekstowych. Jednym z powodów jest to, że
są one niemodyfikowalne. Innym, że nie każda zawartość łańcucha tekstowego jest
znana podczas pisania programu. Na przykład: co zrobić, jeśli chcemy pobrać dane
od użytkownika i je przechować? Dla takich sytuacji Rust przewiduje drugi typ
łańcuchowy: `String`. Typ ten alokowany jest na stercie i z tego względu może
przechowywać dane, których ilość jest nieznana podczas kompilacji. Można
przekształcić niemodyfikowalny literał łańcuchowy w zmienną typu `String` za pomocą
funkcji `from`. Wygląda to tak:

```rust
let s = String::from("witaj");
```

Podwójny dwukropek `::` jest operatorem umożliwiającym wykorzystanie funkcji
`from` z przestrzeni nazw typu `String`, zamiast konieczności utworzenia
ogólnej funkcji o przykładowej nazwie `string_from`. Ten rodzaj składni będzie
szerzej omawiany w sekcji [„Składnia metod”][method-syntax]<!-- ignore --> w
rozdziale 5 oraz podczas rozważań o przestrzeniach nazw modułów w sekcji
[„Ścieżki odnoszenia się do elementów w hierarchii modułów”][paths-module-tree]<!-- ignore -->
w rozdziale 7.

Ten rodzaj łańcucha znaków *można* modyfikować:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-01-can-mutate-string/src/main.rs:here}}
```

Jaka jest zatem różnica? Dlaczego `String` może być modyfikowalny, a literał
nie? Różnica polega na sposobie, w jakim oba te typy korzystają z pamięci.

### Pamięć i alokacja

W przypadku literału łańcuchowego, jego wartość znana jest już w czasie kompilacji,
więc przechowywany tekst jest na stałe zakodowany w docelowym pliku
wykonywalnym, co czyni literały szybkimi i wydajnymi. Ale cechy te wynikają z
niemodyfikowalności literałów. Niestety, nie możemy w pliku binarnym umieścić
bańki pamięci pod każdy potrzebny tekst, którego rozmiar jest nieznany podczas
kompilacji i może się zmienić w trakcie działania programu.

Mając typ `String`, w celu obsługi modyfikowalnego i potencjalnie rosnącego
tekstu, musimy zaalokować pewną ilość pamięci na stercie, nieznaną podczas
kompilacji. To oznacza, że:

* O przydział pamięci należy poprosić alokator w trakcie wykonywania programu.
* Potrzebny jest sposób na oddanie pamięci do alokatora, kiedy `String` nie będzie już potrzebny.

Pierwszą część robimy sami, wywołując funkcję `String::from`, której
implementacja zawiera prośbę o wymaganą pamięć. Podobne rozwiązanie jest w
w wielu innych językach programowania.

Druga część znacznie się za to różni. W językach wyposażonych w systemy
odśmiecania (*garbage collector - GC*), GC śledzi i zwalnia pamięć, która nie
jest już używana, a my nie musimy już o tym myśleć. W językach pozbawionych GC,
naszą odpowiedzialnością jest identyfikowanie nieużywanej już pamięci i
bezpośrednie wywoływanie zarówno kodu, który tę pamięć zwalnia, jak i tego,
który ją alokuje. Poprawne wykonanie tej operacji stanowiło historycznie trudny,
programistyczny problem. Jeśli zapomnimy, marnujemy pamięć. Jeśli zrobimy to za
wcześnie, zostaniemy z unieważnioną zmienną. Zrobimy to dwukrotnie - to też błąd.
Musimy połączyć w pary dokładnie jedną `alokację` z dokładnie jednym
`zwolnieniem`.

Rust prezentuje inne podejście: pamięć jest automatycznie zwalniana,
kiedy skończy się zasięg zmiennej będącej jej właścicielem. Oto wersja naszego
przykładu z listingu 4-1, który używa typu `String` zamiast literału:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-02-string-scope/src/main.rs:here}}
```

Istnieje naturalny moment, w którym można oddać pamięć wykorzystywaną przez
nasz `String` do alokatora - kiedy kończy się zasięg zmiennej `s`.
Kiedy zasięg jakiejś zmiennej się kończy, Rust wywołuje za nas specjalną
funkcję. Funkcja ta nosi nazwę [`drop`][drop]<!-- ignore --> (*porzuć, upuść*), a w jej treści autor typu
`String` umieścił kod zwalniający pamięć. Funkcja `drop` zostaje wywołana przez
Rusta automatycznie, przy klamrze zamykającej.

> Uwaga: W C++ schemat dealokacji zasobów przy końcu czasu życia jakiegoś
> elementu jest czasem nazywany *Inicjowaniem Przy Pozyskaniu Zasobu* (*Resource
> Acquisition Is Initialization (RAII)*). Funkcja `drop` z Rusta może wydać
> się znajoma osobom, które miały styczność ze schematami RAII.

Schemat ten ma ogromny wpływ na sposób pisania kodu w Ruście. Na tym etapie może
wydawać się to proste, ale program może zachować się niespodziewanie w bardziej
złożonych przypadkach, kiedy chcemy, aby kilka zmiennych używało tej samej
danej, alokowanej na stercie. Zbadajmy teraz kilka takich sytuacji.

<!-- Old heading. Do not remove or links may break. -->
<a id="ways-variables-and-data-interact-move"></a>

#### Przenoszenie Zmiennych i Danych

Kilka zmiennych może w Ruście odnosić się do tej samej danej na różne sposoby.
Spójrzmy na przykład w listingu 4-2, z wykorzystaniem liczby całkowitej:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-02/src/main.rs:here}}
```

<span class="caption">Listing 4-2: Przypisanie całkowitej wartości zmiennej `x`
do zmiennej `y`</span>

Z całą pewnością możemy odgadnąć, co ten kod robi: „przypisuje `5` do `x`,
a następnie wykonuje kopię wartości przechowywanej w `x` i przypisuje ją do
`y`.”. Mamy teraz dwie zmienne: `x` i `y`, obie o wartości `5`. Dzieje się
dokładnie tak, ponieważ liczby całkowite są prostymi wartościami o
stałym rozmiarze, więc obie wartości `5` zostają odłożone na stos.

Teraz przyjrzyjmy się wersji z typem `String`:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-03-string-move/src/main.rs:here}}
```

Wygląda to bardzo podobnie do wcześniejszego kodu, więc możemy zakładać, że jego
działanie też będzie podobne: w drugiej linii powstaje kopia wartości w `s1` i
zostaje ona przypisana do `s2`. Ale tak się akurat nie dzieje.

Rysunek 4-1 objaśnia, co dzieje się we wnętrzu typu `String`. Typ `String`
składa się z trzech części, pokazanych po lewej stronie. Są to: wskaźnik do
pamięci przechowującej właściwy łańcuch znaków, znacznik jego długości
(*length*) i dane o ilości pamięci dostępnej dla danego ciągu (*capacity*). Ta
grupa danych przechowywana jest na stosie. Po prawej pokazano obszar pamięci na
stercie, który zawiera tekst.

<img alt="Two tables: the first table contains the representation of s1 on the
stack, consisting of its length (5), capacity (5), and a pointer to the first
value in the second table. The second table contains the representation of the
string data on the heap, byte by byte." src="img/trpl04-01.svg" class="center"
style="width: 50%;" />

<span class="caption">Rysunek 4-1: Reprezentacja pamięci dla typu `String`
przechowującego wartość `"witaj"` przypisaną do `s1`</span>

`Length` (długość) wskazuje, ile bajtów pamięci zajmuje bieżący ciąg znaków w
zmiennej typu `String`, natomiast `capacity` (pojemność) przechowuje dane o całkowitej
ilości pamięci, jaką alokator dla tej zmiennej przydzielił. Różnica
między długością i pojemnością ma znaczenie, ale nie w tym kontekście. Dlatego na
razie możemy zignorować pojemność.

Kiedy przypisujemy `s1` do `s2`, dane ze zmiennej typu `String` zostają
skopiowane. Dotyczy to przechowywanych na stosie: wskaźnika, długości i
pojemności. Dane tekstowe, do których odnosi się wskaźnik nie są kopiowane.
Innymi słowy, reprezentację pamięci w tej sytuacji ilustruje Rysunek 4-2.

<img alt="Three tables: tables s1 and s2 representing those strings on the
stack, respectively, and both pointing to the same string data on the heap."
src="img/trpl04-02.svg" class="center" style="width: 50%;" />

<span class="caption">Rysunek 4-2: Reprezentacja w pamięci zmiennej `s2`
posiadającej kopię wskaźnika i znaczników długości i pojemności zmiennej `s1`</span>

Rysunek 4-3 ukazuje *nieprawdziwą* reprezentację pamięci, w której Rust również
skopiował dane na stercie. Gdyby taka sytuacja miała miejsce, operacja
`s2 = s1` mogłaby potencjalnie zająć dużo czasu, w przypadku sporej ilości
danych na stercie.

<img alt="Cztery tabele: dwie tabele reprezentują dane trzymane na stosie przez s1 i s2,
i każda wskazuje na swoją własną kopię łańcucha na stercie."
src="img/trpl04-03.svg" class="center" style="width: 50%;" />

<span class="caption">Rysunek 4-3: Hipotetyczny wynik operacji `s2 = s1`, gdyby
Rust również kopiował dane sterty</span>

Wcześniej powiedzieliśmy, że kiedy zasięg zmiennej się kończy, Rust wywołuje
automatycznie funkcję `drop` i zwalnia obszar na stercie dla tej zmiennej. Ale
na Rysunku 4-2 przedstawiono sytuację, w której oba wskaźniki wskazują na ten
sam obszar. Jest to problematyczne: kiedy zasięg `s2` i `s1` się skończy,
nastąpi próba dwukrotnego zwolnienia tej samej pamięci. Sytuacja ta jest znana
jako *błąd podwójnego zwolnienia* i należy do grupy bugów bezpieczeństwa
pamięci, o których wcześniej wspomnieliśmy. Podwójne zwalnianie pamięci może
prowadzić do jej *zepsucia*, a w efekcie do potencjalnych luk bezpieczeństwa.

Aby zapewnić bezpieczeństwo pamięci, po linii `let s2 = s1;`, zamiast próbować skopiować zaalokowaną pamięć, Rust
traktuje zmienną `s1` jako unieważnioną i, tym samym, nie musi nic zwalniać, kiedy zasięg `s1` się kończy. Zobaczmy, co stanie się przy próbie użycia zmiennej `s1` po utworzeniu zmiennej `s2`. Próba się nie powiedzie:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/src/main.rs:here}}
```

Rust zwróci poniższy błąd, ponieważ nie zezwala na odnoszenie się do elementów przy użyciu unieważnionych zmiennych:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/output.txt}}
```

Jeśli słyszałeś terminy „płytka kopia” oraz „głęboka kopia” pracując
z innymi językami, to z pewnością wiesz, że skopiowanie wskaźnika ze znacznikami długości
i pojemności, ale bez kopiowania danych, przypomina tworzenie płytkiej kopii.
Ale ponieważ Rust jednocześnie unieważnia źródłową zmienną, zamiast nazywać taki
proces płytką kopią, używa się terminu *przeniesienie*. W tym przypadku
moglibyśmy powiedzieć, że zmienna `s1` została *przeniesiona* do `s2`. Rysunek
4-4 ilustruje, co tak naprawdę dzieje się w pamięci.

<img alt="Three tables: tables s1 and s2 representing those strings on the
stack, respectively, and both pointing to the same string data on the heap.
Table s1 is grayed out be-cause s1 is no longer valid; only s2 can be used to
access the heap data." src="img/trpl04-04.svg" class="center" style="width:
50%;" />

<span class="caption">Rysunek 4-4: Reprezentacja w pamięci po unieważnieniu
zmiennej `s1`</span>

To rozwiązuje nasz problem! Jeśli jedynie zmienna `s2` zachowuje ważność, to w
momencie wyjścia z zasięgu, jako jedyna zwolni zajmowaną pamięć i po sprawie.

Dodatkowo, implikuje to decyzję w budowie języka: Rust nigdy automatycznie nie
tworzy „głębokich” kopii twoich danych. Można zatem założyć, że *automatyczny*
proces kopiowania nie będzie drogą operacją w sensie czasu jej trwania.

<!-- Old heading. Do not remove or links may break. -->
<a id="ways-variables-and-data-interact-clone"></a>

#### Klonowanie zmiennych i danych

W przypadku gdy *chcemy* wykonać głęboką kopię danych ze sterty dla typu
`String`, a nie tylko danych ze stosu, możemy skorzystać z często stosowanej
metody o nazwie `clone` (*klonuj*). Składnia metod będzie omawiana w rozdziale
5, ale ponieważ metody są popularnymi funkcjonalnościami wielu języków, zapewne
już je wcześniej widziałeś.

Oto przykład działania metody `clone`:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-05-clone/src/main.rs:here}}
```

Ten przykład działa bez problemu i ilustruje on celowe odtworzenie zachowania
pokazanego na Rysunku 4-3, na którym dane ze sterty *są* kopiowane.

Kiedy widzisz odwołanie do metody `clone`, możesz się spodziewać, że wykonywana
operacja będzie kosztowna czasowo.

#### Dane przechowywane wyłącznie na stosie: Copy (*kopiowanie*)

Jest jeszcze jeden szczegół, którego nie omówiliśmy. Kod korzystający z liczb
całkowitych, którego treść pokazano na listingu 4-2, działa i jest prawidłowy:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-06-copy/src/main.rs:here}}
```

Zdaje się on przeczyć temu, czego przed chwilą się nauczyliśmy: nie mamy
wywołania `clone`, ale zmienna `x` zachowuje ważność i nie zostaje
przeniesiona do `y`.


Przyczyną jest to, że typy takie jak liczby całkowite, które mają znany rozmiar
już podczas kompilacji, są w całości przechowywane na stosie. Tworzenie kopii
ich wartości jest więc szybkie. To oznacza, że nie ma powodu unieważniać zmiennej
`x` po stworzeniu zmiennej `y`. Innymi słowy, w tym wypadku nie ma różnicy
między głęboką i płytką kopią, więc wywołanie metody `clone` nie różniłoby się
od zwykłego płytkiego kopiowania i można je zatem pominąć.

Rust zawiera specjalną adnotację zwaną „cechą `Copy`”, którą można
zaimplementować dla typów przechowywanych na stosie, takich jak liczby
całkowite (więcej o cechach będzie w [rozdziale 10][traits]<!-- ignore -->). Jeśli dany typ ma
zaimplementowaną cechę `Copy`, zmienną, którą przypisano do innej zmiennej,
można dalej używać.

Rust nie pozwoli dodać adnotacji `Copy` do żadnego typu, dla którego zaimplementowano (lub zaimplementowano dla jakiejkolwiek jego części tego typu) cechę `Drop`. Jeśli typ wymaga
wykonania konkretnych operacji po tym, jak reprezentującej go zmiennej kończy
się zasięg, a dodamy dla tego typu cechę `Copy`, uzyskamy błąd kompilacji. Aby
nauczyć się, jak implementować cechę `Copy` dla danego typu, zajrzyj do
[„Cechy wyprowadzalne”][derivable-traits]<!-- ignore --> w Dodatku C.

Które więc typy mają cechę `Copy`? Dla danego typu można dla pewności sprawdzić
w dokumentacji, ale jako regułę zapamiętaj, że każda grupa wartości skalarnych
może mieć cechę `Copy` i nic, co wymaga alokacji lub jest pewnego rodzaju
zasobem jej nie ma. Oto przykłady typów z zaimplementowaną cechą `Copy`:

* Wszystkie typy całkowite, takie jak `u32`.
* Typ logiczny, `bool`, z wartościami `true` oraz `false`.
* Wszystkie typy zmiennoprzecinkowe, jak `f64`.
* Typ znakowy, `char`.
* Krotki, jeśli zawierają wyłącznie typy z cechą `Copy`. Na
  przykład, `(i32, i32)` ma cechę `Copy`, ale `(i32, String)` już nie.

### Własność i funkcje

Mechanika przekazywania wartości do funkcji jest podobna do przypisania wartości
do zmiennej. Przekazanie zmiennej do funkcji przeniesie ją lub skopiuje, tak jak
przy przypisywaniu. Listing 4-3 ukazuje przykład z kilkoma adnotacjami
ilustrującymi, kiedy zaczynają się lub kończą zasięgi zmiennych:

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-03/src/main.rs}}
```

<span class="caption">Listing 4-3: Funkcje z adnotacjami dotyczącymi własności i
zasięgów</span>

Gdybyśmy spróbowali użyć `s` po wywołaniu `bierze_na_wlasnosc`, Rust
wygenerowałby błąd kompilacji. Te statyczne kontrole chronią nas przed
popełnianiem błędów. Spróbuj dodać do `main` kod, który używa zmiennych `s` oraz
`x`, żeby zobaczyć, gdzie można ich używać, a gdzie zasady systemu własności nam
tego zabraniają.

### Wartości zwracane i ich zasięg

Wartości zwracane mogą również przenosić własność. Listing 4-4 ilustruje
przykład z podobnymi komentarzami do tych z listingu 4-3.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-04/src/main.rs}}
```

<span class="caption">Listing 4-4: Przenoszenie własności wartości
zwracanych</span>

Własność zmiennej zachowuje się zawsze w ten sam sposób: przypisanie wartości do
innej zmiennej przenosi tę wartość. Kiedy kończy się zasięg zmiennej
zawierającej dane ze sterty, dane te zostaną zwolnione przez `drop`, chyba że
przekażemy je na własność innej zmiennej.

Przyjmowanie własności, a następnie oddawanie jej przy wywołaniu każdej funkcji
jest trochę pracochłonne. A co jeśli chcemy zezwolić funkcji na użycie wartości,
ale nie chcemy, by przejęła ją na własność? To dość denerwujące, kiedy wszystko,
co przekazujemy, musi zostać powtórnie zabrane, jeśli chcemy tego ponownie użyć.
Nie mówiąc już o danych generowanych przy okazji normalnego działania funkcji,
które być może także chcielibyśmy zwrócić.

Z funkcji można zwrócić kilka wartości za pomocą krotki. Listing 4-5 ilustruje
ten przypadek.

<span class="filename">Plik: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-05/src/main.rs}}
```

<span class="caption">Listing 4-5: Zwracanie własności przez argumenty</span>

Wymaga to dużo niepotrzebnej pracy, podczas gdy koncept ten spotykany jest powszechnie. Na szczęście dla nas, Rust wyposażony jest w *referencje*, które świetnie obsługują takie przypadki.

[data-types]: ch03-02-data-types.html#typy-danych
[ch8]: ch08-02-strings.html
[traits]: ch10-02-traits.html
[derivable-traits]: appendix-03-derivable-traits.html
[method-syntax]: ch05-03-method-syntax.html#składnia-metod
[paths-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[drop]: ../std/ops/trait.Drop.html#tymethod.drop

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
> Każda dana umieszczona na stosie musi mieć znany, ustalony z góry rozmiar.
> Dane, których rozmiar jest nieznany na etapie kompilacji lub ulega zmianie,
> muszą być przechowywane na stercie. Sterta jest mniej zorganizowana: kiedy coś
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
> Odkładanie na stosie jest szybsze od alokacji na stercie, ponieważ nigdy nie
> trzeba szukać miejsca na dodanie nowych danych; to miejsce znajduje się zawsze
> na szczycie stosu. Alokacja na stercie natomiast wymaga więcej pracy, ponieważ
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

Zmienna `s` odnosi się do literału znakowego, którego wartość jest ustalona w
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
bardziej złożony od tych, które omawiane były w sekcji
[„Typy danych”][data-types]<!-- ignore --> w Rozdziale 3. Wszystkie opisane tam
typy przechowywane są na stosie i są z niego zdejmowane, kiedy skończy się ich
zasięg. Potrzebny jest nam natomiast typ przechowujący zawarte w nim dane na
stercie . Dowiemy się wówczas, skąd Rust wie, kiedy te dane usunąć.

W przykładzie użyjemy typu `String`, koncentrując się na tych jego elementach,
które odnoszą się do systemu własności. Te same elementy mają znaczenie dla
innych złożonych typów, które dostarcza biblioteka standardowa oraz tych, które
stworzysz sam. Typ `String` omawiany będzie dogłębnie w Rozdziale 8.

Widzieliśmy już literały znakowe, których dane na stałe umieszczone są w treści
programu. Takie zmienne są wygodne w użyciu, ale nieprzydatne w wielu
sytuacjach, w których używa się danych tekstowych. Jednym z powodów jest to, że
są one niemodyfikowalne. Innym, że nie każda zawartość łańcucha tekstowego jest
znana podczas pisania programu. Na przykład: co zrobić, jeśli chcemy pobrać dane
od użytkownika i je przechować? Dla takich sytuacji Rust przewiduje drugi typ
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
szerzej omawiany w sekcji [„Składnia metod”][method-syntax]<!-- ignore --> w
Rozdziale 5 oraz podczas rozważań o przestrzeniach nazw modułów w sekcji
[„Ścieżki odnoszenia się do elementów w hierarchii modułów”][paths-module-tree]<!-- ignore -->
w Rozdziale 7.

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
jest już używana, a my nie musimy już o tym myśleć. W językach pozbawionych GC,
naszą odpowiedzialnością jest identyfikowanie nieużywanej już pamięci i
wywoływanie bezpośrednio kodu, który tę pamięć zwalnia - tak samo, jak kodu,
który ją alokuje. Poprawne wykonanie tej operacji stanowiło historycznie trudny,
programistyczny problem. Jeśli zapomnimy, marnujemy pamięć. Jeśli zrobimy to za
wcześnie, zostaniemy z nieważną zmienną. Zrobimy to dwukrotnie - to też błąd.
Musimy połączyć w pary dokładnie jedną `alokację` z dokładnie jednym
`zwolnieniem`.

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

#### Metody interakcji między zmiennymi a danymi: Move (*przeniesienie*)

Kilka zmiennych może w Ruście odnosić się do tej samej danej na różne sposoby.
Spójrzmy na przykład w Listingu 4-2, z wykorzystaniem liczby całkowitej:

```rust
let x = 5;
let y = x;
```

<span class="caption">Listing 4-2: Przypisanie całkowitej wartości zmiennej `x`
do zmiennej `y`</span>

Z całą pewnością możemy odgadnąć, co ten kod robi: „Przypisuje wartość `5` do
`x`, a następnie robi kopię wartości przechowywanej w `x` i przypisuje ją do
`y`.”. Mamy teraz dwie zmienne: `x` i `y`, obie o wartości `5`. Dzieje się
dokładnie tak, ponieważ zmienne dla liczb całkowitych są prostymi elementami o
znanym, ustalonym rozmiarze, więc obie wartości `5` zostają odłożone na stos.

Teraz przyjrzyjmy się wersji z typem `String`:

```rust
let s1 = String::from("hello");
let s2 = s1;
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

<img alt="String w pamięci" src="img/trpl04-01.svg" class="center" style="width: 50%;" />

<span class="caption">Rysunek 4-1: Reprezentacja pamięci dla typu `String`
przechowującego wartość `"hello"` przypisaną do `s1`</span>

Znacznik `length` wskazuje, ile bajtów pamięci zajmuje bieżący ciąg znaków w
zmiennej typu `String`, natomiast `capacity` przechowuje dane o całkowitej
ilości pamięci, jaką system operacyjny dla tej zmiennej przydzielił. Różnica
między `length` i `capacity` ma znaczenie, ale nie w tym kontekście. Dlatego na
razie możemy zignorować `capacity`.

Kiedy przypisujemy `s1` do `s2`, dane ze zmiennej typu `String` zostają
skopiowane. Dotyczy to przechowywanych na stosie: wskaźnika, długości i
pojemności. Dane tekstowe, do których odnosi się wskaźnik nie są kopiowane.
Innymi słowy, reprezentację pamięci w tej sytuacji ilustruje Rysunek 4-2.

<img alt="s1 i s2 wskazujące tę samą wartość" src="img/trpl04-02.svg" class="center" style="width: 50%;" />

<span class="caption">Rysunek 4-2: Reprezentacja w pamięci zmiennej `s2`
posiadającej kopię wskaźnika i znaczników długości i pojemności zmiennej `s1`</span>

Rysunek 4-3 ukazuje *nieprawdziwą* reprezentację pamięci, w której Rust również
skopiował dane na stercie. Gdyby taka sytuacja miała miejsce, operacja
`s2 = s1` mogłaby potencjalnie zająć dużo czasu, w przypadku sporej ilości
danych na stercie.

<img alt="s1 i s2 wskazujące do dwóch obszarów" src="img/trpl04-03.svg" class="center" style="width: 50%;" />

<span class="caption">Rysunek 4-3: Hipotetyczny wynik operacji `s2 = s1`, gdyby
Rust również kopiował dane sterty</span>

Wcześniej powiedzieliśmy, że kiedy zasięg zmiennej się kończy, Rust wywołuje
automatycznie funkcję `drop` i zwalnia obszar na stercie dla tej zmiennej. Ale
na Rysunku 4-2 przedstawiono sytuację, w której oba wskaźniki wskazują na ten
sam obszar. Jest to problematyczne: kiedy zasięg `s2` i `s1` się skończy,
nastąpi próba dwukrotnego zwolnienia tej samej pamięci. Sytuacja ta jest znana
jako *błąd podwójnego zwolnienia* i należy do grupy bugów bezpieczeństwa
pamięci, o których wcześniej wspomnieliśmy. Podwójne zwalnianie pamięci może
prowadzić do jej *zepsucia*, a w efekcie potencjalnie do luk w zabezpieczeniach.

Aby zapewnić bezpieczeństwo pamięci, w Ruście ma miejsce w takiej sytuacji
jeszcze jeden szczegół. Zamiast próbować skopiować zaalokowaną pamięć, Rust
traktuje zmienną `s1` jako nieważną i, tym samym, nie musi nic zwalniać, kiedy
zasięg `s1` się kończy. Zobacz, co stanie się przy próbie użycia zmiennej `s1`
po utworzeniu zmiennej `s2`. Próba się nie powiedzie:

```rust,ignore,does_not_compile
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

Rust zwróci poniższy błąd, ponieważ nie zezwala na odnoszenie się do elementów
przy użyciu nieważnych zmiennych:

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

Jeśli zdarzyło ci się słyszeć terminy „płytka kopia” oraz „głęboka kopia” przy
pracy z innymi językami, koncept kopiowania wskaźnika ze znacznikami długości
i pojemności, ale bez kopiowania danych, przypomina tworzenie płytkiej kopii.
Ale ponieważ Rust jednocześnie unieważnia źródłową zmienną, zamiast nazywać taki
proces płytką kopią, używa się terminu *przeniesienie*. W tym przypadku
moglibyśmy powiedzieć, że zmienna `s1` została *przeniesiona* do `s2`. Rysunek
4-4 ilustruje, co tak naprawdę dzieje się w pamięci.

<img alt="s1 przeniesiona do s2" src="img/trpl04-04.svg" class="center" style="width: 50%;" />

<span class="caption">Rysunek 4-4: Reprezentacja w pamięci po unieważnieniu
zmiennej `s1`</span>

To rozwiązuje nasz problem! Jeśli tylko zmienna `s2` zachowuje ważność w
momencie wyjścia z zasięgu, sama zwolni zajmowaną pamięć i po sprawie.

Dodatkowo, implikuje to decyzję w budowie języka: Rust nigdy automatycznie nie
tworzy „głębokich” kopii twoich danych. Można zatem założyć, że automatyczny
proces kopiowania nie będzie drogą operacją w sensie czasu jej trwania.

#### Metody interakcji między zmiennymi a danymi: Clone (*klonowanie*)

W przypadku jeśli *chcemy* wykonać głęboką kopię danych ze sterty dla typu
`String`, a nie tylko danych ze stosu, możemy skorzystać z często stosowanej
metody o nazwie `clone` (*klonuj*). Składnia metod będzie omawiana w Rozdziale
5, ale ponieważ metody są popularnymi funkcjonalnościami wielu języków, zapewne
już je wcześniej widziałeś.

Oto przykład działania metody `clone`:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

Ten przykład działa bez problemu i ilustruje on celowe odtworzenie zachowania
pokazanego na Rysunku 4-3, na którym dane ze sterty *są* kopiowane.

Kiedy widzisz odwołanie do metody `clone`, możesz się spodziewać, że wykonywana
operacja będzie kosztowna czasowo.

#### Dane przechowywane wyłącznie na stosie: Copy (*kopiowanie*)

Jest jeszcze jeden szczegół, którego nie omówiliśmy. Część kodu korzystającego z
liczb całkowitych, którego treść pokazano na Listingu 4-2, działa i jest
prawidłowa:

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

Zdaje się on przeczyć temu, czego przed chwilą się nauczyliśmy: nie mamy
odwołania do `clone`, ale zmienna `x` zachowuje ważność i nie zostaje
przeniesiona do `y`.

Przyczyną jest to, że typy takie jak liczby całkowite, które mają znany rozmiar
już podczas kompilacji, są w całości przechowywane na stosie. Tworzenie kopii
ich wartości jest więc szybkie. To oznacza, że nie ma powodu unieważniać zmienną
`x` po stworzeniu zmiennej `y`. Innymi słowy, w tym wypadku nie ma różnicy
między głęboką i płytką kopią, więc wywołanie metody `clone` nie różniłoby się
od zwykłego płytkiego kopiowania i można je zatem pominąć.

Rust zawiera specjalną adnotację zwaną „cechą `Copy`”, którą można
zaimplementować dla typów przechowywanych na stosie, takich jak liczby
całkowite (więcej o cechach będzie w Rozdziale 10). Jeśli dany typ ma
zaimplementowaną cechę `Copy`, zmienną, którą przypisano do innej zmiennej,
można dalej używać po tej operacji. Rust nie pozwoli zaimplementować cechy
`Copy` dla żadnego typu, dla którego całości lub jakiejkolwiek jego części
zaimplementowano wcześniej cechę `Drop`. Jeśli specyfikacja typu wymaga
wykonania konkretnych operacji po tym, jak reprezentującej go zmiennej kończy
się zasięg, a dodamy dla tego typu cechę `Copy`, wywołamy błąd kompilacji. Aby
nauczyć się, jak implementować cechę `Copy` dla danego typu, zajrzyj do
[“Derivable Traits”][derivable-traits]<!-- ignore --> w Dodatku C.

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

# Wstęp

> Uwaga: Niniejsze tłumaczenie oparto na wydaniu książki [The Rust Programming
> Language][nsprust] dostępnej w druku oraz w wersji elektronicznej, wydanych
> przez [No Starch Press][nsp].

[nsprust]: https://nostarch.com/rust
[nsp]: https://nostarch.com/

<<<<<<< HEAD
Witaj na łamach *Języka Programowania Rust* - wprowadzającej ksiażki o Ruście.
Język programowania Rust pomaga w tworzeniu szybszego, bardziej niezawodnego
oprogramowania. Konstrukcja języków stawia często wysokopoziomową ergonomię w
bezpośrednim konflikcie z niskopoziomowym stopniem kontroli. Rust rzuca wyzwanie
temu problemowi. Dzięki zrównoważeniu potężnych możliwości technicznych i
świetnego doświadczenia dla programistów, Rust daje szansę objęcia kontroli nad
niskopoziomowymi szczegółami (takimi jak zarządzanie pamięcią) bez napotykania
na problemy, które tradycyjnie takiej możliwości towarzyszyły.

## Dla kogo jest Rust

Rust jest idealnym wyborem dla wielu ludzi z wielu różnych powodów. Przyjrzyjmy
się kilku najważniejszym grupom.

### Zespoły deweloperów

Rust dowodzi swojej produktywności jako narzędzie kolaboracji wśród dużych
zespołów deweloperów, ze zróżnicowaną wiedzą na temat programowania systemowego.
Niskopoziomowy kod jest podatny na rozmaite, subtelne błędy (*bugi*), które w
większości innych języków programowania da się wyłapać jedynie poprzez rozległe
testy i ostrożne sprawdzanie kodu przez doświadczonych programistów. W Ruście
kompilator pełni rolę bramkarza, który nie dopuszcza do skompilowania kodu
zawiarającego tego typu błędy. Dotyczy to również błędów w programach
współbieżnych (*wielowątkowych*). Współpraca z kompilatorem pozwala zespołowi
spędzić więcej czasu na dopracowywaniu logiki programu niż na polowaniu na bugi.

Rust jednocześnie wnosi do świata programowania systemowego nowoczesne narzędzia
deweloperskie:

* Cargo - dołączony menedżer zależności i narzędzie do budowania, czyni
  dodawanie, kompilowanie i zarządzanie zależnościami bezbolesnym i spójnym
  procesem w obrębie ekosystemu Rusta.
* Rustfmt zapewnia spójny styl kodu wśród deweloperów.
* Serwer języka Rust zapewnia w konkretnych Zintegrowanych Środowiskach
  Programistycznych (IDE) auto-uzupełnianie oraz ostrzeżenia inline o błędach.

Korzystając z tych oraz innych narzędzi w ekosystemie Rusta, deweloperzy mogą
pozostać produktywni również podczas pisania kodu niskopoziomowego.

### Uczniowie

Rust jest językiem dla uczniów i osób zainteresowanych uczeniem się systemowych
pojęć. Z pomocą Rusta, wielu ludzi poznało tematy z zakresu rozwoju systemów
operacyjnych. Społeczność Rusta jest niezwykle gościnna i chętnie odpowiada na
pytania uczniów. Poprzez zaangażowanie w projekty takie jak niniejsza książka,
zespoły tworzące Rusta chcą uczynić pojęcia systemowe osiagalnymi dla większej
ilości osób, szczególnie tych, dla których programowanie jest czymś zupełnie
nowym.

### Firmy

W setkach dużych i małych firm używa się roboczo Rusta do różnych zadań, w
poczet których można zaliczyć: narzędzia linii poleceń, usługi sieciowe,
narzędzia w DevOps, urządzenia wbudowane, analizę i przetwarzanie audio i video,
kryptowaluty, bioinformatykę, silniki wyszukiwarek, aplikacje w Internecie
Rzeczy, uczenie maszynowe, a nawet główne komponenty przeglądarki sieciowej
Firefox.

### Programiści open source

Rust jest dla tych, którzy chcą tworzyć również sam język Rust, jego
społeczność, narzędzia deweloperskie oraz biblioteki. Z przyjemnością
ujrzelibyśmy Twój wkład w tworzenie Rusta.

### Ludzie, którzy cenią szybkość i stabilność

Rust powstał dla ludzi, którzy poszukują w języku programowania szybkości i
stabilności. Przez szybkość rozumiemy zarówno szybkość działania programów
napisanych w Ruście, jak i oferowaną przez ten język szybkość ich pisania.
Weryfikacje przez kompilator Rusta zapewniają stabilność programu podczas
refaktorowania i dodawania nowych funkcjonalności. Kontrastuje to z kruchym,
zaległym kodem, którego deweloperzy często boją się modyfikować, co ma miejsce
w językach pozbawionych podobnych mechanizmów kontroli. Dążąc do wykorzystania
bezkosztowych abstrakcji oraz wysokopoziomowych rozwiązań, które kompilują się
do kodu niskopoziomowego równie szybkiego, co ten pisany ręcznie, Rust stara
się, aby bezpieczny kod mógł być jednocześnie szybki.

Twórcy Rusta mają nadzieję wspierać również wielu innych użytkowników.
Wymienieni stanowią jedynie kilkoro z największych interesariuszy. Ogólnie rzecz
ujmując, największą aspiracją Rusta jest wyeliminowanie kompromisów, które
programiści zaakceptowali w ciągu dekad, poprzez zaoferowanie jednocześnie:
bezpieczeństwa *i* produktywności, szybkości *oraz* ergonomii. Wypróbuj Rusta
i sprawdź, czy i tobie pasują jego wybory.

## Dla kogo jest ta książka

Niniejsza książka zakłada, że czytelnik pisał już kod w innym języku, ale nie
określa w żaden sposób, w jakim. Starano się dostarczyć materiał szeroko
dostępny dla osób o wysoce zróżnicowanych podstawach w programowaniu.
Nie poświęcono wiele czasu tłumaczeniu, czym *jest* programowanie lub jak o
nim myśleć. Jeżeli jesteś osobą zupełnie nową w temacie programowania, większe
korzyści przyniesie ci przeczytanie innych pozycji, wprowadzających do
programowania.

## Jak używać tej książki

Głównym założeniem w książce jest to, że jest ona czytana kolejno, od początku
do końca. Późniejsze rozdziały opierają się na pojęciach z rozdziałów
wcześniejszych, a te z kolei mogą nie wchodzić w niektóre szczegóły omawianego
tematu. Do wielu tematów wracamy zazwyczaj w późniejszych rozdziałach.

W książce znajdują się rozdziały dwojakie: pojęciowe i projektowe. W rozdziałach
pojęciowych zapoznasz się z aspektami Rusta, a w rozdziałach projektowych
stworzymy razem krótkie programy wykorzystujące wiedzę przekazaną do danego
momentu. Rozdziałami projektowymi są te o numerach 2, 12 i 20. Pozostałe to
rozdziały pojęciowe.

Rozdział 1 wyjaśnia, jak zainstalować Rusta, jak napisać program „Hello,
world!” oraz jak używać Cargo - menedżera paczek i narzędzie budowania Rusta.
Rozdział 2 jest praktycznym wprowadzeniem do języka Rust. Poszczególne pojęcia
omawiamy tu ogólnie, podczas gdy szczegóły zostaną wyjaśnione w późniejszych
rozdziałach. Jeżeli chcesz od razu pobrudzić sobie ręce, rozdział 2 jest do tego
idealnym miejscem. W pierwszej chwili możesz zechcieć przeskoczyć do rozdziału
3, w którym omawiane są funkcje języka Rust podobne do tych z innych języków
programowania, a następnie prosto do rozdziału 4, aby nauczyć się systemu
własności w Ruście. Jeśli jednak należysz do szczególnie skrupulatnych
czytelników, którzy wolą poznać każdy szczegół przed przejściem do kolejnych
tematów, możesz wstępnie pominąć rozdział 2 i przejść prosto do rozdziału 3,
po czym cofać się do rozdziału 2 dopiero, gdy zechcesz wykorzystać w opisanym
tam projekcie rzeczy, których się nauczyłeś.

Rozdział 5 omawia struktury i metody, a rozdział 6 obejmuje typy wyliczeniowe
`enum`, wyrażenia `match` oraz konstrukcję `if let`. Struktury i typy
wyliczeniowe stosowane są w Ruście do tworzenia własnych typów.

W rozdziale 7 dowiadujemy się o systemie modułów w Ruście oraz o zasadach
prywatności przy organizacji kodu oraz jego publicznego Interfejsu Programowania
Aplikacji (API). Rozdział 8 omawia niektóre popularne struktury danych kolekcji
dostarczone przez bibliotekę standardową, takie jak wektory, ciągi znaków
(*string*) i tablice mieszające (*hash map*). Rozdział 9 odkrywa filozofię oraz
techniki obsługi błędów w Ruście.

Rozdział 10 wchodzi w temat typów ogólnych, cech (*traits*) oraz trwałości
(*lifetimes*), które dają ci możliwość definiowania kodu, odnoszącego się do
wielu typów danych. Rozdział 11 jest w całości poświęcony testowaniu, które mimo
gwarancji bezpieczeństwa Rusta nadal jest konieczne, by zapewnić prawidłową
logikę wykonywania programów. W rozdziale 12 budujemy natomiast własną
implementację fragmentu funkcjonalności polecenia `grep`, które wyszukuje tekst
w treści plików. Do tego celu będzie potrzebny cały szereg pojęć omówionych w
poprzednich rozdziałach.

Rozdział 13 omawia domknięcia (*closures*) oraz iteratory: cechy Rusta wywodzące
się z języków funkcyjnych. W rozdziale 14 głębiej przyglądamy się Cargo i mówimy
o najlepszych praktykach dotyczących dzielenia się swoimi bibliotekami z innymi
programistami. Rozdział 15 omawia dostarczone w bibliotece standardowej
inteligentne wskaźniki oraz cechy, które zapewniają im funkcjonalność.

W rozdziale 16 przechodzimy przez różne modele programowania współbieżnego oraz
tłumaczymy, jak Rust pomaga w programowaniu wielowątkowym bez obaw. Rozdział 17
wyjaśnia z kolei, jak idiomy Rusta można porównać do zasad programowania
obiektowego, z którym możesz być już zaznajomiony.

Rozdział 18 omawia wzorce oraz dopasowywanie wzorców, co jest potężnym
narzędzie do wyrażania myśli w programach pisanych w Ruście. Rozdział 19 jest
swoistym tyglem ciekawszych, zaawansowanych tematów, wliczając niebezpiecznego
Rusta i więcej na temat trwałości, cech, typów, funkcji i domknięć.

W rozdziale 20 tworzymy projekt, w którym implementujemy wielowątkowy,
niskopoziomowy serwer sieciowy!

Na zakończenie, kilka dodatków zawiera przydatne informacje o języku w formacie
przypominajacym książkę referencyjną. Dodatek A obejmuje słowa kluczowe Rusta,
Dodatek B operatory i symbole, Dodatek C omawia dostarczone przez bibliotekę
standardową cechy wyprowadzane, natomiast Dodatek D - kilka przydatnych narzędzi
deweloperskich, a Dodatek E wyjaśnia koncepcję edycji Rusta.

Nie ma złego sposobu na czytanie tej ksiażki - jeśli masz ochotę skakać w przód,
proszę bardzo! Powrót do wcześniejszych rozdziałów może okazać się konieczny,
jeżeli doświadczysz uczucia zagubienia. Rób wszystko, co ci pomaga.
=======
Welcome to *The Rust Programming Language*, an introductory book about Rust.
The Rust programming language helps you write faster, more reliable software.
High-level ergonomics and low-level control are often at odds in programming
language design; Rust challenges that conflict. Through balancing powerful
technical capacity and a great developer experience, Rust gives you the option
to control low-level details (such as memory usage) without all the hassle
traditionally associated with such control.

## Who Rust Is For

Rust is ideal for many people for a variety of reasons. Let’s look at a few of
the most important groups.

### Teams of Developers

Rust is proving to be a productive tool for collaborating among large teams of
developers with varying levels of systems programming knowledge. Low-level code
is prone to a variety of subtle bugs, which in most other languages can be
caught only through extensive testing and careful code review by experienced
developers. In Rust, the compiler plays a gatekeeper role by refusing to
compile code with these elusive bugs, including concurrency bugs. By working
alongside the compiler, the team can spend their time focusing on the program’s
logic rather than chasing down bugs.

Rust also brings contemporary developer tools to the systems programming world:

* Cargo, the included dependency manager and build tool, makes adding,
  compiling, and managing dependencies painless and consistent across the Rust
  ecosystem.
* Rustfmt ensures a consistent coding style across developers.
* The Rust Language Server powers Integrated Development Environment (IDE)
  integration for code completion and inline error messages.

By using these and other tools in the Rust ecosystem, developers can be
productive while writing systems-level code.

### Students

Rust is for students and those who are interested in learning about systems
concepts. Using Rust, many people have learned about topics like operating
systems development. The community is very welcoming and happy to answer
student questions. Through efforts such as this book, the Rust teams want to
make systems concepts more accessible to more people, especially those new to
programming.

### Companies

Hundreds of companies, large and small, use Rust in production for a variety of
tasks. Those tasks include command line tools, web services, DevOps tooling,
embedded devices, audio and video analysis and transcoding, cryptocurrencies,
bioinformatics, search engines, Internet of Things applications, machine
learning, and even major parts of the Firefox web browser.

### Open Source Developers

Rust is for people who want to build the Rust programming language, community,
developer tools, and libraries. We’d love to have you contribute to the Rust
language.

### People Who Value Speed and Stability

Rust is for people who crave speed and stability in a language. By speed, we
mean the speed of the programs that you can create with Rust and the speed at
which Rust lets you write them. The Rust compiler’s checks ensure stability
through feature additions and refactoring. This is in contrast to the brittle
legacy code in languages without these checks, which developers are often
afraid to modify. By striving for zero-cost abstractions, higher-level features
that compile to lower-level code as fast as code written manually, Rust
endeavors to make safe code be fast code as well.

The Rust language hopes to support many other users as well; those mentioned
here are merely some of the biggest stakeholders. Overall, Rust’s greatest
ambition is to eliminate the trade-offs that programmers have accepted for
decades by providing safety *and* productivity, speed *and* ergonomics. Give
Rust a try and see if its choices work for you.

## Who This Book Is For

This book assumes that you’ve written code in another programming language but
doesn’t make any assumptions about which one. We’ve tried to make the material
broadly accessible to those from a wide variety of programming backgrounds. We
don’t spend a lot of time talking about what programming *is* or how to think
about it. If you’re entirely new to programming, you would be better served by
reading a book that specifically provides an introduction to programming.

## How to Use This Book

In general, this book assumes that you’re reading it in sequence from front to
back. Later chapters build on concepts in earlier chapters, and earlier
chapters might not delve into details on a topic; we typically revisit the
topic in a later chapter.

You’ll find two kinds of chapters in this book: concept chapters and project
chapters. In concept chapters, you’ll learn about an aspect of Rust. In project
chapters, we’ll build small programs together, applying what you’ve learned so
far. Chapters 2, 12, and 20 are project chapters; the rest are concept chapters.

Chapter 1 explains how to install Rust, how to write a “Hello, world!” program,
and how to use Cargo, Rust’s package manager and build tool. Chapter 2 is a
hands-on introduction to the Rust language. Here we cover concepts at a high
level, and later chapters will provide additional detail. If you want to get
your hands dirty right away, Chapter 2 is the place for that. At first, you
might even want to skip Chapter 3, which covers Rust features similar to those
of other programming languages, and head straight to Chapter 4 to learn about
Rust’s ownership system. However, if you’re a particularly meticulous learner
who prefers to learn every detail before moving on to the next, you might want
to skip Chapter 2 and go straight to Chapter 3, returning to Chapter 2 when
you’d like to work on a project applying the details you’ve learned.

Chapter 5 discusses structs and methods, and Chapter 6 covers enums, `match`
expressions, and the `if let` control flow construct. You’ll use structs and
enums to make custom types in Rust.

In Chapter 7, you’ll learn about Rust’s module system and about privacy rules
for organizing your code and its public Application Programming Interface
(API). Chapter 8 discusses some common collection data structures that the
standard library provides, such as vectors, strings, and hash maps. Chapter 9
explores Rust’s error-handling philosophy and techniques.

Chapter 10 digs into generics, traits, and lifetimes, which give you the power
to define code that applies to multiple types. Chapter 11 is all about testing,
which even with Rust’s safety guarantees is necessary to ensure your program’s
logic is correct. In Chapter 12, we’ll build our own implementation of a subset
of functionality from the `grep` command line tool that searches for text
within files. For this, we’ll use many of the concepts we discussed in the
previous chapters.

Chapter 13 explores closures and iterators: features of Rust that come from
functional programming languages. In Chapter 14, we’ll examine Cargo in more
depth and talk about best practices for sharing your libraries with others.
Chapter 15 discusses smart pointers that the standard library provides and the
traits that enable their functionality.

In Chapter 16, we’ll walk through different models of concurrent programming
and talk about how Rust helps you to program in multiple threads fearlessly.
Chapter 17 looks at how Rust idioms compare to object-oriented programming
principles you might be familiar with.

Chapter 18 is a reference on patterns and pattern matching, which are powerful
ways of expressing ideas throughout Rust programs. Chapter 19 contains a
smorgasbord of advanced topics of interest, including unsafe Rust, macros, and
more about lifetimes, traits, types, functions, and closures.

In Chapter 20, we’ll complete a project in which we’ll implement a low-level
multithreaded web server!

Finally, some appendices contain useful information about the language in a
more reference-like format. Appendix A covers Rust’s keywords, Appendix B
covers Rust’s operators and symbols, Appendix C covers derivable traits
provided by the standard library, Appendix D covers some useful development
tools, and Appendix E explains Rust editions.

There is no wrong way to read this book: if you want to skip ahead, go for it!
You might have to jump back to earlier chapters if you experience any
confusion. But do whatever works for you.
>>>>>>> english/master

<span id="ferris"></span>

Ważną częścią procesu poznawania Rusta jest uczenie się odczytywania komunikatów
błędów wyświetlanych przez kompilator: będą cię one nakierowywać na uzyskanie
działającego kodu. Dlatego też zaprezentujemy wiele przykładów kodu, który się
nie kompiluje, wraz z komunikatami błędów zwracanych w każdym przypadku przez
kompilator. Miej świadomość, że wybrany przez ciebie na chybił-trafił przykład
może się nie skompilować. Koniecznie przeczytaj okalający go tekst aby
sprawdzić, czy dany kod przy próbie kompilacji powinien zwrócić błąd. Również
Ferris pomoże ci rozpoznać kod, który nie powinien zadziałać:

| Ferris                                                                 | Znaczenie                                 |
|------------------------------------------------------------------------|-------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain"/>    | Ten kod się nie kompiluje!                |
| <img src="img/ferris/panics.svg" class="ferris-explain"/>              | Ten kod panikuje!                         |
| <img src="img/ferris/unsafe.svg" class="ferris-explain"/>              | Ten blok kodu zawiera blok niebezpieczny. |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain"/>| Ten kod nie wywołuje zamierzonego efektu. |

W większości przypadków pokierujemy cię do uzykania prawidłowej wersji kodu,
który pierwotnie się nie kompiluje.

## Kod źródłowy

Pliki źródłowe, z których wygenerowana została niniejsza książka, można znaleźć
na [GitHubie][book].

[book]: https://github.com/paytchoo/book-pl/tree/master/src

## Informacje od tłumaczy

W niniejszej książce znajdziesz wiele przykładów programów oraz wycinki kodu,
w których: łańcuchy znaków w makrach `println!`, komentarze, a czasem nawet
nazwy projektów Cargo; zostały przetłumaczone. Jest to zabieg tłumaczy mający
na celu jak największe usunięcie bariery jaką stwarza użycie języka angielskiego w zrozumieniu
pojęć zawartych w tej książce. Prosimy, abyś rozważył(a) użycie języka angielskiego, kiedy
zdecydujesz się na udostępnienie swojego kodu, zarówno w przypadku Rusta przy udostępnianiu własnych skrzyń,
jak i w przypadku wszelkich innych języków programowania.

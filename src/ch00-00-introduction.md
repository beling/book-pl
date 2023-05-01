# Wstęp

> Uwaga: Niniejsze tłumaczenie oparto na wydaniu książki [The Rust Programming
> Language][nsprust] dostępnej w druku oraz w wersji elektronicznej, wydanych
> przez [No Starch Press][nsp].

[nsprust]: https://nostarch.com/rust-programming-language-2nd-edition
[nsp]: https://nostarch.com/

Witaj na łamach *Języka Programowania Rust* - wprowadzającej książki o Ruście.
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
zawierającego tego typu błędy. Dotyczy to również błędów w programach
współbieżnych (*wielowątkowych*). Współpraca z kompilatorem pozwala zespołowi
spędzić więcej czasu na dopracowywaniu logiki programu niż na polowaniu na bugi.

Rust jednocześnie wnosi do świata programowania systemowego nowoczesne narzędzia
deweloperskie:

* Cargo - dołączony menedżer zależności i narzędzie do budowania, czyni
  dodawanie, kompilowanie i zarządzanie zależnościami bezbolesnym i spójnym
  procesem w obrębie ekosystemu Rusta.
* Rustfmt zapewnia spójny styl kodu wśród deweloperów.
* Serwer języka Rust (Rust Language Server) zapewnia w konkretnych Zintegrowanych Środowiskach
  Programistycznych (IDE) auto-uzupełnianie oraz ostrzeżenia inline o błędach.

Korzystając z tych oraz innych narzędzi w ekosystemie Rusta, deweloperzy mogą
pozostać produktywni również podczas pisania kodu niskopoziomowego.

### Uczniowie

Rust jest językiem dla uczniów i osób zainteresowanych uczeniem się systemowych
pojęć. Z pomocą Rusta, wielu ludzi poznało tematy z zakresu rozwoju systemów
operacyjnych. Społeczność Rusta jest niezwykle gościnna i chętnie odpowiada na
pytania uczniów. Poprzez zaangażowanie w projekty takie jak niniejsza książka,
zespoły tworzące Rusta chcą uczynić pojęcia systemowe osiągalnymi dla większej
ilości osób, szczególnie tych, dla których programowanie jest czymś zupełnie
nowym.

### Firmy

W setkach dużych i małych firm używa się Rusta do różnych zadań, w
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

Rozdział 1 wyjaśnia, jak zainstalować Rusta, jak napisać program „Witaj,
świecie!” oraz jak używać Cargo - menedżera pakietów i narzędzie budowania Rusta.
Rozdział 2 jest praktycznym wprowadzeniem do języka Rust. Poszczególne pojęcia
omawiamy tu ogólnie, podczas gdy szczegóły zostaną wyjaśnione w późniejszych
rozdziałach. Jeżeli chcesz od razu pobrudzić sobie ręce, rozdział 2 jest do tego
idealnym miejscem. W rozdziale 3 omawiane są funkcje języka Rust podobne do tych z innych języków
programowania. Z rozdziału 4 można nauczyć się systemu
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
deweloperskich, a Dodatek E wyjaśnia koncepcję edycji Rusta. W dodatku F można
znaleźć tłumaczenia książki, a w dodatku G zajmiemy się tym, jak powstaje Rust
i czym jest Rust Nightly.

Nie ma złego sposobu na czytanie tej książki - jeśli masz ochotę skakać w przód,
proszę bardzo! Powrót do wcześniejszych rozdziałów może okazać się konieczny,
jeżeli doświadczysz uczucia zagubienia. Rób wszystko, co ci pomaga.

<span id="ferris"></span>

Ważną częścią procesu poznawania Rusta jest uczenie się odczytywania komunikatów
błędów wyświetlanych przez kompilator: będą cię one nakierowywać na uzyskanie
działającego kodu. Dlatego też zaprezentujemy wiele przykładów kodu, który się
nie kompiluje, wraz z komunikatami błędów zwracanych w każdym przypadku przez
kompilator. Miej świadomość, że wybrany przez ciebie na chybił-trafił przykład
może się nie skompilować. Koniecznie przeczytaj okalający go tekst aby
sprawdzić, czy dany kod przy próbie kompilacji powinien zwrócić błąd. Również
Ferris pomoże ci rozpoznać kod, który nie powinien zadziałać:

| Ferris                                                                                                           | Znaczenie                                          |
|------------------------------------------------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris with a question mark"/>            | Ten kod się nie kompiluje!                      |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris throwing up their hands"/>                   | Ten kod panikuje!                               |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris with one claw up, shrugging"/> | Ten kod nie daje pożądanego zachowania.         |

W większości przypadków pokierujemy cię do uzykania prawidłowej wersji kodu,
który pierwotnie się nie kompiluje.

## Kod źródłowy

Pliki źródłowe, z których wygenerowana została niniejsza książka, można znaleźć
na [GitHubie][book].

[book]: https://github.com/beling/book-pl/tree/master/src

## Informacje od tłumaczy

W niniejszej książce znajdziesz wiele przykładów programów oraz wycinki kodu,
w których: łańcuchy znaków w makrach `println!`, komentarze, a czasem nawet
nazwy projektów Cargo; zostały przetłumaczone. Jest to zabieg tłumaczy mający
na celu jak największe usunięcie bariery jaką stwarza użycie języka angielskiego w zrozumieniu
pojęć zawartych w tej książce. Prosimy, abyś rozważył(a) użycie języka angielskiego, kiedy
zdecydujesz się na udostępnienie swojego kodu, zarówno w przypadku Rusta przy udostępnianiu własnych skrzyń,
jak i w przypadku wszelkich innych języków programowania.

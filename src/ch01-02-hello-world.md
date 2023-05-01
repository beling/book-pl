## Witaj, świecie!

Ponieważ Rust jest już zainstalowany, napiszmy pierwszy program. Przy
poznawaniu nowego języka tradycją stało się tworzenie krótkiego programu, który
wyświetla na ekranie tekst „Witaj, świecie! (ang. „Hello World!”). Postąpimy tu
tak samo.

> Uwaga: Książka zakłada u czytelnika podstawową znajomość wiersza poleceń. Sam
> Rust nie wymaga konkretnego edytora, narzędzi czy też miejsca, gdzie zapisany
> jest kod, więc jeśli preferujesz używanie zintegrowanego
> środowiska programistycznego zamiast wiersza poleceń, masz do tego pełne
> prawo. Wiele IDE obsługuje Rusta w pewnym stopniu - możesz sprawdzić
> dokumentację swojego IDE, aby uzyskać szczegóły. Zespół Rusta ostatnio zadbał
> o obsługę języka w niektórych IDE dzięki `rust-analyzer`.
> Więcej szczegółów zawiera [Dodatek D][devtools]<!-- ignore -->.

### Tworzenie katalogu projektu

W pierwszej kolejności utwórz katalog, w którym umieścisz swój kod. Rust
w żaden sposób nie narzuca lokalizacji dla kodu, ale dla potrzeb niniejszej
książki sugerujemy utworzenie katalogu *projects* w katalogu domowym i
przechowywanie tam wszystkich swoich programów.

Otwórz terminal i wprowadź następujące polecenia, aby utworzyć katalog
*projects*, w którym umieścimy kolejny, pod projekt „Witaj, świecie!”:

Pod Linuksem, macOS, lub w PowerShell pod Windowsem, wpisz to:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

W Windows CMD, wpisz to:

```cmd
> mkdir %USERPROFILE%\projects
> cd /d %USERPROFILE%\projects
> mkdir hello_world
> cd hello_world
```

### Pisanie i uruchamianie programu w Ruście

Następnie utwórz nowy plik źródłowy i nazwij go *main.rs*. Pliki języka Rust
zawsze zakończone są rozszerzeniem *.rs*. Jeśli w nazwie pliku znajduje się
więcej niż jedno słowo, użyj znaku podkreślenia jako separatora. Na przykład,
*hello_world.rs* zamiast *helloworld.rs*.

Otwórz plik *main.rs*, który właśnie utworzyłeś i wprowadź kod podany w
Listingu 1-1:

<span class="filename">Plik: main.rs</span>

```rust
fn main() {
    println!("Witaj, świecie!");
}
```

<span class="caption">Listing 1-1: Program wyświetlający „Witaj, świecie!”</span>

Zapisz plik i wróć do okna terminala w katalogu *~/projects/hello_world*. Pod Linuksem lub macOS, wprowadź podane
polecenia, żeby skompilować i uruchomić program:

```console
$ rustc main.rs
$ ./main
Witaj, świecie!
```

Pod Windowsem, uruchom `.\main.exe` zamiast `./main`. 

```powershell
> rustc main.rs
> .\main.exe
Witaj, świecie!
```

Bez względu na posiadany system operacyjny, powinieneś/powinnaś zobaczyć w oknie
terminala wyświetlony tekst `Witaj, świecie!`. Jeśli nic takiego nie widzisz,
zajrzyj do sekcji [„Rozwiązywanie problemów”][troubleshooting]<!-- ignore --> w
rozdziale poświęconym instalacji, po sposoby na otrzymanie pomocy.

Jeśli widzisz tekst `Witaj, świecie!`, to gratulacje! Właśnie oficjalnie napisałeś
program w Ruście, a to czyni z ciebie programistę Rusta - Witaj!

### Anatomia programu w Ruście

Przyjrzyjmy się teraz dokładnie, co właściwie dzieje się w twoim programie
„Witaj, świecie!”. Oto pierwsza część układanki:

```rust
fn main() {

}
```

Powyższe linie definiują w Ruście funkcję. Funkcja `main` jest szczególna: to
pierwsza rzecz, która wykonuje się w każdym programie napisanym w Ruście. Pierwsza
linia deklaruje funkcję o nazwie `main`, która nie przyjmuje argumentów oraz
niczego nie zwraca. Gdyby funkcja przyjmowała jakieś argumenty, ich nazwy
umieszczone byłyby w nawiasach, `()`.

Zwróć też uwagę, że ciało funkcji otoczone jest nawiasami klamrowymi, `{}`.
Rust wymaga tych znaków wokół ciał wszystkich funkcji. Za dobry styl uważa się
umieszczenie klamry otwierającej w tej samej linii, co deklaracja funkcji i
oddzielenie jej od poprzedzającego kodu jedną spacją.

> Uwaga: Jeśli chcesz trzymać się ściśle standardowego stylu używanego w
> Rustowych projektach, możesz użyć `rustfmt` -- narzędzia do automatycznego
> formatowania kodu do konkretnego stylu (więcej o `rustfmt` jest w
> [Dodatku D][devtools]<!-- ignore -->).
> Programu zawarty jest w standardowej dystrybucji Rusta, tak jak `rustc`,
> więc `rustfmt` prawdopodobnie już zainstalowany na twoim komputerze!

Wewnątrz funkcji `main` mamy następujący kod:

```rust
    println!("Witaj, świecie!");
```

Ta linia wykonuje całą robotę w naszym krótkim programie: wyświetla tekst na
ekranie. Należy tu zwrócić uwagę na cztery szczegóły.

Po pierwsze, wcięcie tekstu w Ruście składa się z czterech spacji, a nie z tabulatora.

Po drugie, `println!` wywołuje w Ruście makro. Gdyby wywoływana była
funkcja, wpisana byłaby jako `println` (bez `!`). Makra będą szerzej opisane w
rozdziale 19. Jedyne, co musisz wiedzieć teraz, to to, że użycie `!` wywołuje
makro zamiast zwykłej funkcji i że makra nie zawsze podlegają tym samym regułą co funkcje.

Po trzecie, w programie widzimy łańcuch znaków (*string*) `"Witaj, świecie!"`.
Przekazujemy go jako argument do `println!`, którego ostatecznym efektem jest
wyświetlenie łańcucha na ekranie.

Po czwarte, linia zakończona jest średnikiem (`;`), który oznacza, że bieżące
wyrażenie jest zakończone i można zacząć kolejne. Większość linii w Ruście
kończy się znakiem średnika.

### Kompilacja i uruchomienie to oddzielne kroki

Przed momentem uruchomiliśmy nowo napisany program. Zbadajmy teraz kolejno każdy
poprzedzający ten moment etap.

Zanim uruchomisz program w Ruście, należy go skompilować kompilatorem Rusta,
wprowadzając polecenie `rustc` i przekazując do niego jako argument nazwę pliku
źródłowego. Wygląda to tak:

```console
$ rustc main.rs
```

Jeśli masz doświadczenie w C lub C++, zauważysz, że jest to podobne do
wywoływania `gcc` lub `clang`. Po udanej kompilacji, Rust tworzy binarny plik
wykonywalny.

Pod Linuksem, macOS lub w PowerShell pod Windowsem możesz sprawdzić obecność
pliku wykonywalnego używając w terminalu polecenia `ls`:

```console
$ ls
main  main.rs
```

W przypadku Linuksa i macOS zobaczysz dwa pliki. PowerShell pod Windowsem wyświetli trzy pliki - te
same, które wyświetliłby CMD. W CMD pod Windowsem możesz wpisać:

```cmd
> dir /B %= opcja /B powoduje wyświetlenie jedynie nazw plików =%
main.exe
main.pdb
main.rs
```

To potwierdza obecność kodu źródłowego z rozszerzeniem *.rs*, programu
wykonywalnego (*main.exe* pod Windowsem, *main* wszędzie indziej), a także, w
przypadku Windowsa, pliku z rozszerzeniem *.pdb* zawierającego informacje
debugujące. Wszystko, co pozostało do zrobienia, to uruchomienie pliku *main*
lub *main.exe*, w taki sposób:

```console
$ ./main # lub .\main.exe pod Windowsem
```

Jeżeli *main.rs* jest twoim programem „Witaj, świecie!”, wyświetli to w oknie
terminala tekst `Witaj, świecie!`.

Jeśli jesteś doświadczony w językach dynamicznych, takich jak Ruby, Python lub
JavaScript, możesz nie być przyzwyczajony do traktowania kompilacji i
uruchomienia programu jako oddzielnych kroków. Rust jest językiem kompilowanym
*z wyprzedzeniem*, co oznacza, że możesz skompilować program i dać go komuś,
kto uruchomi go nawet nie mając zainstalowanego Rusta. Jeśli natomiast dasz
komuś plik `.rb`, `.py` lub `.js`, odbiorca musi mieć zainstalowane
odpowiednio implementacje Ruby, Pythona lub JavaScript. Jednak w tych językach
jedna komenda wystarczy, aby jednocześnie skompilować i uruchomić program.
Wszystko jest kompromisem w konstrukcji danego języka.

Kompilacja poprzez `rustc` jest wystarczająca dla prostych programów, ale w
miarę rozrastania się twojego projektu, odczujesz potrzebę zarządzania
wszystkimi dostępnymi w nim opcjami i ułatwienia dzielenia się kodem.
W dalszym ciągu przedstawimy narzędzie zwane Cargo, które pomoże ci w pisaniu
*prawdziwych* programów w Ruście.

[troubleshooting]: ch01-01-installation.html#rozwiązywanie-problemów
[devtools]: appendix-04-useful-development-tools.md

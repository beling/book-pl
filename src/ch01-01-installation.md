## Instalacja

Naszym pierwszym krokiem jest zainstalowanie Rusta. Ściągniemy go za pomocą
`rustup`, narzędzia uruchamianego z linii poleceń, które służy do zarządzania
wersjami Rusta i powiązanych narzędzi. Będzie do tego potrzebne połączenie z
internetem.

> Uwaga: Jeśli z jakiegoś powodu wolisz nie używać narzędzia `rustup`, odwiedź
> [Stronę instalacyjną Rusta](https://www.rust-lang.org/install.html),
> aby dowiedzieć się, jakie masz inne opcje.

Wykonanie kolejnych kroków spowoduje zainstalowanie najnowszej, stabilnej wersji
kompilatora Rusta. Stabilność Rusta gwarantuje, że wszystkie przykłady z
książki, które poprawnie się kompilują, kompilowały się będą nadal w nowszych
wersjach języka. Komunikaty zwrotne błędów i ostrzeżeń mogą nieco różnić się od
siebie w zależności od wersji, ponieważ dość często są one poprawiane. Innymi
słowy, każda nowsza, stabilna wersja Rusta zainstalowana przy użyciu tych samych
kroków powinna współgrać z treścią książki zgodnie z oczekiwaniami.

> ### Oznaczenia w linii poleceń
>
> W tym rozdziale oraz w dalszych częściach książki, będziemy korzystać z
> poleceń wydawanych przy użyciu terminala. Wszystkie linie, które powinno się
> wprowadzić w terminalu będą rozpoczynać się od znaku `$`. Nie musisz go
> wpisywać. Pokazany jest on jedynie celem zidentyfikowania początku każdego
> polecenia. Linie, które nie zaczynają się od `$`, zazwyczaj pokazują komunikat
> wyświetlony po wydaniu ostatniego polecenia. Dodatkowo, specyficzne przykłady
> dla terminala PowerShell będą wykorzystywały znak `>` zamiast `$`.

### Instalacja `rustup` pod Linuksem lub macOS

Jeśli korzystasz z Linuksa lub macOS, otwórz konsolę / terminal i wpisz
następujące polecenie:

```text
$ curl https://sh.rustup.rs -sSf | sh
```

Spowoduje to ściągnięcie skryptu, który rozpocznie instalację narzędzia
`rustup`, które z kolei zainstaluje najnowszą, stabilną wersję Rusta. Możesz
otrzymać prośbę o wprowadzenie swojego hasła. Jeśli wszystko się powiedzie,
zobaczysz na ekranie taki tekst:

```text
Rust is installed now. Great!
```

Jeśli chcesz, możesz ściągnąć wspomniany skrypt i skontrolować go przed
uruchomieniem.

Skrypt instalacyjny automatycznie dodaje Rusta do środowiskowej zmiennej PATH w
systemie po kolejnym zalogowaniu się. Jeśli chcesz od razu zacząć używać Rusta,
bez ponownego logowania, wprowadź następujące polecenie, dodając Rusta do
zmiennej PATH ręcznie:

```text
$ source $HOME/.cargo/env
```

Alternatywnie, dodaj następującą linię do swojego pliku *~/.bash_profile*:

```text
$ export PATH="$HOME/.cargo/bin:$PATH"
```

Następnie będziesz potrzebować programu linkującego (*linkera*). Prawdopodobnie
masz już taki zainstalowany, ale jeżeli przy próbie kompilacji programu Rusta
uzyskasz błędy informujące, że nie można uruchomić linkera, oznacza to, że w
danym systemie linker nie jest zainstalowany i konieczna jest jego ręczna
instalacja. Kompilatory języka C zazwyczaj wyposażone są w odpowiedni linker.
Sprawdź dokumentację swojej platformy, aby dowiedzieć się, jak przeprowadzić
ewentualną instalację kompilatora C. Wiele popularnych pakietów Rusta korzysta z
kodu C i wymaga również kompilatora C. Z tego względu jego instalacja w tym
momencie może być zasadna.

### Instalacja `rustup` pod Windowsem

Z poziomu Windowsa, odwiedź stronę
[https://www.rust-lang.org/tools/install][install] i kieruj się instrukcjami
instalacji Rusta. W którymś momencie procesu otrzymasz komunikat informujący, że
będziesz również potrzebować narzędzi budowania C++ dla Visual Studio 2013 lub
nowszego. Najprościej jest w tym celu zainstalować
[narzędzia budowania dla Visual Studio 2019][visualstudio], które można znaleźć
w sekcji „Other Tools and Frameworks” (*inne narzędzia i platformy
programistyczne).

[install]: https://www.rust-lang.org/tools/install
[visualstudio]: https://www.visualstudio.com/downloads/#build-tools-for-visual-studio-2019

Dalsza część książki będzie zawierać polecenia działające zarówno w *cmd.exe*
jak i w PowerShell. Jeżeli pojawią się jakieś różnice, zostaną one wyjaśnione.

### Aktualizowanie i odinstalowanie

Kiedy już zainstalujesz Rusta używając `rustup`, aktualizacja do najnowszej
wersji jest prosta. W terminalu uruchom następujący skrypt aktualizacji:

```text
$ rustup update
```

Aby odinstalować Rusta oraz `rustup`, w terminalu uruchom skrypt dezinstalacji:

```text
$ rustup self uninstall
```

### Rozwiązywanie problemów

Aby sprawdzić, czy Rust jest prawidłowo zainstalowany, otwórz terminal i wpisz
następujące polecenie:

```text
$ rustc --version
```

Powinien się wyświetlić numer wersji, hasz oraz data commita najnowszej wersji
stabilnej, w formacie podobnym do poniższego:

```text
rustc x.y.z (abcabcabc rrrr-mm-dd)
```

Jeśli widzisz coś takiego, Rust zainstalował się prawidłowo! Jeśli nie, a
pracujesz pod Windowsem, sprawdź, czy Rust został dodany do zmiennej
środowiskowej `%PATH%`. Jeżeli wciąż coś nie działa, jest kilka miejsc, w
których możesz uzyskać pomoc. Najłatwiej na
[oficjalnym kanale Rusta na Discordzie][discord]. 
Tam możesz czatować z innymi Rustowcami (*Rustaceans - od angielskiego terminu
“crustaceans” - “skorupiaki” - przyp. tłum.*) (taką wspólną ksywką się
określamy), którzy mogą ci pomóc. Do innych świetnych źródeł należy [forum
użytkowników][users] oraz [Stack Overflow][stackoverflow].

[discord]: https://discord.gg/rust-lang
[users]: https://users.rust-lang.org/
[stackoverflow]: http://stackoverflow.com/questions/tagged/rust

### Lokalna dokumentacja

Instalacja dołącza również lokalną kopię dokumentacji, więc można ją czytać
offline. Uruchom `rustup doc`, żeby otworzyć lokalną dokumentację w swojej
przeglądarce.

Za każdym razem, kiedy przytoczone są typ lub funkcja z biblioteki standardowej,
a nie masz pewności co robią lub jak ich użyć, użyj dokumentacji API
(Interfejs Programowania Aplikacji), aby się dowiedzieć.

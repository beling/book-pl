## Instalacja

Naszym pierwszym krokiem jest zainstalowanie Rusta. Ściągniemy go za pomocą
`rustup`, narzędzia uruchamianego z linii poleceń, które służy do zarządzania
wersjami Rusta i powiązanych narzędzi. Będzie do tego potrzebne połączenie z
internetem.

> Uwaga: Jeśli z jakiegoś powodu wolisz nie używać narzędzia `rustup`, odwiedź
> [Stronę opisującą inne metody instalacji Rusta][otherinstall].

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

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Spowoduje to ściągnięcie skryptu, który rozpocznie instalację narzędzia
`rustup`, które z kolei zainstaluje najnowszą, stabilną wersję Rusta. Możesz
otrzymać prośbę o wprowadzenie swojego hasła. Jeśli wszystko się powiedzie,
zobaczysz na ekranie taki tekst:

```text
Rust is installed now. Great!
```

Będziesz też potrzebować programu linkującego (*linkera*), którego Rust używa do
złączenia wyników kompilowania w jeden plik. Prawdopodobnie
masz już taki zainstalowany, ale jeżeli przy próbie kompilacji programu Rusta
uzyskasz błędy informujące, że nie można uruchomić linkera, oznacza to, że w
danym systemie linker nie jest zainstalowany i konieczna jest jego ręczna
instalacja. Kompilatory języka C zazwyczaj wyposażone są w odpowiedni linker.
Sprawdź dokumentację swojej platformy, aby dowiedzieć się, jak przeprowadzić
ewentualną instalację kompilatora C. Wiele popularnych pakietów Rusta korzysta z
kodu C i wymaga również kompilatora C. Z tego względu jego instalacja w tym
momencie może być zasadna.

On macOS, you can get a C compiler by running:

```console
$ xcode-select --install
```

Linux users should generally install GCC or Clang, according to their
distribution’s documentation. For example, if you use Ubuntu, you can install
the `build-essential` package.


### Instalacja `rustup` pod Windowsem

Z poziomu Windowsa, odwiedź stronę
[https://www.rust-lang.org/tools/install][install] i kieruj się instrukcjami
instalacji Rusta. W którymś momencie procesu otrzymasz komunikat informujący, że
będziesz również potrzebować narzędzi budowania MSVC dla Visual Studio 2013 lub
nowszego.

To acquire the build tools, you’ll need to install [Visual Studio
2022][visualstudio]. When asked which workloads to install, include:

* „Desktop Development with C++”
* The Windows 10 or 11 SDK
* The English language pack component, along with any other language pack of
  your choosing

Dalsza część książki będzie zawierać polecenia działające zarówno w *cmd.exe*
jak i w PowerShell. Jeżeli pojawią się jakieś różnice, zostaną one wyjaśnione.

### Aktualizowanie i odinstalowanie

Kiedy już zainstalujesz Rusta używając `rustup`, aktualizacja do najnowszej
wersji jest prosta. W terminalu uruchom następujący skrypt aktualizacji:

```console
$ rustup update
```

Aby odinstalować Rusta oraz `rustup`, w terminalu uruchom skrypt dezinstalacji:

```console
$ rustup self uninstall
```


### Rozwiązywanie problemów

Aby sprawdzić, czy Rust jest prawidłowo zainstalowany, otwórz terminal i wpisz
następujące polecenie:

```console
$ rustc --version
```

Powinien się wyświetlić numer wersji, hasz oraz data commita najnowszej wersji
stabilnej, w formacie podobnym do poniższego:

```text
rustc x.y.z (abcabcabc rrrr-mm-dd)
```

Jeśli widzisz coś takiego, Rust zainstalował się prawidłowo! Jeśli nie, to sprawdź
czy Rust został dodany do zmiennej środowiskowej `%PATH%`.

In Windows CMD, use:

```console
> echo %PATH%
```

In PowerShell, use:

```powershell
> echo $env:Path
```

In Linux and macOS, use:

```console
$ echo $PATH
```

If that’s all correct and Rust still isn’t working, there are a number of
places you can get help. Find out how to get in touch with other Rustaceans (a
silly nickname we call ourselves) on [the community page][community].

### Lokalna dokumentacja

Instalacja dołącza również lokalną kopię dokumentacji, więc można ją czytać
offline. Uruchom `rustup doc`, żeby otworzyć lokalną dokumentację w swojej
przeglądarce.

Za każdym razem, kiedy przytoczone są typ lub funkcja z biblioteki standardowej,
a nie masz pewności co robią lub jak ich użyć, użyj dokumentacji API
(Interfejs Programowania Aplikacji), aby się dowiedzieć.


[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[visualstudio]: https://visualstudio.microsoft.com/downloads/
[community]: https://www.rust-lang.org/community


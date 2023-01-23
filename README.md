# Język Programowania Rust

![Build Status](https://github.com/beling/book-pl/workflows/CI/badge.svg)

Niniejsze repozytorium stanowi próbę przetłumaczenia książki
[“Język Programowania Rust”][rust-book] na język polski. Druga edycja
angielskiej wersji została niedawno wydana w formie papierowej przez wydawnictwo
NoStarch Press. Odwiedź [stronę No Starch][nostarch], aby uzyskać więcej
informacji dotyczących daty wydania i zamówień.

[rust-book]: https://github.com/rust-lang/book
[nostarch]: https://nostarch.com/rust

## Wymagania

Do zbudowania książki potrzebne jest narzędzie [mdBook], najlepiej w tej samej
wersji, do której odnosi się
[ten plik w repozytorium rust-lang/rust][rust-mdbook].
Aby je zainstalować, wpisz:

[mdBook]: https://github.com/rust-lang-nursery/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --version <numer_wersji>
```

## Budowanie

Celem zbudowania książki, wykonaj polecenie:

```bash
$ mdbook build
```

Gotowa książka trafi do podfolderu `book`. Można ją wówczas otworzyć z poziomu
przeglądarki internetowej.

_Firefox:_
```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_
```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

Lub po prostu otwórz dany plik html z poziomu Eksplorera Windows czy innej
graficznej przeglądarki plików.

Aby uruchomić testy:

```bash
$ mdbook test
```

## Pomoc w tłumaczeniu
Chętnie przyjmiemy każdą pomoc! Przed zaczynaniem tłumaczenia zapoznaj się z
informacjami w pliku [CONTRIBUTING.md](./CONTRIBUTING.md)!

## Sprawdzanie pisowni
Aby przeskanować pliki źródłowe w poszukiwaniu błędów ortograficznych, możesz użyć skryptu `spellcheck.sh`.
dostępnego w katalogu `ci`. Potrzebuje on słownika poprawnych słów,
który jest dostarczany w `ci/dictionary.txt`. Jeśli skrypt daje fałszywy
pozytywny (powiedzmy, że użyłeś słowa `BTreeMap`, które skrypt uważa za niepoprawne),
musisz dodać to słowo do `ci/dictionary.txt` (zachowaj posortowaną kolejność dla
spójności).

Uwaga: metoda nie została sprawdzona z polskojęzyczną wersją książki. Skrpyt może wymagać przystosowania.
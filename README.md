# Język Programowania Rust

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
$ cargo install mdbook --vers [numer-wersji]
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

## Graphviz dot

Używamy narzędzia [Graphviz](http://graphviz.org/) do utworzenia niektórych
diagramów w książce. Źródła tych plików znajdują się w podfolderze `dot`. Aby
przekonwertować plik `dot`, przykładowo `dot/trpl-01.dot` na format `svg`,
wpisz:

```bash
$ dot dot/trpl04-01.dot -Tsvg > src/img/trpl04-01.svg
```

W wygenerowanym pliku SVG usuń atrybuty `width` i `height` z elementu `svg` i
ustaw atrybut `viewBox` na wartość `0.00 0.00 1000.00 1000.00` lub inne, o ile
nie przytną one rysunku.


## Pomoc w tłumaczeniu

Chętnie przyjmiemy każdą pomoc! Przed zaczynaniem tłumaczenia zapoznaj się z
informacjami w pliku [CONTRIBUTING.md](./CONTRIBUTING.md)!

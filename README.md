# Język Programowania Rust

![Build Status](https://github.com/beling/book-pl/workflows/CI/badge.svg)

Niniejsze repozytorium stanowi próbę przetłumaczenia książki
[„Język Programowania Rust”][rust-book] na język polski. Druga edycja
angielskiej wersji została niedawno wydana w formie papierowej przez wydawnictwo
NoStarch Press. Odwiedź [stronę No Starch][nostarch], aby uzyskać więcej
informacji dotyczących daty wydania i zamówień.

[rust-book]: https://github.com/rust-lang/book
[nostarch]: https://nostarch.com/rust-programming-language-2nd-edition

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

Uwaga: metoda nie została sprawdzona z polskojęzyczną wersją książki. Skrypt może wymagać przystosowania.


<!-- dalej jest treść oryginalnego by pozwolić git-owemu merge ją automatycznie aktualizować:

# The Rust Programming Language

![Build Status](https://github.com/rust-lang/book/workflows/CI/badge.svg)

This repository contains the source of "The Rust Programming Language" book.

[The book is available in dead-tree form from No Starch Press][nostarch].

[nostarch]: https://nostarch.com/rust-programming-language-2nd-edition

You can also read the book for free online. Please see the book as shipped with
the latest [stable], [beta], or [nightly] Rust releases. Be aware that issues
in those versions may have been fixed in this repository already, as those
releases are updated less frequently.

[stable]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[nightly]: https://doc.rust-lang.org/nightly/book/

See the [releases] to download just the code of all the code listings that appear in the book.

[releases]: https://github.com/rust-lang/book/releases

## Requirements

Building the book requires [mdBook], ideally the same version that
rust-lang/rust uses in [this file][rust-mdbook]. To get it:

[mdBook]: https://github.com/rust-lang-nursery/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --version <version_num>
```

## Building

To build the book, type:

```bash
$ mdbook build
```

The output will be in the `book` subdirectory. To check it out, open it in
your web browser.

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

To run the tests:

```bash
$ mdbook test
```

## Contributing

We'd love your help! Please see [CONTRIBUTING.md][contrib] to learn about the
kinds of contributions we're looking for.

[contrib]: https://github.com/rust-lang/book/blob/main/CONTRIBUTING.md

Because the book is [printed][nostarch], and because we want
to keep the online version of the book close to the print version when
possible, it may take longer than you're used to for us to address your issue
or pull request.

So far, we've been doing a larger revision to coincide with [Rust
Editions](https://doc.rust-lang.org/edition-guide/). Between those larger
revisions, we will only be correcting errors. If your issue or pull request
isn't strictly fixing an error, it might sit until the next time that we're
working on a large revision: expect on the order of months or years. Thank you
for your patience!

### Translations

We'd love help translating the book! See the [Translations] label to join in
efforts that are currently in progress. Open a new issue to start working on
a new language! We're waiting on [mdbook support] for multiple languages
before we merge any in, but feel free to start!

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/rust-lang-nursery/mdBook/issues/5

## Spellchecking

To scan source files for spelling errors, you can use the `spellcheck.sh`
script available in the `ci` directory. It needs a dictionary of valid words,
which is provided in `ci/dictionary.txt`. If the script produces a false
positive (say, you used word `BTreeMap` which the script considers invalid),
you need to add this word to `ci/dictionary.txt` (keep the sorted order for
consistency).

 -->

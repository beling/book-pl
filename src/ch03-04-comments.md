## Komentarze

Wszyscy programiści starają się tworzyć czytelny i zrozumiały kod,
jednak czasem potrzebne jest dodatkowe wyjaśnienie. W takich wypadkach
programiści umieszczają w kodzie notki lub *komentarze*, które mimo że
są ignorowane przez kompilator, mogą okazać się przydatne dla osób
czytających kod.

Oto prosty komentarz:

```rust
// Hello, world
```

W Ruście, komentarze muszą rozpoczynać się dwoma ukośnikami i ciągną się
do końca linii. W przypadku komentarzy, których długość przekracza jedną
linię, musisz wstawić `//` na początku każdej linii, tak jak poniżej:

```rust
// A więc opisujemy tu coś skomplikowanego, wystarczająco długo,
// że potrzebujemy wieloliniowych komentarzy, aby to zrobić!
// Fiu! Fiu! Na szczęście ten komentarz wyjaśni, o co chodzi.
```

Komentarze mogą być też umieszczone na końcu linii zawierającej kod:

<span class="filename">Plik: src/main.rs</span>

```rust
fn main() {
    let lucky_number = 7; // Czuję, że sprzyja mi dziś szczęście
}
```

Jednak częściej spotkasz się z zapisem, w którym komentarz znajduje się
w oddzielnej linii nad kodem, który jest przez niego opisywany:

<span class="filename">Plik: src/main.rs</span>

```rust
fn main() {
    // Czuję, że sprzyja mi dziś szczęście
    let lucky_number = 7;
}
```

Rust posiada też wbudowany inny typ komentarzy, komentarze dokumentacji,
które omówimy w rozdziale 14.

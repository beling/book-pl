## Nieodwracalne Błędy z `panic!`

Czasami w kodzie dzieją się złe rzeczy i nic nie można na to poradzić.
W takich przypadkach Rust posiada makro `panic!`. Istnieją dwa sposoby na wywołanie go
w praktyce: podejmując działanie, które powoduje, że nasz kod wpada w panikę (np.
dostęp do tablicy poza jej końcem) lub poprzez jawne wywołanie makra `panic!`.
W obu przypadkach wywołujemy panikę w naszym programie. Domyślnie, te paniki 
wypisują komunikat o niepowodzeniu, zwijają, czyszczą stos i kończą działanie. Poprzez
zmienną środowiskową, można również nakazać Rustowi wyświetlanie stosu wywołań, gdy wystąpi
panika aby ułatwić odnalezienie źródła paniki.

> ### Zwijanie Stosu lub Przerwanie w Odpowiedzi na Panikę
>
> Domyślnie, gdy wystąpi panika, program rozpoczyna *zwijanie*, co oznacza, że
> Rust cofa się w górę stosu i czyści dane każdej napotkanej funkcji.
> Jednak cofanie się i czyszczenie to dużo pracy. Rust dlatego pozwala wybrać 
> alternatywę natychmiastowego *przerwania*, co kończy program bez czyszczenia.
>
> Pamięć, z której korzystał program, będzie musiała zostać wyczyszczona przez
> system operacyjny. Jeśli w swoim projekcie chcesz, aby wynikowy plik binarny był tak
> mały jak to tylko możliwe, możesz przełączyć się ze zwijania na przerywanie w przypadku paniki poprzez
> dodanie `panic = 'abort'` do odpowiednich sekcji `[profile]` w pliku
> *Cargo.toml*. Na przykład, jeśli chcesz przerwać po panice w trybie zwijania,
> dodaj to:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

Spróbujmy wywołać `panic!` w prostym programie:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

Po uruchomieniu programu zobaczysz coś takiego:

```console
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

Wywołanie `panic!` powoduje wyświetlenie komunikatu o błędzie zawarty w dwóch ostatnich wierszach.
Pierwsza linia pokazuje komunikat paniki i miejsce w naszym kodzie źródłowym, w którym
wystąpiła panika: *src/main.rs:2:5* wskazuje, że jest to druga linia,
piąty znak w pliku *src/main.rs*.

W tym przypadku wskazana linia jest częścią naszego kodu, a jeśli do niej przejdziemy
zobaczymy wywołanie makra `panic!`. W innych przypadkach wywołanie `panic!` może znajdować 
się w kodzie wywoływanym przez nasz kod, a nazwa pliku i numer linii zgłoszone przez 
komunikat o błędzie będą kodem innej osoby, w którym wywoływane jest makro `panic!`, 
a nie linią naszego kodu, która ostatecznie doprowadziła do wywołania `panic!`.
Możemy użyć stosu wywołań funkcji, z których pochodzi wywołanie `panic!`, 
aby dowiedzieć się, która część naszego kodu powoduje problem. Stosy wywołań omówimy
bardziej szczegółowo w następnej części.

### Używanie Stosu Wywołań `panic!`

Spójrzmy na inny przykład, aby zobaczyć, jak to jest, gdy wywołanie `panic!` 
pochodzi z biblioteki z powodu błędu w naszym kodzie, a nie z naszego kodu 
wywołującego makro bezpośrednio. 
Listing 9-1 zawiera kod, który próbuje uzyskać 
dostęp do indeksu w wektorze poza zakresem prawidłowych indeksów.

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

<span class="caption">Listing 9-1: Próba uzyskania dostępu do elementu poza
końcem wektora, co spowoduje wywołanie `panic!`.</span>

Tutaj próbujemy uzyskać dostęp do setnego elementu naszego wektora (który 
znajduje się w indeksie 99, ponieważ indeksowanie zaczyna się od zera),
ale wektor ma tylko 3 elementy. W tej sytuacji Rust wpadnie w panikę. 
Użycie `[]` powinno zwrócić element, ale jeśli podasz nieprawidłowy indeks, 
Rust nie będzie w stanie zwrócić żadnego elementu, który byłby poprawny.

W języku C próba odczytu poza końcem struktury danych jest niezdefiniowanym 
zachowaniem. Można uzyskać to, co znajduje się w miejscu w pamięci, które
odpowiadałoby temu elementowi w strukturze danych, nawet jeśli pamięć nie należy
do tej struktury. Nazywa się to *przepełnieniem bufora* i może prowadzić do luk
w zabezpieczeniach, jeśli atakujący jest w stanie manipulować indeksem 
w taki sposób, aby odczytać dane, do których nie powinien mieć dostępu,
a które są przechowywane za strukturą danych.

Aby chronić swój program przed tego rodzaju lukami, jeśli spróbujesz odczytać
element w indeksie, który nie istnieje, Rust zatrzyma wykonywanie 
i odmówi kontynuowania. Wypróbujmy to i zobaczmy:

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

Ten błąd wskazuje na linię 4 naszego pliku `main.rs`, gdzie próbujemy uzyskać
dostęp do indeksu 99. Następna linia notatki mówi nam, że możemy ustawić 
zmienną środowiskową `RUST_BACKTRACE`, aby uzyskać stos wysołań tego, 
co dokładnie spowodowało błąd. *Stos wywołań* to lista wszystkich funkcji,
które zostały wywołane, aby dotrzeć do tego punktu. Stos wywołań w Ruście działa 
podobnie jak w innych językach: kluczem do odczytania stosu wywołań jest
rozpoczęcie od góry i czytanie, aż zobaczysz pliki, które napisałeś. Jest to miejsce,
w którym pojawił się problem. Linie powyżej tego miejsca to kod, który został wywołany
przez twój kod; linie poniżej to kod, który wywołał twój kod. Te linie przed i po mogą
zawierać kod rdzenia Rusta, kod biblioteki standardowej lub używane skrzynki.
Spróbujmy uzyskać stos wywołań, ustawiając zmienną środowiskową `RUST_BACKTRACE`
na dowolną wartość z wyjątkiem 0. Listing 9-2 pokazuje dane wyjściowe podobne do tego, co zobaczysz.

<!-- manual-regeneration
cd listings/ch09-error-handling/listing-09-01
RUST_BACKTRACE=1 cargo run
copy the backtrace output below
check the backtrace number mentioned in the text below the listing
-->

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:142:14
   2: core::panicking::panic_bounds_check
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:84:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:242:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:18:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/alloc/src/vec/mod.rs:2591:9
   6: panic::main
             at ./src/main.rs:4:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

<span class="caption">Listing 9-2: Stos wywołań wygenerowany przez wywołanie `panic!` wyświetlany,
gdy ustawiona jest zmienna środowiskowa `RUST_BACKTRACE`.</span>

To bardzo dużo danych wyjściowych! Dokładne dane wyjściowe mogą się różnić
w zależności od systemu operacyjnego i wersji Rusta. Aby uzyskać stosy wywołań
z tymi informacjami, symbole debugowania muszą być włączone. Symbole debugowania
są domyślnie włączone podczas korzystania z `cargo build` lub `cargo run`
bez flagi `--release`, tak jak tutaj.

W danych wyjściowych na listingu 9-2, linia 6 stosu wywołań wskazuje na linię
w naszym projekcie, która powoduje problem: linia 4 pliku *src/main.rs*. Jeśli nie chcemy, 
aby nasz program wpadł w panikę, powinniśmy rozpocząć nasze dochodzenie w miejscu wskazywanym
przez pierwszą linię wspominającą o napisanym przez nas pliku. Na listingu 9-1, 
gdzie celowo napisaliśmy kod, który spowodowałby panikę, sposobem na naprawienie paniki jest
nie żądanie elementu spoza zakresu indeksów wektora. Gdy kod będzie wpadał w panikę w przyszłości, 
trzeba będzie dowiedzieć się, jakie działania kod wykonuje z jakimi wartościami, aby wywołać panikę
i co kod powinien zrobić zamiast tego.

Wrócimy do `panic!` i omówimy kiedy powinniśmy, a kiedy nie powinniśmy używać `panic!` 
do obsługi warunków błędu w sekcji ["To `panic!` or Not to`panic!`"][to-panic-or-not-to-panic]
w dalszej części tego rozdziału. Następnie przyjrzymy się, jak odzyskać
dane po błędzie przy użyciu `Result`.

[to-panic-or-not-to-panic]:
ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic

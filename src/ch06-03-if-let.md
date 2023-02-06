<!-- ## Concise Control Flow with `if let` -->
## Zwięzła Kontrola Przepływu z `if let`.

Składnia `if let` łączy `if` i `let`, by obsłużyć wartości pasujące do wzorca. Składnia ta jest zwięzła, ale (bez powtarzania `if let`) pozwala podać tylko jeden wzorzec.
Rozważmy program z Listingu 6-6, który dopasowuje wartość zmiennej `config_max` typu `Option<u8>`, ale chce wykonać kod tylko jeśli ta wartość jest wariantem `Some`.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

<span class="caption">Listing 6-6: `match` wykonujący kod jedynie gdy wartość jest `Some`</span>

Jeśli wariantem jest `Some`, to wypisujemy zawartą w nim wartość przypisując ją uprzednio do zmiennej `max` we wzorcu.
Z wariantem `None` nie chcemy nic robić. Aby spełnić jednak wymóg wyczerpywalności wyrażenia `match`, musimy dodać niewiele znaczące, szablonowe `_ => ()` po przetworzeniu tylko jednego wariantu, co jest irytuje.

W zamian możemy napisać to samo krócej używając `if let`.
Następujący kod zachowuje się tak samo jak `match` z Listingu 6-6:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

Składnia `if let` przyjmuje wzorzec i wyrażenie oddzielone znakiem równości.
Działa tak samo jak `match`, gdzie wyrażenie jest podane do `match`, a wzorzec jest jego pierwszą odnogą.
W tym przypadku wzorzec to `Some(max)`, a `max` zostaje zainicjowane wartością z wnętrza `Some`.
Możemy wtedy użyć `max` w ciele bloku `if let` w taki sam sposób, w jaki użyliśmy `max` w odpowiedniej odnodze `match`.
Kod w bloku `if let` nie jest uruchamiany, jeśli wartość nie pasuje do wzorca.

Używanie `if let` oznacza mniej pisania, mniej wcięć i mniej niewiele znaczącego, szablonowego kodu.
Jednakże, w stosunku do `match`, tracimy sprawdzanie wyczerpywalności.
Wybór pomiędzy `match` a `if let` zależy tego, co jest dla nas w danej sytuacji ważniejsze, uzyskanie zwięzłości czy sprawdzanie wyczerpywalności.

Innymi słowy, można myśleć o `if let` jako o składniowym lukrze dla `match`, który uruchamia kod tylko gdy wartość pasuje do podanego wzorca, równocześnie nie robiąc nic gdy nie pasuje.

Można także do `if let` dołączyć `else`.
Blok kodu stojący za `else` pełni taką samą rolę, jak blok dla odnogi `_` w wyrażeniu `match` równoważnym do danego `if let` z `else`.
Proszę sobie przypomnieć definicję typu wyliczeniowego `Coin` z Listingu 6-4, w którym wariant `Quarter` posiada wartość `UsState`.
Za pomocą następującego wyrażenia `match` możemy policzyć wszystkie widziane monety niebędące ćwiartkami, jednocześnie informując o stanie, z którego pochodzą ćwiartki:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

To samo możemy też uzyskać za pomocą wyrażenia `if let` z `else`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

O `if let` warto pamiętać w sytuacji, w której wyrażenie logiki za pomocą `match` jest zbyt rozwlekłe.

<!-- ## Summary -->
## Podsumowanie

Pokazaliśmy, jak używać enumeracji do tworzenia typów, których zmienne mogą być jedną z zestawu wyliczonych wartości.
Wskazaliśmy też, jak typ `Option<T>` biblioteki standardowej wykorzystuje systemu typów, aby uniknąć błędów.
W zależności od tego, ile przypadków trzeba obsłużyć, można użyć `match` lub `if let` do wyodrębnienia i użycia wartości zawartych wewnątrz wariantów enuma.

Programy Rusta mogą teraz wyrażać koncepcje w danej domenie za pomocą struktur i enumów.
Utworzenie niestandardowych typów i użycie ich w API zapewnia bezpieczeństwo: kompilator dba o to, aby funkcje otrzymywały tylko wartości oczekiwanego typu.

Przejdźmy teraz do omówienia modułów Rusta, które pozwalają wyrazić API, które jest dobrze zorganizowane, proste w użyciu i eksponuje tylko to, czego potrzebują użytkownicy.
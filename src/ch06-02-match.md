<!-- Old heading. Do not remove or links may break. -->
<a id="the-match-control-flow-operator"></a>
<!-- ## The `match` Control Flow Construct -->
## Konstrukcja Przepływu Sterowania `match`

Rust posiada niezwykle potężną konstrukcję przepływu sterowania `match`, która pozwala na porównanie wartości z serią wzorców, a następnie wykonanie kodu przypisanego do pasującego wzorca.
Wzorce mogą składać się z literałów, nazw zmiennych, wieloznaczników (ang. woldcards) i wielu innych rzeczy;
[rozdział 18][ch18-00-patterns]<!-- ignore --> objaśnia działanie wszystkich rodzajów wzorców.
Siła `match` wynika z ekspresyjności wzorców i faktu, że kompilator sprawdza, czy wszystkie możliwe przypadki są obsługiwane.

Na wyrażeniu `match` można spojrzeć jak na maszynę do sortowania monet:
monety zjeżdżają po torze wzdłuż którego znajdują się różnej wielkości otwory i każda wpada w pierwszy napotkany otwór, do którego pasuje. W ten sam sposób wartości przechodzą przez każdy wzorzec w `match`, aż do napotkania pierwszego wzorca, do którego wartość "pasuje". Po jego napotkaniu, wartość wpada do powiązanego z tym wzorcem bloku kodu, który zostaje wykonany.

Skoro mowa o monetach, to użyjmy ich jako przykładu z wykorzystaniem `match`!
Możemy napisać funkcję, która pobiera nieznaną amerykańską monetę i tak jak maszyna licząca określa, jaka to moneta, oraz zwraca jej wartość w centach, jak pokazano na listingu 6-3.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

<span class="caption">Listing 6-3: Wyrażenie `match` dopasowujące warianty enuma do wzorców</span>

Rozłóżmy `match` w funkcji `value_in_cents` na czynniki pierwsze.
Najpierw umieszczamy słowo kluczowe `match`, po którym następuje wyrażenie, którym w tym przypadku jest wartość `coin`.
To wyrażenie pełni podobną rolę do wyrażenia warunkowego używanego z `if`, ale jest duża różnica: w `if` warunek musi dawać wartość boolowską, zaś tutaj wyraźnie może być dowolnego typu. Typem `coin` w tym przykładzie jest enum `Coin`, który zdefiniowaliśmy w pierwszej linii.

Następne są odnogi `match`. Odnoga składa się z dwóch części: wzorca i kodu. Pierwsza odnoga ma wzorzec, którym jest wartość `Coin::Penny`, a następnie operator `=>`, który oddziela wzorzec i kod do uruchomienia. Kodem w tym przypadku jest po prostu wartość `1`. Każda odnoga jest oddzielone od następnej przecinkiem.

Wykonanie wyrażenia `match` polega na porównaniu wartości wynikowej z wzorcem każdej z odnóg, w kolejności ich wystąpienia.
Jeśli wzorzec pasuje do wartości, wykonywany jest kod związany z tym wzorcem.
Jeśli wzorzec nie pasuje do wartości, wykonanie przechodzi do następnej odnogi, zupełnie jak w maszynie do sortowania monet.
Możemy mieć tyle odnóg ile potrzebujemy. Na listingu 6-3, nasz `match` ma ich cztery.

Kod związany z każdą odnogą jest wyrażeniem, a wartość wynikowa wyrażenia w pasującej odnodze jest wartością, która zostaje zwrócona z całego wyrażenia `match`.

Zazwyczaj nie używamy nawiasów klamrowych, jeśli kod ramienia odnogi jest krótki, tak jak na listingu 6-3, gdzie każda odnoga jedynie zwraca wartość. Chcąc uruchomić wiele linii kodu w jednej odnodze należy użyć nawiasów klamrowych, a przecinek po odnodze jest wtedy opcjonalny. Na przykład poniższy kod drukuje „Szczęśliwy pens!“ za każdym razem, gdy metoda jest wywoływana z `Coin::Penny`, ale wciąż zwraca ostatnią wartość bloku, `1`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

<!-- ### Patterns That Bind to Values -->
### Wzorce Deklarujące Zmienne

Inną przydatną cechą odnóg match jest to, że mogą one tworzyć zmienne zainicjowane fragmentami wartości pasującej do wzorca.
Tym samym pozwalają wyodrębnić wartości z wariantów enuma.

By to zilustrować, zmienimy jeden z wariantów naszego wyliczenia tak, aby przechowywał on wewnątrz dane.
Od 1999 do 2008 roku Stany Zjednoczone biły ćwierćdolarówki mające po jednej ze stron różne wzory dla każdego z 50 stanów.
Żadna inna moneta nie miała wzorów stanowych, więc tylko ćwiartki będą miały dodatkową wartość. Możemy ją uwzględnić w naszym typie `enum` poprzez zmianę wariantu `Quarter` tak, aby zawierał wartość typu `UsState`, co zrobiliśmy na listingu 6-4.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

<span class="caption">Listing 6-4: Enum `Coin` z wariantem `Quarter` trzymającym wartość typu `UsState`</span>

Wyobraźmy sobie, że znajomy chce zebrać wszystkie 50 ćwiartek stanowych.
Segregując nasze drobniaki według typów monet, będziemy podawać nazwę stanu związanego z każdą ćwiartką,
by nasz przyjaciel mógł dodać ją do swojej kolekcji, gdy jeszcze takiej ćwiartki nie posiada.

W kodzie wyrażenia match dodajemy zmienną o nazwie `state` do wzorca dopasowującego wariant `Coin::Quarter`.
Kiedy `Coin::Quarter` zostanie dopasowane, zmienna `state` zostanie utworzona i zainicjowana wartością wskazującą stan, z którego pochodzi ćwiartka.
Następnie `state` może zostać użyte w kodzie tej odnogi, co pokazuje przykład:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

W wywołaniu `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin` miałoby wartość `Coin::Quarter(UsState::Alaska)`.
Próby dopasowania tej wartości do kolejnych odnóg match zakończyłyby się sukcesem dopiero po dotarciu do `Coin::Quarter(state)`.
Wtedy zostałaby utworzona zmienna `state` o wartość `UsState::Alaska`.
Ta zmienna zostałaby użyta w wyrażeniu `println!`, dając mu dostęp do wartości przechowywanej wewnątrz wariantu `Quarter` enuma `Coin`.

<!-- ### Matching with `Option<T>` -->
### Dopasowywanie do `Option<T>`

W poprzedniej sekcji chcieliśmy wydobyć wewnętrzną wartość typu `T` z wariantu `Some` enuma `Option<T>`; możemy również obsłużyć `Option<T>` używając `match`, podobnie jak zrobiliśmy to z typem wyliczeniowym `Coin`!
Zamiast porównywać monety, będziemy porównywać warianty `Option<T>`, ale sposób działania wyrażenia `match` będzie taki sam.

Powiedzmy, że chcemy napisać funkcję, która przyjmuje `Option<i32>` i jeśli w środku jest jakaś wartość, to dodaje do niej 1.
Jeśli w środku nie ma żadnej wartości, funkcja powinna zwrócić wartość `None`, nie robiąc nic więcej.

Dzięki `match` taka funkcja jest bardzo łatwa do napisania i wygląda jak na listingu 6-5.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

<span class="caption">Listing 6-5: Funkcja używająca wyrażenia `match` dla `Option<i32>`</span>

Przeanalizujmy bardziej szczegółowo pierwsze wykonanie `plus_one`.
W wywołaniu `plus_one(five)`, zmienna `x` ma wartość `Some(5)` i zostanie porównana z każdą odnogą `match`:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

Wartość `Some(5)` nie pasuje do wzorca `None`, nastąpi więc przejęcie do kolejnej odnogi:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

Czy `Some(5)` pasuje do `Some(i)`? Tak! To ten sam wariant.
Zostaje zadeklarowana zmienna `i`, zainicjowana wartością zawartą w `Some`, czyli `5`.
Następnie wykonywany jest kod w wybranej odnodze match, który dodaje 1 do wartości `i` i tworzymy nową wartość `Some` z uzyskaną sumą `6` w środku.

Rozważmy teraz drugie wywołanie `plus_one` z listingu 6-5, w którym `x` jest `None`.
Następuje jego porównanie do pierwszej odnogi `match`:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

Pasuje! Nie ma żadnej wartości do zwiększenia, więc program zatrzymuje się i zwraca wartość `None` po prawej stronie `=>`.
Ponieważ pierwsza odnoga pasowała, to pozostałe nie są już sprawdzane.

Używanie `match` z typami wyliczeniowymi jest przydatne w wielu sytuacjach.
Często można spotkać następujący scenariusz: enum jest dopasowywany za pomocą `match`, następnie z jego wewnętrznymi danymi związywana jest zmienna, która jest używana w kodzie przewidzianym dla wybranego wariantu. Początkowo może się to wydawać nieco trudne, ale po przyzwyczajeniu, okazuje się bardzo wygodne. Jest to niezmiennie ulubione narzędzie Rustowców.

<!-- ### Matches Are Exhaustive -->
### Match Jest Wyczerpujący

Jest jeszcze jeden aspekt `match`, który musimy omówić: wzorce odnóg muszą uwzględniać wszystkie możliwości.
Rozważmy następującą, błędną wersję naszej funkcji `plus_one`, która się nie skompiluje:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

Ten kod spowoduje błąd, bo nie uwzględnia przypadku `None`.
Na szczęście Rust z łatwością zauważy problem.
Jeśli spróbujemy skompilować ten kod, otrzymamy taki błąd:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

Rust wie, że nie uwzględniliśmy wszystkich możliwych przypadków, a nawet wie, o którym wzorcu zapomnieliśmy!
Dopasowania w Rustowym `match` są *wyczerpujące*: musimy wyczerpać wszelkie możliwości, aby kod był poprawny.
Szczególnie w przypadku `Option<T>`, gdy Rust pilnuje byśmy jawnie obsłużyli przypadek `None`, chroni nas przed błędnym założeniem, że wartość zawsze jest dostępna, uniemożliwiając tym samym omawiany wcześniej błąd wart miliarda dolarów.

### Wzorce Pasujące do Wszystkiego i Placeholder `_`

Możemy też podjąć specjalne czynności jedynie dla kilku konkretnych wartości enuma, zaś dla wszystkich pozostałych wykonać jedno domyślne działanie.
Wyobraźmy sobie, że implementujemy grę, w której, jeśli wyrzucimy 3 na kostce, nasz gracz nie porusza się, ale za to dostaje nowy kapelusz. Jeśli wyrzucimy 7, nasz gracz traci kapelusz. Dla wszystkich innych wartości, nasz gracz przesuwa się po planszy o wyrzuconą liczbę oczek. Oto `match`, który implementuje tę logikę, z wynikiem rzutu kostką zakodowanym na sztywno zamiast wylosowanym, oraz całą pozostałą logiką reprezentowaną przez funkcje, których ciał nie podano, ponieważ ich implementacja wykracza poza zakres omawianego przykładu:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

Dla dwóch pierwszych odnóg wzorcami są literały `3` i `7`.
Dla ostatniej odnogi, która obejmuje każdą inną możliwą wartość, wzorcem jest zmienna, którą nazwaliśmy `other`.
Kod uruchomiony dla tej odnogi przekazuje wartość `other` do funkcji `move_player`.

Ten kod kompiluje się, pomimo że nie wymieniliśmy wszystkich możliwych wartości, jakie może przyjąć `u8`, ponieważ ostatni wzorzec dopasuje wszystkie wartości, które nie zostały wymienione. Dzięki temu pasującemu do wszystkiego wzorcowi spełniony jest wymóg, że `match` musi być wyczerpujący.
Proszę zauważyć, że odnoga pasująca do wszystkiego powinna być ostatnia, ponieważ wzorce są analizowane po kolei.
Jeśli umieścilibyśmy ją wcześniej, dalsze odnogi nigdy by nie zostały wybrane.
Dlatego Rust ostrzega, gdy dodamy odnogi po takiej, która pasuje do wszystkiego.

Rust posiada również wzorzec, którego możemy użyć, gdy chcemy dopasować wszystko, ale nie chcemy *używać* dopasowanej wartości: `_` jest specjalnym wzorcem, który pasuje do dowolnej wartości i równocześnie nie przypisuje sobie wartością. Jego użycie mówi Rustowi, że nie zamierzamy używać wartości, więc Rust nie będzie ostrzegał o nieużywanej zmiennej.

Zmieńmy zasady gry: teraz po wyrzuceniu czegokolwiek innego niż 3 lub 7, należy rzucić ponownie.
I wtedy nie ma znaczenia co wypadło, więc zmienna `other` nie jest dalej potrzebna i możemy ją zastąpić symbolem `_`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

Ten przykład również spełnia wymóg wyczerpywalności, ponieważ wszystkie pozostałe wartości są w ostatniej odnodze ignorowane jawnie; nie zapomnieliśmy o niczym.

Na koniec zmienimy jeszcze raz zasady gry tak, aby wyrzucenie czegokolwiek innego niż 3 lub 7, nie miało żadnych następstw.
Możemy to wyrazić używając jako kodu w odnodze `_` wartości jednostkowej (czyli pustej krotki, o czym wspominaliśmy w sekcji [„Krotki“][tuples]<!-- ignore -->):

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

Tutaj mówimy Rustowi wprost, że nie będziemy używać żadnej wartości, która nie pasuje do wzorców poprzednich odnóg, i nie chcemy uruchamiać żadnego kodu w tym przypadku.

Więcej o wzorcach i dopasowywaniu powiemy w [rozdziale 18][ch18-00-patterns]<!-- ignore -->.
Na razie jednak przejdziemy do składni `if let`, która może być przydatna w sytuacjach, w których wyrażenie `match` jest zbyt rozwlekłe.

[tuples]: ch03-02-data-types.html#krotki
[ch18-00-patterns]: ch18-00-patterns.html

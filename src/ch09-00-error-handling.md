# Obsługa błędów

Prędzej czy później napotkasz błędy w oprogramowaniu, które tworzysz, dlatego
Rust posiada szereg funkcji pozwalających radzić sobie z sytuacjami, w których
coś pójdzie nie tak. W wielu przypadkach Rust wymaga, abyś potwierdził możliwość
wystąpienia błędu i podjął pewne działania, zanim kod zostanie skompilowany. To
sprawia, że Twój program jest bardziej niezawodny, zapewniając wykrycie błędów i
odpowiednią ich obsługę przed wdrożeniem na środowisko produkcyjne!

Rust dzieli błędy na dwie główne kategorie: błędy *możliwe do naprawienia* i
błędy *nienaprawialne*. W przypadku błędu, które można naprawić, np. *plik
nieodnaleziony*, najprawdopodobniej chcemy po prostu zgłosić problem
użytkownikowi i ponowić operacje. Błędy nienaprawialne są zawsze objawami
problemów, takimi jak próba uzyskania dostępu do lokalizacji znajdującej się
poza końcem tablicy, dlatego chcemy natychmiast zatrzymać program.

Większość języków nie rozróżnia tych dwóch rodzajów błędów i obsługuje oba w ten
sam sposób, wykorzystując mechanizmy takie jak wyjątki (ang. exception). Rust
nie ma wyjątków. Zamiast tego ma typ `Result<T, E>` dla możliwych do naprawienia
błędów i makro `panic!`, które zatrzymuje wykonywanie, gdy program napotka
nieodwracalny błąd. W tym rozdziale omówiono wywoływanie makra `panic!`, a
następnie omówiono zwracanie wartości `Result<T, E>`. Dodatkowo przeanalizowano
kwestie, które należy wziąć pod uwagę przy podejmowaniu decyzji, czy spróbować
zakończyć program na wybrany sposób, po wystąpieniu błędu, czy zatrzymać
wykonywanie.

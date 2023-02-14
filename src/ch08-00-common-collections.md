<!-- # Common Collections -->
# Często Wykorzystywane Kolekcje

Biblioteka standardowa Rusta zawiera wiele bardzo użytecznych struktur danych zwanych *kolekcjami*.
Podczas gdy większość innych typów danych reprezentuje pojedynczą wartość, kolekcje zwykle przechowują ich wiele.
Równocześnie, w przeciwieństwie do wbudowanych typów tablic i krotek, kolekcje przechowują dane na stercie.
To oznacza, że ilość tych danych nie musi być znana w czasie kompilacji i może rosnąć lub maleć w trakcie działania programu.
Każdy rodzaj kolekcji cechują inne możliwości oraz koszty, zaś dobranie odpowiedniej kolekcji do sytuacji jest umiejętnością, której nabycie zwykle zajmuje trochę czasu.
W tym rozdziale omówimy trzy kolekcje, które są bardzo często wykorzystywane w programach napisanych w Ruście:

* *Wektor* (*vector*) przechowuje pewną liczbę wartości, jedna obok drugiej.
* *Łańcuch* (*string*) jest kolekcją znaków. O typie `String` wspominaliśmy już wcześniej, ale w tym rozdziale omówimy go dogłębnie.
* *Mapa haszująca* (*hash map*) wiąże wartości z kluczami, stanowiąc tym samym szczególną implementację bardziej ogólnej struktury danych zwanej *mapą*.

Informacje na temat innych rodzai kolekcji zawartych w bibliotece standardowej, można znaleźć w jej [dokumentacji][collections].

Omówimy, jak tworzyć i aktualizować wektory, łańcuchy i mapy haszujące, a także co czyni każdą z tych kolekcji wyjątkową.

[collections]: ../std/collections/index.html

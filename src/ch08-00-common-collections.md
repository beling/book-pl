<!-- # Common Collections -->
# Często Wykorzystywane Kolekcje

Biblioteka standardowa Rusta zawiera wiele bardzo użytecznych struktur danych zwanych *kolekcjami*.
Podczas gdy większość innych typów danych reprezentuje pojedynczą wartość, kolekcje zwykle przechowują ich wiele.
Równocześnie, w przeciwieństwie do wbudowanych typów tablic i krotek, kolekcje przechowują dane na stercie.
To oznacza, że ilość tych danych nie musi być znana w czasie kompilacji i może rosnąć lub maleć w trakcie działania programu.
Każdy rodzaj kolekcji cechują inne możliwości oraz koszty, zaś dobranie odpowiedniej kolekcji do sytuacji jest umiejętnością, której nabycie zwykle zajmuje trochę czasu.
W tym rozdziale omówimy trzy kolekcje, które są bardzo często wykorzystywane w programach napisanych w Ruście:

* A *vector* allows you to store a variable number of values next to each other.
* A *string* is a collection of characters. We’ve mentioned the `String` type
  previously, but in this chapter we’ll talk about it in depth.
* A *hash map* allows you to associate a value with a particular key. It’s a
  particular implementation of the more general data structure called a *map*.

To learn about the other kinds of collections provided by the standard library,
see [the documentation][collections].

We’ll discuss how to create and update vectors, strings, and hash maps, as well
as what makes each special.

[collections]: ../std/collections/index.html

<!-- # Managing Growing Projects with Packages, Crates, and Modules -->
# Zarządzanie Rozrastającymi Się Projektami Za Pomocą Pakietów, Skrzyń i Modułów

W miarę pisania dużych programów, coraz ważniejsza staje się organizacja kodu.
Dzięki pogrupowaniu powiązanych funkcjonalności i porozdzielaniu kodu,
ułatwiamy odnalezienie w nim miejsc odpowiedzialnych za daną funkcjonalność,
a zatem i ewentualne dokonanie zmian sposobu jej działania.

Programy, które dotychczas napisaliśmy, mieściły się w jednym module, w jednym pliku.
Gdy projekt się rozrasta, warto uporządkować kod, dzieląc go na wiele modułów, a następnie wiele plików.
Pakiet może zawierać wiele skrzyń binarnych i opcjonalnie jedną skrzynię biblioteczną.
W miarę jak pakiet rośnie, można wyodrębnić jego części do oddzielnych skrzyń, które stają się zewnętrznymi zależnościami.
Ten rozdział omawia wszystkie te techniki.
Dla bardzo dużych projektów składających się z zestawu powiązanych ze sobą pakietów, które są rozwijane wspólnie, Cargo udostępnia *przestrzenie robocze*, które omówimy w sekcji [„Przestrzenie Robocze Cargo“][workspaces]<!-- ignore --> rozdziału 14.

Omówimy również enkapsulację szczegółów implementacji, która pozwala na ponowne użycie kodu na wyższym poziomie: po zaimplementowaniu operacji, inny kod może wywołać nasz kod poprzez jego publiczny interfejs, bez konieczności znajomości szczegółów implementacji.
Sposób, w jaki piszemy kod określa, które części są publiczne do wykorzystania przez inny kod, a które są prywatnymi szczegółami implementacji i, w związku z tym, zastrzegamy sobie prawo do ich swobodnej zmiany.
Jest to kolejny sposób na ograniczenie liczby szczegółów, nad którymi musimy zapanować.

A related concept is scope: the nested context in which code is written has a
set of names that are defined as “in scope.” When reading, writing, and
compiling code, programmers and compilers need to know whether a particular
name at a particular spot refers to a variable, function, struct, enum, module,
constant, or other item and what that item means. You can create scopes and
change which names are in or out of scope. You can’t have two items with the
same name in the same scope; tools are available to resolve name conflicts.

Rust has a number of features that allow you to manage your code’s
organization, including which details are exposed, which details are private,
and what names are in each scope in your programs. These features, sometimes
collectively referred to as the *module system*, include:

* **Packages:** A Cargo feature that lets you build, test, and share crates
* **Crates:** A tree of modules that produces a library or executable
* **Modules** and **use:** Let you control the organization, scope, and
  privacy of paths
* **Paths:** A way of naming an item, such as a struct, function, or module

In this chapter, we’ll cover all these features, discuss how they interact, and
explain how to use them to manage scope. By the end, you should have a solid
understanding of the module system and be able to work with scopes like a pro!

[workspaces]: ch14-03-cargo-workspaces.html

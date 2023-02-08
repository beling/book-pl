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
Wszystko to jest kolejnym sposobem na ograniczenie liczby szczegółów, które musimy mieć w głowie.

Powiązanym pojęciem jest zasięg: osadzony kontekst, w którym pisany jest kod i który zawiera zestaw nazw, o których mówimy, że są „w zasięgu“. Czytając, pisząc i kompilując kod, programiści i kompilatory muszą wiedzieć, czy dana nazwa w danym miejscu odnosi się do zmiennej, funkcji, struktury, enumeracji, modułu, stałej lub innego elementu i co ten element oznacza.
Można tworzyć zasięgi i decydować, które nazwy są w zasięgu a które poza nim. Nie można mieć jednak dwóch elementów o tej samej nazwie w tym samym zasięgu; ale dostępne są narzędzia do rozwiązywania konfliktów nazw.

Rust posiada szereg rozwiązań, które pozwalają zarządzać organizacją kodu. W szczególności pozwalają one decydować które szczegóły są eksponowane, które są prywatne, oraz jakie nazwy znajdują się w poszczególnych zasięgach. Te rozwiązania czasami nazywane są zbiorczo *systemem modułów* i obejmują:

* **Pakiety** (ang. *packages*): Funkcjonalność Cargo umożliwiająca budowanie, testowanie i udostępnianie skrzyń
* **Skrzynie** (ang. *crates*): Drzewo modułów tworzących bibliotekę lub plik wykonywalny
* **Moduły** (ang. *modules*) i **użycia** (ang. *use*): Pozwala kontrolować organizację, zasięg i prywatność ścieżek
* **Ścieżki** (ang. *paths*): Sposób nazywania elementu, takiego jak struktura, funkcja lub moduł

W tym rozdziale omówimy wszystkie te rozwiązania i interakcje między nimi. Wyjaśnimy też jak używać ich do zarządzania zasięgiem.
Na koniec powinieneś dogłębnie zrozumieć system modułów i być w stanie pracować z zasięgami jak zawodowiec!

[workspaces]: ch14-03-cargo-workspaces.html

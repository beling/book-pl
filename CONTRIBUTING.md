## Dla początkujących tłumaczy Książki Rusta
[![Polski Krabuś](https://user-images.githubusercontent.com/43069023/76151831-6fda1180-60b9-11ea-9dcd-056206e80a45.png)](https://rustbookpl.zulipchat.com/join/vymxgwc8gkwtqymcrp2933yn/)

Zarówno oryginalny *The Rust Programming Language*, jak i
polska próba tłumaczenia *Język Programowania Rust* są projektami
open-source i istnieją dzięki pomocy społeczności.

Chętnie przyjmiemy wszelką pomoc, zarówno jeśli znajdziesz błąd
w tłumaczeniu, literówkę, jakąś niejasność, zbytne odejście
od oryginału czy może chciałbyś przetłumaczyć jakiś rozdział,
lub cokolwiek innego co przykuło twoją uwagę... 
Dziękujemy Ci za pomoc!

### Co i jak

Sama książka znajduje się w folderze `src` i jest napisana w formacie Markdown.
W folderze `listing` znajdziesz wycinku kodu Rusta,
outputy programów, które także należy przetłumaczyć!
W folderze `dot` znajdziesz rysunki; rzadko są one załączane.
[W szablonie *Pull Requestów*](.github/pull_request_template.md) znajdziesz
checkboxy zawierające zasady tłumaczenia, których powineneś przestrzegać, np.
`rozdział` z małej itp.

### A więc chcesz pomóc tłumaczyć?

Świetnie! Jest jeszcze wiele do zrobienia! Oto kilka kroków,
które powinieneś wykonać, aby wszystko poszło gładko:

1. Aby wybrać (pod)rozdział do tłumaczenia otwórz *Bieżący stan tłumaczenia*
   w *Issue* #1.
   1. Wybierz wolne miejsce.
   2. Sprawdź obecnie otworzone *Pull Requesty*; paytchoo też człowiek, może
      zapomniał zaznaczyć :stuck_out_tongue_winking_eye:. 
   3. Jeśli w obu przypadkach nikt inny obecnie nie tłumaczy,
      przejdź do następnego kroku.
   
2. Jeśli jeszcze tego nie zrobiłeś zforkuj [to](https://github.com/paytchoo/book-pl)
   repozytorium.

3. W swoim nowym repo stwórz nowy branch o nazwie `chapter/xx-yy`, 
   - gdzie `xx-yy`
   to numer podrozdziału, np. dla pliku `ch12-00-an-io-project.md` byłby to 
   `chapter/12-00`.
   
4. Następnie, abyś był pewien, że podczas twojej pracy nikt inny nie zacznie
   tłumaczyć tej samej części;
   stwórz tzw. *Draft Pull Request*, 
   - jest to typ *Pull Requesta*, który nie jest
   jeszcze gotowy mergowania.
   1. Zatytułuj swój *Pull Request*, tak aby jasno
   wyjaśnił jaką zmianę wprowadziłeś! 
   2. W trakcie kiedy twój *PR* nie jest jeszcze
   gotowy do mergowania, możesz rozpocząc jego tytuł skrótem `[WIP]`
   (ang. Work In Progress).
   
5. Kiedy spełnisz wszystkie wymagania zawarte w szablonie (a znajdziesz go 
   [tutaj](.github/pull_request_template.md)) aktywuj swój *Pull Request*,
   aby inni wiedzieli, że uważasz swoje zmiany za gotowe do dodania.

6. Jeśli wszystko poszło gładko, po zmergowaniu twój *Pull Request*
   powinien pojawić się w głównym branchu `master`! Brawo!
   Dziękujemy ci serdecznie w umożliwianiu
   polskim Rustafarianom czytanie Książki w ojczystym języku!

### Chcę tłumaczyć, ale nie znam gita :(

[Tutaj](https://github.com/freeCodeCamp/how-to-contribute-to-open-source#using-version-control)
znajdziesz świetne materiały dotyczące gita, samego GitHuba,
tego jak tworzyć *Issue'y*, *Pull Requesty* itp. Jeśli mimo wszystko,
uważasz że wykracza to poza twoje możliwości (obiecujemy, że tak nie jest)
lub brakuje ci po prostu czasu, wyślij swoje tłumaczenie jednemu z *contributorów*,
aby wdrożył tą zmianę za ciebie.

### Chcę tłumaczyć, ale nie znam Markdowna

Nie szkodzi. Jest spore uproszczenie HTMLa, więc jeśli go znasz to będzie ci łatwo;
jeśli nie to też nie problem. [Tutaj](https://github.com/lauragift21/awesome-learning-resources#markdown)
znajdziesz materiały pozwalające szybko ci poznać Markdown.

### Chcę tłumaczyć, ale nie wiem czy mój angielski jest wystarczająco dobry

Jeżeli masz chęci, a nikt nie zaczął jeszcze tłumaczyć danego
(pod)rozdziału, to nie czekaj na oklaski i zacznij tłumaczyć!
Nie jesteś sam, poproś innych o pomoc, otwórz *Issue* lub
napisz na naszym [Zulipie](https://rustbookpl.zulipchat.com/).
Możesz potraktować to tłumaczenie jako ćwiczenie swojego angielskiego.


### Chcę tłumaczyć, ale wolno mi to idzie

*First world problem, huh?* Oczywiście żartuję, czas jest bardzo istotny
i może ci go brakować, zwłaszcza żonglując pracę, rodzinę, naukę, swoje hobby
i pomoc w open-source.

Poniżej stworzyłem (@JakubKoralewski) mój własny sposób, jaki wynalazłem, który
uważam, za dość wydajny. Absolutnie nie musisz go przestrzegać, jeśli nie chcesz!
Jest to tylko sugestia; wybierz elementy które ci pasują - lub też nic, to też
jest w porządku. Każdy ma własny sposób pracy. Jeśli uważasz, że można ten proces
jeszcze usprawnić, to koniecznie daj znać!!!

Korzystam z Windowsa 10.

#### [Computer Assisted Translation](https://pl.wikipedia.org/wiki/T%C5%82umaczenie_wspomagane_komputerowo).

Do swojego tłumaczenia używam [OmegaT](https://pl.wikipedia.org/wiki/OmegaT).
Jest to program open-source zawierający glosariusze, słowniki, pamięć tłumaczeniową; ale ja
używam go głównie po to, aby w prosty sposób wyświetlić obok siebie oryginalny
tekst w języku angielskim oraz własne tłumaczenie.

Po pierwsze, OmegaT domyślnie nie obsługuje formatu Markdown, w którym 
napisana jest Książka Rusta. Musisz zainstalować plugin [Okapi Filters](https://okapiframework.org/wiki/index.php?title=Okapi_Filters_Plugin_for_OmegaT).

Podczas otwierania pierwszego projektu w OmegaT, powita cię następujące okienko;
w moim wypadku jest już ono wypełnione:

![](https://i.imgur.com/4c9aAh9.png)

Zauważ, że pliki źródłowe i docelowe znajdują się w różnych folderach.

#### Machine translation

Aby w trakcie tłumaczenia wyświetliło ci się tłumaczenie maszynowe
(często pozwala ci wyłapać proste przypadki kiedy zbytnio komplikujesz proste
zdania widząc, że maszyna tłumaczy lepiej od ciebie :flushed:), w opcjach
OmegaT wejdź w `Ustawienia` -> `Preferences` -> `Machine Translation`.
Wybierz swoją usługę, ja korzystam z Yandex Translate (bo jest darmowy).
Zaznacz go, kliknij `Configure` i podaj klucz `API`, który możesz wygenerować
na https://translate.yandex.com/developers. Proces widoczny poniżej:

![](https://i.imgur.com/XoHIgKc.png)

Ustawiłem, też sobie ciemne tło bo szanuję oczy :sunglasses:
Możesz to zrobić (niestety ręcznie) wchodząc w `Appearance` -> `Colours`.

Po wybraniu pliku i przetłumaczeniu go, musisz wygenerować przetłumaczony plik:

![Projekt -> Utwórz dokumenty docelowe](https://i.imgur.com/8JtKAhG.png)

Następnie, plik zostanie zapisany w folderze, którym wybrałeś podczas 
tworzenia projektu. Skopiuj go teraz do folderu z plikami źródłowymi.
Upewnij się, że jesteś już wtedy na odpowiednim branchu, typu `chapter/06-02`.

Następnie kontynuuj tak jak zwykle.

### To by było na tyle
Podziel się swoimi sposobami tłumaczenia!


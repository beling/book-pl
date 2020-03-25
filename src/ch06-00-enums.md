# Wyliczenia i dopasowywanie wzorców

W tym rozdziale przyjrzymy się *wyliczeniom* (*ang. enumerations*),
czasem streszczanym także do samego: *enum*.
Wyliczeniami możesz zdefiniować jakiś typ wymieniając wszystkie jego wartości. 
Najpierw zdefiniujemy i wykorzystamy wyliczenia, pokazując że mogą one przekazywać zarówno
znaczenie, jak i dane. Później omówimy szczególnie przydatne wyliczenie, mianowicie `Option`, które
deklaruje, że dana wartość może być albo obecna lub nieobecna. 
Po tym, zerkniemy na to,
jak dopasowywanie wzorców w wyrażeniach `match` ułatwia wybór uruchomionego kodu
dla różnych wartości wyliczeń. A na koniec, zajmiemy się wyrażeniem `if let`
które jest kolejnym poręcznym i zwięzłym idiomem przydatnym do radzenia sobie
z wyliczeniami.

Wyliczenia istnieją w wielu językach, ale ich możliwości
w każdym języku się od siebie różnią. Wyliczenia w Ruście są najbardziej zbliżone do *algebraicznych typów danych* w
funkcjonalnych językach programowania, takich jak F#, OCaml i Haskell.

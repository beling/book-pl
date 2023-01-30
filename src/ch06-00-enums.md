# Wyliczenia i dopasowywanie wzorców

W tym rozdziale przyjrzymy się *wyliczeniom* (*ang. enumerations*),
czasem streszczanym także do samego: *enum*.
Wyliczeniami można zdefiniować jakiś typ wymieniając wszystkie jego wartości. 
Najpierw zdefiniujemy i wykorzystamy wyliczenia, pokazując że mogą one przekazywać zarówno
znaczenie, jak i dane. Później omówimy szczególnie przydatne wyliczenie, mianowicie `Option`, które
deklaruje, że dana wartość może albo być obecna albo nieobecna. 
Po tym, zerkniemy na to, jak dopasowywanie wzorców w wyrażeniach `match` pozwala wybrać kod do uruchomienia
dla różnych wartości wyliczeń. A na koniec, zajmiemy się wyrażeniem `if let`,
które jest kolejnym poręcznym i zwięzłym idiomem przydatnym w pracy z wyliczeniami.
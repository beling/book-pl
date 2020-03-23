fn main() {
    let s1 = daje_wlasnosc();       // daje_wlasnosc przenosi zwracaną
                                    // wartość do s1.

    let s2 = String::from("witaj"); // Rozpoczyna się zakres s2.

    let s3 = bierze_i_oddaje(s2);   // s2 zostaje przeniesiona do
                                    // bierze_i_oddaje, która jednocześnie
                                    // przenosi swoją wartość zwracaną do s3.
} // Tutaj kończy się zasięg s3 i jej dane zostają usunięte. Zasięg s2 też, ale
  // ponieważ jej dane przeniesiono, nic się nie dziejej. Zasięg s1 kończy się,
  // a jej dane zostają usunięte.

fn daje_wlasnosc() -> String {                // daje_wlasnosc przenosi jej
                                              // wartość zwracaną do funkcji,
                                              // która ją wywołała.

    let jakis_string = String::from("witaj"); // Początek zasięgu jakis_string.

    jakis_string                              // jakis_string jest zwracany i
                                              // przeniesiony do funkcji
                                              // wywołującej.
}

// bierze_i_oddaje pobiera dane w zmiennej String i zwraca je w innej.
fn bierze_i_oddaje(a_string: String) -> String { // Rozpoczyna się zasięg
                                                 // zmiennej a_string.

    a_string // a_string zostaje zwrócona i przeniesiona do funkcji wywołującej.
}

fn main() {
	let s = String::from("witaj");       // s pojawia się w zasięgu

	bierze_na_wlasnosc(s);      // Wartość zmiennej s przenosi się do funkcji...
	// ...i w tym miejscu zmienna jest już nieważna.

	let x = 5;                      // Zaczyna się zasięg zmiennej x.

	robi_kopie(x);           // Wartość x przeniosłaby się do funkcji, ale
	// typ i32 ma cechę Copy, więc w dalszym ciągu
	// można używać zmiennej x.

} // Tu kończy sie zasięg zmiennych x, a potem s. Ale ponieważ wartość s
  // została przeniesiona, nie dzieje się nic szczególnego.

fn bierze_na_wlasnosc(jakis_string: String) { // Zaczyna się zasięg some_string.
	println!("{}", jakis_string);
} // Tu kończy się zasięg jakis_string i wywołana zostaje funkcja `drop`.
  // Zajmowana pamięć zostaje zwolniona.

fn robi_kopie(jakas_calkowita: i32) { // Zaczyna się zasięg jakas_calkowita.
	println!("{}", jakas_calkowita);
} // Tu kończy się zasięg jakas_calkowita. Nic szczególnego się nie dzieje.


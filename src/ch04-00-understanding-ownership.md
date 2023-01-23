# Zrozumienie systemu własności

System własności (*ownership*) jest najbardziej unikalną cechą Rusta, która pozwala zagwarantować bezpieczeństwo pamięci bez konieczności użycia automatycznego systemu odśmiecania (*garbage collector*). Zrozumienie, jak w
Ruście funkcjonuje ten mechanizm jest zatem niezwykle istotne. W niniejszym rozdziale będziemy rozmawiać o własności oraz kilku powiązanych z nią funkcjonalnościach: pożyczaniu, wycinkach, a także o tym, jak Rust rozmieszcza dane w pamięci.

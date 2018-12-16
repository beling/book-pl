# Zrozumienie systemu własności

System własności (*ownership*) jest najbardziej unikalną cechą Rusta, która
pozwala językowi zagwarantować bezpieczeństwo pamięci bez konieczności użycia
automatycznego systemu odśmiecania (*garbage collector*). Bardzo ważne jest
zatem zrozumienie, jak własność funkcjonuje w Ruście. W tym rozdziale będziemy
rozmawiać o własności oraz kilku powiązanych funkcjonalnościach: pożyczaniu,
slices, a także o tym, jak Rust rozmieszcza dane w pamięci.

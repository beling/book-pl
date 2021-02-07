# Język Programowania Rust

Witaj na łamach książki „Język Programowania Rust”! Bieżąca wersja tekstu
zakłada, że używasz wersji Rusta 1.49 lub nowszej, z wyrażeniem
`edition="2018"` w pliku *Cargo.toml* wszystkich projektów, co pozwoli na użycie
wszystkich idiomów właściwych dla Edycji 2018. Zajrzyj do [sekcji "Instalacja"
w rozdziale 1][install]<!-- ignore --> celem uzyskania informacji o sposobach
instalacji lub aktualizacji Rusta oraz do [Dodatku E][editions]<!-- ignore -->,
żeby dowiedzieć się, czym charakteryzują się poszczególne edycje Rusta.

The 2018 Edition of the Rust language includes a number of improvements that
make Rust more ergonomic and easier to learn. This iteration of the book
contains a number of changes to reflect those improvements:

- Chapter 7, “Managing Growing Projects with Packages, Crates, and Modules,”
  has been mostly rewritten. The module system and the way paths work in the
  2018 Edition were made more consistent.
- Chapter 10 has new sections titled “Traits as Parameters” and “Returning
  Types that Implement Traits” that explain the new `impl Trait` syntax.
- Chapter 11 has a new section titled “Using `Result<T, E>` in Tests” that
  shows how to write tests that use the `?` operator.
- The “Advanced Lifetimes” section in Chapter 19 was removed because compiler
  improvements have made the constructs in that section even rarer.
- The previous Appendix D, “Macros,” has been expanded to include procedural
  macros and was moved to the “Macros” section in Chapter 19.
- Appendix A, “Keywords,” also explains the new raw identifiers feature that
  enables code written in the 2015 Edition and the 2018 Edition to interoperate.
- Appendix D is now titled “Useful Development Tools” and covers recently
  released tools that help you write Rust code.
- We fixed a number of small errors and imprecise wording throughout the book.
  Thank you to the readers who reported them!

Note that any code in earlier iterations of *The Rust Programming Language*
that compiled will continue to compile without `edition="2018"` in the
project’s *Cargo.toml*, even as you update the Rust compiler version you’re
using. That’s Rust’s backward compatibility guarantees at work!

Oryginalna, anglojęzyczna wersja książki w wersji HTML dostępna jest na stronie
[https://doc.rust-lang.org/stable/book/](https://doc.rust-lang.org/stable/book/)
oraz w trybie offline, instalowana wraz z Rustem przy użyciu `rustup`. Użyj
polecenia `rustup docs --book`, żeby ją otworzyć.

Anglojęzyczna wersja dostępna jest też w wersji [papierowej oraz ebooka poprzez wydawnictwo
No Starch Press][nsprust].

Niniejsze tłumaczenie jest zaś dostępna na stronie [http://rust.w8.pl/book/](http://rust.w8.pl/book/).

[install]: ch01-01-installation.html
[editions]: appendix-05-editions.html
[nsprust]: https://nostarch.com/rust

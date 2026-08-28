using macro `panic!()` do not alter fn signature. `#[should_panic]` annotation can assert that we expect the code to panic, you can also check the panic message by using `expected` -> `#[should_panic(expected = "Custom message")]`

Write factorail fn using (1) recursion (2) while loop. ==[a003_factorial]; (!!!) .fold()==

Kolejne macro `todo!()` które "suppressing type errors" ale podczas runtime będzie panic. Fajny dev-workflow

`while` loop dla pętli w których incrementujemy licznik wydaje się toporne, przykład while factorailrozwiązaniem jest `for` loop, Pyhon `range()` -> `1..5` ==(!!!)== `for i in i..(end + 1)` czyli mogą to expr. i to jeszcze w nawiasie.

Napisz `fn factorial(n)` za pomocą (3) `for` loop, (4) `match` recursion.

**number overflow/underflow** poprzez mechanizm radzenia sobie, wprowadza "Profile" do kompilacji, cztery podstawowe to: `dev, release, test, bench`, Dwa podstawowe do zaznajomienia to `dev` i `release` , podstawowe cmd cargo są by default `dev` co oznacza że `overflow-check` jest ustawiony na `true`, natomiast kompilując kod z flagą `--release`, `overflow-check` jest ustawiony na `false` czyli zawija over i underflow. Domyślną konfigurację można zmieniać poprzez `Cargo.toml`

> [!info] meme
> “Have you built your project in release mode?” is almost a meme in the Rust community.  
It refers to developers who are not familiar with Rust and complain about its performance on social media (e.g. Reddit, Twitter) before realizing they haven’t built their project in release mode.
Jeśli potrzebujesz czegoś bardziej granularnego w powyższej kwestii to masz do dyspozycji metody `wrapping_` oraz `saturating_` eg. `wrapping_add`

Konwersja typów a w zasadzie liczb poprzes `as`; `let b = a as u64;` oraz explicite syntax poprzez sufix dla literałów `47u16` lub czytelniej `47_u16`

Duży krok na przód to `struct` oraz implementowanie `impl` metod na struct, ==(!!!)==  zaskoczeniem okazało się, bo wydawało mi się że to nie jest możliwe, że konstruktor jest "static method" i zwraca zainicjowany struct. <mark style="background-color: #FF5582A6;">Coś tutaj zamieszałeś</mark>.
j
Mamy dwie metody wywoływania metod zaimplementowanych na struct **method call syntax** oraz **function call syntax** `ticket.is_open()` to samo co `Ticket::is_open(ticket)` tj. function call syntax explicite i nadmiarowo wskazuje na bazowy struct/type oraz przekazuje do self konkretny obiekt

> [!info] `|` vs. `||`
> Rule of thumb:
> - inside a pattern position (`match` arms, `matches!`, `if let`) use `|` for alternatives,
> - inside a boolen expression comparing values -> ues `||`

Porównywanie `String` do różnych możliwości:
- macro `matches!(s.as_str(), "A" | "B" | "C")`
- pattern matching: 
```rust
match s.as_str() {
	"A" | "B" | "C" => {},
	_ => panic!(),
}
```

> [!info] `.into()` vs. `.clone()`
> `clone()` - same type, new owned copy, does not consume
> `into()` - different type (ususally), no duplication requirement, consume
> Main purpose of `.into()` is *type conversion* and the fact that the target type is *inferred*.
> To jest banalny przykład ale niech Cie nie zmyli co do możliwości użycia `into()`
> ```rust
> let s: String = "hello".into() // target type inferred from let annotation
> ```
> next level kiedy przekazujesz paramentr fn. i właśnie za pomocą `.into()` robisz type conversion w locie ponieważ w fn. signature masz target type lub zwracasz coś z fn.

==(!!!)== Pytanie które pcha Cię do przodu: masz struct, implementujesz konstruktor w którym implemntujesz ograniczenia na parametry na podstawie których inicjujesz struct ale co powstrzymuje developera od inicjowania poprzez zwykły syntax `MyStruct{}` zamiast twojego konstruktora `MyStruct::new` ? ~~Właściwie nic, DLATEGO mamy "moduły".~~ nie tyle moduły co scope który tworzymy modułami oraz przedewszystkim public/private

> [!tip] Visualizing the module tree
> If you’re struggling to picture the module tree of your project, you can try using [`cargo-modules`](https://crates.io/crates/cargo-modules) to visualize it!

Moduły i nowe keywords: `mod, use, super` , konsekwencją modułów i submodułów jest widoczność funkcji, zmiennuch, struktur itp. co do zasady wszystkie na dzieńdobry są prywatne a dostęp do nich jest jedynie możliwy z wewnątrz tego samego modułu lub submodułu (widać rozwiązanie naszego pytania)

Znaczniki **Visibility mod** `pub, pub(crate), pub(super), pub(in path::to::module`

==(!!!)== zrobienie `pub` danej `MyStruct` nie powoduje z automatu że pola są też publiczne, nie są. dostęp do nich przez "public API" czyli jeślipoprzez publiczny konstruktor ustawiam color ale żeby z poza modułu wyświetlić color musze udostępnić **public getter** also called an **accessor method**. `pub(crate)` oraz `pub(super)` przydatne w bibliotekach do udostępniania pól bez udostępniania na zewnątrz.

> [!info] **invariant** (pl. niezmiennik)
> **niezmiennik** to zasada logiczna a prywatne pole to jedynie sposób implementacji, który pozwala tej reguły pilnować.

Rozwiązanie naszego problemu ma swoją fachową nazwę **enkapsulacja** jest to implementacja niezminników poprzez prywatne pola oraz zapewnienie **public API** które z jednej strony pozwolą na użytkowanie/kontakt z instancją a z drugiej strony zapewnią zachowanie niezmiennika. (np. saldo bankowe nie może spaść poniżej 0). Czyli minimum to implementacja **konstruktora** i **accessor methods**. 

> [!tip] `rustc --explain E0599`
> używaj wszędzie gdzie możesz, wszędzie gdzie się da

> [!tip] `getset` crate
> it auto-generates getters (and setters) via derive macros, eliminating repetitive boilerplate on structs with many fields.

> [!info] `self, &self, &mut self` 
> `&self` borrows the instance immutably — the method can read fields but not consume or mutate the struct, and the caller keeps ownership afterward. 
> `self` takes ownership — the method consumes the instance, and the caller can't use it again afterward (unless the type is `Copy`).
>There's also `&mut self`, which borrows mutably: it can modify fields but doesn't take ownership.
>```rust
>pub fn status(self) -> String {
>	self.status
>}
>	// vs.
>pub fn status(&self) -> &str {
>	&self.status
>}
>```
  
**setter** method możesz zaimplemntować co najmniej na dwa sposoby ale preferowanym jest ten który pozwoli Ci na **chain multiple calls**, minusem jest konsumowanie `self` i w konsekwencji ponowne przypisanie zmiennej ale dzieki **variable shadows** przypisujesz do tej samej zmiennej. <mark style="background-color: #D2B3FFA6;">To też pewnie sposób na implementacje default values dla struct.</mark>

pwyższe rozważania koncentrują się na wyborze pomiędzy `pub fn set_attr(mut self, ...) -> Self {}` vs. `pub fn set_attr(&mut self, ...) -> {}` czyli (a) konsumuje siebie mutując i zwraca siebie vs. mutuje przez referencje czyli nic nie zwraca.

==(!!!)== kolejny krok to wyodrębnienie logiki niezminnika do prywatnych funkcji `validate_param` DRY i używanie ich zarówno w konstruktorze jaki i **setter** methods `set_attr`

==(!!!)== duży krok/blok do przodu "Traits", tak na szybko to dla mne interface, protokół. A rozwiazuje problem gdy chce prównać/sprawdzić czy dwa struct są takie same, kompilator zgłasza błąd w postaci braku zaimplementowania `PartialEq` 

> [!info] Trait
> A trait defines a set of methods that a type must implement to satisfy the trait’s contract.

**Defining a trait** vs. **Implementing a trait** to pierwsze używ keyword `trait` a to drugie `impl <TraitName> for <TypeNAme> {}`, przykład to implementacja `MaybeZero` dla własnej ilościowej struktury/typu

> [!info] prelude
> The trait is defined in the standard library’s **prelude**. The prelude is a set of traits and types that are automatically imported into every Rust program. It’s as if `use std::prelude::*;` was added at the beginning of every Rust module.

Bardzo waży przykład żeby zrozumieć gdzie co się dzieje implicite i dlaczego działa i jaki mechanizm się kryje pod implicite, w tym przypadku **forwarding impls** for primitive numeric types, generated by an internal macro `forward_ref_binop!`

ponizej dwie implementacje Trait `IsEven` jedna używa `self` a druga `*self`, obydwie działają prawidłowo

```rust
impl IsEven for u32 {
	fn is_even(&self) -> bool {
	self % 2 == 0 // implicite
	}
}

impl IsEven for i32 {
	fn is_even(&self) -> bool {
	*self % 2 == 0 // explicite
	}
}
```

|                         | `self % 2`                                      | `*self % 2`                                           |
| ----------------------- | ----------------------------------------------- | ----------------------------------------------------- |
| Relies on               | std's ref-forwarding impls                      | nothing special                                       |
| Works for custom types? | Only if you implement `Rem` for `&YourType` too | Always works if `YourType: Copy` and implements `Rem` |
| Clarity                 | Slightly implicit                               | Explicit — "I'm working with the value"               |

For `u32`/`i32` specifically, it's a non-issue — pick whichever reads better to you. But **the habit worth building** is: for your own types, don't assume operator overloads work through references unless you've implemented them that way. `*self` is the safer default because it doesn't depend on whether someone bothered to add ref-forwarding impls.

==(!!!)== rozważania na temat `Copy`types, definicja `IsEven` poprzez move semantic też w tym przypadku będzie ok (definicja nie implementacja ponieważ implementacja musi być zgodna z definicją czyli nie moża w receiver zrobić `self` jeśli w definicji było `&self`) tj. `fn is_even(self) -> bool;` 

tabelka do zważań:

| Receiver    | Who decides it   | Typical use                                                                                                  |
| ----------- | ---------------- | ------------------------------------------------------------------------------------------------------------ |
| `&self`     | Trait definition | Read-only inspection — this is what almost all "predicate" methods (`is_even`, `is_empty`, `len`) should use |
| `&mut self` | Trait definition | Mutating in place                                                                                            |
| `self`      | Trait definition | Consuming builders, conversions (`into_inner`), or cheap `Copy` types where consuming is fine                |
> [!info] What "overload" means in general
> **Overloading** = giving one _name_ (a function name, or in this case an operator symbol) _multiple implementations_, and letting the compiler pick the right one based on the types involved.

**overload** to nie to samo co **override** to drugie pozwala na nadpisanie built-in implementacji

**defining trait** może także zawierać logikę, przykład `PartialEq`, patrz `ne`
```rust
pub const trait PartialEq<Rhs: PointeeSized = Self>: PointeeSized {
    /// Tests for `self` and `other` values to be equal, and is used by `==`.
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_diagnostic_item = "cmp_partialeq_eq"]
    fn eq(&self, other: &Rhs) -> bool;

    /// Tests for `!=`. The default implementation is almost always sufficient,
    /// and should not be overridden without very good reason.
    #[inline]
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_diagnostic_item = "cmp_partialeq_ne"]
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}
```

**override** external Trait, wciągnij do scope e.g. `use std::cmp::PartialEq;`

korzystaj ile się da z **Derive macros** e.g. `#[derive(PartialEq,Debug)]`

==(!!!)== Problem typu DRY: wiele typów np. numerycznych jaki jest sens pisania kilku tych samych funkcji z tym samym body a różniących się jedynie sygnaturą funkcji i nazwą funkcji. Rozwiązaniem różnych nazwy funkcji można walczyć poprzez implementacje Trait ale do nie rozwiązuje duplikacji kodu. 
Rozwiązanie to **generic programming** które pociąga za sobą **trait bounds**, syntax: `fn name<T>(n: T)` oraz keyword `where` lub **inline trait bound** `<T: IsEven + Debug>`

==(!!!)== **deref coercion** dalej tego nie rozumiem
> [!ifno] `String` implements `Deref`
> `String` implement `Deref` with `Target = str`
> Thanks to this implementation and deref coercion, a `&String` is automatically converted into a `&str` when needed.

==(???)== **autoref/autoderef** to jest jakieś szaleństwo, wywoływanie `str::trim(&self) -> &str` które jest zdefiniowane na `str` a nie na `String` powoduje że compilator wywołuje **method resolution algorithm** który robi `Deref` bo `String` implementuje `Defer` gdzie targetem jest `str` i jeszcze na dododatek na koniec robi **autoref** bo `.trim` przyjmuje `&self`, szaleństwo. To wszystko sprowadza się do `str::trim(&*self.title)`

**Dynamically sized types** (DST); **fat pointer** pointer na stosie który nie tylko ma adres ale też metadane takie jak długość w przypadku `&str`
**The `Sized` trait**

> [!info] cannot take `mem::size_of()` any DST

**infallible conversions** `From` and `Into`; **blanket impl**; jaki mental model do tego?

jprdl! **generic and associated types**

Co to jest `type Output;` i dlaczego `Self::Output`? Dlaczego zamiast generic type mamy `<Exponent = Self>`? I jaki problem adresuje?
```rust
trait Power<Exponent = Self> {
	type Output;
	fn power(self, n: Exponent) -> Self::Output;
}

// Dlaczego nie w ten sposób:
trait Power<Exponent = Self> {
	fn power(self, n: Exponent) -> Self; // hardcoded!
}
```
1. `<Exponent = Self>`  `Exponent` to jest **generic parameter** tak samo jak `<T>` reszt w nawiasie to default value, która pozwala na `impl` zapis typu:
```rust
impl Power for u32 {} // == impl Power<u32> for u32
```
chyba jest jeszcze jakiś niuans ale nie potrafie go wyłapać
2. `type Output;` jest to **associated type** i jedyne zastosowanie jakie wyłapałem to takie że przy implementacji `impl Power<u32> for u8` podnoszenie czegokolwiek do n-th potęgi na pewno zwróci wynik większy niż `u8::MAX` 255 czyli zwracanie `Self` w tym wypadku `u8` jest sporym ograniczeniem a wręcz bez sensu, dlatego:
```rust
impl Power<u32> for u8 {
	type Output = u64;  // <-- deliberately NOT Self (u8)!
	fn power(self, n: u32) -> u64 {
		(self as u64).pow(n)	
	}
}
```

**enum** zapewne będzie częścią `MyStruct` które będzie miało **derive** lub **impl** taits takie jak `Debug, PartialEq` najlepiej poprzez macro `#[derive()]`; warunkiem jest to że wyszytkie typy składące się na `MyStruct` muszą mieć też zaimplementowane te same Traits -> używaj **derive** na enum, dodawaj `Clone, Copy`

połączenie **enum** z **pattern matching** 

Pytanie o użycie `self` vs. `&self` a body metody (nie jako argument)
```rust
enum Shape {
	Circle,
	Square,
}

impl Shape {
	pub fn n_sides(&self) -> u8 {  // tutaj ok bo nie chce konsumować
		match self {  // <-- self vs. &self ???
			Shape::Cirlce => 0,
			Shape::Square => 4,	
		}	
	}
}
```
w tym przypadku `self: &Shape` i nie ma dużego znaczenia bo typ `Shape` nie prznosi danych ale miło by znaczenie w przypadku gdyby jedno z pól było `Polygon(u8, String)`

> [!info] match ergonomics
> chyba nie łapie całości
> [05_ticket_v2/03_variant_with_data](https://github.com/mainmatter/100-exercises-to-learn-rust/tree/main/exercises/05_ticket_v2/03_variants_with_data) na tym zadaniu zaczynam rozumieć, chodzi o **binding** jaki powstaje przy match na **variant-specific** jedno powoduje **move** a drugie tj. `&self` jest jedynie borrow.

**C-style enum** vs. **variants can hold data** chodzi o to że prosty enum ma tylko label tj. `Shape::Circle` lub `Shape::Square` ale Rust może więcej -> **variant-specific** i tutaj plot twist mozemy się dostać do tych wartości jedynie poprzez **pattern matchig**

==(!!!)== **match syntax** logic, tak też można:
```rust
	Status::ToDo | Status::Done => {}
```

==(!!!)== Problem: zależy mi tylko na jednym variant-specific więc po co opisywać resztę? tutaj można wpaść w pułpkę złego kodu omijając **exhaustiveness** pooprzez placeholder `_` 
Rozwiązaniem jest **`if let`** lub **`let/else`** 
1. pamiętej że `let` zamykamy ; to samo z wewnętrznym `panic!()`
2. syntax ma postać: *opis formy* = *wsad do formy/co wkłądamy*
3. binding (zmienna) zostaje wyciągnięta do "same indentation level as the code taht preceded it"

Dlaczego to nie działa i musi być explicite `return` ?
```rust
    pub fn assigned_to(&self) -> Option<&String> {
        let Status::InProgress { assigned_to } = &self.status else {
            Option::None  // <-- WHY NOT ?
            // return Option::None;
        };
        Some(assigned_to)
    }
```
`if let` jest **value-producing**:
```rust
    pub fn assigned_to(&self) -> Option<&String> {
        let result = if let Status::InProgress { assigned_to } = &self.status {
            Some(assigned_to)
        } else {
            None
        };
        result
    }
```

> [!faq] co to jest never type?

> [!info] value-producing arm vs. diverging arm
> o the real contrast isn't "if/else vs let-else" — it's **value-producing arm vs. diverging arm**. `let-else`'s `else` block is _only_ ever allowed to be the diverging kind , never the value-producing kind.

Pamiętj że `panic!()` to **never type** i jeśli refaktorujesz `panic!()` -> `Result<>` to musisz podmienić na diverging arm  a nie zostawić value-producing:
```rust
panic!("Error");

Err("Panic")  // ŹLE, to jest value-producing 

return Err("Panic")  // OK, to jest diverging arm
```

==(!!!)== Problem: obencnie nasz konstruktor podczas walidacji parametrów w przypadku nie spełnienia warunków brzegowch `panic!()` czyli nie daje szansy na obsłużenie błędu **handle the error** 
Rozwiązanie: `Result` type

> [!info] `Result` return type, force you to encode fallibility in the funcion's sugnature.

tylko patrząc na "fn signature" od razu widzisz że ta funckcja może failować

Masz już zwrócony `Result` typle co dalej? odpakuj-unsafe `.unwrap()` albo wzorcowo "destructure" poprzez pattern matching:
```rust
match fn_zwraca_Result("43") {
	OK(number) => {},
	Err(err) => {},
}
```

Gotcha: match konsumuje parametry konstruktora, gubisz pierwotny opis błędu
```rust
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title, description, status) { // parametry są konsumowane, move
        Ok(ticket) => ticket,
        Err(err) => {
            if err.contains("Description") {
                Ticket::new(title, "Description not provided".to_string(), status).unwrap()
		                //  ^^^-- tile zostało już skonsumowane przez match 
            } else {
                panic!("error") // Gubisz pierwotny opis błędu
            }
        }
    }
}
```
Rozwiązanie:
```rust
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(), description, status.clone()) { // <-- clone 
        Ok(ticket) => ticket,
        Err(err) => {
            if err.contains("Description") {
                Ticket::new(title, "Description not provided".to_string(), status).unwrap()
                // zobacz że tutaj bezpiecznie robisz unwrap()
            } else {
                panic!("{err}") // <-- banalne
            }
        }
    }
}
```

==(!!!)== Problem: operowanie na `String` jest dosyć toporne i łatwo o błędy
Rozwiazanie: **Error enums**

A gdyby tak nie musieć odpakowywać Error enum do String tylko móc użyć `panic!()` bezpośrednio na typie enum tj. `TicketNewError` -> `panic!("{err: TicketNewError}")`
```rust
        Err(TicketNewError::TitleErr(err: String)) => panic!("{err}"),
```
Yes You Can: implementując **Trait `Error`** na **error enum** oraz `Display`
```rust
impl std::fmt::Display for TicketNewError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TicketNewError::TitleError(msg) => write!(f, "{}", msg),
            TicketNewError::DescriptionError(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for TicketNewError {}
```

moje idiomatic `match Ok-Err` 
```rust
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(), description, status.clone()) {
        Ok(ticket) => ticket,
        Err(err) => match err {  // <-- nice
            TicketNewError::TitleError(_) => panic!("{err}"), // <-- wymaga Error + Debug + Display
            TicketNewError::DescriptionError(_) => {
                Ticket::new(title, "Description not provided".to_string(), status).unwrap()
            }
        },
    }
}
```

> [!info] `thiserror` crate
> We can remove some of the boilerplate by using [`thiserror`](https://docs.rs/thiserror/latest/thiserror/), a Rust crate that provides a **procedural macro** to simplify the creation of custom error types.


**`TryFrom`** and **`TryInto`** uffff too much


# GOTCHA

Dla kodu:
```rust
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
```
1. Dlaczego nie dereferece `i` ?  -> macro `println!()` używa `Display::fmt` które przeprowadza dereference

Dla kodu:
```rust
fn main() {
    println!("Hello, world!");
    
    let v = vec![100, 32, 57];
    let mut sum = 0;
    for i in &v {
        sum += i; // powinno być `*i` dereference czy nie ?
        println!("{sum}");
    }
    
}
```
1. Czy powinno być dereference? A jeśli tak to dlaczego działa bez? -> (???) ponieważ `i32` implementuje `AddAssign<&i32>` a nie dlatego że magicznie działa, czyli wniosek taki że powinno być ale tutaj nie musi bo **Trait impl**
---
1. Dlaczego to się nie skompiluje?
```rust
fn main() {
	let v = vec![String::from("Hello ")];
	let mut s = v[0];
	s.push_str("world");
	println!("{s}");
}
```
Non-copyable types cannot be moved out of a vector by indexing.
Only methods such as `Vec::remove` permit moving out a vector.

2. Co można zrobić aby to było możliwe?
```rust
fn main() {
  let mut v = vec![String::from("Hello ")];
  let s = &mut v[0];
  s.push_str("world");
  
  println!("{s}");    
}
```
- przekazać do `s` "mut ref"
- `v` musi być mutable
---
1. Dlaczego to się nie skompiluje?
```rust
fn main()
	let mut v = vec![1, 2, 3];
	for i in &mut v {
		v.push(*i);
	}
	println!("{} {} {}", v[3], v[4], v[5]);
```
Even though `v` is mutably borrowed, that only allows `i` to be mutated inside the for-loop, not `v`. Therefore calling `v.push` is an ownership error.

2. Czym jest `v2` pomyśl, tip: dlaczego nie deref i? `*i` , kolejna wskazówka to `let a = *v2[0];`
```rust
fn main() {
	let mut v: Vec<i32> = vec![1, 2, 3];
	let mut v2: Vec<&mut i32> = Vec::new();
	for i in &mut v {
		v2.push(i);	
	}
	*v2[0] = 5;
	
	let a = *v2[0];
	let b = v[0];
	println!("{a} {b}");
}
```
`i` has type `&mut i32`, meaning it is a pointer to a number within `v`. So if we push `i` into `v2`, then `v2` contains pointers to `v`. Therefore mutating `v2[0]` actually mutates `v[0]`.

---

1. co jest nie tak, fundamentalne rozumienie pattern matching z udziałem referencji ==Mental Model==
```rust
fn signature(numbers: &[i32], index: usize) -> Option<i32> {
	match numbers.get(index) {
		Some(value) => Some(value),
		None => None,
	}
}
```
1. w pierwszej kolejności co zwraca `.get()`? -> `Option<&T>` to że `Option` to jedno ale że `&T` referencja, nad tym warto się chwilę zastanowić -> alternatywą jest `T` ale to by oznaczało *move* co jest 1. bez sensu bo niby jak by miał funkcjonować dalej Vector z dziurą
2. teraz syntax match -> jeśli wiem że dostate `Some<&T>` a mam zwrócić `Option<T>` to capture: `Some(value)` "wkłada" do value `&T` referencje -> czyli w **tai arm** musze zrobić dereferencje `Some(*value)`
3. **idomatic rust** match syntax, rozważania powyżej tj `Some(value) => Some(*value)` można przenieść na lewą stronę `Some(&value) => Some(value)` ==(!!!)== ==Broken Mental Model== `&value` nie oznacza tutaj zrób referencje do `value` tylko: **destructure the reference in the pattern itself** , po lewej stronie jest opis formy (destructure)==Good Mental Model==, The `&` in the pattern matches against the `&` in the scrutinee's type `&i32` and *that reference gets stripped off during matching.* What's left to bind is the pointee `i32` itself. Since `i32: Copy`, this is a copy out of the slice, not a move (which would be illegat since you only have `&[i32]`, not ownership)
```rust
fn signature(numbers: &[i32], index: usize) -> Option<i32> {
	match numbers.get(index) {
		Some(&value) => Some(value), // <--
		None => None,
	}
}
```

Above one more time:
- Scrutinee type: `Option<&i32>`
- Pattern `Some(value)`: matches `Option<_>`, leaves `_` = `&i32` → `value: &i32`
- Pattern `Some(&value)`: matches `Option<&_>`, leaves `_` = `i32` → `value: i32`

---
^^^ -- next step:

But there's a subtlety worth knowing:
```rust
// If i32 were NOT Copy, this would fail to compile:
Some(&value) => Some(value), // ERROR: cannot move out of a shared reference
```

`Some(&value)` in pattern position, when the inner type isn't `Copy`, tries to _move_ the value out of a borrow — which the borrow checker rejects, since you don't own it. `*value` in an arm body has the identical restriction; both require `Copy` (or you'd need `.clone()` instead). So the two snippets aren't just cosmetically different — they're two views into the same ownership rule, and neither one lets you dodge it.

**Cleanest idiomatic version**, since `Option<&T>` → `Option<T>` for `Copy` types is exactly what `.copied()` exists for:
```rust
fn ex4_safe_get(numbers: &[i32], index: usize) -> Option<i32> {
    numbers.get(index).copied()
}
```

Worth a look next: `Option::cloned()` (the `Clone`-based analog for non-`Copy` types), and how this same `&value`-in-pattern mechanism shows up when iterating — e.g. `for &x in slice.iter()` vs `for x in slice.iter()`.

---
^^^ -- follow up:
Here's a snippet that demonstrates it — with the failing line commented out (since we want this to actually compile so you can run it), and the real compiler error shown alongside.

```rust
fn ex_box_slice(boxes: &[Box<i32>], index: usize) -> Option<Box<i32>> {
    match boxes.get(index) {
        // Some(&value) => Some(value),
        // ERROR: cannot move out of a shared reference
        //
        //   Some(&value) => Some(value),
        //        ^^^^^^ data moved here
        //               move occurs because `value` has type `Box<i32>`,
        //               which does not implement the `Copy` trait
        //
        // `boxes.get(index)` returns `Option<&Box<i32>>`.
        // The pattern `&value` tries to peel off the reference and BIND
        // `value` by value — i.e. move the Box<i32> out of the slice.
        // But `boxes` is `&[Box<i32>]`: we don't own the slice, only borrow it.
        // Moving a Box out would leave a "hole" in someone else's data —
        // exactly what the borrow checker exists to prevent.
        
        // Some(value) => Some(*value),
        // ERROR: cannot move out of `*value` whichc is behind a shared reference
        
        None => None,
        Some(value) => {
            // This is fine: `value` binds as `&Box<i32>` (no `&` in pattern,
            // so no attempted move — we just borrow the reference itself).
            todo!()
        }
    }
}

fn main() {
    let boxes: Vec<Box<i32>> = vec![Box::new(1), Box::new(2), Box::new(3)];

    // Option A — clone it out (Box<i32>: Clone since i32: Clone)
    let cloned: Option<Box<i32>> = boxes.get(0).cloned();
    println!("cloned: {:?}", cloned);

    // Option B — just copy/read the i32 inside, no need to move the Box at all
    let inner: Option<i32> = boxes.get(0).map(|b| **b);
    println!("inner value: {:?}", inner);

    // Option C — work with the borrow directly, never try to own it
    if let Some(b) = boxes.get(0) {
        println!("borrowed: {}", b); // Box<i32> derefs to i32 for Display
    }
}
```

> [!info] btw `.copied()` - converts an `Option<&T>` into an `Option<T>`
> dla `Option<&T>` gdzie `<T: Copy>` zamias destructora poprzez pattern matching możesz użyć `.copied()`

What's actually happening

`boxes.get(index)` returns `Option<&Box<i32>>` — a reference to a `Box`, not the `Box` itself. The pattern `Some(&value)` says "match a reference here, and bind `value` to what's _inside_ it." Following the same mechanism from our earlier discussion, that would make `value : Box<i32>` — but to produce that binding, the compiler would have to **move** the `Box<i32>` out from behind the `&Box<i32>` reference.

That's the violation. `boxes` is `&[Box<i32>]` — you don't own the slice's contents, you're borrowing them. If the compiler let you move the `Box` out, the original `Vec<Box<i32>>` would be left with an uninitialized slot at that index, while still believing it owns (and will eventually `Drop`) every element. Rust statically forbids "partial moves out of a shared reference" for exactly this reason — it cannot be made safe without runtime bookkeeping, which contradicts Rust's zero-cost philosophy.

Contrast with your original `i32` snippet: `Some(&value) => Some(value)` worked there because `i32: Copy`. `*value` (or the pattern's implicit deref) doesn't _move_ a `Copy` type — it duplicates it bit-for-bit, leaving the original in place. No hole, no problem. `Box<i32>` has no such escape hatch since it owns a heap allocation and implements `Drop`-adjacent semantics (technically `Box` doesn't hand-implement `Drop`, but it's treated specially by the compiler to deallocate on scope exit) — so the compiler must track it has exactly one owner at all times.

 Try it yourself

If you uncomment that line and run it, `rustc` will give you almost exactly the error shown in the comment — it's a great one to trigger once by hand so the message becomes recognizable on sight.

**Natural next step**: this is the exact motivating case for `Cow<'_, T>` and for understanding why iterator adapters like `.into_iter()` vs `.iter()` vs `.iter_mut()` exist as three separate things on `Vec<T>` — each is solving "how do I get _this_ kind of access to _owned_ elements without violating the single-owner invariant."

---
1. Dlczego zwracając `numbers` potrzbuje `&` ?
```rust
fn ex7_middle_slice(numbers: &[i32]) -> &[i32] {
    numbers[1..numbers.len() - 1] // ERROR 
}
```
- sprowadza się to do tego że *range* zmienia się w `Index` które zwraca **DST dynamically sized type** które opisujemy `[i32]` a ja potrzebuje **fat pointer** a nie pojedyńczego miejsca w pamięci, swoją drogą ciekawe jak na podstawie jednego miejsca w pamięci kompilator wie jaka powinna być długość żeby stworzyć **fat pointer**
- poniżej elaborat

it comes down to whta `Index` actually returns.

The core issue: `[i32]` is unsized
`numbers` has type `&[i32]`. When you write `numbers[1..len-1]`, you're invoking `Index<Range<usize>>` on the slice (via auto-deref). Look at what that impl returns:

```rust
impl<T> Index<Range<usize>> for [T] {
    type Output = [T];
    fn index(&self, range: Range<usize>) -> &[T] { ... }
}
```

The `Output` type is `[T]` — the _bare_, unsized slice type, not `&[T]`. But `numbers[1..len-1]` is sugar for `*Index::index(numbers, 1..len-1)` — Rust automatically dereferences the `&[T]` that `index()` actually returns, because indexing syntax is expected to hand you a place, not a pointer.

So `numbers[1..len-1]` on its own has type `[i32]` — a slice with no compile-time-known size. You _cannot_ return that by value; Rust needs to know the size of anything returned from a function (or stored on the stack), and `[i32]` could be 3 elements or 3 million.

Why `&` fixes it
```rust
fn ex7_middle_slice(numbers: &[i32]) -> &[i32] {
    &numbers[1..numbers.len() - 1]
}
```

By writing `&numbers[1..len-1]`, you re-reference the unsized place, turning it back into `&[i32]` — a _thin conceptually, but actually fat_ pointer (pointer + length) that's `Sized` (it's always 16 bytes on a 64-bit target: 8 for the data pointer, 8 for the length). That matches your return type exactly.

**The mental model**
Think of it as two separate steps happening in one expression:

1. `numbers[range]` → "give me the _place_ in memory that is this sub-slice" → type `[i32]` (unsized, can't exist as a standalone value)
2. `&(...)` → "give me a reference to that place" → type `&[i32]` (sized, a fat pointer)

This is exactly analogous to why `let s: String = ...; &s[0..3]` gives you `&str`, not `str` — indexing a `String` with a range also yields the unsized `str`, and you always need `&` to get something you can actually hold onto.

**Contrast with full-slice indexing**
You _don't_ need this dance when you're not slicing — e.g. `numbers[0]` returns `i32` by value (via `Index<usize>`, `Output = i32`), because `i32` is `Sized`. It's specifically range-indexing on slices/strings that produces an unsized `Output`, which is why the `&` becomes necessary there but not for single-element access.

---



# dalej tego nie rozumiem

## deref coercion
[[FAQ/deref coercion]]


# FAQ

## what is basic test mod template ?

## what exactly is "cheap copy?"
[[FAQ/cheap copy]]
follow up [[cheap copy#cheap copy is restricted to "lives on the stack"?]]

## basic usage of `dbg!()` macro
[[FAQ/dgb macro]]

## dlaczego `Copy` Trait wymaga `Clone`?

## podaj przykład dla którego lifetime elision fails
[[FAQ/elision fails]]
mental model: przyjmujesz więcej niż jeden parametr a zwracasz borrow value -> kompilator nie wie z którego imputu jest (borrow) return value -> dla naszego przykłądu oznacza to że borrow value powinno być powiazane z jedym i drugim inputem.

Jescze jedna uwaga, istotne jak na wejściu i wyjściu jest slice ale już dla liczb nie ma sensu bo to są Copy type.

## Why not `String + String` ?
[[FAQ/why not String + String]]

## `v.clone()` vs. `v.to_vec()`
[[FAQ/v.cone() vs. v.to_vec()]]

## `clone()` vs. `cloned()`
[[FAQ/clone vs. cloned]]

## explain unsized type `[i32]` vs. `&[i32]`
[[FAQ/unsized type]]
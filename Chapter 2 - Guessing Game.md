Important: `let mut guess = String::new()` _creates_ a new mutable variable `guess` and _binds_ it to the value of an empty `String`, that `String.new()` generates. In other words: `guess` is mutable, and its value _currently_ is a new _instance_ of `String`, namely, an empty string. The value will change later when the user submits their input (this comes from the guessing game example).

Question: so `String` is an immutable type? - No, `String` is a mutable type! Once again: the value must match the type of the variable, right? So if we create a mutable `guess`, it must be assigned a mutable String. Moreover, the type is inferred from the right-hand side, right? `guess` can't be anything else but a `String`.

Question: what happens if we do `let guess = String::new()`? What could happen:
- `guess` is immutable, anthough its... value is mutable? Nonsense
- or is it a compiler error because we assign a mutable value to an (by default) immutable variable?
The second option is correct:
> cannot mutate immutable variable `guess`

Next line:
```
io::stdin()
	.read_line(&mut guess)
	.expect("Failed to read line");
```

OK, it's clear why we pass `guess` to `read_line`, but why do we need `&mut`? It's a... reference to `guess`, sure, but why to we need to once again specify that `guess` is mutable, although it's already defined in its type? What happens if we do `.read_line(&guess)`? This is what happens:
> expected &mut String, found &String

Conclusion: `read_line` expects a mutable string, but `&` passes `guess` as an immutable string. So... does it mean that `guess` can change its type on the fly, from being mutable to being immutable? Or is it the _reference_ that changes the type, while the underlying _string_ remains... what? This is unclear!

Indeed, the next paragraph (page 17) explains:
> ...like variables, references are immutable by default

From which I conclude for now:
1. variables are immutable by default;
2. references are immutable by default;
3. reference mutability must match variable mutability (??? = this is likely untrue);
4. to modify a variable's value, both the variable itself **and** a reference to it must me mutable (i.e., explicitly marked with `mut`, as long as everything is immutable by default).

Then, `Result` in an enumeration with variants `Ok` and `Err`. Question: are variants types? Would it be correct to say that a normal string would be "wrapped" into an object of type `Ok`, and an error would be (wrapped in? simply be?) an object of type `Err`?

Meta-observation: what kinds of relationships can there be between type A and type B?
- A is the same type as B
- A is a sub-type of B (i.e., A is B but B is not A)
- A is a wrapper over B (i.e., B is not A, but I can access B "inside" A)
- what else? something something traits?

> Values of the `Result` type have methods defined on them, like and `expect` method

Types of crates: binary crate (an executable project) and a library crate (can't be executed on its own).

`Cargo.lock` lists versions of crates used: the highest version **that match the major version** specified in `Cargo.toml`. For example, if I specify version 0.8.5, then 0.8.6. will get downloaded when available (when running `cargo update`), but 0.9.0 will not. Semantic versioning: API breaks possible from 0.8.x to 0.9.0.

```
use rand::Rng;
let secret_number = rand::thread_rng().gen_range(1..=100);
```

Here we bring the namespace (?) `rand::Rng` into our file, and use `rand`... object?? Can we do this?
```
//use rand::Rng;
let secret_number = rand::Rng::rand::thread_rng().gen_range(1..=100);
```
Apparently no, and I don't understand why:
> trait objects must include the `dyn` keyword
> ambiguous associated type

Ok, so `Rng` is a _trait_ we bring into the scope... This doesn't explain much, but let's wait for later chapters on traits.

Open local documentation _compiled for my project locally_ from all crates I actually use:
```
cargo doc --open
```
Opens locally in the browser.

Pattern-matching:
```
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```

Question: why can't we just compare with `<>`? And pattern-match w.r.t. the boolean type. Or do comparisons return a boolean in Rust? Answer: we could do it, but we need to handle three cases, not two, so the code is less elegant:
```
if guess == secret_number {
	println!("You win!");
	break;
}

match guess < secret_number {
	true => println!("Too small!"),
	false => println!("Too big!"),
}
```

Type-conversion from `String` to `i32` (the default integer type):
```
let mut guess = String::new();
io::stdin()
.read_line(&mut guess)
.expect("Failed to read line");

let guess: u32 = guess
.trim()
.parse()
.expect("Please type a number!");
```

This illustrates _variable shadowing_. We define another variable `guess` of a different type, and the previous definition is shadowed... forever? See [[Chapter 3 - Common Concepts]]. Often used for type conversions to avoid `let guess: u32 = guess_str.trim()` etc.

Question: do we really need `trim()`, why? Because `5` (press enter) is internally `5 newline_char` or something (this is indeed the case, see end of page 25). Indeed, without trimming, the code... **crashes at runtime**:

> thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:22:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

Note: the above is what we _wanted_ to happen, this is how `Result`'s `Err` case is handled.

Then the question is: why didn't the compiler catch this? Probably because it's impossible to predict what garbage the user might submit... But anyway, isn't Rust philosophy about catching bugs at compile time? And this isn't a difficult bug to notice (like, parse expects a certain format, but input may take other values). Need some kind of filter? Note that I can cause panic even with trimming by submitting characters instead of digits.

Two use cases for pattern matching. In the first one, we assign the given value to `guess` or continue the loop if parsing failed (and assign nothing I guess):
```
let guess: u32 = match guess
	.trim()
	.parse() {
		Ok(num) => num,
		Err(_) => continue,
	};
```
In the second usage, we print different messages depending on the ordering, and optionally break the cycle. Note that we don't assign the result of match to anything (does `match` produce any value here at all? Probably not; depends on whether `println!` produces a value):
```
match guess.cmp(&secret_number) {
	Ordering::Less => println!("Too small!"),
	Ordering::Greater => println!("Too big!"),
	Ordering::Equal => {
		println!("You win!");
		break;
	}
}
```


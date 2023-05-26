Structure of a test:
- set up the state;
- run the function under test;
- assert the results are as expected.

> ...a test is a Rust function annotated with the `test` attribute.

A default function and a test created with `cargo new adder --lib` (`adder` could have been any library name):
```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

The tests module may contain non-test functions to set up scenarios, etc.

A test that always fails:
```
#[test]
fn another() {
	panic!("Make this test fail!");
}
```

Running just one test:
```
cargo test another
```

The example of a test for Rectangles says:
> Let's put this code in the `src/lib.rs` file, then write some tests

Question: does the code under test **have to** be in a library? Why not test functions in `main.rs`?

Question: I forgot how to separate code into main and library :( What do I add to `main.rs` such that it understands that the definition of the `Rectangle` is now in `lib.rs`? Referring to [[Chapter 7 - Modules]]...

Gray box on page 129:
> ...both crates (`src/main.rs` and `src/lib.rs`) will have the package name by default

The package name is `rectangles` (because we created it with `cargo new rectangles`), hence we add to `main.rs`:
```
use rectangles::Rectangle as Rectangle;
```
But that's not all: now we have to make `Rectangle` public, as well as function we use further in `main.rs`.

Oh, and we also could make `width` and `height` public to create new instances of `Rectangle` from `main.rs`, but isn't a better way to set up a public constructor `new` and keep concrete fields hidden? Let's try this. In `main`, we now do:
```
let rect1 = Rectangle::new(
	30 * scale,
	50,
);
```
And in `lib`, we do (`width` and `height` remain private):
```
impl Rectangle {
	// ...
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height,
        }
    }
}
```

Back to Chapter 11.

Define tests in a new `tests` module. Add `use super::**`. Why? Because:

> the `tests` module is a regular module that follows the usual visibility rules... we need to bring the code under test in the outer module into the scope of the inner module.

> ...the parameters to equality assertion functions... are called `left` and `right`, and the order doesn't matter

(Rust has no `expected` vs `actual` convention in tests.)

> The values being compared (with `assert_eq!` and `assert_ne!`) must implement the `PartialEq` and `Desub` traits.

> For custom types... this is usually as straightforward as adding the `#[derive(PartialEq, Debug)]` annotation

Indeed, if all primitive types have these traits, and my custom struct or enum consists of primitive types, their comparison and debug-printing functionality can simply be "inherited".

Adding a message to assertion:
```
assert!(result.contains("Carol"), "Greeting did not contain name, value was {result}");
```

Checking that the code should panic (any reason for panic, imprecise):
```
#[test]
#[should_panic]
fn greater_than_100() {
	// constructor checks that 0 < guess < 101
	Guess::new(200);
}
```

Additionally check that the panic message contains the given text:
```
#[should_panic(expected = "less than or equal to 100")]
```

Tests can return a Result instead of panicking:
```
#[test]
fn it_works() -> Result<(), String> {
	if 2 + 2 == 4 {
		Ok(())
	} else {
		Err(String::from("two plus two does not equal four"))
	}
}
```

Question: what are the pros and cons of tests that panic vs tests that return a Result? Answer: the Result-returning tests
> enable you to use the question mark operator in the body of the test

Memba `?` operator? See [[Chapter 9 - Errors]]:
```
File::open("hello.txt")?.read_to_string(&mut username)?;
```
This returns an `Err` on the first failed operation before the `?`, or returns `Ok` if everything goes through. Hence, in a test, we can use `?` to return the first error as it occurs. Question: how is this fundamentally different from panicking when the first error occurs? And why is "panicking" spelled with "ck" while "panic" isn't?

### Controlling how tests are run

Tests are run in parallel by default, therefore

> you must **make sure that your tests don't depend on each other** or on any share state, including a shared environment

Run tests sequentially:
```
cargo test -- --test-threads=1
```

(Note the `--` - a weird syntax...)

Show output (normally suppressed):
```
cargo test -- --show-output
```

Running one test:
```
cargo test test_name
```

Running only test whose name contain `foo`:
```
cargo test foo
```

Note: this is the same syntax. Is there no way to run test that contain `foo` or `bar`? Regex or something? Over-complication anyway.

Ignore a test:
```
#[test]
#[ignore]
fn expensive_test() {
	//...
}
```

Run only the ignored tests:
```
cargo run -- --ignored
```

Run all tests, including ignored:
```
cargo run -- --include-ignored
```

### Test organization

Unit test and integration tests.

Unit tests:

> you'll put unit tests in the `src` directory in each file with the code that they're testing

That means, there is no `foo.rs` and `foo_test.rs`, rather there is just `foo.rs` with all necessary tests inside.

> The convention is to **create a module named `tests` in each file** and to **annotate the module with `#[cfg(test)]`**.

There is a debate on whether private functions should be tested (as opposed to only testing public functions). Rust allows testing private functions. Indeed, if `tests` is a child module of the module it is testing, it can use all its ancestor's functions with `user super::*` (see [[Chapter 7 - Modules]]).

Integration tests:

> Integration tests are entirely external to your library and **use your code in the same way any other external code would**, using only the public interface and potentially exercising multiple modules per test.

> We create a `tests` directory **at the top level of our project directory, next to `src`**.

> We don't need to annotate any code in `tests/integration_test.rs` with `#[cfg(test)]`.

We can also separate common functionality for integration tests, like setup, into a `mod.rs` file (older naming conventions for modules); see scheme on page 241.

Integration tests for binary crates (`main.rs`) is arguably not needed:

> If the important functionality (in `lib.rs`) works, the small amount of code in the `src/main.rs` file will work as well, and ... doesn't need to be tested.

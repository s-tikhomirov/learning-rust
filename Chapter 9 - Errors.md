**Rust doesn't have exceptions.** Errors:
- recoverable - `Result`;
- unrecoverable - `panic!`.

### Panic
Includes unwinding, i.e., cleaning up the stack etc. May panic without cleaning up - that's called aborting (enabled separately). 

Panic happens for example in case of array out-of-bounds access by index.

### Result
Recoverable error: opening a file that doesn't exist:
```
use std::fs:File;
let greeting_file_result = File::open("hello.txt");
let greeting_file match greeting_file_result {
	Ok(File) => file,
	Err(error) => {
		panic!("Problem opening the file {:?}", error);
	}
}
```
So we _do_ panic in the end here too, but with a more explicit error message.

Matching by error type:
```
Err(error) => match error.kind() {
	ErrorKind::NotFound => {
		// create file
	}
	other_error => {
		panic!("...");
	}
}
```
Looks a lot like exception handling to me.

Alternative: with closures and `unwrap_or_else` (closures will be introduced in [[Chapter 13]]).

`unwrap`: unwrap the result if it's an `Ok`; panic if it's an `Err`.

`expect`: same but with a custom error message on panic:
```
let greeting_file = File::open("hello.txt").expect("hello.txt should be included");
```
Propagating errors:
```
let mut username_file = match username_file_result {
	Ok(file) => file,
	Err(e) => return Err(e),
}
```
Error propagation with `?` operator:
```
File::open("hello.txt")?.read_to_string(&mut username)?;
```
This only works if the return type of the function is **compatible**, that is, is an `Result` (may be an error) or `Option` (analogously but with `None` as the "bad case"). Can't "mix and match" `Result` and `Option`, but can convert manually.

For prototypes and tests it's ok to `panic!`, mostly via `unwrap` and `expect`. Also good to panic when some invariant is broken (_bad state_).

Implementing restricted types (this might be useful!) for the guessing game from [[Chapter 2 - Guessing Game]]:
```
pub struct Guess{
	value: i32,
};

impl Guess {
	pub fn new(value: i32) -> Guess {
		if value < 1 | value > 100 {
			panic!("Guess must be between 1 and 100, got {}", value)
		};
		Guess{value}
	}
	pub fn value(&self) -> i32 {
		self.value
	}
}
```
Important: we need the `value` function (a getter) because the value itself is (and must be) private, to force the caller to instantiate using `new()`, which enforces constraints.




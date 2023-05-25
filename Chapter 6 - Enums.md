Why trailing comma in enum definition? Not strictly needed but prevents error when shuffling fields around. Also, no unnecessary diffs when adding new fields (existing lines are not changed).

The first example:
```
enum IpAddrKind {
	V4,
	V6,
}
```
and then
```
let four = IpAddrKind::V4;
```
-- here we define a variable of type `IpAddrKind::V4` but don't initialize it yet? - Yes, becaure there is no way to store the actual address there yet.

A more usable example: storing strings inside enum variants:
```
enum IpAddr {
	V4(String),
	V6(String),
}
```
and initializing as:
```
let home = IpAddr::V4(String::from("127.0.0.1"));
```

> We attach data to each variant of the enum directly, so there is no need for an extra struct.

Question: however, if we want the data to be more complex, we would need to use some custom struct instead of `String`, right?

> the name of each enum also becomes a function that constructs an instance of that enum.

(a.k.a. constructor)

Implement associated functions for a enum similar to how we do it for a struct:
```
impl Message {
    fn call(&self) {
        // ...
    }
}
```

### Option
> Rust **doesn't have a `null` feature.

Instead, Rust has `Option` to handle cases where a value may be something or nothing.

```
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
```
Note that the following doesn't compile:
```
    let absent_number = None;
```
because the compiler needs to know **which type of None** the variable is (as above, it may be `Option<i32>` for example).

> ... you have to convert `Option<T>` to a `<T>` before you can perform `<T>` operations with it.

To unpack an `Option`, `match` is useful (though other methods exist).

Example with coin denominations:
```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Question: why take ownership of `coin`? This also works:
```
fn value_in_cents(coin: &Coin) {
	// ...
}
```

Question: why do we have to repeat `Coin::` in match case
s? The compiler already knows that `coin` is of type `Coin` from the function signature. Why can't we simply write:
```
match coin {
	Penny => 1,
	Nickel => 5,
	Dime => 10,
	Quarter => 25,
}
```
The compiler says:
```
error[E0170]: pattern binding `Penny` is named the same as one of the variants of the type `Coin`
  --> src/main.rs:12:9
   |
12 |         Penny => 1,
   |         ^^^^^ help: to match on the variant, qualify the path: `Coin::Penny`
   |
   = note: `#[deny(bindings_with_variant_name)]` on by default
```

Match must be exhaustive. Catch-all value placeholder is `_` (if the value is not used in the matching arm). If nothing is to be done, the arm is unit:
```
_ => ()
```

### `if let`
This is kinda like a match, but with an implicit catch-all in the end that does nothing. So we don't have to type `_ => ()`. Like this:
```
let config_max = Some(3u8);
if let Some(max) = config_max {
	println!("The maximum is condigured to be {max}");
}
```

Aaah, here the _pattern_ and the _arm_ are kinda swapped. We have a `config_max` variable that may be `None`, and we match it against the pattern `Some(max)`. With the full match, this would be:
```
match config_max {
	Some(max) => {
		// println!(...)
	}
	None => ()
}
```
...or would it? The Book:
> ...you lost the exhaustive checking that `match enforces`.

So... if `config_max` is `None`, then what happens? Simply nothing (no panic):
```
let config_max: Option<u8> = None;
if let Some(max) = config_max {
	println!("The maximum is condigured to be {max}");
}
```
So the problem is not that we might crash on `None` (this is cared for), but that we might not have handled all `Some`-variants. Which is sometimes OK, i guess? If we only need to do something for one variant, and for all others we do nothing.

By the way, do match arms require trailing comma? Answer: commas not required, see [SO](https://stackoverflow.com/questions/51662829/do-rust-match-statements-require-commas):
>As for what is idiomatic:
    If the right hand side ends with a closing brace }, omit the comma.
    If not: use a comma, even when it's the last match arm.

